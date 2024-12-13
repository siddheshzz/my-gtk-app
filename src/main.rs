use gtk::{prelude::*, Button};
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    
    
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(480)
        .margin_bottom(480)
        .margin_start(460)
        .margin_end(460)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello Gladiator!");
    });
    
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Gladiators of Justice")
        .default_width(800)
        .default_height(800)
        .child(&button)
        .build();

    // Present window
    window.present();
}