use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub(crate) struct Book {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) category: String,
    pub(crate) year: i32,
}
