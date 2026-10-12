//! In-memory fingerprint similarity search.
//!
//! [`Arena`] stores fingerprints sorted by popcount with a per-popcount bin index,
//! and runs exact BitBound search (Swamidass & Baldi, 2007) over it. The bound uses
//! integer rational arithmetic, so no floating-point rounding can drop a true hit.
//! [`SegmentedIndex`] adds incremental growth: each appended batch becomes its own
//! sorted segment, and a query searches all segments with the same hit set as one
//! arena. Load an [`Arena`] from an FPS file with [`fps::load_fps`].

#![forbid(unsafe_code)]

mod arena;
mod column;
pub mod fps;
mod inverted;
mod search;
mod segmented;

pub use arena::{Arena, MAX_NBITS};
pub use column::ColumnBitmap;
pub use inverted::{BitmapScratch, InvertedBitmap, InvertedIndex, InvertedScratch};
pub use mossflux_core;
pub use search::{bitbound_bins, threshold_to_rational, Hit};
pub use segmented::{GlobalHit, SegmentedIndex};
