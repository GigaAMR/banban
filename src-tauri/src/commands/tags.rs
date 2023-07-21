use entity::category_tags;
use sea_orm::DbConn;
use serde::Deserialize;
use tauri::State;

use crate::{errors::AppError, database::tags::Mutation};

#[derive(Deserialize)]
pub struct CreateTagInput {
    pub tag_name: String,
    pub category_id: Option<i32>,
}

#[tauri::command]
pub async fn create_tag(db: State<'_, DbConn>, data: CreateTagInput) -> Result<category_tags::Model, AppError> {
    Mutation::create_tag(db.inner(), data).await
}

#[derive(Deserialize)]
pub struct AttachTagToCategoryInput {
    pub category_tag_id: i32,
    pub category_id: Option<i32>,
}

#[tauri::command]
pub async fn attach_tag_to_category(db: State<'_, DbConn>, data: AttachTagToCategoryInput) -> Result<(), AppError> {
    Mutation::update_category_tag(db.inner(), data).await
}

#[derive(Deserialize)]
pub struct UpdateTagNameInput {
    pub category_tag_id: i32,
    pub tag_name: String,
}

#[tauri::command]
pub async fn update_tag_name(db: State<'_, DbConn>, data: UpdateTagNameInput) -> Result<(), AppError> {
    Mutation::update_tag_name(db.inner(), data).await
}

#[tauri::command]
pub async fn delete_tag(db: State<'_, DbConn>, category_tag_id: i32) -> Result<(), AppError> {
    Mutation::delete_tag_by_id(db.inner(), category_tag_id).await
}