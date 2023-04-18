use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::entities::Book;

#[derive(Debug, Serialize)]
pub(crate) struct BookListModel {
    pub id: String,
    pub title: String,
    pub author: String,
}

impl From<Book> for BookListModel {
    fn from(value: Book) -> Self {
        Self {
            id: value.id,
            title: value.title,
            author: value.author,
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct BookDetailsModel {
    pub id: String,
    pub title: String,
    pub author: String,
    pub category: String,
    pub year: i32,
}

impl From<Book> for BookDetailsModel {
    fn from(value: Book) -> Self {
        Self {
            id: value.id,
            title: value.title,
            author: value.author,
            category: value.category,
            year: value.year,
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct CreateBookModel {
    pub title: String,
    pub author: String,
    pub category: String,
    pub year: i32,
}

impl TryFrom<&Option<Bytes>> for CreateBookModel {
    type Error = anyhow::Error;

    fn try_from(value: &Option<Bytes>) -> Result<Self, Self::Error> {
        match value {
            Some(b) => Ok(serde_json::from_slice::<CreateBookModel>(b)?),
            None => Err(anyhow::anyhow!("No body")),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateBookModel {
    pub title: String,
    pub author: String,
    pub category: String,
    pub year: i32,
}

impl TryFrom<&Option<Bytes>> for UpdateBookModel {
    type Error = anyhow::Error;

    fn try_from(value: &Option<Bytes>) -> Result<Self, Self::Error> {
        match value {
            Some(b) => Ok(serde_json::from_slice::<UpdateBookModel>(b)?),
            None => Err(anyhow::anyhow!("No body received!")),
        }
    }
}
