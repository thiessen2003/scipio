mod controllers;
mod responses;

use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use axum::{routing, Router};
use utoipa::OpenApi;

use crate::app::api::middleware::make_rbac;
use crate::app::state::Services;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::fetch_basic_stats,
    ),
    security(("http" = ["JWT"]))
)]
pub struct StatsApi;

pub async fn build(ctx: Arc<Services>) -> Router<()> {
    let read_guard = make_rbac(vec!["read:stats".to_owned()]).await;

    let fetch_basic_stats = routing::get(controllers::fetch_basic_stats);

    Router::new()
        .route("/:project_cycle_id/basic", fetch_basic_stats)
        .route_layer(from_fn_with_state(ctx.clone(), read_guard))
        .with_state(ctx.clone())
}
