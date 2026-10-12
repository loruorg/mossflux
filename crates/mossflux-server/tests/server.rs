//! End-to-end tests: answers equal brute force, and every transport works.

use std::sync::Arc;
use std::time::Duration;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use mossflux_server::grpc::pb;
use mossflux_server::{Database, Engine, Limits};
use mossflux_substruct::fingerprint::ecfp4;
use mossflux_substruct::{brute_matches, parse_smarts, parse_smiles};
use serde_json::{json, Value};
use tower::ServiceExt;

const CORPUS: &[&str] = &[
    "CC(=O)Oc1ccccc1C(=O)O",
    "Oc1ccccc1C(=O)O",
    "COc1ccccc1C(=O)O",
    "CN1C=NC2=C1C(=O)N(C(=O)N2C)C",
    "CC(C)Cc1ccc(cc1)C(C)C(=O)O",
    "c1ccc2[nH]ccc2c1",
    "NS(=O)(=O)c1ccc(N)cc1",
    "CCO",
    "CCN(CC)CC",
    "c1ccncc1",
    "C1CCCCC1",
    "O=C(O)CCC(=O)O",
    "Clc1ccc(Cl)cc1",
    "[Na+].[Cl-]",
    "not a smiles",
    "CC(=O)Nc1ccc(O)cc1",
    "c1ccc(cc1)-c1ccccc1",
    "OC[C@H]1OC(O)[C@H](O)[C@@H](O)[C@@H]1O",
];

fn database() -> Database {
    let rows = CORPUS
        .iter()
        .enumerate()
        .map(|(i, s)| (format!("M{i}"), (*s).to_string()));
    let (db, skipped) = Database::build("test", rows);
    assert_eq!(skipped, vec![14]);
    db
}

fn engine() -> Arc<Engine> {
    Engine::with_databases(vec![database()], Limits::default())
}

fn tanimoto(a: &[u64], b: &[u64]) -> f64 {
    let i: u32 = a.iter().zip(b).map(|(x, y)| (x & y).count_ones()).sum();
    let u: u32 = a.iter().zip(b).map(|(x, y)| (x | y).count_ones()).sum();
    if u == 0 {
        0.0
    } else {
        f64::from(i) / f64::from(u)
    }
}

fn kept() -> Vec<(String, &'static str)> {
    CORPUS
        .iter()
        .enumerate()
        .filter(|(_, s)| parse_smiles(s).is_ok())
        .map(|(i, s)| (format!("M{i}"), *s))
        .collect()
}

#[test]
fn substructure_equals_brute_force() {
    let db = database();
    for q in [
        "c1ccccc1",
        "C(=O)[OH]",
        "S(=O)(=O)N",
        "[#7]",
        "[R]",
        "C",
        "[Cl]",
        "c1ccc2[nH]ccc2c1",
        "[$(CO)]",
        "*~*",
    ] {
        let query = parse_smarts(q).unwrap();
        let mut want: Vec<String> = kept()
            .into_iter()
            .filter(|(_, s)| brute_matches(&query, &parse_smiles(s).unwrap()))
            .map(|(id, _)| id)
            .collect();
        want.sort();
        let r = db
            .substructure(q, 0, 1000, Duration::from_secs(10))
            .unwrap();
        let mut got: Vec<String> = r.hits.into_iter().map(|h| h.id).collect();
        got.sort();
        assert_eq!(got, want, "query {q}");
        assert_eq!(r.count as usize, want.len());
        assert!(r.screened >= r.count);
    }
}

#[test]
fn similarity_equals_brute_force() {
    let db = database();
    for q in [
        "CC(=O)Oc1ccccc1C(=O)O",
        "c1ccccc1",
        "CCO",
        "CC(=O)Nc1ccc(O)cc1",
    ] {
        let qfp = ecfp4(&parse_smiles(q).unwrap());
        for t in [0.1, 0.3, 0.5, 0.7, 1.0] {
            let mut want: Vec<(String, f64)> = kept()
                .into_iter()
                .map(|(id, s)| (id, tanimoto(&qfp, &ecfp4(&parse_smiles(s).unwrap()))))
                .filter(|(_, sc)| *sc >= t)
                .collect();
            want.sort_by(|a, b| a.0.cmp(&b.0));
            let r = db.similarity(q, Some(t), None, 1000).unwrap();
            let mut got: Vec<(String, f64)> = r.hits.into_iter().map(|h| (h.id, h.score)).collect();
            got.sort_by(|a, b| a.0.cmp(&b.0));
            assert_eq!(got, want, "query {q} t {t}");
        }
        let knn = db.similarity(q, None, Some(3), 3).unwrap();
        assert_eq!(knn.hits.len(), 3);
        assert!(knn.hits.windows(2).all(|w| w[0].score >= w[1].score));
    }
}

async fn call(
    engine: &Arc<Engine>,
    method: &str,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let app = mossflux_server::rest::router(engine.clone(), &[]);
    let mut req = Request::builder().method(method).uri(uri);
    let body = match body {
        Some(b) => {
            req = req.header("content-type", "application/json");
            Body::from(b.to_string())
        }
        None => Body::empty(),
    };
    let res = app.oneshot(req.body(body).unwrap()).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let v = serde_json::from_slice(&bytes)
        .unwrap_or(Value::String(String::from_utf8_lossy(&bytes).into()));
    (status, v)
}

#[tokio::test]
async fn rest_endpoints() {
    let e = engine();
    let (s, v) = call(&e, "GET", "/v1/databases", None).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(v["databases"][0]["molecules"], 17);

    let (s, v) = call(
        &e,
        "POST",
        "/v1/similarity",
        Some(json!({"smiles": "CC(=O)Oc1ccccc1C(=O)O", "k": 2})),
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(v["hits"][0]["id"], "M0");
    assert_eq!(v["hits"][0]["score"], 1.0);
    assert_eq!(v["hits"].as_array().unwrap().len(), 2);

    let (s, v) = call(&e, "GET", "/v1/substructure?smarts=c1ccccc1&limit=1", None).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(v["hits"].as_array().unwrap().len(), 1);
    assert!(v["count"].as_u64().unwrap() > 1);

    let (s, v) = call(
        &e,
        "POST",
        "/v1/substructure",
        Some(json!({"smarts": "C(("})),
    )
    .await;
    assert_eq!(s, StatusCode::BAD_REQUEST);
    assert!(v["error"].as_str().unwrap().contains("SMARTS"));

    let (s, _) = call(
        &e,
        "POST",
        "/v1/similarity",
        Some(json!({"smiles": "CCO", "threshold": 1.5})),
    )
    .await;
    assert_eq!(s, StatusCode::BAD_REQUEST);

    let (s, _) = call(
        &e,
        "POST",
        "/v1/similarity",
        Some(json!({"smiles": "CCO", "db": "nope"})),
    )
    .await;
    assert_eq!(s, StatusCode::NOT_FOUND);

    let (s, v) = call(
        &e,
        "GET",
        "/v1/molecule?smiles=CC(%3DO)Oc1ccccc1C(%3DO)O",
        None,
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(v["formula"], "C9H8O4");

    let (s, v) = call(&e, "GET", "/v1/databases/test/molecules/M7", None).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(v["smiles"], "CCO");

    let (s, _) = call(&e, "GET", "/ready", None).await;
    assert_eq!(s, StatusCode::OK);
}

#[tokio::test]
async fn not_ready_until_loaded() {
    let e = Engine::new(
        vec![mossflux_server::DbSpec::parse("x=/nonexistent")],
        Limits::default(),
    );
    let (s, _) = call(&e, "GET", "/ready", None).await;
    assert_eq!(s, StatusCode::SERVICE_UNAVAILABLE);
    let (s, _) = call(&e, "GET", "/v1/substructure?smarts=C", None).await;
    assert_eq!(s, StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn mcp_tools() {
    let e = engine();
    let init = mossflux_server::mcp::handle_message(
        &e,
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18"}}"#,
    )
    .await
    .unwrap();
    let v: Value = serde_json::from_str(&init).unwrap();
    assert_eq!(v["result"]["protocolVersion"], "2025-06-18");

    let note = mossflux_server::mcp::handle_message(
        &e,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
    )
    .await;
    assert!(note.is_none());

    let list = mossflux_server::mcp::handle_message(
        &e,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
    )
    .await
    .unwrap();
    let v: Value = serde_json::from_str(&list).unwrap();
    assert_eq!(v["result"]["tools"].as_array().unwrap().len(), 5);

    let (s, v) = call(
        &e,
        "POST",
        "/mcp",
        Some(json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"substructure_search","arguments":{"smarts":"c1ccncc1"}}})),
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(v["result"]["isError"], false);
    assert_eq!(v["result"]["structuredContent"]["count"], 1);

    let (_, v) = call(
        &e,
        "POST",
        "/mcp",
        Some(json!({"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"similarity_search","arguments":{"smiles":"(("}}})),
    )
    .await;
    assert_eq!(v["result"]["isError"], true);
}

#[tokio::test]
async fn grpc_roundtrip() {
    let e = engine();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(mossflux_server::grpc::service(e))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener)),
    );
    let mut client = pb::search_client::SearchClient::connect(format!("http://{addr}"))
        .await
        .unwrap();
    let dbs = client
        .list_databases(pb::ListDatabasesRequest {})
        .await
        .unwrap()
        .into_inner();
    assert_eq!(dbs.databases[0].molecules, 17);
    let r = client
        .substructure(pb::SubstructureRequest {
            smarts: "S(=O)(=O)N".into(),
            ..Default::default()
        })
        .await
        .unwrap()
        .into_inner();
    assert_eq!(r.count, 1);
    assert_eq!(r.hits[0].id, "M6");
    let r = client
        .similarity(pb::SimilarityRequest {
            smiles: "CCO".into(),
            k: 1,
            ..Default::default()
        })
        .await
        .unwrap()
        .into_inner();
    assert_eq!(r.hits[0].id, "M7");
    let err = client
        .substructure(pb::SubstructureRequest {
            smarts: "[".into(),
            ..Default::default()
        })
        .await
        .unwrap_err();
    assert_eq!(err.code(), tonic::Code::InvalidArgument);
    let m = client
        .molecule(pb::MoleculeRequest { smiles: "O".into() })
        .await
        .unwrap()
        .into_inner();
    assert_eq!(m.formula, "H2O");
}
