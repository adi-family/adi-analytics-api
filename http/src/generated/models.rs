//! Auto-generated models from TypeSpec.
//! DO NOT EDIT.

#![allow(unused_imports)]

use super::enums::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateRangeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverviewStats {
    pub total_users: i64,
    pub active_users_today: i64,
    pub active_users_week: i64,
    pub active_users_month: i64,
    pub total_tasks: i64,
    pub tasks_today: i64,
    pub task_success_rate: f64,
    pub total_cocoons: i64,
    pub active_cocoons: i64,
    pub total_integrations: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyActiveUsers {
    pub day: DateTime<Utc>,
    pub active_users: i64,
    pub total_events: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStats {
    pub day: DateTime<Utc>,
    pub created: i64,
    pub started: i64,
    pub completed: i64,
    pub failed: i64,
    pub cancelled: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_duration_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_95_duration_ms: Option<f64>,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStatsOverview {
    pub total_created: i64,
    pub total_completed: i64,
    pub total_failed: i64,
    pub total_cancelled: i64,
    pub success_rate: f64,
    pub avg_duration_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointLatency {
    pub hour: DateTime<Utc>,
    pub service: String,
    pub endpoint: String,
    pub method: String,
    pub request_count: i64,
    pub avg_duration_ms: f64,
    pub p_50_duration_ms: f64,
    pub p_95_duration_ms: f64,
    pub p_99_duration_ms: f64,
    pub error_4_xx_count: i64,
    pub error_5_xx_count: i64,
}
