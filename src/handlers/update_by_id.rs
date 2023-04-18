use anyhow::Result;
use spin_sdk::{
    http::{Params, Request, Response},
    key_value,
};

use crate::{entities::Book, models::UpdateBookModel};

pub(crate) fn handle_update_by_id(req: Request, params: Params) -> Result<Response> {
    let Some(id) = params.get("id") else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    };
    let Ok(model) = UpdateBookModel::try_from(&req.body().clone()) else {
        return Ok(http::Response::builder()
        .status(http::StatusCode::BAD_REQUEST)
        .body(None)?);
    };
    let store = key_value::Store::open_default()?;
    let Ok(found) = store.get(id) else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    };
    let mut found = serde_json::from_slice::<Book>(found.as_slice())?;

    found.title = model.title;
    found.author = model.author;
    found.category = model.category;
    found.year = model.year;

    let serialized = serde_json::to_vec(&found)?;
    store.set(id, &serialized)?;
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(serialized.into()))?)
}
