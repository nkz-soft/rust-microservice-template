use actix_web::error::{JsonPayloadError, QueryPayloadError};
use actix_web::web::Json;
use actix_web::{HttpResponse, ResponseError};
use application::ApplicationError;
use http::StatusCode as HttpStatusCode;
use problem_details::{JsonProblemDetails, ProblemDetails};
use serde_json::Error as SerdeError;
use std::fmt::{Display, Formatter};
use validator::ValidationErrors;

#[derive(Debug)]
pub enum HttpError {
    Problem(ProblemDetails),
}

impl HttpError {
    pub fn bad_request(detail: impl Into<String>) -> Self {
        HttpError::Problem(
            ProblemDetails::new()
                .with_status(HttpStatusCode::BAD_REQUEST)
                .with_detail(detail.into()),
        )
    }

    pub fn internal_server_error(detail: impl Into<String>) -> Self {
        HttpError::Problem(
            ProblemDetails::new()
                .with_status(HttpStatusCode::INTERNAL_SERVER_ERROR)
                .with_title("Internal Server Error")
                .with_detail(detail.into()),
        )
    }

    pub fn precondition_failed(detail: impl Into<String>) -> Self {
        HttpError::Problem(
            ProblemDetails::new()
                .with_status(HttpStatusCode::PRECONDITION_FAILED)
                .with_title("Precondition Failed")
                .with_detail(detail.into()),
        )
    }

    pub fn precondition_required(detail: impl Into<String>) -> Self {
        HttpError::Problem(
            ProblemDetails::new()
                .with_status(HttpStatusCode::PRECONDITION_REQUIRED)
                .with_title("Precondition Required")
                .with_detail(detail.into()),
        )
    }

    pub fn unauthorized(detail: impl Into<String>) -> Self {
        HttpError::Problem(
            ProblemDetails::new()
                .with_status(HttpStatusCode::UNAUTHORIZED)
                .with_title("Unauthorized")
                .with_detail(detail.into()),
        )
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        match self {
            HttpError::Problem(problem) => HttpResponse::build(problem.status_code())
                .content_type(JsonProblemDetails::<()>::CONTENT_TYPE)
                .json(Json(problem)),
        }
    }
}

impl From<SerdeError> for HttpError {
    fn from(err: SerdeError) -> Self {
        HttpError::bad_request(err.to_string())
    }
}

impl From<ValidationErrors> for HttpError {
    fn from(err: ValidationErrors) -> Self {
        HttpError::bad_request(err.to_string())
    }
}

impl From<JsonPayloadError> for HttpError {
    fn from(err: JsonPayloadError) -> Self {
        HttpError::bad_request(err.to_string())
    }
}

impl From<QueryPayloadError> for HttpError {
    fn from(err: QueryPayloadError) -> Self {
        HttpError::bad_request(err.to_string())
    }
}

impl From<ApplicationError> for HttpError {
    fn from(err: ApplicationError) -> Self {
        match err {
            ApplicationError::NotFound { .. } => HttpError::Problem(
                ProblemDetails::new()
                    .with_status(HttpStatusCode::NOT_FOUND)
                    .with_title("Not Found")
                    .with_detail(err.to_string()),
            ),
            ApplicationError::Conflict { .. } => HttpError::precondition_failed(err.to_string()),
            ApplicationError::Internal { .. } => {
                HttpError::internal_server_error("an internal error occurred")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;
    use actix_web::ResponseError;
    use uuid::Uuid;

    #[actix_web::test]
    async fn maps_not_found_application_errors_to_404_problem_details() {
        let error = HttpError::from(ApplicationError::NotFound { id: Uuid::nil() });

        let response = error.error_response();

        assert_eq!(
            response.status().as_u16(),
            HttpStatusCode::NOT_FOUND.as_u16()
        );
        let body = to_bytes(response.into_body()).await.unwrap();
        let body = String::from_utf8(body.to_vec()).unwrap();
        assert!(body.contains("\"status\":404"));
        assert!(body.contains("todo item with id"));
    }

    #[actix_web::test]
    async fn sanitizes_internal_application_errors() {
        let error = HttpError::from(ApplicationError::internal("db exploded"));

        let response = error.error_response();

        assert_eq!(
            response.status().as_u16(),
            HttpStatusCode::INTERNAL_SERVER_ERROR.as_u16()
        );
        let body = to_bytes(response.into_body()).await.unwrap();
        let body = String::from_utf8(body.to_vec()).unwrap();
        assert!(body.contains("an internal error occurred"));
        assert!(!body.contains("db exploded"));
    }
}
