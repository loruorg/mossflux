//! Model Context Protocol server: JSON-RPC 2.0 over stdio (`mossflux mcp`) or
//! over HTTP (`POST /mcp`). Exposes the searches as tools.

use std::sync::Arc;

use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::db::{MoleculeRequest, SimilarityRequest, SubstructureRequest};
use crate::Engine;

const PROTOCOL_VERSIONS: [&str; 3] = ["2025-06-18", "2025-03-26", "2024-11-05"];

const INSTRUCTIONS: &str = "Exact molecular search. similarity_search finds molecules by \
Tanimoto similarity to a SMILES; substructure_search finds every molecule that contains a \
SMARTS pattern. Counts are exact; hit lists are capped by limit.";

fn tools() -> Value {
    let db =
        json!({"type": "string", "description": "Database name. Default: the first database."});
    json!([
        {
            "name": "list_databases",
            "description": "List the loaded databases with molecule counts.",
            "inputSchema": {"type": "object", "properties": {}}
        },
        {
            "name": "similarity_search",
            "description": "Exact Tanimoto search (ECFP4, 2048 bits). Give threshold for every molecule at or above it, or k for the k nearest.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "smiles": {"type": "string", "description": "Query molecule as SMILES."},
                    "threshold": {"type": "number", "minimum": 0, "maximum": 1, "description": "Tanimoto threshold. Default 0.7."},
                    "k": {"type": "integer", "minimum": 1, "description": "Return the k nearest molecules instead."},
                    "limit": {"type": "integer", "minimum": 0, "description": "Maximum hits returned. Default 100."},
                    "db": db
                },
                "required": ["smiles"]
            }
        },
        {
            "name": "substructure_search",
            "description": "Exact substructure search: every molecule that contains the SMARTS pattern.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "smarts": {"type": "string", "description": "SMARTS pattern (a SMILES also works)."},
                    "limit": {"type": "integer", "minimum": 0, "description": "Maximum hits returned. Default 100."},
                    "offset": {"type": "integer", "minimum": 0},
                    "timeout_ms": {"type": "integer", "minimum": 1},
                    "db": db
                },
                "required": ["smarts"]
            }
        },
        {
            "name": "molecule_properties",
            "description": "Formula, molecular weight, atom, bond and ring counts of a SMILES.",
            "inputSchema": {
                "type": "object",
                "properties": {"smiles": {"type": "string"}},
                "required": ["smiles"]
            }
        },
        {
            "name": "get_molecule",
            "description": "Fetch a stored molecule by id.",
            "inputSchema": {
                "type": "object",
                "properties": {"id": {"type": "string"}, "db": db},
                "required": ["id"]
            }
        }
    ])
}

fn reply(id: &Value, result: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "result": result})
}

fn error(id: &Value, code: i64, message: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message}})
}

fn tool_result(r: Result<Value, String>) -> Value {
    match r {
        Ok(v) => json!({
            "content": [{"type": "text", "text": v.to_string()}],
            "structuredContent": v,
            "isError": false
        }),
        Err(e) => json!({
            "content": [{"type": "text", "text": e}],
            "isError": true
        }),
    }
}

fn to_value<T: serde::Serialize>(v: Result<T, crate::ApiError>) -> Result<Value, String> {
    v.map(|x| serde_json::to_value(x).unwrap_or(Value::Null))
        .map_err(|e| e.error)
}

fn args<T: serde::de::DeserializeOwned>(v: Value) -> Result<T, String> {
    serde_json::from_value(v).map_err(|e| format!("invalid arguments: {e}"))
}

async fn call_tool(engine: &Engine, name: &str, a: Value) -> Option<Value> {
    let r = match name {
        "list_databases" => Ok(json!({"databases": engine.databases()})),
        "similarity_search" => match args::<SimilarityRequest>(a) {
            Ok(req) => to_value(engine.similarity(req).await),
            Err(e) => Err(e),
        },
        "substructure_search" => match args::<SubstructureRequest>(a) {
            Ok(req) => to_value(engine.substructure(req).await),
            Err(e) => Err(e),
        },
        "molecule_properties" => match args::<MoleculeRequest>(a) {
            Ok(req) => to_value(engine.molecule(&req)),
            Err(e) => Err(e),
        },
        "get_molecule" => {
            let db = a.get("db").and_then(Value::as_str).map(str::to_string);
            match a.get("id").and_then(Value::as_str) {
                Some(id) => to_value(engine.lookup(db, id.to_string()).await),
                None => Err("invalid arguments: id is required".into()),
            }
        }
        _ => return None,
    };
    Some(tool_result(r))
}

async fn handle(engine: &Engine, msg: Value) -> Option<Value> {
    let id = msg.get("id").cloned();
    let method = msg.get("method").and_then(Value::as_str).unwrap_or("");
    let Some(id) = id else {
        // A notification (or a response from the client): no reply.
        return None;
    };
    let params = msg.get("params").cloned().unwrap_or(Value::Null);
    Some(match method {
        "initialize" => {
            let asked = params.get("protocolVersion").and_then(Value::as_str);
            let version = asked
                .filter(|v| PROTOCOL_VERSIONS.contains(v))
                .unwrap_or(PROTOCOL_VERSIONS[0]);
            reply(
                &id,
                json!({
                    "protocolVersion": version,
                    "capabilities": {"tools": {"listChanged": false}},
                    "serverInfo": {"name": "mossflux", "version": env!("CARGO_PKG_VERSION")},
                    "instructions": INSTRUCTIONS
                }),
            )
        }
        "ping" => reply(&id, json!({})),
        "tools/list" => reply(&id, json!({"tools": tools()})),
        "tools/call" => {
            let name = params.get("name").and_then(Value::as_str).unwrap_or("");
            let a = params.get("arguments").cloned().unwrap_or(json!({}));
            match call_tool(engine, name, a).await {
                Some(r) => reply(&id, r),
                None => error(&id, -32602, &format!("unknown tool {name:?}")),
            }
        }
        "" => error(&id, -32600, "invalid request"),
        _ => error(&id, -32601, &format!("method not found: {method}")),
    })
}

/// Handles one JSON-RPC message or batch. `None` means no reply is due.
pub async fn handle_message(engine: &Engine, text: &str) -> Option<String> {
    let msg: Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => {
            return Some(error(&Value::Null, -32700, &format!("parse error: {e}")).to_string())
        }
    };
    match msg {
        Value::Array(batch) => {
            let mut out = Vec::new();
            for m in batch {
                if let Some(r) = handle(engine, m).await {
                    out.push(r);
                }
            }
            (!out.is_empty()).then(|| Value::Array(out).to_string())
        }
        m => handle(engine, m).await.map(|v| v.to_string()),
    }
}

/// Serves MCP over stdin/stdout, one JSON message per line, until stdin
/// closes. Databases load before the first message is read.
///
/// # Errors
/// I/O errors, or a database that fails to load.
pub async fn serve_stdio(engine: Arc<Engine>) -> std::io::Result<()> {
    {
        let engine = engine.clone();
        tokio::task::spawn_blocking(move || engine.load_all())
            .await
            .map_err(std::io::Error::other)??;
    }
    let mut lines = BufReader::new(tokio::io::stdin()).lines();
    let mut out = tokio::io::stdout();
    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }
        if let Some(r) = handle_message(&engine, &line).await {
            out.write_all(r.as_bytes()).await?;
            out.write_all(b"\n").await?;
            out.flush().await?;
        }
    }
    Ok(())
}
