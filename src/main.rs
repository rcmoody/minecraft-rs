use log::error;
use minecraft::Application;

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(e) = Application::new().await.and_then(|app| app.run()) {
        error!("{e}");
        std::process::exit(1);
    }
}
