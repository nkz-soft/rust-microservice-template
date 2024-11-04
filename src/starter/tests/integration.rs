mod utils;


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::utils::server_utils;
    use serial_test::serial;
    use uuid::Uuid;
    use crate::prepare_test_environment;

    pub(crate) const CONFIG_FILE_PATH: &str = "./../../";
    #[serial]
    #[tokio::test]
    async fn start_server_and_test() {
        let client = prepare_test_environment!();
        assert!(client.get("http://localhost:8181").send().await.is_ok());
    }

    #[serial]
    #[tokio::test]
    async fn test_get_all() {
        let client = prepare_test_environment!();

        // Act
        let response = client
            .get("http://localhost:8181/to-do-items")
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success());
    }

    #[serial]
    #[tokio::test]
    async fn test_get_by_id() {
        let client = prepare_test_environment!();
        let mut map_create = HashMap::new();
        map_create.insert("title", "title1");
        map_create.insert("note", "note1");

        // Act
        let id = client
            .post("http://localhost:8181/to-do-items")
            .json(&map_create)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let response = client
            .get(format!("http://localhost:8181/to-do-items/{id}", id = id))
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success());
    }

    #[serial]
    #[tokio::test]
    async fn test_create() {
        let client = prepare_test_environment!();
        let mut map = HashMap::new();
        map.insert("title", "title1");
        map.insert("note", "note1");

        // Act
        let response = client
            .post("http://localhost:8181/to-do-items")
            .json(&map)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success());
    }

    #[serial]
    #[tokio::test]
    async fn test_update() {
        let client = prepare_test_environment!();
        let mut map_create = HashMap::new();
        map_create.insert("title", "title1");
        map_create.insert("note", "note1");

        // Act
        let id = client
            .post("http://localhost:8181/to-do-items")
            .json(&map_create)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let mut map_update = HashMap::new();
        map_update.insert("id", id.to_string());
        map_update.insert("title", String::from("title1"));
        map_update.insert("note", String::from("note1"));

        let response = client
            .put("http://localhost:8181/to-do-items")
            .json(&map_update)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success());
    }

    #[serial]
    #[tokio::test]
    async fn test_delete() {
        let client = prepare_test_environment!();
        let mut map_create = HashMap::new();
        map_create.insert("title", "title1");
        map_create.insert("note", "note1");

        // Act
        let id = client
            .post("http://localhost:8181/to-do-items")
            .json(&map_create)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let response = client
            .delete(format!("http://localhost:8181/to-do-items/{id}", id = id))
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success());
    }
}



