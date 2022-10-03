use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use relm4::{send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};
use vte4::prelude::*;

struct AppModel {
    counter: u8,
    term_data: String,
}

enum AppMsg {
    RunApp,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::RunApp => {
                self.counter = self.counter.wrapping_add(1);
                self.term_data = String::from("qwerty");
            }
        }
        true
    }
}

struct AppWidgets {
    window: gtk::ApplicationWindow,
    _vbox: gtk::Box,
    term: vte4::Terminal,
    _run_app_button: gtk::Button,
}

impl Widgets<AppModel, ()> for AppWidgets {
    type Root = gtk::ApplicationWindow;

    /// Initialize the UI.
    fn init_view(_model: &AppModel, _parent_widgets: &(), sender: Sender<AppMsg>) -> Self {
        let window = gtk::ApplicationWindow::builder()
            .title("Simple app")
            .default_width(600)
            .default_height(400)
            .build();
        let _vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();
        _vbox.set_margin_all(5);

        let term = vte4::Terminal::new();

        let font = pango::FontDescription::from_string("Consolas, Monaco, console 12");
        term.set_font(Some(&font));

        let _run_app_button = gtk::Button::with_label("Run App");

        // Connect the widgets
        window.set_child(Some(&_vbox));
        _vbox.append(&term);
        _vbox.append(&_run_app_button);

        // Connect events
        _run_app_button.connect_clicked(move |_| {
            send!(sender, AppMsg::RunApp);
        });

        Self {
            window,
            _vbox,
            term,
            _run_app_button,
        }
    }

    /// Return the root widget.
    fn root_widget(&self) -> Self::Root {
        self.window.clone()
    }

    /// Update the view to represent the updated model.
    fn view(&mut self, model: &AppModel, _sender: Sender<AppMsg>) {
        // self.label.set_label(&format!("Counter: {}", model.counter));
        if !model.term_data.is_empty() {
            self.term.feed(
                format!("data: {}, counter: {}\r\n", model.term_data, model.counter).as_bytes(),
            );
        }
    }
}

fn main() {
    let model = AppModel {
        counter: 0,
        term_data: String::default(),
    };
    let app = RelmApp::new(model);
    app.run();
}
