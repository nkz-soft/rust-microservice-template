mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::test_server;
use serial_test::serial;
    use std::collections::HashMap;
    use ctor::dtor;
    use uuid::Uuid;

    use crate::{prepare_test_environment,
        utils::test_server::TEST_SERVER_ONCE,

    };

    const WEB_SERVER_PATH: &str = "http://localhost:8181/";

    //see https://stackoverflow.com/questions/78969766/how-can-i-call-drop-in-a-tokio-static-oncelock-in-rust
    #[dtor]
    fn cleanup() {
        //This is crazy but it works
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

        // Act
        let response = client
            .get(WEB_SERVER_PATH.to_owned() + "to-do-items")
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
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&map_create)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let response = client
            .get(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}", id = id).as_str())
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
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
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
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&map_create)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let mut map_update = HashMap::new();
        map_update.insert("title", String::from("title1"));
        map_update.insert("note", String::from("note1"));

        let response = client
            .put(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}", id = id).as_str())
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
            .post(WEB_SERVER_PATH.to_owned() + "to-do-items")
            .json(&map_create)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let response = client
            .delete(WEB_SERVER_PATH.to_owned() + format!("to-do-items/{id}", id = id).as_str())
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success());
    }
}
