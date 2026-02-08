use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

pub async fn get_overview(
    State(pool): State<PgPool>,
) -> Result<Json<analytics_core::OverviewStats>, StatusCode> {
    analytics_core::get_overview_stats(&pool)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to fetch overview stats: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
