extern crate application;
extern crate starter;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use testcontainers::{clients, RunnableImage};
    use testcontainers_modules::postgres;

    const CONFIG_FILE_PATH: &str = "./../";

    #[tokio::test]
    async fn test_index_get() {
        let docker = clients::Cli::default();
        let image = RunnableImage::from(postgres::Postgres::default())
            .with_tag("15-alpine")
            .with_mapped_port((5432, 5432));

        let node = docker.run(image);

        prepare_database().await;
        spawn_app().await;

        let client = reqwest::Client::new();

        {
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
        }

        {
            // Act
            let response = client
                .get("http://127.0.0.1:8181/to-do-items")
                .send()
                .await
                .expect("Failed to execute request.");

            // Assert
            assert!(response.status().is_success());
        }
    }

    async fn prepare_database() {
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
    }

    async fn spawn_app() {
        let server = starter::run_with_config(&CONFIG_FILE_PATH)
            .await
            .expect("Failed to bind address");
        let _ = tokio::spawn(server);
    }
}
