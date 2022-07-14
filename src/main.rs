mod disk;

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, CheckButton, Entry, Label, ListBox, Orientation, ScrolledWindow,
};
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

const APP_ID: &str = "solar.unneon.Taakverse";

fn main() {
    let tree = Rc::new(RefCell::new(None));
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate({
        let tree = tree.clone();
        move |app| on_activate(tree.clone(), app)
    });
    app.connect_shutdown(move |app| on_shutdown(tree.clone(), app));
    app.run();
}

fn build_task(id: Uuid, description: String, completed: bool) -> (gtk4::Box, TaskTree) {
    let check_button = CheckButton::builder().active(completed).build();
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
    let tree = TaskTree {
        id,
        check_button,
        label,
    };
    (row, tree)
}

fn add_task(tree: &mut AppTree, id: Uuid, description: String, completed: bool) {
    let (row, task_tree) = build_task(id, description, completed);
    tree.task_list.append(&row);
    tree.tasks.push(task_tree);
}

fn on_activate(mut tree: Rc<RefCell<Option<AppTree>>>, app: &Application) {
    let data = disk::load().unwrap();

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

    tree.borrow_mut().replace(Some(AppTree {
        task_list,
        tasks: Vec::new(),
    }));

    for task in data.tasks {
        add_task(
            (*tree).borrow_mut().as_mut().unwrap(),
            task.id,
            task.description,
            task.completed,
        );
    }

    entry.connect_activate({
        let tree = tree.clone();
        move |entry| {
            add_task(
                (*tree).borrow_mut().as_mut().unwrap(),
                Uuid::new_v4(),
                entry.buffer().text(),
                false,
            );
            entry.buffer().delete_text(0, None);
        }
    });
    entry.connect_icon_press(|entry, _| entry.emit_activate());

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
        .title("Taakverse")
        .default_width(400)
        .default_height(600)
        .child(&content)
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
