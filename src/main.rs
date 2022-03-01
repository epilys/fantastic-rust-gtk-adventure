use gtk::prelude::*;

mod app;
mod window;
use app::MyApp;

fn main() {
    gtk::init().expect("Failed to initialize gtk");

    let app = MyApp::new();
    app.run();
}
