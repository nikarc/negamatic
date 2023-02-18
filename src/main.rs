use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn load_image() {
    println!("Load image");
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(350)
        .default_height(200)
        .title("Hello world")
        .build();


    let button = Button::with_label("Open Image");
    button.connect_clicked(|_| {
        load_image();
    });

    window.set_child(Some(&button));

    window.show();
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();


    app.connect_activate(build_ui);


    app.run()
}
