extern crate application;
extern crate starter;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::Duration;

    use serial_test::serial;
    use testcontainers_modules::postgres::Postgres;
    use testcontainers_modules::testcontainers::core::IntoContainerPort;
    use testcontainers_modules::testcontainers::runners::AsyncRunner;
    use testcontainers_modules::testcontainers::ImageExt;
    use uuid::Uuid;
    use std::sync::Once;


    const CONFIG_FILE_PATH: &str = "./../../";

    #[macro_export]
    macro_rules! prepare_test_container {
        () => {
            let _node = Postgres::default()
                .with_db_name("rust_template_db")
                .with_mapped_port(5432, 5432.tcp())
                .with_tag("16-alpine")
                .start().await.unwrap();

            let server = starter::run_with_config(&CONFIG_FILE_PATH)
                .await
                .expect("Failed to bind address");
            let _server_task = tokio::spawn(server);
        };
    }

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            let _node = Postgres::default()
                .with_db_name("rust_template_db")
                .with_mapped_port(5432, 5432.tcp())
                .with_tag("16-alpine")
                .start().await.unwrap();

            let server = starter::run_with_config(&CONFIG_FILE_PATH)
                .await
                .expect("Failed to bind address");
            let _server_task = tokio::spawn(server);
        });
    }

    #[serial]
    #[tokio::test]
    async fn test_get_all() {
        prepare_test_container!();

        tokio::time::sleep(Duration::from_secs(5)).await;

        let server = starter::run_with_config(&CONFIG_FILE_PATH)
            .await
            .expect("Failed to bind address");
        let _server_task = tokio::spawn(server);

        let client = reqwest::Client::new();

        // Act
        let response = client
            .get("http://127.0.0.1:8181/to-do-items")
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success());

        //make sure that the server will be unloaded, there is an error on github action
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create() {
        prepare_test_container!();

        let client = reqwest::Client::new();
        let mut map = HashMap::new();
        map.insert("title", "title1");
        map.insert("note", "note1");

        // Act
        let response = client
            .post("http://127.0.0.1:8181/to-do-items")
            .json(&map)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success());

        //make sure that the server will be unloaded, there is an error on github action
        tokio::time::sleep(Duration::from_millis(500)).await;

    }

    #[serial]
    #[tokio::test]
    async fn test_update() {
        prepare_test_container!();

        let client = reqwest::Client::new();
        let mut map_create = HashMap::new();
        map_create.insert("title", "title1");
        map_create.insert("note", "note1");

        // Act
        let id = client
            .post("http://127.0.0.1:8181/to-do-items")
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
            .put("http://127.0.0.1:8181/to-do-items")
            .json(&map_update)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success());

        //make sure that the server will be unloaded, there is an error on github action
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    #[serial]
    #[tokio::test]
    async fn test_get_by_id() {
        prepare_test_container!();

        let client = reqwest::Client::new();
        let mut map_create = HashMap::new();
        map_create.insert("title", "title1");
        map_create.insert("note", "note1");

        // Act
        let id = client
            .post("http://127.0.0.1:8181/to-do-items")
            .json(&map_create)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let response = client
            .get(format!("http://127.0.0.1:8181/to-do-items/{id}", id = id))
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success());

        //make sure that the server will be unloaded, there is an error on github action
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete() {
        prepare_test_container!();

        let client = reqwest::Client::new();
        let mut map_create = HashMap::new();
        map_create.insert("title", "title1");
        map_create.insert("note", "note1");

        // Act
        let id = client
            .post("http://127.0.0.1:8181/to-do-items")
            .json(&map_create)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<Uuid>()
            .await
            .expect("Failed to deserialize response.");

        let response = client
            .delete(format!("http://127.0.0.1:8181/to-do-items/{id}", id = id))
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success());

        //make sure that the server will be unloaded, there is an error on github action
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}
