use crate::dtos::{AuthenticatedPrincipal, PrincipalType, TokenClaims, TokenResponse};
use crate::handlers::*;
use crate::queries::ProtectedEndpointPolicy;
use crate::repositories::ToDoItemRepository;
use crate::settings::{AuthSettings, AuthUser, ServiceApiKey};
use argon2::{Argon2, PasswordVerifier};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use password_hash::PasswordHash;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

/// Service container that manages all query handlers with proper dependency injection.
#[derive(Clone)]
pub struct ToDoItemService {
    get_handler: Arc<GetToDoItemQueryHandler>,
    get_all_handler: Arc<GetAllToDoItemQueryHandler>,
    create_handler: Arc<CreateToDoItemQueryHandler>,
    update_handler: Arc<UpdateToDoItemQueryHandler>,
    delete_handler: Arc<DeleteToDoItemQueryHandler>,
    get_deleted_for_audit_handler: Arc<GetDeletedToDoItemForAuditQueryHandler>,
}

impl ToDoItemService {
    pub fn new(repository: Arc<dyn ToDoItemRepository + Send + Sync>) -> Self {
        Self {
            get_handler: Arc::new(GetToDoItemQueryHandler::new(repository.clone())),
            get_all_handler: Arc::new(GetAllToDoItemQueryHandler::new(repository.clone())),
            create_handler: Arc::new(CreateToDoItemQueryHandler::new(repository.clone())),
            update_handler: Arc::new(UpdateToDoItemQueryHandler::new(repository.clone())),
            delete_handler: Arc::new(DeleteToDoItemQueryHandler::new(repository.clone())),
            get_deleted_for_audit_handler: Arc::new(GetDeletedToDoItemForAuditQueryHandler::new(
                repository,
            )),
        }
    }

    pub fn get_handler(&self) -> Arc<GetToDoItemQueryHandler> {
        self.get_handler.clone()
    }

    pub fn get_all_handler(&self) -> Arc<GetAllToDoItemQueryHandler> {
        self.get_all_handler.clone()
    }

    pub fn create_handler(&self) -> Arc<CreateToDoItemQueryHandler> {
        self.create_handler.clone()
    }

    pub fn update_handler(&self) -> Arc<UpdateToDoItemQueryHandler> {
        self.update_handler.clone()
    }

    pub fn delete_handler(&self) -> Arc<DeleteToDoItemQueryHandler> {
        self.delete_handler.clone()
    }

    pub fn get_deleted_for_audit_handler(&self) -> Arc<GetDeletedToDoItemForAuditQueryHandler> {
        self.get_deleted_for_audit_handler.clone()
    }
}

pub struct ToDoItemServiceBoxed {
    repository: Arc<dyn ToDoItemRepository + Send + Sync>,
}

impl ToDoItemServiceBoxed {
    pub fn new(repository: Arc<dyn ToDoItemRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub fn create_get_handler(&self) -> Box<GetToDoItemQueryHandler> {
        Box::new(GetToDoItemQueryHandler::new(self.repository.clone()))
    }

    pub fn create_get_all_handler(&self) -> Box<GetAllToDoItemQueryHandler> {
        Box::new(GetAllToDoItemQueryHandler::new(self.repository.clone()))
    }

    pub fn create_create_handler(&self) -> Box<CreateToDoItemQueryHandler> {
        Box::new(CreateToDoItemQueryHandler::new(self.repository.clone()))
    }

    pub fn create_update_handler(&self) -> Box<UpdateToDoItemQueryHandler> {
        Box::new(UpdateToDoItemQueryHandler::new(self.repository.clone()))
    }

    pub fn create_delete_handler(&self) -> Box<DeleteToDoItemQueryHandler> {
        Box::new(DeleteToDoItemQueryHandler::new(self.repository.clone()))
    }

    pub fn create_get_deleted_for_audit_handler(
        &self,
    ) -> Box<GetDeletedToDoItemForAuditQueryHandler> {
        Box::new(GetDeletedToDoItemForAuditQueryHandler::new(
            self.repository.clone(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct AuthService {
    settings: AuthSettings,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
    users: HashMap<String, AuthUser>,
    services: Vec<ServiceApiKey>,
}

#[derive(Debug, Clone, Error)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("missing bearer token")]
    MissingBearerToken,
    #[error("invalid bearer token")]
    InvalidBearerToken,
    #[error("missing service API key")]
    MissingServiceApiKey,
    #[error("invalid service API key")]
    InvalidServiceApiKey,
    #[error("forbidden")]
    Forbidden,
    #[error("auth configuration error: {0}")]
    Configuration(String),
}

impl AuthService {
    pub fn new(settings: AuthSettings) -> Result<Self, AuthError> {
        if settings.jwt_signing_secret.trim().is_empty() {
            return Err(AuthError::Configuration(
                "jwt_signing_secret must not be blank".into(),
            ));
        }

        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(std::slice::from_ref(&settings.jwt_issuer));
        validation.set_audience(std::slice::from_ref(&settings.jwt_audience));

        Ok(Self {
            encoding_key: EncodingKey::from_secret(settings.jwt_signing_secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(settings.jwt_signing_secret.as_bytes()),
            users: settings
                .users
                .iter()
                .cloned()
                .map(|user| (user.username.clone(), user))
                .collect(),
            services: settings.services.clone(),
            validation,
            settings,
        })
    }

    pub fn authenticate_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<TokenResponse, AuthError> {
        let username = username.trim();
        let user = self
            .users
            .get(username)
            .ok_or(AuthError::InvalidCredentials)?;

        self.verify_password(password, &user.password_hash)?;
        self.issue_user_token(user)
    }

    pub fn authenticate_bearer_token(
        &self,
        token: &str,
    ) -> Result<AuthenticatedPrincipal, AuthError> {
        let token = token.trim();
        if token.is_empty() {
            return Err(AuthError::MissingBearerToken);
        }

        let claims = decode::<TokenClaims>(token, &self.decoding_key, &self.validation)
            .map_err(|_| AuthError::InvalidBearerToken)?
            .claims;

        Ok(AuthenticatedPrincipal::new(
            claims.sub.clone(),
            PrincipalType::User,
            claims.permissions.clone(),
        ))
    }

    pub fn authenticate_service_api_key(
        &self,
        header_name: &str,
        key: &str,
    ) -> Result<AuthenticatedPrincipal, AuthError> {
        let normalized_key = key.trim();
        if normalized_key.is_empty() {
            return Err(AuthError::MissingServiceApiKey);
        }

        let service = self
            .services
            .iter()
            .find(|service| {
                service.header_name.eq_ignore_ascii_case(header_name)
                    && service.key == normalized_key
            })
            .ok_or(AuthError::InvalidServiceApiKey)?;

        Ok(AuthenticatedPrincipal::new(
            service.service_name.clone(),
            PrincipalType::Service,
            service.permissions.clone(),
        ))
    }

    pub fn service_header_names(&self) -> Vec<String> {
        let mut header_names: Vec<String> = self
            .services
            .iter()
            .map(|service| service.header_name.clone())
            .collect();
        header_names.sort();
        header_names.dedup();
        header_names
    }

    pub fn authorize(
        &self,
        principal: &AuthenticatedPrincipal,
        policy: &ProtectedEndpointPolicy,
    ) -> Result<(), AuthError> {
        if !policy
            .accepted_principal_types
            .contains(&principal.principal_type)
        {
            return Err(AuthError::Forbidden);
        }

        if !principal.has_permission(policy.required_permission) {
            return Err(AuthError::Forbidden);
        }

        Ok(())
    }

    fn verify_password(&self, password: &str, password_hash: &str) -> Result<(), AuthError> {
        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|err| AuthError::Configuration(err.to_string()))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AuthError::InvalidCredentials)
    }

    fn issue_user_token(&self, user: &AuthUser) -> Result<TokenResponse, AuthError> {
        let now = chrono::Utc::now().timestamp();
        let claims = TokenClaims {
            sub: user.username.clone(),
            iss: self.settings.jwt_issuer.clone(),
            aud: self.settings.jwt_audience.clone(),
            iat: now,
            exp: now + self.settings.jwt_ttl_seconds,
            permissions: user.permissions.clone(),
        };

        let access_token = encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|err| AuthError::Configuration(err.to_string()))?;

        Ok(TokenResponse {
            access_token,
            token_type: "Bearer".into(),
            expires_in: self.settings.jwt_ttl_seconds,
            permissions: claims
                .permissions
                .iter()
                .map(|permission| permission.as_str().to_string())
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GetAllToDoItemsQuery, LoginQuery, PaginatedResult, Permission};
    use async_trait::async_trait;
    use domain::ToDoItem;
    use std::sync::{Arc, Mutex};
    use tokio::task;
    use uuid::Uuid;

    const PASSWORD_HASH: &str =
        "$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQ$PL01amPyeUuxG7H0vIr5X+qHkZvWnHmGBGXFYvh8z2E";

    struct MockToDoItemRepository {
        items: Arc<Mutex<Vec<ToDoItem>>>,
        call_count: Arc<Mutex<usize>>,
    }

    impl MockToDoItemRepository {
        fn new() -> Self {
            Self {
                items: Arc::new(Mutex::new(Vec::new())),
                call_count: Arc::new(Mutex::new(0)),
            }
        }

        fn get_call_count(&self) -> usize {
            *self.call_count.lock().unwrap()
        }

        fn add_item(&self, item: ToDoItem) {
            self.items.lock().unwrap().push(item);
        }
    }

    #[async_trait]
    impl ToDoItemRepository for MockToDoItemRepository {
        async fn get_all(
            &self,
            query: GetAllToDoItemsQuery,
        ) -> anyhow::Result<PaginatedResult<ToDoItem>> {
            *self.call_count.lock().unwrap() += 1;
            let items = self.items.lock().unwrap().clone();
            let total_items = items.len() as i64;
            let paged_items = items
                .into_iter()
                .skip(query.offset() as usize)
                .take(query.limit() as usize)
                .collect();

            Ok(PaginatedResult::new(
                paged_items,
                query.page,
                query.page_size,
                total_items,
            ))
        }

        async fn get_by_id(&self, id: Uuid) -> anyhow::Result<ToDoItem> {
            *self.call_count.lock().unwrap() += 1;
            self.items
                .lock()
                .unwrap()
                .iter()
                .find(|item| item.id == id)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Item not found"))
        }

        async fn create(&self, entity: ToDoItem) -> anyhow::Result<Uuid> {
            *self.call_count.lock().unwrap() += 1;
            let id = entity.id;
            self.items.lock().unwrap().push(entity);
            Ok(id)
        }

        async fn update(&self, entity: ToDoItem) -> anyhow::Result<Uuid> {
            *self.call_count.lock().unwrap() += 1;
            let id = entity.id;
            let mut items = self.items.lock().unwrap();
            let existing = items
                .iter_mut()
                .find(|item| item.id == id)
                .ok_or_else(|| anyhow::anyhow!("Item not found"))?;

            if existing.version != entity.version {
                return Err(anyhow::anyhow!("Version conflict"));
            }

            existing.title = entity.title;
            existing.note = entity.note;
            existing.status = entity.status;
            existing.due_at = entity.due_at;
            existing.updated_at = chrono::Utc::now();
            existing.version += 1;

            Ok(id)
        }

        async fn delete(&self, id: Uuid, _deleted_by: Option<Uuid>) -> anyhow::Result<()> {
            *self.call_count.lock().unwrap() += 1;
            let mut items = self.items.lock().unwrap();
            items.retain(|item| item.id != id);
            Ok(())
        }

        async fn get_deleted_by_id_for_audit(&self, id: Uuid) -> anyhow::Result<ToDoItem> {
            *self.call_count.lock().unwrap() += 1;
            self.items
                .lock()
                .unwrap()
                .iter()
                .find(|item| item.id == id && item.deleted_at.is_some())
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Item not found"))
        }
    }

    fn auth_settings() -> AuthSettings {
        AuthSettings {
            jwt_issuer: "rust-template-service".into(),
            jwt_audience: "rust-template-clients".into(),
            jwt_signing_secret: "replace-for-local-dev-only".into(),
            jwt_ttl_seconds: 3600,
            users: vec![AuthUser {
                username: "demo-user".into(),
                password_hash: PASSWORD_HASH.into(),
                permissions: vec![Permission::TodoRead, Permission::TodoWrite],
                roles: vec!["writer".into()],
            }],
            services: vec![ServiceApiKey {
                service_name: "audit-client".into(),
                header_name: "X-Service-Api-Key".into(),
                key: "local-service-key".into(),
                permissions: vec![Permission::AuditRead],
            }],
        }
    }

    #[tokio::test]
    async fn test_service_creation_with_arc() {
        let repository = Arc::new(MockToDoItemRepository::new());
        let service = ToDoItemService::new(repository);

        assert!(Arc::strong_count(&service.get_handler()) >= 1);
        assert!(Arc::strong_count(&service.get_all_handler()) >= 1);
        assert!(Arc::strong_count(&service.create_handler()) >= 1);
        assert!(Arc::strong_count(&service.update_handler()) >= 1);
        assert!(Arc::strong_count(&service.delete_handler()) >= 1);
        assert!(Arc::strong_count(&service.get_deleted_for_audit_handler()) >= 1);
    }

    #[tokio::test]
    async fn test_concurrent_access() {
        let repository = Arc::new(MockToDoItemRepository::new());
        let service = Arc::new(ToDoItemService::new(repository.clone()));

        repository.add_item(ToDoItem::new("Test 1".to_string(), "Note 1".to_string()));
        repository.add_item(ToDoItem::new("Test 2".to_string(), "Note 2".to_string()));

        let mut handles = vec![];

        for i in 0..10 {
            let service_clone = service.clone();
            let handle = task::spawn(async move {
                let handler = service_clone.get_all_handler();
                let result = handler.execute(GetAllToDoItemsQuery::default()).await;
                (i, result)
            });
            handles.push(handle);
        }

        for handle in handles {
            let (task_id, result) = handle.await.unwrap();
            assert!(result.is_ok(), "Task {} failed", task_id);
            assert_eq!(result.unwrap().items.len(), 2);
        }

        assert_eq!(repository.get_call_count(), 10);
    }

    #[test]
    fn authenticate_user_issues_token() {
        let auth_service = AuthService::new(auth_settings()).expect("auth service should build");

        let response = auth_service
            .authenticate_user("demo-user", "password")
            .expect("credentials should authenticate");

        assert_eq!(response.token_type, "Bearer");
        assert_eq!(response.permissions, vec!["todo:read", "todo:write"]);
    }

    #[test]
    fn authenticate_user_rejects_invalid_password() {
        let auth_service = AuthService::new(auth_settings()).expect("auth service should build");

        let error = auth_service
            .authenticate_user("demo-user", "wrong-password")
            .expect_err("invalid password must fail");

        assert!(matches!(error, AuthError::InvalidCredentials));
    }

    #[test]
    fn authenticate_bearer_token_rejects_invalid_claims() {
        let auth_service = AuthService::new(auth_settings()).expect("auth service should build");

        let error = auth_service
            .authenticate_bearer_token("not-a-jwt")
            .expect_err("bad token must fail");

        assert!(matches!(error, AuthError::InvalidBearerToken));
    }

    #[test]
    fn authenticate_service_api_key_resolves_service_principal() {
        let auth_service = AuthService::new(auth_settings()).expect("auth service should build");

        let principal = auth_service
            .authenticate_service_api_key("X-Service-Api-Key", "local-service-key")
            .expect("service key should authenticate");

        assert_eq!(principal.subject, "audit-client");
        assert_eq!(principal.principal_type, PrincipalType::Service);
        assert!(principal.has_permission(Permission::AuditRead));
    }

    #[test]
    fn authorize_rejects_missing_permission() {
        let auth_service = AuthService::new(auth_settings()).expect("auth service should build");
        let principal =
            AuthenticatedPrincipal::new("demo-user", PrincipalType::User, [Permission::TodoRead]);
        let policy = ProtectedEndpointPolicy::new(Permission::TodoWrite, vec![PrincipalType::User]);

        let error = auth_service
            .authorize(&principal, &policy)
            .expect_err("missing permission must fail");

        assert!(matches!(error, AuthError::Forbidden));
    }

    #[tokio::test]
    async fn login_handler_uses_auth_service() {
        let handler = LoginQueryHandler::new(Arc::new(
            AuthService::new(auth_settings()).expect("auth service should build"),
        ));

        let response = handler
            .execute(LoginQuery::new("demo-user", "password"))
            .await
            .expect("login should succeed");

        assert_eq!(response.token_type, "Bearer");
    }
}
