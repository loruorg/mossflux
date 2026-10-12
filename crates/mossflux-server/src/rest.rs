//! REST API (JSON over HTTP) and the MCP endpoint at `/mcp`.
//!
//! | Method | Path | Body or query |
//! | --- | --- | --- |
//! | GET | `/health` | liveness |
//! | GET | `/ready` | 200 once every database is loaded, else 503 |
//! | GET | `/v1/databases` | |
//! | GET, POST | `/v1/similarity` | `smiles`, `threshold` or `k`, `limit`, `db` |
//! | GET, POST | `/v1/substructure` | `smarts`, `limit`, `offset`, `timeout_ms`, `db` |
//! | GET, POST | `/v1/molecule` | `smiles` |
//! | GET | `/v1/databases/{db}/molecules/{id}` | |
//! | POST | `/mcp` | MCP JSON-RPC (streamable HTTP, JSON responses) |

use std::sync::Arc;

use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;
use serde_json::json;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;

use crate::db::{MoleculeRequest, SimilarityRequest, SubstructureRequest};
use crate::{mcp, ApiError, Engine, ErrorKind};

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.kind {
            ErrorKind::BadRequest => StatusCode::BAD_REQUEST,
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            ErrorKind::Timeout => StatusCode::REQUEST_TIMEOUT,
            ErrorKind::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        };
        (status, Json(self)).into_response()
    }
}

type Reply<T> = Result<Json<T>, ApiError>;

fn body<T>(r: Result<Json<T>, JsonRejection>) -> Result<T, ApiError> {
    r.map(|Json(v)| v)
        .map_err(|e| ApiError::bad_request(e.body_text()))
}

/// The HTTP router. `cors_origins` empty allows any origin.
pub fn router(engine: Arc<Engine>, cors_origins: &[String]) -> Router {
    let origin = if cors_origins.is_empty() {
        AllowOrigin::any()
    } else {
        AllowOrigin::list(
            cors_origins
                .iter()
                .filter_map(|o| HeaderValue::from_str(o).ok()),
        )
    };
    let cors = CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .max_age(std::time::Duration::from_secs(86_400));

    Router::new()
        .route("/", get(index))
        .route("/health", get(|| async { "ok" }))
        .route("/ready", get(ready))
        .route("/v1/databases", get(databases))
        .route("/v1/databases/{db}/molecules/{id}", get(lookup))
        .route("/v1/similarity", get(similarity_get).post(similarity))
        .route("/v1/substructure", get(substructure_get).post(substructure))
        .route("/v1/molecule", get(molecule_get).post(molecule))
        .route("/mcp", post(mcp_http).get(mcp_get))
        .layer(RequestBodyLimitLayer::new(64 * 1024))
        .layer(cors)
        .with_state(engine)
}

async fn index() -> Json<serde_json::Value> {
    Json(json!({
        "name": "mossflux",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": ["/v1/databases", "/v1/similarity", "/v1/substructure", "/v1/molecule", "/mcp"],
        "docs": "https://github.com/zhom/mossflux/blob/main/docs/api.md"
    }))
}

async fn ready(State(e): State<Arc<Engine>>) -> Response {
    if e.ready() {
        (StatusCode::OK, "ready").into_response()
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "loading").into_response()
    }
}

#[derive(Serialize)]
struct Databases {
    databases: Vec<crate::DbInfo>,
}

async fn databases(State(e): State<Arc<Engine>>) -> Json<Databases> {
    Json(Databases {
        databases: e.databases(),
    })
}

async fn lookup(
    State(e): State<Arc<Engine>>,
    Path((db, id)): Path<(String, String)>,
) -> Reply<crate::db::Hit> {
    e.lookup(Some(db), id).await.map(Json)
}

async fn similarity(
    State(e): State<Arc<Engine>>,
    req: Result<Json<SimilarityRequest>, JsonRejection>,
) -> Reply<crate::db::SimilarityResponse> {
    e.similarity(body(req)?).await.map(Json)
}

async fn similarity_get(
    State(e): State<Arc<Engine>>,
    Query(req): Query<SimilarityRequest>,
) -> Reply<crate::db::SimilarityResponse> {
    e.similarity(req).await.map(Json)
}

async fn substructure(
    State(e): State<Arc<Engine>>,
    req: Result<Json<SubstructureRequest>, JsonRejection>,
) -> Reply<crate::db::SubstructureResponse> {
    e.substructure(body(req)?).await.map(Json)
}

async fn substructure_get(
    State(e): State<Arc<Engine>>,
    Query(req): Query<SubstructureRequest>,
) -> Reply<crate::db::SubstructureResponse> {
    e.substructure(req).await.map(Json)
}

async fn molecule(
    State(e): State<Arc<Engine>>,
    req: Result<Json<MoleculeRequest>, JsonRejection>,
) -> Reply<crate::db::MoleculeResponse> {
    e.molecule(&body(req)?).map(Json)
}

async fn molecule_get(
    State(e): State<Arc<Engine>>,
    Query(req): Query<MoleculeRequest>,
) -> Reply<crate::db::MoleculeResponse> {
    e.molecule(&req).map(Json)
}

async fn mcp_http(State(e): State<Arc<Engine>>, body: String) -> Response {
    match mcp::handle_message(&e, &body).await {
        Some(reply) => (
            [(axum::http::header::CONTENT_TYPE, "application/json")],
            reply,
        )
            .into_response(),
        None => StatusCode::ACCEPTED.into_response(),
    }
}

/// This server sends no server-initiated messages, so there is no SSE stream.
async fn mcp_get() -> StatusCode {
    StatusCode::METHOD_NOT_ALLOWED
}
