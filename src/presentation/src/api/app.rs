use actix_web::http::header::{ETAG, IF_MATCH};
use actix_web::web::Data;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Result};
use application::{
    audit_read_policy, todo_read_policy, todo_write_policy, AuthService, AuthenticatedPrincipal,
    GetAllToDoItemsQuery, GetDeletedToDoItemForAuditQuery, GetToDoItemQuery, LoginQueryHandler,
    ToDoItemService,
};
use uuid::Uuid;
use validator::Validate;

use crate::errors::HttpError;
use crate::requests::{
    parse_bearer_token_header, parse_optional_delete_actor_id, parse_service_api_key_header,
    CreateToDoItemRequest, GetAllToDoItemsQueryRequest, TokenRequest, UpdateToDoItemRequest,
    DEFAULT_SERVICE_API_KEY_HEADER,
};
use crate::responses::{
    AuditToDoItemResponse, ProblemDetailsResponse, ToDoItemResponse, ToDoItemsPageResponse,
    TokenResponseBody,
};

const TODO: &str = "todo";
const AUTH: &str = "auth";

/// Issues a bearer token for a configured user.
#[utoipa::path(
    context_path = "/api/v1/auth",
    tag = AUTH,
    request_body = TokenRequest,
    responses(
        (status = 200, description = "Token issued successfully", body = TokenResponseBody),
        (status = 401, description = "Invalid credentials", body = ProblemDetailsResponse)
    )
)]
#[post("/token")]
pub async fn issue_token(
    auth_service: Data<AuthService>,
    request: web::Json<TokenRequest>,
) -> Result<HttpResponse, HttpError> {
    request.validate()?;

    let handler = LoginQueryHandler::new(auth_service.clone().into_inner());
    let response = handler.execute(request.to_query()).await?;

    Ok(HttpResponse::Ok().json(TokenResponseBody::from(response)))
}

/// Retrieves a paginated list of active to-do items with optional text search.
#[utoipa::path(
    context_path = "/api/v1/to-do-items",
    tag = TODO,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "List active to-do items filtered by the optional search term", body = ToDoItemsPageResponse),
        (status = 400, description = "Validation error for blank or malformed query parameters", body = ProblemDetailsResponse),
        (status = 401, description = "Missing or invalid bearer token", body = ProblemDetailsResponse),
        (status = 403, description = "Authenticated caller lacks the required permission", body = ProblemDetailsResponse)
    ),
    params(GetAllToDoItemsQueryRequest)
)]
#[get("")]
pub async fn get_all(
    service: Data<ToDoItemService>,
    auth_service: Data<AuthService>,
    request: HttpRequest,
    query: web::Query<GetAllToDoItemsQueryRequest>,
) -> Result<HttpResponse, HttpError> {
    authorize_bearer_request(&auth_service, &request, todo_read_policy())?;
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
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Get todo item by id", body = ToDoItemResponse),
        (status = 401, description = "Missing or invalid bearer token", body = ProblemDetailsResponse),
        (status = 403, description = "Authenticated caller lacks the required permission", body = ProblemDetailsResponse)
    ),
    params(
        ("id" = Uuid, Path, description = "Id of the to-do item")
    ),
)]
#[get("/{id}")]
pub async fn get_by_id(
    service: Data<ToDoItemService>,
    auth_service: Data<AuthService>,
    request: HttpRequest,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, HttpError> {
    authorize_bearer_request(&auth_service, &request, todo_read_policy())?;
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
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Create todo item", body = Uuid),
        (status = 400, description = "Validation error", body = ProblemDetailsResponse),
        (status = 401, description = "Missing or invalid bearer token", body = ProblemDetailsResponse),
        (status = 403, description = "Authenticated caller lacks the required permission", body = ProblemDetailsResponse)
    ),
    request_body = CreateToDoItemRequest,
)]
#[post("")]
pub async fn create(
    service: Data<ToDoItemService>,
    auth_service: Data<AuthService>,
    request: HttpRequest,
    item: web::Json<CreateToDoItemRequest>,
) -> Result<HttpResponse, HttpError> {
    authorize_bearer_request(&auth_service, &request, todo_write_policy())?;
    item.validate()?;
    let handler = service.create_handler();
    let data = handler.execute(item.to_query()).await?;

    Ok(HttpResponse::Created().json(data))
}

/// Updates a to-do item by Id.
#[utoipa::path(
    context_path = "/api/v1/to-do-items",
    tag = TODO,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Update todo item"),
        (status = 400, description = "Validation error", body = ProblemDetailsResponse),
        (status = 401, description = "Missing or invalid bearer token", body = ProblemDetailsResponse),
        (status = 403, description = "Authenticated caller lacks the required permission", body = ProblemDetailsResponse),
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
    auth_service: Data<AuthService>,
    request: HttpRequest,
    id: web::Path<Uuid>,
    item: web::Json<UpdateToDoItemRequest>,
) -> Result<HttpResponse, HttpError> {
    authorize_bearer_request(&auth_service, &request, todo_write_policy())?;
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
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Delete todo item"),
        (status = 401, description = "Missing or invalid bearer token", body = ProblemDetailsResponse),
        (status = 403, description = "Authenticated caller lacks the required permission", body = ProblemDetailsResponse)
    ),
    params(
        ("id", description = "Id of the to-do item to delete")
    )
)]
#[delete("/{id}")]
pub async fn delete(
    service: Data<ToDoItemService>,
    auth_service: Data<AuthService>,
    request: HttpRequest,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, HttpError> {
    authorize_bearer_request(&auth_service, &request, todo_write_policy())?;
    let deleted_by = parse_optional_delete_actor_id(&request).map_err(HttpError::bad_request)?;
    let handler = service.delete_handler();

    handler
        .execute(application::DeleteToDoItemQuery::new(
            id.into_inner(),
            deleted_by,
        ))
        .await?;

    Ok(HttpResponse::Ok().finish())
}

/// Retrieves a deleted to-do item by Id for audit purposes.
#[utoipa::path(
    context_path = "/api/v1/audit/to-do-items",
    tag = TODO,
    security(("serviceApiKey" = [])),
    responses(
        (status = 200, description = "Get deleted todo item by id for audit", body = AuditToDoItemResponse),
        (status = 401, description = "Missing or invalid service API key", body = ProblemDetailsResponse),
        (status = 403, description = "Authenticated service principal lacks the required permission", body = ProblemDetailsResponse),
        (status = 404, description = "Deleted todo item not found", body = ProblemDetailsResponse)
    ),
    params(
        ("id" = Uuid, Path, description = "Id of the deleted to-do item"),
        ("X-Service-Api-Key" = String, Header, description = "Service API key")
    ),
)]
#[get("/{id}")]
pub async fn get_deleted_by_id_for_audit(
    service: Data<ToDoItemService>,
    auth_service: Data<AuthService>,
    request: HttpRequest,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, HttpError> {
    authorize_service_request(&auth_service, &request, audit_read_policy())?;

    let handler = service.get_deleted_for_audit_handler();
    let item = handler
        .execute(GetDeletedToDoItemForAuditQuery::new(id.into_inner()))
        .await?;
    let data = AuditToDoItemResponse::from(item);

    Ok(HttpResponse::Ok().json(data))
}

#[allow(clippy::result_large_err)]
fn authorize_bearer_request(
    auth_service: &Data<AuthService>,
    request: &HttpRequest,
    policy: application::ProtectedEndpointPolicy,
) -> Result<AuthenticatedPrincipal, HttpError> {
    let token =
        parse_bearer_token_header(request).ok_or(application::AuthError::MissingBearerToken)?;
    let principal = auth_service.authenticate_bearer_token(&token)?;
    auth_service.authorize(&principal, &policy)?;
    Ok(principal)
}

#[allow(clippy::result_large_err)]
fn authorize_service_request(
    auth_service: &Data<AuthService>,
    request: &HttpRequest,
    policy: application::ProtectedEndpointPolicy,
) -> Result<AuthenticatedPrincipal, HttpError> {
    let mut header_names = auth_service.service_header_names();
    if header_names.is_empty() {
        header_names.push(DEFAULT_SERVICE_API_KEY_HEADER.to_string());
    }
    for header_name in header_names {
        if let Some(value) = parse_service_api_key_header(request, &header_name) {
            let principal = auth_service.authenticate_service_api_key(&header_name, &value)?;
            auth_service.authorize(&principal, &policy)?;
            return Ok(principal);
        }
    }

    Err(application::AuthError::MissingServiceApiKey.into())
}

fn format_etag(version: i32) -> String {
    format!("\"{version}\"")
}

#[allow(clippy::result_large_err)]
fn parse_if_match(request: &HttpRequest) -> Result<i32, HttpError> {
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
