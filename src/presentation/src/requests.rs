use application::{GetAllToDoItemsQuery, SortDirection, ToDoItemSort, ToDoItemSortField};
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
    /// Sort order. Supported values: `id:asc`, `id:desc`, `title:asc`, `title:desc`.
    #[serde(default)]
    pub sort: Option<String>,
}

impl Default for GetAllToDoItemsQueryRequest {
    fn default() -> Self {
        Self {
            page: default_page(),
            page_size: default_page_size(),
            search: None,
            sort: None,
        }
    }
}

impl GetAllToDoItemsQueryRequest {
    pub fn normalized_search(&self) -> Option<String> {
        self.search.as_ref().map(|value| value.trim().to_string())
    }

    pub fn validate_search(&self) -> Result<(), String> {
        if let Some(value) = self.search.as_ref() {
            validate_not_blank(value).map_err(|err| err.to_string())?;
        }

        Ok(())
    }

    pub fn validate_sort(&self) -> Result<(), String> {
        if let Some(value) = self.sort.as_ref() {
            validate_not_blank(value).map_err(|err| err.to_string())?;
            parse_sort(value)?;
        }

        Ok(())
    }

    pub fn to_query(&self) -> Result<GetAllToDoItemsQuery, String> {
        Ok(GetAllToDoItemsQuery::new(
            self.page,
            self.page_size,
            self.normalized_search(),
            self.sort
                .as_deref()
                .map(parse_sort)
                .transpose()?
                .unwrap_or_default(),
        ))
    }
}

fn parse_sort(value: &str) -> Result<ToDoItemSort, String> {
    let normalized = value.trim().to_ascii_lowercase();
    let (field, direction) = normalized
        .split_once(':')
        .ok_or_else(|| "sort must use the format field:direction".to_string())?;

    let field = match field {
        "id" => ToDoItemSortField::Id,
        "title" => ToDoItemSortField::Title,
        _ => return Err("sort field must be one of: id, title".to_string()),
    };

    let direction = match direction {
        "asc" => SortDirection::Asc,
        "desc" => SortDirection::Desc,
        _ => return Err("sort direction must be one of: asc, desc".to_string()),
    };

    Ok(ToDoItemSort { field, direction })
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
        let mapped = query.to_query().expect("defaults should map");
        assert_eq!(mapped.page, DEFAULT_PAGE);
        assert_eq!(mapped.page_size, DEFAULT_PAGE_SIZE);
    }

    #[test]
    fn query_request_rejects_blank_search() {
        let query = GetAllToDoItemsQueryRequest {
            page: 1,
            page_size: 20,
            search: Some("   ".into()),
            sort: None,
        };

        assert!(query.validate_search().is_err());
    }

    #[test]
    fn query_request_rejects_invalid_sort() {
        let query = GetAllToDoItemsQueryRequest {
            page: 1,
            page_size: 20,
            search: None,
            sort: Some("status:asc".into()),
        };

        assert!(query.validate_sort().is_err());
    }
}
