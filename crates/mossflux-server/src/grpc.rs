//! gRPC service `mossflux.v1.Search` (see `proto/mossflux.proto`).

use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::{db, ApiError, Engine, ErrorKind};

/// Generated protobuf types and service stubs.
#[allow(missing_docs, clippy::all, clippy::pedantic)]
pub mod pb {
    tonic::include_proto!("mossflux.v1");
}

use pb::search_server::{Search, SearchServer};

impl From<ApiError> for Status {
    fn from(e: ApiError) -> Status {
        match e.kind {
            ErrorKind::BadRequest => Status::invalid_argument(e.error),
            ErrorKind::NotFound => Status::not_found(e.error),
            ErrorKind::Timeout => Status::deadline_exceeded(e.error),
            ErrorKind::Unavailable => Status::unavailable(e.error),
        }
    }
}

struct Service(Arc<Engine>);

fn opt<T: Default + PartialEq>(v: T) -> Option<T> {
    (v != T::default()).then_some(v)
}

#[tonic::async_trait]
impl Search for Service {
    async fn list_databases(
        &self,
        _: Request<pb::ListDatabasesRequest>,
    ) -> Result<Response<pb::ListDatabasesResponse>, Status> {
        let databases = self
            .0
            .databases()
            .into_iter()
            .map(|d| pb::Database {
                name: d.name,
                molecules: d.molecules,
                fingerprint_kind: d.fingerprint.kind.to_string(),
                fingerprint_bits: d.fingerprint.bits,
                skipped: d.skipped,
                build_ms: d.build_ms,
            })
            .collect();
        Ok(Response::new(pb::ListDatabasesResponse { databases }))
    }

    async fn similarity(
        &self,
        req: Request<pb::SimilarityRequest>,
    ) -> Result<Response<pb::SimilarityResponse>, Status> {
        let r = req.into_inner();
        let res = self
            .0
            .similarity(db::SimilarityRequest {
                db: opt(r.db),
                smiles: r.smiles,
                threshold: opt(r.threshold),
                k: opt(r.k as usize),
                limit: opt(r.limit as usize),
            })
            .await?;
        Ok(Response::new(pb::SimilarityResponse {
            hits: res
                .hits
                .into_iter()
                .map(|h| pb::SimilarityHit {
                    id: h.id,
                    smiles: h.smiles,
                    score: h.score,
                })
                .collect(),
            count: res.count,
            took_us: res.took_us,
            scanned: res.scanned,
            total: res.total,
        }))
    }

    async fn substructure(
        &self,
        req: Request<pb::SubstructureRequest>,
    ) -> Result<Response<pb::SubstructureResponse>, Status> {
        let r = req.into_inner();
        let res = self
            .0
            .substructure(db::SubstructureRequest {
                db: opt(r.db),
                smarts: r.smarts,
                limit: opt(r.limit as usize),
                offset: opt(r.offset as usize),
                timeout_ms: opt(u64::from(r.timeout_ms)),
            })
            .await?;
        Ok(Response::new(pb::SubstructureResponse {
            hits: res
                .hits
                .into_iter()
                .map(|h| pb::Hit {
                    id: h.id,
                    smiles: h.smiles,
                })
                .collect(),
            count: res.count,
            took_us: res.took_us,
            screened: res.screened,
            total: res.total,
        }))
    }

    async fn molecule(
        &self,
        req: Request<pb::MoleculeRequest>,
    ) -> Result<Response<pb::MoleculeResponse>, Status> {
        let m = self.0.molecule(&db::MoleculeRequest {
            smiles: req.into_inner().smiles,
        })?;
        Ok(Response::new(pb::MoleculeResponse {
            formula: m.formula,
            weight: m.weight,
            atoms: m.atoms,
            heavy_atoms: m.heavy_atoms,
            bonds: m.bonds,
            rings: m.rings,
            aromatic_atoms: m.aromatic_atoms,
            fingerprint_bits_set: m.fingerprint_bits_set,
        }))
    }
}

/// The gRPC service over `engine`.
#[must_use]
pub fn service(engine: Arc<Engine>) -> SearchServer<impl Search> {
    SearchServer::new(Service(engine))
}
