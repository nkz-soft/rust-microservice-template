use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use uuid::Uuid;

#[readonly::make]
#[derive(Deserialize, Serialize)]
pub struct ToDoItemDto {
    pub id: Uuid,
    pub title: String,
    pub note: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

impl ToDoItemDto {
    pub fn get() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: "Test title".into(),
            note: "Test note".into(),
            status: "pending".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            due_at: None,
            deleted_at: None,
            deleted_by: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrincipalType {
    User,
    Service,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    #[serde(rename = "todo:read")]
    TodoRead,
    #[serde(rename = "todo:write")]
    TodoWrite,
    #[serde(rename = "audit:read")]
    AuditRead,
}

impl Permission {
    pub fn as_str(self) -> &'static str {
        match self {
            Permission::TodoRead => "todo:read",
            Permission::TodoWrite => "todo:write",
            Permission::AuditRead => "audit:read",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticatedPrincipal {
    pub subject: String,
    pub principal_type: PrincipalType,
    pub permissions: BTreeSet<Permission>,
}

impl AuthenticatedPrincipal {
    pub fn new(
        subject: impl Into<String>,
        principal_type: PrincipalType,
        permissions: impl IntoIterator<Item = Permission>,
    ) -> Self {
        Self {
            subject: subject.into(),
            principal_type,
            permissions: permissions.into_iter().collect(),
        }
    }

    pub fn has_permission(&self, permission: Permission) -> bool {
        self.permissions.contains(&permission)
    }

    pub fn permission_names(&self) -> Vec<String> {
        self.permissions
            .iter()
            .map(|permission| permission.as_str().to_string())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iss: String,
    pub aud: String,
    pub iat: i64,
    pub exp: i64,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub permissions: Vec<String>,
}
