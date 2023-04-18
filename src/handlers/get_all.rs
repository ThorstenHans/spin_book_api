use anyhow::Result;
use spin_sdk::{
    http::{Params, Request, Response},
    key_value,
};

use crate::{entities::Book, models::BookListModel};

pub(crate) fn handle_get_all(_req: Request, _params: Params) -> Result<Response> {
    let store = key_value::Store::open_default()?;
    let Ok(keys) = store.get_keys() else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(None)?);
    };
    let mut books = Vec::<BookListModel>::default();
    keys.iter()
        .flat_map(|id| store.get(id))
        .flat_map(|json| serde_json::from_slice::<Book>(&json))
        .map(BookListModel::from)
        .for_each(|book| books.push(book));

    let body = serde_json::to_string(&books)?;
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(body.into()))?)
}
