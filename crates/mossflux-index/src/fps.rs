//! Loader for chemfp [FPS files](https://chemfp.com/fps_format/) (`.fps` or
//! `.fps.gz`).
//!
//! An FPS file is UTF-8 text: `#key=value` header lines (notably
//! `#num_bits=<N>`) followed by one record per line, `<hex fingerprint>\t<id>`.
//! Hex is decoded two characters per byte, left to right; byte `i` carries
//! global bits `8*i .. 8*i+7`. Bytes are packed little-endian into `u64` lanes,
//! the bit ordering used everywhere in `mossflux`.

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use std::time::Instant;

use flate2::read::MultiGzDecoder;
use mossflux_core::lanes_for_bits;

use crate::arena::{Arena, MAX_NBITS};

/// Summary of a load.
#[non_exhaustive]
pub struct LoadStats {
    /// Number of fingerprint records read.
    pub n: usize,
    /// Fingerprint width in bits (from `#num_bits=`, or inferred from record 1).
    pub nbits: usize,
    /// Records with no bit set. They are loaded, but have Tanimoto 0 with every
    /// query, so they match only at `t <= 0`.
    pub zero_popcount: usize,
    /// Wall-clock time spent parsing and packing the file.
    pub elapsed: std::time::Duration,
}

/// The longest line the loader accepts, in bytes. The widest fingerprint is
/// 16,368 hex characters, so this leaves room for an id while bounding memory.
pub const MAX_LINE_BYTES: usize = 1 << 20;

const HEX: [u8; 256] = build_hex_lut();

const fn build_hex_lut() -> [u8; 256] {
    let mut lut = [0xffu8; 256];
    let mut c = 0u8;
    while c < 10 {
        lut[(b'0' + c) as usize] = c;
        c += 1;
    }
    c = 0;
    while c < 6 {
        lut[(b'a' + c) as usize] = 10 + c;
        lut[(b'A' + c) as usize] = 10 + c;
        c += 1;
    }
    lut
}

/// An `InvalidData` error that names the 1-based line it came from.
fn bad_line(lineno: usize, msg: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, format!("line {lineno}: {msg}"))
}

/// Decode one hex fingerprint, appending exactly `nwords` packed lanes to `out`.
///
/// The caller guarantees `hex.len() == 2 * ceil(nbits/8) <= nwords*8` bytes.
/// Spare high bits beyond `nbits` are masked off.
fn decode_hex_into(hex: &[u8], nbits: usize, nwords: usize, out: &mut Vec<u64>) -> Result<(), ()> {
    let start = out.len();
    out.resize(start + nwords, 0);
    let nbytes = hex.len() / 2;
    for bi in 0..nbytes {
        let hi = HEX[hex[bi * 2] as usize];
        let lo = HEX[hex[bi * 2 + 1] as usize];
        if hi == 0xff || lo == 0xff {
            return Err(());
        }
        let byte = (hi << 4) | lo;
        out[start + bi / 8] |= (byte as u64) << ((bi % 8) * 8);
    }
    if !nbits.is_multiple_of(64) {
        let last = start + nwords - 1;
        out[last] &= (1u64 << (nbits % 64)) - 1;
    }
    Ok(())
}

/// Parse a `#num_bits=` value: a positive width an [`Arena`] can hold.
fn parse_num_bits(rest: &[u8], lineno: usize) -> io::Result<usize> {
    let n: usize = std::str::from_utf8(rest)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .ok_or_else(|| bad_line(lineno, "bad #num_bits header"))?;
    if n == 0 {
        return Err(bad_line(
            lineno,
            "#num_bits=0: a fingerprint needs at least one bit",
        ));
    }
    // MAX_NBITS is a whole number of lanes, so this is the arena's lane limit.
    if n > MAX_NBITS {
        return Err(bad_line(
            lineno,
            format!("#num_bits={n} exceeds the widest supported fingerprint ({MAX_NBITS} bits)"),
        ));
    }
    Ok(n)
}

/// Load an FPS file into an [`Arena`]. A gzip file is detected by its magic
/// bytes (`1f 8b`), not by its name.
///
/// The file must be well formed; anything else is an `InvalidData` error that
/// names the line, never a silent repair:
///
/// * Header lines (`#...`) come before the first record. A header after a record
///   would change the meaning of rows already read.
/// * `#num_bits=` is positive and at most 65,472. Without it, the first record
///   sets the width. Every record has exactly that width.
/// * A `#type=` that names a count encoding is rejected: only binary
///   fingerprints have a binary Tanimoto.
/// * A record is `<hex>\t<id>`, optionally followed by more TAB-separated fields,
///   which are ignored. The id is valid UTF-8 (two different byte strings must
///   never load as the same id).
/// * No line is longer than [`MAX_LINE_BYTES`].
///
/// A leading UTF-8 byte-order mark and CRLF line ends are accepted.
pub fn load_fps(path: &Path) -> io::Result<(Arena, LoadStats)> {
    let (arena, _order, stats) = load_fps_with_order(path)?;
    Ok((arena, stats))
}

/// Like [`load_fps`], plus the popcount-sort permutation of
/// [`Arena::build_with_order`]: `order[row]` is the 0-based record number (in
/// file order, headers and blank lines not counted) stored at arena `row`.
pub fn load_fps_with_order(path: &Path) -> io::Result<(Arena, Vec<u32>, LoadStats)> {
    let started = Instant::now();
    let mut file = BufReader::with_capacity(1 << 20, File::open(path)?);
    let is_gz = file.fill_buf()?.starts_with(&[0x1f, 0x8b]);
    let reader: Box<dyn Read> = if is_gz {
        Box::new(MultiGzDecoder::new(file))
    } else {
        Box::new(file)
    };
    let mut reader = BufReader::with_capacity(1 << 20, reader);

    let mut nbits: usize = 0;
    let mut nwords: usize = 0;
    let mut flat: Vec<u64> = Vec::new();
    let mut ids: Vec<String> = Vec::new();

    let mut line: Vec<u8> = Vec::with_capacity(1024);
    let mut lineno = 0usize;
    loop {
        line.clear();
        lineno += 1;
        let read = (&mut reader)
            .take(MAX_LINE_BYTES as u64 + 1)
            .read_until(b'\n', &mut line)?;
        if read == 0 {
            break;
        }
        if line.len() > MAX_LINE_BYTES && line.last() != Some(&b'\n') {
            return Err(bad_line(
                lineno,
                format!("longer than {MAX_LINE_BYTES} bytes"),
            ));
        }
        if lineno == 1 && line.starts_with(b"\xEF\xBB\xBF") {
            line.drain(..3);
        }
        // Trim trailing \n and \r.
        while matches!(line.last(), Some(b'\n') | Some(b'\r')) {
            line.pop();
        }
        if line.is_empty() {
            continue;
        }
        if line[0] == b'#' {
            if !ids.is_empty() {
                return Err(bad_line(
                    lineno,
                    "header line after the first record (headers must come first)",
                ));
            }
            if let Some(rest) = line.strip_prefix(b"#num_bits=") {
                nbits = parse_num_bits(rest, lineno)?;
                nwords = lanes_for_bits(nbits);
            } else if let Some(rest) = line.strip_prefix(b"#type=") {
                // Count fingerprints store integer counts, not bits, so binary
                // Tanimoto over them is meaningless. Match raw bytes so a
                // non-UTF-8 byte cannot hide the word.
                if rest.to_ascii_lowercase().windows(5).any(|w| w == b"count") {
                    return Err(bad_line(
                        lineno,
                        "count fingerprints are unsupported (binary Tanimoto only); \
                         #type header declares a count encoding",
                    ));
                }
            }
            continue;
        }
        let tab = line
            .iter()
            .position(|&b| b == b'\t')
            .ok_or_else(|| bad_line(lineno, "record line missing TAB"))?;
        let (hex, rest) = line.split_at(tab);
        // The id ends at the next TAB; later fields are ignored.
        let rest = &rest[1..];
        let id = &rest[..rest.iter().position(|&b| b == b'\t').unwrap_or(rest.len())];
        if hex.is_empty() || hex.len() % 2 != 0 {
            return Err(bad_line(
                lineno,
                "record has empty or odd-length fingerprint hex",
            ));
        }
        if nwords == 0 {
            // No #num_bits header: infer width from the first record.
            nbits = (hex.len() / 2) * 8;
            if nbits > MAX_NBITS {
                return Err(bad_line(
                    lineno,
                    format!(
                        "{nbits}-bit fingerprint exceeds the widest supported ({MAX_NBITS} bits)"
                    ),
                ));
            }
            nwords = lanes_for_bits(nbits);
        }
        // Every record must have exactly the file's width.
        let nbytes_expected = nbits.div_ceil(8);
        if hex.len() != 2 * nbytes_expected {
            return Err(bad_line(
                lineno,
                format!(
                    "fingerprint is {} hex characters; the file's width of {nbits} bits needs {}",
                    hex.len(),
                    2 * nbytes_expected
                ),
            ));
        }
        decode_hex_into(hex, nbits, nwords, &mut flat)
            .map_err(|()| bad_line(lineno, "invalid hex digit in fingerprint"))?;
        let id = std::str::from_utf8(id).map_err(|_| bad_line(lineno, "id is not valid UTF-8"))?;
        ids.push(id.to_owned());
        if ids.len() > u32::MAX as usize {
            return Err(bad_line(
                lineno,
                "more than 2^32 records; an arena holds at most u32::MAX rows",
            ));
        }
    }

    if ids.is_empty() || nbits == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "no fingerprint records found",
        ));
    }

    let n = ids.len();
    let (arena, order) = Arena::build_with_order(nbits, flat, ids);
    // Rows with popcount 0 occupy the first bin.
    let zero_popcount = arena.popcount_offsets()[1] as usize;
    Ok((
        arena,
        order,
        LoadStats {
            n,
            nbits,
            zero_popcount,
            elapsed: started.elapsed(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_SEQ: AtomicU64 = AtomicU64::new(0);

    /// A scratch file on a path unique to this process and call, deleted on drop
    /// (also on panic), so concurrent test runs cannot race on one path.
    struct TempFps(std::path::PathBuf);

    impl TempFps {
        fn new(tag: &str, contents: &str) -> Self {
            let seq = TEMP_SEQ.fetch_add(1, Ordering::Relaxed);
            let mut p = std::env::temp_dir();
            p.push(format!(
                "mossflux_fps_{tag}_{}_{seq}.fps",
                std::process::id()
            ));
            File::create(&p)
                .unwrap()
                .write_all(contents.as_bytes())
                .unwrap();
            Self(p)
        }
        fn path(&self) -> &std::path::Path {
            &self.0
        }
    }

    impl Drop for TempFps {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    #[test]
    fn masks_spare_bits_on_non_64_multiple_width() {
        // 12-bit width (2 bytes). "ffff" sets bits 0..15, but bits 12..15 are
        // spare and must be masked -> popcount 12 (not 16).
        let f = TempFps::new("12bit", "#num_bits=12\nffff\tA\n0f00\tB\n");
        let (arena, stats) = load_fps(f.path()).unwrap();
        assert_eq!(stats.nbits, 12);
        let mut pops: Vec<u16> = (0..arena.len()).map(|i| arena.popcount_of(i)).collect();
        pops.sort();
        assert_eq!(pops, vec![4, 12], "spare bits 12..15 must be masked off");
    }

    #[test]
    fn rejects_ragged_width() {
        let f = TempFps::new("ragged", "#num_bits=16\nffff\tA\nff\tB\n");
        assert!(
            load_fps(f.path()).is_err(),
            "a short record must be rejected, not zero-padded"
        );
    }

    #[test]
    fn rejects_empty_hex() {
        let f = TempFps::new("empty", "#num_bits=16\n\tA\n");
        assert!(load_fps(f.path()).is_err());
    }

    #[test]
    fn rejects_count_fingerprints() {
        // Count fingerprints have no binary Tanimoto; the file must be rejected.
        let f = TempFps::new(
            "count",
            "#num_bits=16\n#type=RDKit-Morgan-count/1 radius=2 fpSize=16\nffff\tA\n",
        );
        let err = load_fps(f.path())
            .err()
            .expect("count fps must be rejected");
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }

    /// A binary `#type` header (no "count") loads, with the right bits.
    #[test]
    fn accepts_binary_morgan_type_header() {
        // Hex "0f31" over 16 bits: byte 0 = 0x0f -> bits 0..3, byte 1 = 0x31 ->
        // bits 8, 12, 13. Byte i carries global bits 8i..8i+7, packed
        // little-endian, so the single lane must be exactly 0x310f.
        let f = TempFps::new(
            "binary_type",
            "#num_bits=16\n#type=RDKit-Morgan/1 radius=2 fpSize=16\n0f31\tmol-A\n8000\tmol-B\n",
        );
        let (arena, stats) = load_fps(f.path()).expect("binary #type must load");
        assert_eq!(stats.nbits, 16, "declared width must be honoured");
        assert_eq!(stats.n, 2, "both records must load");
        assert_eq!(arena.len(), 2);
        assert_eq!(arena.nbits(), 16);
        // Rows are popcount-sorted, so "8000" (popcount 1) comes first.
        assert_eq!(arena.row(0), &[0x0080u64][..], "bit 15 only");
        assert_eq!(arena.popcount_of(0), 1);
        assert_eq!(arena.id(0), "mol-B");
        assert_eq!(
            arena.row(1),
            &[0x310fu64][..],
            "hex 0f31 must pack to 0x310f"
        );
        assert_eq!(arena.popcount_of(1), 7);
        assert_eq!(arena.id(1), "mol-A");
    }

    /// The error for `contents`, which must fail to load.
    fn load_err(tag: &str, contents: &[u8]) -> String {
        let seq = TEMP_SEQ.fetch_add(1, Ordering::Relaxed);
        let mut p = std::env::temp_dir();
        p.push(format!(
            "mossflux_fps_{tag}_{}_{seq}.fps",
            std::process::id()
        ));
        File::create(&p).unwrap().write_all(contents).unwrap();
        let r = load_fps(&p);
        let _ = std::fs::remove_file(&p);
        match r {
            Ok(_) => panic!("{tag}: a hostile file loaded"),
            Err(e) => {
                assert_eq!(e.kind(), io::ErrorKind::InvalidData, "{tag}: {e}");
                e.to_string()
            }
        }
    }

    /// Hostile files are `InvalidData` errors that name the line.
    #[test]
    fn hostile_files_are_errors_with_line_numbers() {
        for (tag, contents, want) in [
            // Count encoding hidden behind a non-UTF-8 byte in the #type header.
            (
                "count_bypass",
                &b"#num_bits=16\n#type=RDKit-Morgan-count\xff/1\nffff\tA\n"[..],
                "line 2: count fingerprints are unsupported",
            ),
            // A header after a record would change the width of rows already read.
            (
                "late_header",
                b"ff\tA\n#num_bits=16\nffff\tB\n",
                "line 2: header line after",
            ),
            (
                "late_narrow",
                b"ffff\tA\n#num_bits=8\nff\tB\n",
                "line 2: header line after",
            ),
            ("zero_bits", b"#num_bits=0\nff\tA\n", "line 1: #num_bits=0"),
            (
                "too_wide",
                b"#num_bits=70000\n",
                "line 1: #num_bits=70000 exceeds",
            ),
            (
                "bad_hex",
                b"#num_bits=16\nfg00\tA\n",
                "line 2: invalid hex digit",
            ),
            (
                "ragged",
                b"#num_bits=16\nffff\tA\nff\tB\n",
                "line 3: fingerprint is 2 hex",
            ),
            (
                "no_tab",
                b"#num_bits=16\nffff\n",
                "line 2: record line missing TAB",
            ),
            (
                "bad_utf8_id",
                b"#num_bits=16\nffff\t\xff\n",
                "line 2: id is not valid UTF-8",
            ),
            ("empty", b"", "no fingerprint records found"),
        ] {
            let err = load_err(tag, contents);
            assert!(err.contains(want), "{tag}: want {want:?}, got {err:?}");
        }
        // Inferred from an over-wide first record, with no header at all.
        let wide = format!("{}\tA\n", "f".repeat(2 * 8192 + 2));
        assert!(load_err("inferred_wide", wide.as_bytes()).contains("exceeds the widest"));
        // One endless line is refused before it can take unbounded memory.
        let long = vec![b'f'; MAX_LINE_BYTES + 10];
        assert!(load_err("long_line", &long).contains("longer than"));
    }

    /// Accepted variants load the right rows: extra TAB fields (the id ends at the
    /// next TAB), a UTF-8 byte-order mark, CRLF line ends, and gzip data under a
    /// plain `.fps` name.
    #[test]
    fn tolerated_variants_load_the_right_rows() {
        let f = TempFps::new(
            "extra_fields",
            "\u{feff}#num_bits=16\r\n0f00\tA\textra\tmore\r\n0000\tZ\r\n",
        );
        let (arena, stats) = load_fps(f.path()).unwrap();
        assert_eq!((stats.n, stats.nbits, stats.zero_popcount), (2, 16, 1));
        assert_eq!(arena.id(1), "A", "the id ends at the next TAB");
        assert_eq!(arena.id(0), "Z");

        let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        gz.write_all(b"#num_bits=16\n0f00\tA\n").unwrap();
        let bytes = gz.finish().unwrap();
        let seq = TEMP_SEQ.fetch_add(1, Ordering::Relaxed);
        let mut p = std::env::temp_dir();
        p.push(format!("mossflux_fps_gz_{}_{seq}.fps", std::process::id()));
        File::create(&p).unwrap().write_all(&bytes).unwrap();
        let r = load_fps(&p);
        let _ = std::fs::remove_file(&p);
        let (arena, _) = r.expect("gzip is detected by content, not by name");
        assert_eq!((arena.len(), arena.id(0)), (1, "A".to_string()));
    }

    /// `load_fps_with_order` maps arena rows back to file records.
    #[test]
    fn order_maps_rows_to_file_records() {
        let f = TempFps::new("order", "#num_bits=16\nffff\tR0\n0100\tR1\n0f00\tR2\n");
        let (arena, order, _) = load_fps_with_order(f.path()).unwrap();
        for (row, &rec) in order.iter().enumerate() {
            assert_eq!(arena.id(row), format!("R{rec}"));
        }
    }
}
