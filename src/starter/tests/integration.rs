#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::collections::HashMap;
    use uuid::Uuid;

    // Local test utilities
    use std::time::Duration;
    use testcontainers::core::IntoContainerPort;
    use testcontainers::{ContainerAsync, ImageExt};
    use testcontainers_modules::postgres::Postgres;
    use testcontainers_modules::testcontainers::runners::AsyncRunner;
    use tokio::sync::OnceCell;
    use tokio::task::JoinHandle;

    const CONFIG_FILE_PATH: &str = "./../../";
    const WEB_SERVER_PATH: &str = "http://localhost:8181/";

    struct Server {
        #[allow(dead_code)]
        server_handle: JoinHandle<()>,
        container: ContainerAsync<Postgres>,
    }

    impl Server {
        async fn start() -> Self {
            let container = Postgres::default()
                .with_db_name("rust_template_db")
                .with_mapped_port(5432, 5432.tcp())
                .with_tag("16-alpine")
                .start()
                .await
                .unwrap();

            let server_handle = tokio::spawn(async move {
                let server = starter::run_with_config(CONFIG_FILE_PATH)
                    .await
                    .expect("Failed to bind address");
                let _server_task = tokio::spawn(server);
            });
            tokio::time::sleep(Duration::from_secs(1)).await;
            Server {
                server_handle,
                container,
            }
        }
    }

    static TEST_SERVER_ONCE: OnceCell<Server> = OnceCell::const_new();

    async fn init_test_env() -> reqwest::Client {
        TEST_SERVER_ONCE.get_or_init(Server::start).await;
        reqwest::Client::new()
    }
    
    #[serial]
    #[tokio::test]
    async fn start_server_and_test() {
        let client = init_test_env().await;
        assert!(client.get(WEB_SERVER_PATH).send().await.is_ok());
    }

    #[serial]
    #[tokio::test]
    async fn test_get_all() {
        let client = init_test_env().await;

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
        let client = init_test_env().await;
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
        let client = init_test_env().await;
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
        let client = init_test_env().await;
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
        let client = init_test_env().await;
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
