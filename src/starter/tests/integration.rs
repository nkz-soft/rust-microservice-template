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

    #[dtor]
    fn cleanup() {
        let id = TEST_SERVER_ONCE.get().unwrap().container().id();

        std::process::Command::new("docker")
            .arg("kill")
            .arg(id)
            .output()
            .expect("failed to kill container");
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
        let body = response
            .json::<Value>()
            .await
            .expect("Failed to deserialize response.");
        assert!(body.get("items").is_some());
        assert!(body.get("meta").is_some());
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
}
