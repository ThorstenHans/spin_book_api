use anyhow::Result;
use spin_sdk::{
    http::{Params, Request, Response},
    key_value,
};

pub(crate) fn handle_delete_by_id(_req: Request, params: Params) -> Result<Response> {
    let Some(id) = params.get("id") else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    };

    let store = key_value::Store::open_default()?;
    let Ok(exists) = store.exists(id) else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    };

    if !exists {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    }

    let Ok(_) = store.delete(id) else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(None)?);
    };
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(None)?)
}
