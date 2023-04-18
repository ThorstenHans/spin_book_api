mod entities;
mod handlers;
mod models;
use anyhow::Result;
use crate::handlers::{
    handle_create, handle_delete_by_id, handle_get_all, handle_get_by_id, handle_update_by_id, handle_healthz,
};
use spin_sdk::{
    http::{Request, Response, Router},
    http_component,
};

#[http_component]
fn handle_foo(req: Request) -> Result<Response> {
    let mut router = Router::default();
    router.get("/", handle_get_all);
    router.get("/:id", handle_get_by_id);
    router.post("/", handle_create);
    router.put("/:id", handle_update_by_id);
    router.delete("/:id", handle_delete_by_id);
    router.add("/healthz/readiness", http::Method::GET, handle_healthz);
    
    router.handle(req)
}
