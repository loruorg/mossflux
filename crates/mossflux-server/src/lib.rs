//! REST, gRPC and MCP server for mossflux.
//!
//! One [`Engine`] holds the databases and the request limits; [`rest`], [`grpc`]
//! and [`mcp`] are thin transports over it. Every search runs on a blocking
//! thread behind a concurrency limit, and every substructure search has a
//! deadline.

#![forbid(unsafe_code)]

use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use serde::Serialize;
use tokio::sync::Semaphore;

pub mod db;
pub mod grpc;
pub mod input;
pub mod mcp;
pub mod rest;

pub use db::{Database, DbInfo, DbSpec};

/// Request limits.
#[derive(Clone, Debug)]
pub struct Limits {
    /// Hits returned when a request gives no `limit`.
    pub default_hits: usize,
    /// Largest `limit` and `k` accepted.
    pub max_hits: usize,
    /// Substructure deadline when a request gives none.
    pub default_timeout: Duration,
    /// Largest substructure deadline accepted.
    pub max_timeout: Duration,
    /// Searches run at once; more wait in line.
    pub max_concurrent: usize,
    /// Longest SMILES or SMARTS accepted, in bytes.
    pub max_query_len: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            default_hits: 100,
            max_hits: 10_000,
            default_timeout: Duration::from_secs(10),
            max_timeout: Duration::from_secs(60),
            max_concurrent: 4,
            max_query_len: 4096,
        }
    }
}

/// Error class, mapped to an HTTP status, a gRPC code and an MCP error.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    /// The request is malformed.
    BadRequest,
    /// No such database or molecule.
    NotFound,
    /// The search passed its deadline.
    Timeout,
    /// The databases are still loading.
    Unavailable,
}

/// A request error.
#[derive(Clone, Debug, Serialize)]
pub struct ApiError {
    /// Error class.
    #[serde(skip)]
    pub kind: ErrorKind,
    /// Message for the client.
    pub error: String,
}

impl ApiError {
    pub(crate) fn bad_request(msg: impl Into<String>) -> Self {
        ApiError {
            kind: ErrorKind::BadRequest,
            error: msg.into(),
        }
    }
    pub(crate) fn not_found(msg: impl Into<String>) -> Self {
        ApiError {
            kind: ErrorKind::NotFound,
            error: msg.into(),
        }
    }
    pub(crate) fn timeout() -> Self {
        ApiError {
            kind: ErrorKind::Timeout,
            error: "timeout".into(),
        }
    }
    pub(crate) fn unavailable() -> Self {
        ApiError {
            kind: ErrorKind::Unavailable,
            error: "databases are loading".into(),
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.error)
    }
}

impl std::error::Error for ApiError {}

/// Result of an engine call.
pub type ApiResult<T> = Result<T, ApiError>;

/// The databases plus the limits, shared by every transport.
pub struct Engine {
    specs: Vec<DbSpec>,
    dbs: RwLock<Vec<Arc<Database>>>,
    limits: Limits,
    permits: Semaphore,
}

impl Engine {
    /// An engine whose databases load later with [`load_all`](Self::load_all).
    #[must_use]
    pub fn new(specs: Vec<DbSpec>, limits: Limits) -> Arc<Engine> {
        Arc::new(Engine {
            specs,
            dbs: RwLock::new(Vec::new()),
            permits: Semaphore::new(limits.max_concurrent.max(1)),
            limits,
        })
    }

    /// An engine over databases that are already built.
    #[must_use]
    pub fn with_databases(dbs: Vec<Database>, limits: Limits) -> Arc<Engine> {
        let engine = Engine::new(Vec::new(), limits);
        *engine.dbs.write().unwrap() = dbs.into_iter().map(Arc::new).collect();
        engine
    }

    /// Loads every database spec in order. Blocking.
    ///
    /// # Errors
    /// The first database that fails to load.
    pub fn load_all(&self) -> std::io::Result<()> {
        for spec in &self.specs {
            tracing::info!(db = %spec.name, path = %spec.path.display(), "loading");
            let db = Database::load(spec).map_err(|e| {
                std::io::Error::new(e.kind(), format!("{}: {e}", spec.path.display()))
            })?;
            self.dbs.write().unwrap().push(Arc::new(db));
        }
        Ok(())
    }

    /// True when every database is loaded.
    #[must_use]
    pub fn ready(&self) -> bool {
        let n = self.dbs.read().unwrap().len();
        n > 0 && n >= self.specs.len()
    }

    /// The request limits.
    #[must_use]
    pub fn limits(&self) -> &Limits {
        &self.limits
    }

    /// Summaries of the loaded databases.
    #[must_use]
    pub fn databases(&self) -> Vec<DbInfo> {
        self.dbs.read().unwrap().iter().map(|d| d.info()).collect()
    }

    fn db(&self, name: Option<&str>) -> ApiResult<Arc<Database>> {
        if !self.ready() {
            return Err(ApiError::unavailable());
        }
        let dbs = self.dbs.read().unwrap();
        match name.filter(|n| !n.is_empty()) {
            None => Ok(dbs[0].clone()),
            Some(n) => dbs
                .iter()
                .find(|d| d.name() == n)
                .cloned()
                .ok_or_else(|| ApiError::not_found(format!("no database named {n:?}"))),
        }
    }

    fn check_query(&self, what: &str, q: &str) -> ApiResult<()> {
        if q.trim().is_empty() {
            return Err(ApiError::bad_request(format!("{what} is empty")));
        }
        if q.len() > self.limits.max_query_len {
            return Err(ApiError::bad_request(format!(
                "{what} is longer than {} bytes",
                self.limits.max_query_len
            )));
        }
        Ok(())
    }

    fn hits(&self, limit: Option<usize>) -> ApiResult<usize> {
        let n = limit.unwrap_or(self.limits.default_hits);
        if n > self.limits.max_hits {
            return Err(ApiError::bad_request(format!(
                "limit is above {}",
                self.limits.max_hits
            )));
        }
        Ok(n)
    }

    async fn blocking<T: Send + 'static>(
        &self,
        f: impl FnOnce() -> ApiResult<T> + Send + 'static,
    ) -> ApiResult<T> {
        let _permit = self
            .permits
            .acquire()
            .await
            .expect("semaphore is never closed");
        tokio::task::spawn_blocking(f)
            .await
            .unwrap_or_else(|e| std::panic::resume_unwind(e.into_panic()))
    }

    /// Exact Tanimoto search.
    ///
    /// # Errors
    /// Bad input, an unknown database, or databases still loading.
    pub async fn similarity(
        &self,
        req: db::SimilarityRequest,
    ) -> ApiResult<db::SimilarityResponse> {
        self.check_query("smiles", &req.smiles)?;
        let db = self.db(req.db.as_deref())?;
        let limit = self.hits(req.limit)?;
        if let Some(k) = req.k {
            if k == 0 || k > self.limits.max_hits {
                return Err(ApiError::bad_request(format!(
                    "k must be in 1..={}",
                    self.limits.max_hits
                )));
            }
        }
        let limit = match req.k {
            Some(k) => req.limit.map_or(k, |l| l.min(k)),
            None => limit,
        };
        self.blocking(move || db.similarity(&req.smiles, req.threshold, req.k, limit))
            .await
    }

    /// Exact SMARTS search.
    ///
    /// # Errors
    /// Bad input, an unknown database, a timeout, or databases still loading.
    pub async fn substructure(
        &self,
        req: db::SubstructureRequest,
    ) -> ApiResult<db::SubstructureResponse> {
        self.check_query("smarts", &req.smarts)?;
        let db = self.db(req.db.as_deref())?;
        let limit = self.hits(req.limit)?;
        let timeout = req
            .timeout_ms
            .map_or(self.limits.default_timeout, Duration::from_millis)
            .min(self.limits.max_timeout);
        let offset = req.offset.unwrap_or(0);
        self.blocking(move || db.substructure(&req.smarts, offset, limit, timeout))
            .await
    }

    /// Properties of one molecule.
    ///
    /// # Errors
    /// A bad SMILES.
    pub fn molecule(&self, req: &db::MoleculeRequest) -> ApiResult<db::MoleculeResponse> {
        self.check_query("smiles", &req.smiles)?;
        db::molecule(&req.smiles)
    }

    /// One stored molecule by id.
    ///
    /// # Errors
    /// An unknown database or id.
    pub async fn lookup(&self, db: Option<String>, id: String) -> ApiResult<db::Hit> {
        let db = self.db(db.as_deref())?;
        self.blocking(move || {
            let row = db
                .find_id(&id)
                .ok_or_else(|| ApiError::not_found(format!("no molecule {id:?}")))?;
            let (id, smiles) = db.molecule(row);
            Ok(db::Hit {
                id: id.to_string(),
                smiles: smiles.to_string(),
            })
        })
        .await
    }
}

/// Listen addresses.
#[derive(Clone, Debug)]
pub struct Listen {
    /// REST and MCP (`/mcp`).
    pub http: SocketAddr,
    /// gRPC; `None` disables it.
    pub grpc: Option<SocketAddr>,
    /// Allowed CORS origins; empty allows any.
    pub cors_origins: Vec<String>,
}

/// Serves HTTP (REST + MCP) and gRPC until Ctrl-C or SIGTERM. Databases load
/// in the background; `/ready` answers 503 until they are in.
///
/// # Errors
/// A bind failure, or a database that fails to load.
pub async fn serve(engine: Arc<Engine>, listen: Listen) -> std::io::Result<()> {
    let http = tokio::net::TcpListener::bind(listen.http).await?;
    let loader = {
        let engine = engine.clone();
        tokio::task::spawn_blocking(move || engine.load_all())
    };
    tracing::info!(addr = %listen.http, "http listening (REST, /mcp)");
    let app = rest::router(engine.clone(), &listen.cors_origins);
    let http = axum::serve(http, app).with_graceful_shutdown(shutdown());

    let grpc = async {
        match listen.grpc {
            Some(addr) => {
                tracing::info!(%addr, "grpc listening");
                tonic::transport::Server::builder()
                    .add_service(grpc::service(engine.clone()))
                    .serve_with_shutdown(addr, shutdown())
                    .await
                    .map_err(std::io::Error::other)
            }
            None => Ok(()),
        }
    };

    let load = async {
        loader.await.map_err(std::io::Error::other)??;
        tracing::info!("ready");
        Ok::<_, std::io::Error>(())
    };

    let servers = async {
        let (h, g) = tokio::join!(http, grpc);
        h?;
        g
    };
    tokio::pin!(servers);
    tokio::select! {
        r = &mut servers => r,
        r = load => {
            r?;
            servers.await
        }
    }
}

async fn shutdown() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let term = async {
        if let Ok(mut s) = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        {
            s.recv().await;
        }
    };
    #[cfg(not(unix))]
    let term = std::future::pending::<()>();
    tokio::select! {
        () = ctrl_c => {},
        () = term => {},
    }
}
