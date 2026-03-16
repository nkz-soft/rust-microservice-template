use actix_web::http::header::{ETAG, IF_MATCH};
use actix_web::web::Data;
use actix_web::{delete, post, put};
use actix_web::{get, web, HttpResponse, Result};
use application::DeleteToDoItemQuery;
use application::GetAllToDoItemsQuery;
use application::GetToDoItemQuery;
use application::ToDoItemService;
use uuid::Uuid;
use validator::Validate;

use crate::errors::HttpError;
use crate::requests::{CreateToDoItemRequest, GetAllToDoItemsQueryRequest, UpdateToDoItemRequest};
use crate::responses::{ProblemDetailsResponse, ToDoItemResponse, ToDoItemsPageResponse};

const TODO: &str = "todo";

/// Retrieves a list of all to-do items.
#[utoipa::path(
    context_path = "/api/v1/to-do-items",
    tag = TODO,
    responses(
        (status = 200, description = "List current todo items", body = ToDoItemsPageResponse),
        (status = 400, description = "Validation error", body = ProblemDetailsResponse)
    ),
    params(GetAllToDoItemsQueryRequest)
)]
#[get("")]
pub async fn get_all(
    service: Data<ToDoItemService>,
    query: web::Query<GetAllToDoItemsQueryRequest>,
) -> Result<HttpResponse, HttpError> {
    query.validate()?;
    query.validate_search().map_err(HttpError::bad_request)?;
    query.validate_sort().map_err(HttpError::bad_request)?;
    let handler = service.get_all_handler();
    let query: GetAllToDoItemsQuery = query.to_query().map_err(HttpError::bad_request)?;
    let data = ToDoItemsPageResponse::from(handler.execute(query).await?);

    Ok(HttpResponse::Ok().json(data))
}

/// Retrieves a to-do item by Id.
#[utoipa::path(
    context_path = "/api/v1/to-do-items",
    tag = TODO,
    responses(
        (status = 200, description = "Get todo item by id", body = ToDoItemResponse)
    ),
    params(
        ("id" = Uuid, Path, description = "Id of the to-do item")
    ),
)]
#[get("/{id}")]
pub async fn get_by_id(
    service: Data<ToDoItemService>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, HttpError> {
    let handler = service.get_handler();
    let item = handler
        .execute(GetToDoItemQuery::new(Some(id.into_inner())))
        .await?;
    let etag = format_etag(item.version);
    let data = ToDoItemResponse::from(item);

    Ok(HttpResponse::Ok().insert_header((ETAG, etag)).json(data))
}

/// Creates a new to-do item.
#[utoipa::path(
    context_path = "/api/v1/to-do-items",
    tag = TODO,
    responses(
        (status = 201, description = "Create todo item", body = Uuid),
        (status = 400, description = "Validation error", body = ProblemDetailsResponse)
    ),
    request_body = CreateToDoItemRequest,
)]
#[post("")]
pub async fn create(
    service: Data<ToDoItemService>,
    item: web::Json<CreateToDoItemRequest>,
) -> Result<HttpResponse, HttpError> {
    item.validate()?;
    let handler = service.create_handler();
    let data = handler.execute(item.to_query()).await?;

    Ok(HttpResponse::Created().json(data))
}

/// Updates a to-do item by Id.
#[utoipa::path(
    context_path = "/api/v1/to-do-items",
    tag = TODO,
    responses(
        (status = 200, description = "Update todo item"),
        (status = 400, description = "Validation error", body = ProblemDetailsResponse),
        (status = 412, description = "Stale If-Match precondition", body = ProblemDetailsResponse),
        (status = 428, description = "Missing If-Match precondition", body = ProblemDetailsResponse)
    ),
    params(
        ("id", description = "Id of the to-do item to update")
    ),
    request_body = UpdateToDoItemRequest,
)]
#[put("/{id}")]
pub async fn update(
    service: Data<ToDoItemService>,
    id: web::Path<Uuid>,
    request: actix_web::HttpRequest,
    item: web::Json<UpdateToDoItemRequest>,
) -> Result<HttpResponse, HttpError> {
    item.validate()?;
    let handler = service.update_handler();
    let version = parse_if_match(&request)?;
    let id = id.into_inner();

    handler.execute(item.to_query(id, version)).await?;

    Ok(HttpResponse::Ok()
        .insert_header((ETAG, format_etag(version + 1)))
        .finish())
}

/// Deletes a to-do item by Id.
#[utoipa::path(
    context_path = "/api/v1/to-do-items",
    tag = TODO,
    responses(
        (status = 200, description = "Delete todo item")
    ),
    params(
        ("id", description = "Id of the to-do item to delete")
    )
)]
#[delete("/{id}")]
pub async fn delete(
    service: Data<ToDoItemService>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, HttpError> {
    let handler = service.delete_handler();

    handler
        .execute(DeleteToDoItemQuery::new(id.into_inner()))
        .await?;

    Ok(HttpResponse::from(HttpResponse::Ok()))
}

fn format_etag(version: i32) -> String {
    format!("\"{version}\"")
}

fn parse_if_match(request: &actix_web::HttpRequest) -> Result<i32, HttpError> {
    let raw = request
        .headers()
        .get(IF_MATCH)
        .ok_or_else(|| HttpError::precondition_required("missing If-Match header"))?
        .to_str()
        .map_err(|_| HttpError::bad_request("If-Match header must be valid ASCII"))?;

    let normalized = raw.trim();
    if normalized == "*" {
        return Err(HttpError::bad_request(
            "If-Match '*' is not supported for optimistic locking",
        ));
    }

    let normalized = normalized
        .strip_prefix("W/")
        .unwrap_or(normalized)
        .trim()
        .trim_matches('"');

    normalized
        .parse::<i32>()
        .map_err(|_| HttpError::bad_request("If-Match header must contain an integer ETag"))
}
