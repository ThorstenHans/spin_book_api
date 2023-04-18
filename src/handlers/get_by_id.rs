use anyhow::Result;
use spin_sdk::{
    http::{Params, Request, Response},
    key_value,
};

use crate::{entities::Book, models::BookDetailsModel};

pub(crate) fn handle_get_by_id(_req: Request, params: Params) -> Result<Response> {
    let Some(id) = params.get("id") else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    };
    let store = key_value::Store::open_default()?;

    let Ok(found) = store.get(id) else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    };

    let entity = serde_json::from_slice::<Book>(found.as_slice())?;
    let model = BookDetailsModel::from(entity);
    let serialized = serde_json::to_string(&model)?;
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(serialized.into()))?)
}
