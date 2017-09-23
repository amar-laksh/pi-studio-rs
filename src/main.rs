#[macro_use]
extern crate gtk;
extern crate serde_json;

use gtk::prelude::*;
use gtk::{AboutDialog, Button, Window, WindowType, Menu, MenuBar, MenuItem, CheckMenuItem};

fn add_menu_item(item: &str) -> MenuItem {
    return MenuItem::new_with_label(item);
}

fn create_menubar() -> gtk::Box {
    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let menu = Menu::new();
    let menu_bar = MenuBar::new();
    let file = add_menu_item("File");
    let about = add_menu_item("About");

    menu.append(&add_menu_item("Open File"));
    menu.append(&add_menu_item("Save File"));
    menu.append(&add_menu_item("Quit"));

    file.set_submenu(Some(&menu));

    menu_bar.append(&file);
    menu_bar.append(&about);
    v_box.pack_start(&menu_bar, false, false, 0);

    about.connect_activate(move |_| {
        let p = AboutDialog::new();
        p.set_authors(&["Amar Lakshya"]);
        p.set_website_label(Some("GitHUb"));
        p.set_website(Some("http://github.com/amar-laksh/pi-studio-rs"));
        p.set_title("About!");
        p.run();
        p.destroy();
    });

    return v_box;
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let window = Window::new(WindowType::Toplevel);
    window.set_title("PI-studio");
    window.maximize();
    let v_box = create_menubar();
    window.add(&v_box);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    gtk::main();
}
