use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, CheckButton, Entry, Label, ListBox, Orientation, ScrolledWindow,
};
use std::cell::RefCell;
use std::rc::Rc;

struct AppTree {
    task_list: ListBox,
}

const APP_ID: &str = "solar.unneon.TodoThingy";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_task(description: String) -> gtk4::Box {
    let check_button = CheckButton::builder().build();
    let label = Label::builder()
        .margin_start(12)
        .label(&description)
        .build();
    let row = gtk4::Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    row.append(&check_button);
    row.append(&label);
    row
}

fn build_ui(app: &Application) {
    let entry = Entry::builder()
        .placeholder_text("Note a new task...")
        .secondary_icon_name("list-add-symbolic")
        .build();

    let task_list = ListBox::builder().build();

    let scrollable = ScrolledWindow::builder()
        .margin_top(12)
        .vexpand(true)
        .child(&task_list)
        .build();

    let app_tree = Rc::new(RefCell::new(AppTree { task_list }));

    entry.connect_icon_press(move |entry, _| {
        let row = build_task(entry.buffer().text());
        app_tree.borrow_mut().task_list.append(&row);
    });

    let content = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    content.append(&entry);
    content.append(&scrollable);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Todo Thingy")
        .default_width(400)
        .default_height(600)
        .child(&content)
        .build();
    window.present();
}
