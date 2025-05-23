use actix_web::web::Data;
use actix_web::{delete, post, put};
use actix_web::{get, http, web, HttpRequest, HttpResponse, Result};
use application::CreateToDoItemQuery;
use application::DeleteToDoItemQuery;
use application::GetToDoItemQuery;
use application::UpdateToDoItemQuery;
use std::rc::Rc;
use uuid::Uuid;

use crate::errors::HttpError;
use crate::requests::{CreateToDoItemRequest, UpdateToDoItemRequest};
use crate::responses::ToDoItemResponse;
use infrastructure::{DbPool, PostgresToDoItemRepository};

const TODO: &str = "todo";

/// Retrieves a list of all to-do items.
#[utoipa::path(
    context_path = "/to-do-items",
    tag = TODO,
    responses(
        (status = 200, description = "List current todo items", body = [ToDoItemResponse])
    )
)]
#[get("")]
pub async fn get_all(req: HttpRequest) -> Result<HttpResponse, HttpError> {
    let pool = req.app_data::<Data<DbPool>>().unwrap();

    let repository = PostgresToDoItemRepository::new(&pool.clone());

    let get_handler = application::GetAllToDoItemQueryHandler::new(Rc::new(repository));

    let data = get_handler.execute().await?;

    Ok(HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .body(serde_json::to_string(&data)?))
}

/// Retrieves a to-do item by Id.
#[utoipa::path(
    context_path = "/to-do-items",
    tag = TODO,
    responses(
        (status = 200, description = "Get todo item by id", body = [ToDoItemResponse])
    ),
    params(
        ("id" = Uuid, Path, description = "Id of the to-do item")
    ),
)]
#[get("/{id}")]
pub async fn get_by_id(req: HttpRequest, _id: web::Path<Uuid>) -> Result<HttpResponse, HttpError> {
    let pool = req.app_data::<Data<DbPool>>().unwrap();

    let repository = PostgresToDoItemRepository::new(&pool.clone());

    let get_handler = application::GetToDoItemQueryHandler::new(Rc::new(repository));

    let data = get_handler
        .execute(GetToDoItemQuery::new(Some(_id.into_inner())))
        .await?;

    Ok(HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .body(serde_json::to_string(&data)?))
}

/// Creates a new to-do item.
#[utoipa::path(
    context_path = "/to-do-items",
    tag = TODO,
    responses(
        (status = 200, description = "Create todo item", body = [ToDoItemResponse])
    ),
    request_body = CreateToDoItemRequest,
)]
#[post("")]
pub async fn create(
    req: HttpRequest,
    item: web::Json<CreateToDoItemRequest>,
) -> Result<HttpResponse, HttpError> {
    let pool = req.app_data::<Data<DbPool>>().unwrap();

    let repository = PostgresToDoItemRepository::new(&pool.clone());

    let get_handler = application::CreateToDoItemQueryHandler::new(Rc::new(repository));

    let data = get_handler
        .execute(CreateToDoItemQuery::new(&item.title, &item.title))
        .await?;

    Ok(HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .body(serde_json::to_string(&data)?))
}

/// Updates a to-do item by Id.
#[utoipa::path(
    context_path = "/to-do-items",
    tag = TODO,
    responses(
        (status = 200, description = "Update todo item", body = [ToDoItemResponse])
    ),
    params(
        ("id", description = "Id of the to-do item to update")
    ),
    request_body = UpdateToDoItemRequest,
)]
#[put("/{id}")]
pub async fn update(
    req: HttpRequest,
    id: web::Path<Uuid>,
    item: web::Json<UpdateToDoItemRequest>,
) -> Result<HttpResponse, HttpError> {
    let pool = req.app_data::<Data<DbPool>>().unwrap();

    let repository = PostgresToDoItemRepository::new(&pool.clone());

    let get_handler = application::UpdateToDoItemQueryHandler::new(Rc::new(repository));

    get_handler
        .execute(UpdateToDoItemQuery::new(
            id.into_inner(),
            &item.title,
            &item.note,
        ))
        .await?;

    Ok(HttpResponse::from(HttpResponse::Ok()))
}

/// Deletes a to-do item by Id.
#[utoipa::path(
    context_path = "/to-do-items",
    tag = TODO,
    responses(
        (status = 200, description = "Delete todo item", body = [ToDoItemResponse])
    ),
    params(
        ("id", description = "Id of the to-do item to delete")
    )
)]
#[delete("/{id}")]
pub async fn delete(req: HttpRequest, _id: web::Path<Uuid>) -> Result<HttpResponse, HttpError> {
    let pool = req.app_data::<Data<DbPool>>().unwrap();

    let repository = PostgresToDoItemRepository::new(&pool.clone());

    let get_handler = application::DeleteToDoItemQueryHandler::new(Rc::new(repository));

    get_handler
        .execute(DeleteToDoItemQuery::new(_id.into_inner()))
        .await?;

    Ok(HttpResponse::from(HttpResponse::Ok()))
}
