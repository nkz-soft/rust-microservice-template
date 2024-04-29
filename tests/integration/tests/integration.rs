extern crate application;
extern crate starter;

#[cfg(test)]
mod tests {

    use serial_test::serial;
    use std::collections::HashMap;
    use testcontainers::{clients, RunnableImage};
    use testcontainers_modules::postgres;
    use uuid::Uuid;

    const CONFIG_FILE_PATH: &str = "./../../";

    #[macro_export]
    macro_rules! prepare_test_container {
        () => {
            let docker = clients::Cli::default();
            let image = RunnableImage::from(postgres::Postgres::default())
                .with_tag("15-alpine")
                .with_mapped_port((5432, 5432));

            let _node = docker.run(image);

            let (client, connection) = tokio_postgres::connect(
                "host=localhost user=postgres password=postgres",
                tokio_postgres::NoTls,
            )
            .await
            .unwrap();

            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });

            client
                .query("CREATE DATABASE rust_template_db;", &[])
                .await
                .unwrap();
        };
    }

    #[serial]
    #[tokio::test]
    async fn test_get_all() {
        prepare_test_container!();

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

        _server_task.abort();
    }

    #[serial]
    #[tokio::test]
    async fn test_create() {
        prepare_test_container!();

        let server = starter::run_with_config(&CONFIG_FILE_PATH)
            .await
            .expect("Failed to bind address");
        let _server_task = tokio::spawn(server);

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

        _server_task.abort();
    }

    #[serial]
    #[tokio::test]
    async fn test_update() {
        prepare_test_container!();

        let server = starter::run_with_config(&CONFIG_FILE_PATH)
            .await
            .expect("Failed to bind address");
        let _server_task = tokio::spawn(server);

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

        _server_task.abort();
    }

    #[serial]
    #[tokio::test]
    async fn test_get_by_id() {
        prepare_test_container!();

        let server = starter::run_with_config(&CONFIG_FILE_PATH)
            .await
            .expect("Failed to bind address");
        let _server_task = tokio::spawn(server);

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

        _server_task.abort();
    }

    #[serial]
    #[tokio::test]
    async fn test_delete() {
        prepare_test_container!();

        let server = starter::run_with_config(&CONFIG_FILE_PATH)
            .await
            .expect("Failed to bind address");
        let _server_task = tokio::spawn(server);

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

        _server_task.abort();
    }
}
