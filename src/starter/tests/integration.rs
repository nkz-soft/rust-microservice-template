mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::test_server::TEST_SERVER_ONCE;
    use crate::{prepare_test_environment, utils::test_server};
    use ctor::dtor;
    use reqwest::StatusCode;
    use serde_json::{json, Value};
    use serial_test::serial;
    use uuid::Uuid;

    const WEB_SERVER_PATH: &str = "http://localhost:8181/api/v1/";
    const USER_PASSWORD: &str = "password";
    const AUDIT_SERVICE_KEY: &str = "local-service-key";
    const RESTRICTED_SERVICE_KEY: &str = "restricted-service-key";

    #[dtor]
    fn cleanup() {
        let id = TEST_SERVER_ONCE.get().unwrap().container().id();

        std::process::Command::new("docker")
            .arg("kill")
            .arg(id)
            .output()
            .expect("failed to kill container");
    }

    async fn issue_token(client: &reqwest::Client, username: &str, password: &str) -> String {
        let response = client
            .post(format!("{WEB_SERVER_PATH}auth/token"))
            .json(&json!({
                "username": username,
                "password": password
            }))
            .send()
            .await
            .expect("token request should execute");

        assert_eq!(response.status(), StatusCode::OK);
        response
            .json::<Value>()
            .await
            .expect("token response should deserialize")["access_token"]
            .as_str()
            .expect("access token should be present")
            .to_string()
    }

    async fn create_todo(client: &reqwest::Client, token: &str, title: &str) -> Uuid {
        client
            .post(format!("{WEB_SERVER_PATH}to-do-items"))
            .bearer_auth(token)
            .json(&json!({
                "title": title,
                "note": "note1",
                "status": "pending"
            }))
            .send()
            .await
            .expect("create request should execute")
            .json::<Uuid>()
            .await
            .expect("create response should deserialize")
    }

    #[serial]
    #[tokio::test]
    async fn open_health_and_auth_routes_remain_open() {
        let client = prepare_test_environment!();

        let ready_response = client
            .get(format!("{WEB_SERVER_PATH}healthz/ready"))
            .send()
            .await
            .expect("health request should execute");
        assert_eq!(ready_response.status(), StatusCode::OK);

        let auth_response = client
            .post(format!("{WEB_SERVER_PATH}auth/token"))
            .json(&json!({
                "username": "",
                "password": ""
            }))
            .send()
            .await
            .expect("auth request should execute");
        assert_eq!(auth_response.status(), StatusCode::BAD_REQUEST);
    }

    #[serial]
    #[tokio::test]
    async fn token_issuance_and_bearer_protection_work() {
        let client = prepare_test_environment!();

        let missing_auth_response = client
            .get(format!("{WEB_SERVER_PATH}to-do-items"))
            .send()
            .await
            .expect("request should execute");
        assert_eq!(missing_auth_response.status(), StatusCode::UNAUTHORIZED);

        let invalid_login_response = client
            .post(format!("{WEB_SERVER_PATH}auth/token"))
            .json(&json!({
                "username": "demo-user",
                "password": "wrong-password"
            }))
            .send()
            .await
            .expect("invalid login should execute");
        assert_eq!(invalid_login_response.status(), StatusCode::UNAUTHORIZED);

        let token = issue_token(&client, "demo-user", USER_PASSWORD).await;

        let authorized_response = client
            .get(format!("{WEB_SERVER_PATH}to-do-items?page=1&page_size=10"))
            .bearer_auth(&token)
            .send()
            .await
            .expect("authorized request should execute");
        assert_eq!(authorized_response.status(), StatusCode::OK);
    }

    #[serial]
    #[tokio::test]
    async fn authenticated_user_can_complete_todo_crud_flow() {
        let client = prepare_test_environment!();
        let token = issue_token(&client, "demo-user", USER_PASSWORD).await;

        let id = create_todo(&client, &token, "crud-title").await;

        let get_response = client
            .get(format!("{WEB_SERVER_PATH}to-do-items/{id}"))
            .bearer_auth(&token)
            .send()
            .await
            .expect("get request should execute");
        assert_eq!(get_response.status(), StatusCode::OK);
        let etag = get_response
            .headers()
            .get("etag")
            .expect("etag must be present")
            .to_str()
            .expect("etag must be ascii")
            .to_string();

        let update_response = client
            .put(format!("{WEB_SERVER_PATH}to-do-items/{id}"))
            .bearer_auth(&token)
            .header("If-Match", etag.clone())
            .json(&json!({
                "title": "crud-title",
                "note": "updated-note",
                "status": "in_progress"
            }))
            .send()
            .await
            .expect("update request should execute");
        assert_eq!(update_response.status(), StatusCode::OK);

        let stale_update_response = client
            .put(format!("{WEB_SERVER_PATH}to-do-items/{id}"))
            .bearer_auth(&token)
            .header("If-Match", etag)
            .json(&json!({
                "title": "crud-title",
                "note": "stale-note",
                "status": "done"
            }))
            .send()
            .await
            .expect("stale update request should execute");
        assert_eq!(
            stale_update_response.status(),
            StatusCode::PRECONDITION_FAILED
        );

        let delete_response = client
            .delete(format!("{WEB_SERVER_PATH}to-do-items/{id}"))
            .bearer_auth(&token)
            .header("X-Actor-Id", Uuid::new_v4().to_string())
            .send()
            .await
            .expect("delete request should execute");
        assert_eq!(delete_response.status(), StatusCode::OK);

        let hidden_response = client
            .get(format!("{WEB_SERVER_PATH}to-do-items/{id}"))
            .bearer_auth(&token)
            .send()
            .await
            .expect("hidden get request should execute");
        assert_eq!(hidden_response.status(), StatusCode::NOT_FOUND);
    }

    #[serial]
    #[tokio::test]
    async fn service_api_key_can_read_deleted_item_for_audit() {
        let client = prepare_test_environment!();
        let token = issue_token(&client, "demo-user", USER_PASSWORD).await;
        let actor_id = Uuid::new_v4();

        let id = create_todo(&client, &token, "audit-visible").await;

        let delete_response = client
            .delete(format!("{WEB_SERVER_PATH}to-do-items/{id}"))
            .bearer_auth(&token)
            .header("X-Actor-Id", actor_id.to_string())
            .send()
            .await
            .expect("delete request should execute");
        assert_eq!(delete_response.status(), StatusCode::OK);

        let audit_response = client
            .get(format!("{WEB_SERVER_PATH}audit/to-do-items/{id}"))
            .header("X-Service-Api-Key", AUDIT_SERVICE_KEY)
            .send()
            .await
            .expect("audit request should execute");
        assert_eq!(audit_response.status(), StatusCode::OK);

        let body = audit_response
            .json::<Value>()
            .await
            .expect("audit response should deserialize");
        assert_eq!(body["id"], id.to_string());
        assert_eq!(body["deleted_by"], actor_id.to_string());
        assert!(body["deleted_at"].is_string());
    }

    #[serial]
    #[tokio::test]
    async fn audit_endpoint_rejects_missing_or_invalid_service_key() {
        let client = prepare_test_environment!();
        let token = issue_token(&client, "demo-user", USER_PASSWORD).await;
        let id = create_todo(&client, &token, "audit-auth").await;

        let delete_response = client
            .delete(format!("{WEB_SERVER_PATH}to-do-items/{id}"))
            .bearer_auth(&token)
            .send()
            .await
            .expect("delete request should execute");
        assert_eq!(delete_response.status(), StatusCode::OK);

        let missing_key_response = client
            .get(format!("{WEB_SERVER_PATH}audit/to-do-items/{id}"))
            .send()
            .await
            .expect("missing-key request should execute");
        assert_eq!(missing_key_response.status(), StatusCode::UNAUTHORIZED);

        let invalid_key_response = client
            .get(format!("{WEB_SERVER_PATH}audit/to-do-items/{id}"))
            .header("X-Service-Api-Key", "wrong-key")
            .send()
            .await
            .expect("invalid-key request should execute");
        assert_eq!(invalid_key_response.status(), StatusCode::UNAUTHORIZED);
    }

    #[serial]
    #[tokio::test]
    async fn insufficient_user_permission_returns_forbidden() {
        let client = prepare_test_environment!();
        let token = issue_token(&client, "read-only-user", USER_PASSWORD).await;

        let create_response = client
            .post(format!("{WEB_SERVER_PATH}to-do-items"))
            .bearer_auth(&token)
            .json(&json!({
                "title": "forbidden-write",
                "note": "note1",
                "status": "pending"
            }))
            .send()
            .await
            .expect("forbidden create request should execute");

        assert_eq!(create_response.status(), StatusCode::FORBIDDEN);
    }

    #[serial]
    #[tokio::test]
    async fn insufficient_service_permission_returns_forbidden() {
        let client = prepare_test_environment!();
        let token = issue_token(&client, "demo-user", USER_PASSWORD).await;
        let id = create_todo(&client, &token, "forbidden-audit").await;

        let delete_response = client
            .delete(format!("{WEB_SERVER_PATH}to-do-items/{id}"))
            .bearer_auth(&token)
            .send()
            .await
            .expect("delete request should execute");
        assert_eq!(delete_response.status(), StatusCode::OK);

        let forbidden_response = client
            .get(format!("{WEB_SERVER_PATH}audit/to-do-items/{id}"))
            .header("X-Service-Api-Key", RESTRICTED_SERVICE_KEY)
            .send()
            .await
            .expect("forbidden audit request should execute");

        assert_eq!(forbidden_response.status(), StatusCode::FORBIDDEN);
    }

    #[serial]
    #[tokio::test]
    async fn protected_query_validation_and_search_still_work() {
        let client = prepare_test_environment!();
        let token = issue_token(&client, "demo-user", USER_PASSWORD).await;
        let title = format!("search-title-{}", Uuid::new_v4());

        let _ = create_todo(&client, &token, &title).await;

        let invalid_query_response = client
            .get(format!("{WEB_SERVER_PATH}to-do-items?page=0&page_size=10"))
            .bearer_auth(&token)
            .send()
            .await
            .expect("invalid query request should execute");
        assert_eq!(invalid_query_response.status(), StatusCode::BAD_REQUEST);

        let search_response = client
            .get(format!(
                "{WEB_SERVER_PATH}to-do-items?page=1&page_size=10&search={title}&sort=title:asc"
            ))
            .bearer_auth(&token)
            .send()
            .await
            .expect("search request should execute");
        assert_eq!(search_response.status(), StatusCode::OK);

        let body = search_response
            .json::<Value>()
            .await
            .expect("search response should deserialize");
        assert_eq!(body["meta"]["total_items"], 1);
        assert_eq!(body["items"][0]["title"], title);
    }

    #[serial]
    #[tokio::test]
    async fn protected_body_validation_still_returns_bad_request() {
        let client = prepare_test_environment!();
        let token = issue_token(&client, "demo-user", USER_PASSWORD).await;

        let response = client
            .post(format!("{WEB_SERVER_PATH}to-do-items"))
            .bearer_auth(&token)
            .json(&json!({
                "title": "   ",
                "note": "note1",
                "status": "pending"
            }))
            .send()
            .await
            .expect("invalid body request should execute");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
