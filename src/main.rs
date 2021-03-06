mod disk;

use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{
    Align, Button, CheckButton, CssProvider, Entry, Label, ListBox, Orientation, ScrolledWindow,
    SelectionMode, StyleContext,
};
use libadwaita::{Application, ApplicationWindow, HeaderBar};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

struct TaskTree {
    id: Uuid,
    check_button: CheckButton,
    label: Label,
}

struct AppTree {
    task_list: ListBox,
    tasks: Vec<TaskTree>,
}

fn main() {
    let tree = Rc::new(RefCell::new(None));
    let app = Application::builder()
        .application_id("solar.unneon.Taakverse")
        .build();
    app.connect_startup(|_| {
        let provider = CssProvider::new();
        provider.load_from_data(include_bytes!("style.css"));
        StyleContext::add_provider_for_display(
            &Display::default().unwrap(),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_FALLBACK,
        );
    });
    app.connect_activate({
        let tree = tree.clone();
        move |app| on_activate(tree.clone(), app)
    });
    app.connect_shutdown(move |app| on_shutdown(tree.clone(), app));
    app.run();
}

fn build_task(
    tree: Rc<RefCell<Option<AppTree>>>,
    id: Uuid,
    description: String,
    completed: bool,
) -> (gtk4::Box, TaskTree) {
    let check_button = CheckButton::builder().active(completed).build();
    let label = Label::builder()
        .margin_start(4)
        .label(&description)
        .hexpand(true)
        .halign(Align::Start)
        .build();
    let delete_button = Button::builder()
        .icon_name("edit-delete-symbolic")
        .halign(Align::End)
        .build();
    let row = gtk4::Box::builder()
        .margin_top(4)
        .margin_bottom(4)
        .margin_start(8)
        .margin_end(8)
        .build();
    if completed {
        row.add_css_class("completed");
    }
    row.append(&check_button);
    row.append(&label);
    row.append(&delete_button);
    check_button.connect_active_notify({
        let row = row.clone();
        move |check_button| {
            if check_button.is_active() {
                row.add_css_class("completed");
            } else {
                row.remove_css_class("completed");
            }
        }
    });
    delete_button.connect_clicked(move |button| {
        let mut tree = (*tree).borrow_mut();
        let tree = tree.as_mut().unwrap();
        let row_wrapper = button.parent().unwrap().parent().unwrap();
        tree.task_list.remove(&row_wrapper);
        tree.tasks.retain(|task| task.id != id);
    });
    let tree = TaskTree {
        id,
        check_button,
        label,
    };
    (row, tree)
}

fn add_task(
    tree_ptr: Rc<RefCell<Option<AppTree>>>,
    id: Uuid,
    description: String,
    completed: bool,
) {
    let mut tree = (*tree_ptr).borrow_mut();
    let tree = tree.as_mut().unwrap();
    let (row, task_tree) = build_task(tree_ptr.clone(), id, description, completed);
    tree.task_list.append(&row);
    tree.tasks.push(task_tree);
}

fn on_activate(mut tree: Rc<RefCell<Option<AppTree>>>, app: &Application) {
    let data = disk::load().unwrap();

    let entry = Entry::builder()
        .margin_top(8)
        .margin_start(8)
        .margin_end(8)
        .placeholder_text("Note a new task...")
        .secondary_icon_name("list-add-symbolic")
        .build();

    let task_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .build();

    let scrollable = ScrolledWindow::builder()
        .margin_top(8)
        .vexpand(true)
        .child(&task_list)
        .build();

    tree.borrow_mut().replace(Some(AppTree {
        task_list,
        tasks: Vec::new(),
    }));

    for task in data.tasks {
        add_task(tree.clone(), task.id, task.description, task.completed);
    }

    entry.connect_activate({
        let tree = tree.clone();
        move |entry| {
            add_task(tree.clone(), Uuid::new_v4(), entry.buffer().text(), false);
            entry.buffer().delete_text(0, None);
        }
    });
    entry.connect_icon_press(|entry, _| entry.emit_activate());

    let header = HeaderBar::new();

    let content = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    content.append(&header);
    content.append(&entry);
    content.append(&scrollable);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Taakverse")
        .default_width(384)
        .default_height(512)
        .content(&content)
        .build();
    window.present();
}

fn on_shutdown(tree: Rc<RefCell<Option<AppTree>>>, _app: &Application) {
    let tree = tree.borrow();
    let tree = tree.as_ref().unwrap();
    let tasks = tree
        .tasks
        .iter()
        .map(|task| disk::Task {
            id: task.id,
            description: task.label.text().to_string(),
            completed: task.check_button.is_active(),
        })
        .collect();
    let disk_data = disk::Data { tasks };
    disk::save(&disk_data).unwrap();
}
