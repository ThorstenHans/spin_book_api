use anyhow::Result;
use spin_sdk::http::{Params, Request, Response};

pub(crate) fn handle_healthz(_req: Request, _params: Params) -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(None)?)
}
