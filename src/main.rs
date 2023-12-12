use log::error;
use minecraft::Application;

fn main() {
    env_logger::init();

    if let Err(e) = Application::new().and_then(|app| app.run()) {
        error!("{e}");
        std::process::exit(1);
    }
}
