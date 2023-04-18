use anyhow::Result;
use spin_sdk::{
    http::{Params, Request, Response},
    key_value,
};
use uuid::Uuid;

use crate::{entities::Book, models::CreateBookModel};

pub(crate) fn handle_create(req: Request, _params: Params) -> Result<Response> {
    let id = Uuid::new_v4().to_string();
    let Ok(model) = CreateBookModel::try_from(&req.body().clone()) else {
        return Ok(http::Response::builder()
        .status(http::StatusCode::BAD_REQUEST)
        .body(None)?);
    };
    let store = key_value::Store::open_default()?;
    let book = Book {
        id,
        title: model.title,
        author: model.author,
        category: model.category,
        year: model.year,
    };
    let serialized = serde_json::to_vec(&book)?;
    store.set(&book.id, &serialized)?;
    Ok(http::Response::builder()
        .status(http::StatusCode::CREATED)
        .body(Some(serialized.into()))?)
}
