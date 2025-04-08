use actix_web::web::Json;
use actix_web::{HttpResponse, ResponseError};
use http::StatusCode as HttpStatusCode;
use infrastructure as errors;
use problem_details::{JsonProblemDetails, ProblemDetails};
use serde_json::Error as SerdeError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum HttpError {
    Problem(ProblemDetails),
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
        HttpError::Problem(
            ProblemDetails::new()
                .with_status(HttpStatusCode::INTERNAL_SERVER_ERROR)
                .with_detail(err.to_string()),
        )
    }
}

impl From<anyhow::Error> for HttpError {
    fn from(err: anyhow::Error) -> Self {
        if let Some(err) = err.downcast_ref::<errors::Error>() {
            match err {
                errors::Error::ItemNotFound { .. } => HttpError::Problem(
                    ProblemDetails::new()
                        .with_status(HttpStatusCode::NOT_FOUND)
                        .with_detail(err.to_string()),
                ),
                _ => HttpError::Problem(
                    ProblemDetails::new()
                        .with_status(HttpStatusCode::INTERNAL_SERVER_ERROR)
                        .with_detail(err.to_string()),
                ),
            }
        } else {
            HttpError::Problem(
                ProblemDetails::new()
                    .with_status(HttpStatusCode::INTERNAL_SERVER_ERROR)
                    .with_detail(err.to_string()),
            )
        }
    }
}
