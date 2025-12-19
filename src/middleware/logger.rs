use axum::{body::Body, http::Request, middleware::Next, response::Response};
use chrono::Utc;
use serde::Serialize;
use std::{fs::OpenOptions, io::Write, net::SocketAddr};

#[derive(Serialize)]
struct RequestLog {
    timestamp: String,
    method: String,
    path: String,
    ip: String,
}

pub async fn log_requests(req: Request<Body>, next: Next) -> Response {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    // Extract IP
    let ip = req
        .extensions()
        .get::<SocketAddr>()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let log = RequestLog {
        timestamp: Utc::now().to_rfc3339(),
        method,
        path,
        ip,
    };

    // Append to JSON log file
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("request_logs.json")
    {
        let _ = writeln!(file, "{}", serde_json::to_string(&log).unwrap());
    }

    next.run(req).await
}
