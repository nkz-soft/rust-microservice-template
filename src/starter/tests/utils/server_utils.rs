use crate::tests::CONFIG_FILE_PATH;
use ctor::dtor;
use testcontainers::core::IntoContainerPort;
use testcontainers::{ContainerAsync, ImageExt};
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use tokio::sync::OnceCell;
use tokio::task::JoinHandle;

#[macro_export]
macro_rules! prepare_test_environment {
    () => {{
        server_utils::init().await;
        reqwest::Client::new()
    }}
}

pub struct Server {
    #[allow(dead_code)]
    server_handle: JoinHandle<()>,
    container: ContainerAsync<Postgres>,
}

impl Server {
    pub async fn start() -> Self {
        let container = Postgres::default()
            .with_db_name("rust_template_db")
            .with_mapped_port(5432, 5432.tcp())
            .with_tag("16-alpine")
            .start().await.unwrap();

        let server_handle = tokio::spawn(async move {
            let server = starter::run_with_config(&CONFIG_FILE_PATH)
                .await
                .expect("Failed to bind address");
            let _server_task = tokio::spawn(server);
        });
        Server { server_handle, container }
    }

    pub fn container(&self) -> &ContainerAsync<Postgres> {
        &self.container
    }
}

pub(crate) static TEST_SERVER_ONCE: OnceCell<Server> = OnceCell::const_new();

pub(crate) async fn init() {
    TEST_SERVER_ONCE.get_or_init(Server::start).await;
}

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
