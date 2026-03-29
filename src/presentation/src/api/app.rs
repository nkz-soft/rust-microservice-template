use actix_web::http::header::{ETAG, IF_MATCH};
use actix_web::web::Data;
use actix_web::{delete, post, put};
use actix_web::{get, web, HttpResponse, Result};
use application::{
    Audit, DeleteToDoItemQuery, GetAllToDoItemsQuery, GetDeletedToDoItemForAuditQuery,
    GetToDoItemQuery, ToDoItemService,
};
use uuid::Uuid;
use validator::Validate;

use crate::errors::HttpError;
use crate::requests::{
    parse_audit_token_header, parse_optional_delete_actor_id, CreateToDoItemRequest,
    GetAllToDoItemsQueryRequest, UpdateToDoItemRequest,
};
use crate::responses::{
    AuditToDoItemResponse, ProblemDetailsResponse, ToDoItemResponse, ToDoItemsPageResponse,
};

const TODO: &str = "todo";

/// Retrieves a paginated list of active to-do items with optional text search.
#[utoipa::path(
    context_path = "/api/v1/to-do-items",
    tag = TODO,
    responses(
        (status = 200, description = "List active to-do items filtered by the optional search term. Responses include X-Request-Id.", body = ToDoItemsPageResponse),
        (status = 400, description = "Validation error for blank or malformed query parameters. Responses include X-Request-Id.", body = ProblemDetailsResponse)
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
        (status = 200, description = "Get todo item by id. Responses include X-Request-Id.", body = ToDoItemResponse),
        (status = 404, description = "Todo item not found. Responses include X-Request-Id.", body = ProblemDetailsResponse),
        (status = 500, description = "Unexpected internal error. Responses include X-Request-Id.", body = ProblemDetailsResponse)
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
        (status = 201, description = "Create todo item. Responses include X-Request-Id.", body = Uuid),
        (status = 400, description = "Validation error. Responses include X-Request-Id.", body = ProblemDetailsResponse)
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
        (status = 200, description = "Update todo item. Responses include X-Request-Id."),
        (status = 400, description = "Validation error. Responses include X-Request-Id.", body = ProblemDetailsResponse),
        (status = 404, description = "Todo item not found. Responses include X-Request-Id.", body = ProblemDetailsResponse),
        (status = 412, description = "Stale If-Match precondition. Responses include X-Request-Id.", body = ProblemDetailsResponse),
        (status = 428, description = "Missing If-Match precondition. Responses include X-Request-Id.", body = ProblemDetailsResponse),
        (status = 500, description = "Unexpected internal error. Responses include X-Request-Id.", body = ProblemDetailsResponse)
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
        (status = 200, description = "Delete todo item. Responses include X-Request-Id."),
        (status = 500, description = "Unexpected internal error. Responses include X-Request-Id.", body = ProblemDetailsResponse)
    ),
    params(
        ("id", description = "Id of the to-do item to delete")
    )
)]
#[delete("/{id}")]
pub async fn delete(
    service: Data<ToDoItemService>,
    id: web::Path<Uuid>,
    request: actix_web::HttpRequest,
) -> Result<HttpResponse, HttpError> {
    let deleted_by = parse_optional_delete_actor_id(&request).map_err(HttpError::bad_request)?;
    let handler = service.delete_handler();

    handler
        .execute(DeleteToDoItemQuery::new(id.into_inner(), deleted_by))
        .await?;

    Ok(HttpResponse::from(HttpResponse::Ok()))
}

/// Retrieves a deleted to-do item by Id for audit purposes.
#[utoipa::path(
    context_path = "/api/v1/audit/to-do-items",
    tag = TODO,
    responses(
        (status = 200, description = "Get deleted todo item by id for audit. Responses include X-Request-Id.", body = AuditToDoItemResponse),
        (status = 401, description = "Missing or invalid audit token. Responses include X-Request-Id.", body = ProblemDetailsResponse),
        (status = 404, description = "Deleted todo item not found. Responses include X-Request-Id.", body = ProblemDetailsResponse),
        (status = 500, description = "Unexpected internal error. Responses include X-Request-Id.", body = ProblemDetailsResponse)
    ),
    params(
        ("id" = Uuid, Path, description = "Id of the deleted to-do item"),
        ("X-Audit-Token" = String, Header, description = "Audit access token")
    ),
)]
#[get("/{id}")]
pub async fn get_deleted_by_id_for_audit(
    service: Data<ToDoItemService>,
    audit: Data<Audit>,
    id: web::Path<Uuid>,
    request: actix_web::HttpRequest,
) -> Result<HttpResponse, HttpError> {
    let provided_token = parse_audit_token_header(&request)
        .ok_or_else(|| HttpError::unauthorized("missing X-Audit-Token header"))?;
    let configured_token = audit
        .token
        .as_ref()
        .filter(|token| !token.trim().is_empty())
        .ok_or_else(|| HttpError::unauthorized("audit endpoint is not configured"))?;
    if configured_token != &provided_token {
        return Err(HttpError::unauthorized("invalid audit token"));
    }

    let handler = service.get_deleted_for_audit_handler();
    let item = handler
        .execute(GetDeletedToDoItemForAuditQuery::new(id.into_inner()))
        .await?;
    let data = AuditToDoItemResponse::from(item);

    Ok(HttpResponse::Ok().json(data))
}

fn format_etag(version: i32) -> String {
    format!("\"{version}\"")
}

#[allow(clippy::result_large_err)]
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
