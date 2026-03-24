mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::test_server;
    use chrono::DateTime;
    use ctor::dtor;
    use reqwest::StatusCode;
    use serde_json::json;
    use serde_json::Value;
    use serial_test::serial;
    use std::collections::HashMap;
    use uuid::Uuid;

    use crate::{prepare_test_environment, utils::test_server::TEST_SERVER_ONCE};

    const WEB_SERVER_PATH: &str = "http://localhost:8181/api/v1/";
    const METRICS_PATH: &str = "http://localhost:8181/metrics";
    const AUDIT_TOKEN: &str = "local-audit-token";

    #[dtor]
    fn cleanup() {
        if let Some(server) = TEST_SERVER_ONCE.get() {
            let id = server.container().id();
            let _ = std::process::Command::new("docker")
                .arg("kill")
                .arg(id)
                .output();
        }
    }

    #[serial]
    #[tokio::test]
    async fn start_server_and_test() {
        let client = prepare_test_environment!();
        assert!(client.get(WEB_SERVER_PATH).send().await.is_ok());
    }

    #[serial]
    #[tokio::test]
    async fn test_get_all() {
        let client = prepare_test_environment!();

        let response = client
            .get(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert!(response.headers().get("x-request-id").is_some());
        let body = response
            .json::<Value>()
            .await
            .expect("Failed to deserialize response.");
        assert!(body.get("items").is_some());
        assert!(body.get("meta").is_some());
    }

    #[serial]
    #[tokio::test]
    async fn test_request_id_is_generated_when_missing() {
        let client = prepare_test_environment!();

        let response = client
            .get(WEB_SERVER_PATH.to_owned() + "to-do-items?page=1&page_size=1")
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::OK);
        let request_id = response
            .headers()
            .get("x-request-id")
            .expect("x-request-id header should be set")
            .to_str()
            .expect("x-request-id should be ASCII");
        assert!(!request_id.trim().is_empty());
    }

    #[serial]
    #[tokio::test]
    async fn test_request_id_is_preserved_when_provided() {
        let client = prepare_test_environment!();
        let expected_request_id = format!("integration-{}", Uuid::new_v4());

        let response = client
            .get(WEB_SERVER_PATH.to_owned() + "to-do-items?page=1&page_size=1")
            .header("X-Request-Id", expected_request_id.clone())
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::OK);
        let actual_request_id = response
            .headers()
            .get("x-request-id")
            .expect("x-request-id header should be set")
            .to_str()
            .expect("x-request-id should be ASCII");
        assert_eq!(actual_request_id, expected_request_id);
    }

    #[serial]
    #[tokio::test]
    async fn test_failed_response_includes_request_id() {
        let client = prepare_test_environment!();

        let response = client
            .get(WEB_SERVER_PATH.to_owned() + "to-do-items?page=0&page_size=20")
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert!(response.headers().get("x-request-id").is_some());
    }

    #[serial]
    #[tokio::test]
    async fn test_ready() {
        let client = prepare_test_environment!();

        let response = client
            .get(WEB_SERVER_PATH.to_owned() + "healthz/ready")
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
    }

    #[serial]
    #[tokio::test]
    async fn test_get_by_id() {
        let client = prepare_test_environment!();
        let mut map_create = HashMap::new();
        map_create.insert("title", "title1");
        map_create.insert("note", "note1");
        map_create.insert("status", "pending");

        let id = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&map_create)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let response = client
            .get(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert!(response.headers().get("etag").is_some());

        let body = response
            .json::<Value>()
            .await
            .expect("Failed to deserialize response.");
        assert_eq!(body["status"], "pending");
        assert!(body["created_at"].is_string());
        assert!(body["updated_at"].is_string());
        assert!(body["due_at"].is_null());
    }

    #[serial]
    #[tokio::test]
    async fn test_create() {
        let client = prepare_test_environment!();
        let mut map = HashMap::new();
        map.insert("title", "title1");
        map.insert("note", "note1");
        map.insert("status", "pending");

        let response = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&map)
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
    }

    #[serial]
    #[tokio::test]
    async fn test_create_populates_lifecycle_metadata() {
        let client = prepare_test_environment!();

        let id = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&json!({
                "title": "title1",
                "note": "note1"
            }))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let response = client
            .get(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .send()
            .await
            .expect("Failed to execute request.");

        let body = response
            .json::<Value>()
            .await
            .expect("Failed to deserialize response.");

        assert_eq!(body["status"], "pending");
        let created_at = body["created_at"]
            .as_str()
            .expect("created_at should be present");
        let updated_at = body["updated_at"]
            .as_str()
            .expect("updated_at should be present");

        assert!(DateTime::parse_from_rfc3339(created_at).is_ok());
        assert!(DateTime::parse_from_rfc3339(updated_at).is_ok());
        assert_eq!(created_at, updated_at);
        assert!(body["due_at"].is_null());
    }

    #[serial]
    #[tokio::test]
    async fn test_update() {
        let client = prepare_test_environment!();
        let mut map_create = HashMap::new();
        map_create.insert("title", "title1");
        map_create.insert("note", "note1");
        map_create.insert("status", "pending");

        let id = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&map_create)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let item_response = client
            .get(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .send()
            .await
            .expect("Failed to execute request.");
        let etag = item_response
            .headers()
            .get("etag")
            .expect("ETag header must be present")
            .to_str()
            .expect("ETag must be ascii")
            .to_string();
        let item = item_response
            .json::<Value>()
            .await
            .expect("Failed to deserialize response.");
        let original_updated_at = item["updated_at"]
            .as_str()
            .expect("updated_at should be present")
            .to_string();

        let due_at = "2030-01-15T12:00:00Z";
        let response = client
            .put(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .header("If-Match", etag)
            .json(&json!({
                "title": "title1",
                "note": "note1",
                "status": "in_progress",
                "due_at": due_at
            }))
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert_eq!(
            response
                .headers()
                .get("etag")
                .expect("ETag header must be present"),
            "\"2\""
        );

        let updated_item = client
            .get(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Value>()
            .await
            .expect("Failed to deserialize response.");

        assert_eq!(updated_item["status"], "in_progress");
        assert_eq!(updated_item["due_at"], due_at);
        assert_ne!(
            updated_item["updated_at"]
                .as_str()
                .expect("updated_at should be present"),
            original_updated_at
        );
    }

    #[serial]
    #[tokio::test]
    async fn test_delete() {
        let client = prepare_test_environment!();
        let mut map_create = HashMap::new();
        map_create.insert("title", "title1");
        map_create.insert("note", "note1");
        map_create.insert("status", "pending");

        let id = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&map_create)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let response = client
            .delete(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_hides_item_from_standard_get_and_list() {
        let client = prepare_test_environment!();
        let unique_title = format!("hidden-{}", Uuid::new_v4());
        let actor_id = Uuid::new_v4();

        let id = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&json!({
                "title": unique_title,
                "note": "note1",
                "status": "pending"
            }))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let response = client
            .delete(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .header("X-Actor-Id", actor_id.to_string())
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());

        let get_response = client
            .get(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(get_response.status(), StatusCode::NOT_FOUND);

        let list_response = client
            .get(
                WEB_SERVER_PATH.to_owned()
                    + format!("to-do-items?search={}&page=1&page_size=10", unique_title).as_str(),
            )
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(list_response.status(), StatusCode::OK);
        let list_body = list_response
            .json::<Value>()
            .await
            .expect("Failed to deserialize response.");
        assert_eq!(list_body["meta"]["total_items"], 0);
    }

    #[serial]
    #[tokio::test]
    async fn test_update_deleted_item_returns_not_found() {
        let client = prepare_test_environment!();

        let id = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&json!({
                "title": "update-deleted",
                "note": "note1",
                "status": "pending"
            }))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let delete_response = client
            .delete(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .send()
            .await
            .expect("Failed to execute request.");
        assert!(delete_response.status().is_success());

        let update_response = client
            .put(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .header("If-Match", "\"1\"")
            .json(&json!({
                "title": "update-deleted",
                "note": "note-updated",
                "status": "done"
            }))
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(update_response.status(), StatusCode::NOT_FOUND);
    }

    #[serial]
    #[tokio::test]
    async fn test_audit_endpoint_returns_deleted_item_with_metadata() {
        let client = prepare_test_environment!();
        let actor_id = Uuid::new_v4();

        let id = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&json!({
                "title": "audit-visible",
                "note": "note1",
                "status": "pending"
            }))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let delete_response = client
            .delete(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .header("X-Actor-Id", actor_id.to_string())
            .send()
            .await
            .expect("Failed to execute request.");
        assert!(delete_response.status().is_success());

        let audit_response = client
            .get(WEB_SERVER_PATH.to_owned() + format!("audit/to-do-items/{id}").as_str())
            .header("X-Audit-Token", AUDIT_TOKEN)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(audit_response.status(), StatusCode::OK);
        let body = audit_response
            .json::<Value>()
            .await
            .expect("Failed to deserialize response.");
        assert_eq!(body["id"], id.to_string());
        assert!(body["deleted_at"].is_string());
        assert_eq!(body["deleted_by"], actor_id.to_string());
    }

    #[serial]
    #[tokio::test]
    async fn test_audit_endpoint_rejects_missing_or_invalid_token() {
        let client = prepare_test_environment!();

        let id = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&json!({
                "title": "audit-auth",
                "note": "note1",
                "status": "pending"
            }))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let delete_response = client
            .delete(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .send()
            .await
            .expect("Failed to execute request.");
        assert!(delete_response.status().is_success());

        let missing_token_response = client
            .get(WEB_SERVER_PATH.to_owned() + format!("audit/to-do-items/{id}").as_str())
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(missing_token_response.status(), StatusCode::UNAUTHORIZED);

        let invalid_token_response = client
            .get(WEB_SERVER_PATH.to_owned() + format!("audit/to-do-items/{id}").as_str())
            .header("X-Audit-Token", "wrong-token")
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(invalid_token_response.status(), StatusCode::UNAUTHORIZED);
    }

    #[serial]
    #[tokio::test]
    async fn test_audit_endpoint_supports_delete_without_actor() {
        let client = prepare_test_environment!();

        let id = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&json!({
                "title": "audit-no-actor",
                "note": "note1",
                "status": "pending"
            }))
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let delete_response = client
            .delete(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .send()
            .await
            .expect("Failed to execute request.");
        assert!(delete_response.status().is_success());

        let audit_response = client
            .get(WEB_SERVER_PATH.to_owned() + format!("audit/to-do-items/{id}").as_str())
            .header("X-Audit-Token", AUDIT_TOKEN)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(audit_response.status(), StatusCode::OK);
        let body = audit_response
            .json::<Value>()
            .await
            .expect("Failed to deserialize response.");
        assert!(body["deleted_at"].is_string());
        assert!(body["deleted_by"].is_null());
    }

    #[serial]
    #[tokio::test]
    async fn test_create_rejects_blank_title() {
        let client = prepare_test_environment!();

        let response = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&json!({
                "title": "   ",
                "note": "note1",
                "status": "pending"
            }))
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[serial]
    #[tokio::test]
    async fn test_update_rejects_blank_note() {
        let client = prepare_test_environment!();
        let mut map_create = HashMap::new();
        map_create.insert("title", "title1");
        map_create.insert("note", "note1");
        map_create.insert("status", "pending");

        let id = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&map_create)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let response = client
            .put(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .header("If-Match", "\"1\"")
            .json(&json!({
                "title": "title1",
                "note": "   ",
                "status": "pending"
            }))
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[serial]
    #[tokio::test]
    async fn test_update_rejects_stale_version() {
        let client = prepare_test_environment!();
        let mut map_create = HashMap::new();
        map_create.insert("title", "title1");
        map_create.insert("note", "note1");
        map_create.insert("status", "pending");

        let id = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&map_create)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let item_response = client
            .get(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .send()
            .await
            .expect("Failed to execute request.");
        let etag = item_response
            .headers()
            .get("etag")
            .expect("ETag header must be present")
            .to_str()
            .expect("ETag must be ascii")
            .to_string();

        let response = client
            .put(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .header("If-Match", etag.clone())
            .json(&json!({
                "title": "title2",
                "note": "note2",
                "status": "in_progress"
            }))
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::OK);

        let stale_response = client
            .put(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}").as_str())
            .header("If-Match", etag)
            .json(&json!({
                "title": "title3",
                "note": "note3",
                "status": "done"
            }))
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(stale_response.status(), StatusCode::PRECONDITION_FAILED);

        let body = stale_response
            .json::<Value>()
            .await
            .expect("Failed to deserialize response.");
        assert_eq!(body["status"], json!(412));
    }

    #[serial]
    #[tokio::test]
    async fn test_get_all_rejects_invalid_query_parameters() {
        let client = prepare_test_environment!();

        let response = client
            .get(WEB_SERVER_PATH.to_owned() + "to-do-items?page=0&page_size=20")
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[serial]
    #[tokio::test]
    async fn test_get_all_accepts_valid_query_parameters() {
        let client = prepare_test_environment!();

        let response = client
            .get(
                WEB_SERVER_PATH.to_owned()
                    + "to-do-items?page=1&page_size=10&search=title&sort=title:asc",
            )
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .json::<Value>()
            .await
            .expect("Failed to deserialize response.");
        assert_eq!(body["meta"]["page"], 1);
        assert_eq!(body["meta"]["page_size"], 10);
    }

    #[serial]
    #[tokio::test]
    async fn test_get_all_search_matches_title_and_excludes_non_matches() {
        let client = prepare_test_environment!();
        let matching_title = format!("milk-title-{}", Uuid::new_v4());
        let non_matching_title = format!("bread-title-{}", Uuid::new_v4());

        for payload in [
            json!({
                "title": matching_title,
                "note": "ordinary note",
                "status": "pending"
            }),
            json!({
                "title": non_matching_title,
                "note": "completely unrelated",
                "status": "pending"
            }),
        ] {
            let response = client
                .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
                .json(&payload)
                .send()
                .await
                .expect("Failed to execute request.");
            assert_eq!(response.status(), StatusCode::CREATED);
        }

        let response = client
            .get(
                WEB_SERVER_PATH.to_owned()
                    + format!("to-do-items?search={matching_title}&page=1&page_size=10").as_str(),
            )
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .json::<Value>()
            .await
            .expect("Failed to deserialize response.");
        let items = body["items"].as_array().expect("items should be an array");
        assert_eq!(body["meta"]["total_items"], 1);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0]["title"], matching_title);
        assert_ne!(items[0]["title"], non_matching_title);
    }

    #[serial]
    #[tokio::test]
    async fn test_get_all_search_matches_note_content() {
        let client = prepare_test_environment!();
        let matching_note = format!("buy-oats-note-{}", Uuid::new_v4());
        let unique_title = format!("note-search-title-{}", Uuid::new_v4());

        let response = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&json!({
                "title": unique_title,
                "note": matching_note,
                "status": "pending"
            }))
            .send()
            .await
            .expect("Failed to execute request.");
        assert_eq!(response.status(), StatusCode::CREATED);

        let response = client
            .get(
                WEB_SERVER_PATH.to_owned()
                    + format!("to-do-items?search={matching_note}&page=1&page_size=10").as_str(),
            )
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .json::<Value>()
            .await
            .expect("Failed to deserialize response.");
        let items = body["items"].as_array().expect("items should be an array");
        assert_eq!(body["meta"]["total_items"], 1);
        assert_eq!(items[0]["title"], unique_title);
        assert_eq!(items[0]["note"], matching_note);
    }

    #[serial]
    #[tokio::test]
    async fn test_get_all_search_returns_empty_page_for_non_matches() {
        let client = prepare_test_environment!();
        let response = client
            .get(
                WEB_SERVER_PATH.to_owned()
                    + format!(
                        "to-do-items?search=missing-{}&page=1&page_size=10",
                        Uuid::new_v4()
                    )
                    .as_str(),
            )
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .json::<Value>()
            .await
            .expect("Failed to deserialize response.");
        assert_eq!(body["meta"]["total_items"], 0);
        assert_eq!(
            body["items"]
                .as_array()
                .expect("items should be an array")
                .len(),
            0
        );
    }

    #[serial]
    #[tokio::test]
    async fn test_get_all_without_search_preserves_listing_behavior() {
        let client = prepare_test_environment!();
        let title = format!("default-list-{}", Uuid::new_v4());

        let response = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&json!({
                "title": title,
                "note": "visible in default list",
                "status": "pending"
            }))
            .send()
            .await
            .expect("Failed to execute request.");
        assert_eq!(response.status(), StatusCode::CREATED);

        let response = client
            .get(WEB_SERVER_PATH.to_owned() + "to-do-items?page=1&page_size=100")
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .json::<Value>()
            .await
            .expect("Failed to deserialize response.");
        let items = body["items"].as_array().expect("items should be an array");
        assert!(items.iter().any(|item| item["title"] == title));
    }

    #[serial]
    #[tokio::test]
    async fn test_get_all_rejects_blank_search_query() {
        let client = prepare_test_environment!();

        let response = client
            .get(WEB_SERVER_PATH.to_owned() + "to-do-items?search=%20%20%20&page=1&page_size=10")
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[serial]
    #[tokio::test]
    async fn test_create_rejects_oversized_payload() {
        let client = prepare_test_environment!();
        let oversized_body = json!({
            "title": "a".repeat(9000),
            "note": "note1",
            "status": "pending"
        });

        let response = client
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&oversized_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[serial]
    #[tokio::test]
    async fn test_metrics_endpoint_exposes_application_metrics() {
        let client = prepare_test_environment!();

        let _ = client
            .get(WEB_SERVER_PATH.to_owned() + "to-do-items?page=1&page_size=5")
            .send()
            .await
            .expect("Failed to execute request.");
        let _ = client
            .get(WEB_SERVER_PATH.to_owned() + "to-do-items?page=0&page_size=20")
            .send()
            .await
            .expect("Failed to execute request.");

        let metrics_response = client
            .get(METRICS_PATH)
            .send()
            .await
            .expect("Failed to execute request.");
        assert_eq!(metrics_response.status(), StatusCode::OK);

        let body = metrics_response
            .text()
            .await
            .expect("Failed to read metrics response body.");
        assert!(body.contains("http_requests_total"));
        assert!(body.contains("http_request_duration_seconds"));
        assert!(body.contains("http_request_errors_total"));
    }

    #[serial]
    #[tokio::test]
    async fn test_metrics_scrapes_do_not_increment_business_route_count() {
        let client = prepare_test_environment!();

        let before = fetch_route_counter(&client, "/api/v1/to-do-items").await;

        let _ = client
            .get(WEB_SERVER_PATH.to_owned() + "to-do-items?page=1&page_size=5")
            .send()
            .await
            .expect("Failed to execute request.");

        let after_business_request = fetch_route_counter(&client, "/api/v1/to-do-items").await;
        assert!(
            after_business_request > before,
            "Business route counter should increase after normal API request"
        );

        let after_metrics_scrape = fetch_route_counter(&client, "/api/v1/to-do-items").await;
        assert_eq!(
            after_business_request, after_metrics_scrape,
            "Scraping /metrics should not alter business route counters"
        );
    }

    async fn fetch_route_counter(client: &reqwest::Client, route: &str) -> f64 {
        let response = client
            .get(METRICS_PATH)
            .send()
            .await
            .expect("Failed to execute metrics request.");
        let metrics = response
            .text()
            .await
            .expect("Failed to read metrics response body.");

        metrics
            .lines()
            .filter(|line| {
                line.starts_with("http_requests_total{")
                    && line.contains(&format!("route=\"{route}\""))
            })
            .filter_map(|line| line.split_whitespace().last())
            .filter_map(|value| value.parse::<f64>().ok())
            .sum()
    }
}
