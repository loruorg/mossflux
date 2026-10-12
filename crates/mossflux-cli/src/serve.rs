//! `mossflux serve` (REST + MCP over HTTP, gRPC) and `mossflux mcp` (MCP over
//! stdio). Every flag also reads an environment variable, for containers.

use std::net::SocketAddr;
use std::time::Duration;

use mossflux_server::input::Columns;
use mossflux_server::{mcp, DbSpec, Engine, Limits, Listen};

use crate::CliError;

/// Databases and limits shared by `serve` and `mcp`.
#[derive(clap::Args)]
pub struct EngineArgs {
    /// SMILES file to index, as `PATH` or `NAME=PATH` (`.gz` allowed). Repeat
    /// for several databases; the first is the default.
    #[arg(
        short = 'd',
        long = "db",
        env = "MOSSFLUX_DB",
        value_delimiter = ',',
        required = true
    )]
    pub db: Vec<String>,
    /// 1-based SMILES column. Default: auto.
    #[arg(long, env = "MOSSFLUX_SMILES_COL")]
    pub smiles_col: Option<usize>,
    /// 1-based id column; 0 uses the line number. Default: auto.
    #[arg(long, env = "MOSSFLUX_ID_COL")]
    pub id_col: Option<usize>,
    /// Read at most this many rows per database.
    #[arg(long, env = "MOSSFLUX_MAX_MOLS")]
    pub max_mols: Option<usize>,
    /// Worker threads for index build and search (0 = all cores).
    #[arg(long, env = "MOSSFLUX_THREADS", default_value_t = 0)]
    pub threads: usize,
    /// Searches that run at once; more wait in line.
    #[arg(long, env = "MOSSFLUX_MAX_CONCURRENT", default_value_t = 4)]
    pub max_concurrent: usize,
    /// Largest `limit` or `k` a request may ask for.
    #[arg(long, env = "MOSSFLUX_MAX_HITS", default_value_t = 10_000)]
    pub max_hits: usize,
    /// Substructure deadline in seconds when a request gives none.
    #[arg(long, env = "MOSSFLUX_TIMEOUT", default_value_t = 10.0)]
    pub timeout: f64,
    /// Largest substructure deadline in seconds a request may ask for.
    #[arg(long, env = "MOSSFLUX_MAX_TIMEOUT", default_value_t = 60.0)]
    pub max_timeout: f64,
}

/// Arguments for `mossflux serve`.
#[derive(clap::Args)]
pub struct ServeArgs {
    #[command(flatten)]
    pub engine: EngineArgs,
    /// HTTP address for REST and MCP (`/mcp`).
    #[arg(long, env = "MOSSFLUX_LISTEN", default_value = "0.0.0.0:8080")]
    pub listen: SocketAddr,
    /// gRPC address, or `off`.
    #[arg(long, env = "MOSSFLUX_GRPC", default_value = "0.0.0.0:50051")]
    pub grpc: String,
    /// Allowed CORS origin. Repeat or comma-separate. Default: any origin.
    #[arg(
        long = "cors-origin",
        env = "MOSSFLUX_CORS_ORIGINS",
        value_delimiter = ','
    )]
    pub cors_origins: Vec<String>,
}

fn secs(s: f64, flag: &str) -> Result<Duration, CliError> {
    Duration::try_from_secs_f64(s)
        .ok()
        .filter(|d| !d.is_zero())
        .ok_or_else(|| CliError::from(format!("{flag} must be a positive number of seconds")))
}

fn engine(a: &EngineArgs) -> Result<std::sync::Arc<Engine>, CliError> {
    if a.threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(a.threads)
            .build_global()
            .map_err(|e| format!("starting the thread pool: {e}"))?;
    }
    let columns = Columns {
        smiles: a.smiles_col,
        id: a.id_col,
    };
    let mut specs: Vec<DbSpec> = Vec::new();
    for arg in &a.db {
        let mut spec = DbSpec::parse(arg);
        spec.columns = columns;
        spec.max_mols = a.max_mols;
        if specs.iter().any(|s| s.name == spec.name) {
            return Err(format!("two databases are named {:?}", spec.name).into());
        }
        if !spec.path.exists() {
            return Err(format!("{}: no such file", spec.path.display()).into());
        }
        specs.push(spec);
    }
    let limits = Limits {
        max_hits: a.max_hits,
        max_concurrent: a.max_concurrent,
        default_timeout: secs(a.timeout, "--timeout")?,
        max_timeout: secs(a.max_timeout, "--max-timeout")?,
        ..Limits::default()
    };
    Ok(Engine::new(specs, limits))
}

fn logging() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stderr()))
        .init();
}

fn runtime() -> Result<tokio::runtime::Runtime, CliError> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("starting the runtime: {e}").into())
}

/// Entry point for `mossflux serve`.
pub fn run_serve(a: ServeArgs) -> Result<(), CliError> {
    logging();
    let engine = engine(&a.engine)?;
    let grpc = match a.grpc.as_str() {
        "off" | "" => None,
        s => Some(
            s.parse()
                .map_err(|e| format!("--grpc {s:?}: {e} (use HOST:PORT or off)"))?,
        ),
    };
    let listen = Listen {
        http: a.listen,
        grpc,
        cors_origins: a.cors_origins,
    };
    runtime()?
        .block_on(mossflux_server::serve(engine, listen))
        .map_err(|e| format!("{e}").into())
}

/// Entry point for `mossflux mcp`.
pub fn run_mcp(a: EngineArgs) -> Result<(), CliError> {
    logging();
    let engine = engine(&a)?;
    runtime()?
        .block_on(mcp::serve_stdio(engine))
        .map_err(|e| format!("{e}").into())
}
