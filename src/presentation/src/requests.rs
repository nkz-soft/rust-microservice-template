use crate::errors::HttpError;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::{Validate, ValidationError};

const DEFAULT_PAGE: u32 = 1;
const DEFAULT_PAGE_SIZE: u32 = 20;

fn validate_not_blank(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::new("blank"));
    }

    Ok(())
}

fn default_page() -> u32 {
    DEFAULT_PAGE
}

fn default_page_size() -> u32 {
    DEFAULT_PAGE_SIZE
}

#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateToDoItemRequest {
    /// The title of the to-do item
    #[validate(length(min = 1, max = 120), custom(function = "validate_not_blank"))]
    pub title: String,
    /// The note of the to-do item
    #[validate(length(min = 1, max = 1000), custom(function = "validate_not_blank"))]
    pub note: String,
}

#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct UpdateToDoItemRequest {
    /// The title of the to-do item
    #[validate(length(min = 1, max = 120), custom(function = "validate_not_blank"))]
    pub title: String,
    /// The note of the to-do item
    #[validate(length(min = 1, max = 1000), custom(function = "validate_not_blank"))]
    pub note: String,
}

#[readonly::make]
#[derive(Deserialize, Serialize, IntoParams, ToSchema, Validate)]
#[into_params(parameter_in = Query)]
pub struct GetAllToDoItemsQueryRequest {
    /// One-based page number.
    #[serde(default = "default_page")]
    #[validate(range(min = 1, max = 10_000))]
    pub page: u32,
    /// Number of items returned per page.
    #[serde(default = "default_page_size")]
    #[validate(range(min = 1, max = 100))]
    pub page_size: u32,
    /// Optional case-insensitive search across title and note.
    #[serde(default)]
    #[validate(length(max = 100))]
    pub search: Option<String>,
}

impl Default for GetAllToDoItemsQueryRequest {
    fn default() -> Self {
        Self {
            page: default_page(),
            page_size: default_page_size(),
            search: None,
        }
    }
}

impl GetAllToDoItemsQueryRequest {
    pub fn offset(&self) -> usize {
        ((self.page - 1) * self.page_size) as usize
    }

    pub fn limit(&self) -> usize {
        self.page_size as usize
    }

    pub fn normalized_search(&self) -> Option<String> {
        self.search
            .as_ref()
            .map(|value| value.trim().to_ascii_lowercase())
    }

    pub fn validate_search(&self) -> Result<(), HttpError> {
        if let Some(value) = self.search.as_ref() {
            validate_not_blank(value).map_err(|err| HttpError::bad_request(err.to_string()))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_request_rejects_blank_title() {
        let request = CreateToDoItemRequest {
            title: "   ".into(),
            note: "note".into(),
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn update_request_rejects_blank_note() {
        let request = UpdateToDoItemRequest {
            title: "title".into(),
            note: "   ".into(),
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn query_request_uses_defaults() {
        let query = GetAllToDoItemsQueryRequest::default();

        assert_eq!(query.page, DEFAULT_PAGE);
        assert_eq!(query.page_size, DEFAULT_PAGE_SIZE);
        assert_eq!(query.offset(), 0);
        assert_eq!(query.limit(), DEFAULT_PAGE_SIZE as usize);
    }

    #[test]
    fn query_request_rejects_blank_search() {
        let query = GetAllToDoItemsQueryRequest {
            page: 1,
            page_size: 20,
            search: Some("   ".into()),
        };

        assert!(query.validate_search().is_err());
    }
}
