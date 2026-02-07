//! Auto-generated server handlers from TypeSpec.
//! DO NOT EDIT.
//!
//! Implement the handler traits and use the generated router.

#![allow(unused_imports)]

use super::models::*;
use super::enums::*;
use async_trait::async_trait;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post, put};
use axum::{Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;


#[derive(Debug, serde::Serialize)]
pub struct ApiError {
    pub status: u16,
    pub code: String,
    pub message: String,
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}


#[async_trait]
pub trait AnalyticsServiceHandler: Send + Sync + 'static {
    async fn get_overview(&self) -> Result<OverviewStats, ApiError>;
    async fn get_daily_active_users(&self, query: AnalyticsServiceGetDailyActiveUsersQuery) -> Result<Vec<DailyActiveUsers>, ApiError>;
    async fn get_weekly_active_users(&self, query: AnalyticsServiceGetWeeklyActiveUsersQuery) -> Result<Vec<DailyActiveUsers>, ApiError>;
    async fn get_task_stats_daily(&self, query: AnalyticsServiceGetTaskStatsDailyQuery) -> Result<Vec<TaskStats>, ApiError>;
    async fn get_task_stats_overview(&self, query: AnalyticsServiceGetTaskStatsOverviewQuery) -> Result<TaskStatsOverview, ApiError>;
    async fn get_endpoint_latency(&self, query: AnalyticsServiceGetEndpointLatencyQuery) -> Result<Vec<EndpointLatency>, ApiError>;
    async fn get_slowest_endpoints(&self) -> Result<Vec<EndpointLatency>, ApiError>;
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsServiceGetDailyActiveUsersQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsServiceGetWeeklyActiveUsersQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsServiceGetTaskStatsDailyQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsServiceGetTaskStatsOverviewQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsServiceGetEndpointLatencyQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

async fn analytics_service_get_overview<S: AnalyticsServiceHandler>(
    State(state): State<Arc<S>>,
) -> Result<Json<OverviewStats>, ApiError> {
    let result = state.get_overview().await?;
    Ok(Json(result))
}

async fn analytics_service_get_daily_active_users<S: AnalyticsServiceHandler>(
    State(state): State<Arc<S>>,
    Query(query): Query<AnalyticsServiceGetDailyActiveUsersQuery>,
) -> Result<Json<Vec<DailyActiveUsers>>, ApiError> {
    let result = state.get_daily_active_users(query).await?;
    Ok(Json(result))
}

async fn analytics_service_get_weekly_active_users<S: AnalyticsServiceHandler>(
    State(state): State<Arc<S>>,
    Query(query): Query<AnalyticsServiceGetWeeklyActiveUsersQuery>,
) -> Result<Json<Vec<DailyActiveUsers>>, ApiError> {
    let result = state.get_weekly_active_users(query).await?;
    Ok(Json(result))
}

async fn analytics_service_get_task_stats_daily<S: AnalyticsServiceHandler>(
    State(state): State<Arc<S>>,
    Query(query): Query<AnalyticsServiceGetTaskStatsDailyQuery>,
) -> Result<Json<Vec<TaskStats>>, ApiError> {
    let result = state.get_task_stats_daily(query).await?;
    Ok(Json(result))
}

async fn analytics_service_get_task_stats_overview<S: AnalyticsServiceHandler>(
    State(state): State<Arc<S>>,
    Query(query): Query<AnalyticsServiceGetTaskStatsOverviewQuery>,
) -> Result<Json<TaskStatsOverview>, ApiError> {
    let result = state.get_task_stats_overview(query).await?;
    Ok(Json(result))
}

async fn analytics_service_get_endpoint_latency<S: AnalyticsServiceHandler>(
    State(state): State<Arc<S>>,
    Query(query): Query<AnalyticsServiceGetEndpointLatencyQuery>,
) -> Result<Json<Vec<EndpointLatency>>, ApiError> {
    let result = state.get_endpoint_latency(query).await?;
    Ok(Json(result))
}

async fn analytics_service_get_slowest_endpoints<S: AnalyticsServiceHandler>(
    State(state): State<Arc<S>>,
) -> Result<Json<Vec<EndpointLatency>>, ApiError> {
    let result = state.get_slowest_endpoints().await?;
    Ok(Json(result))
}

pub fn analytics_service_routes<S: AnalyticsServiceHandler>() -> Router<Arc<S>> {
    Router::new()
        .route("/overview", get(analytics_service_get_overview::<S>))
        .route("/users/daily", get(analytics_service_get_daily_active_users::<S>))
        .route("/users/weekly", get(analytics_service_get_weekly_active_users::<S>))
        .route("/tasks/daily", get(analytics_service_get_task_stats_daily::<S>))
        .route("/tasks/overview", get(analytics_service_get_task_stats_overview::<S>))
        .route("/api/latency", get(analytics_service_get_endpoint_latency::<S>))
        .route("/api/slowest", get(analytics_service_get_slowest_endpoints::<S>))
}

pub fn create_router<S: AnalyticsServiceHandler>() -> Router<Arc<S>> {
    analytics_service_routes()
}
