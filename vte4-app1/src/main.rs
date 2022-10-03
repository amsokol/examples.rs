use std::path::Path;

use vte4::prelude::*;

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("First GTK Program"));
    window.set_default_size(400, 300);

    let term = vte4::Terminal::new();

    window.set_child(Some(&term));

    term.feed("qwerty\r\n".as_bytes());

    /*
    term.spawn_async(
        vte4::PtyFlags::DEFAULT,
        Some("/Users/amsokol"),
        &[Path::new("zsh")],
        &[],
        glib::SpawnFlags::DO_NOT_REAP_CHILD,
        None,
        1000,
        None,
        None,
    );
    */

    window.show();
}
