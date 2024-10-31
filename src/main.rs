use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

fn main() -> glib::ExitCode {
    const APPID: &str = "com.matixandr.win.clipboardeditor";
    let app = Application::builder().application_id(APPID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Windows Clipboard Editor")
        .build();

    window.present();
}