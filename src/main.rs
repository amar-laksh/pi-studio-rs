#![allow(non_snake_case)]
extern crate gtk;
extern crate serde_json;

use gtk::prelude::*;
use gtk::{
    AboutDialog
    , Window
    , WindowType
    , Menu
    , MenuBar
    , MenuItem
};

fn gtk_main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let window = Window::new(WindowType::Toplevel);
    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 1);

    let file_menu = Menu::new();
    let edit_menu = Menu::new();
    let board_menu = Menu::new();
    let view_menu = Menu::new();
    let menu_bar = MenuBar::new();


    let File = MenuItem::new_with_label("File");

    let file_new = MenuItem::new_with_label("New File");
    let file_open = MenuItem::new_with_label("Open File");
    let file_save = MenuItem::new_with_label("Save File");
    let quit = MenuItem::new_with_label("Quit");
    file_menu.append(&file_new);
    file_menu.append(&file_open);
    file_menu.append(&file_save);
    file_menu.append(&quit);


    let Edit = MenuItem::new_with_label("Edit");

    let undo = MenuItem::new_with_label("Undo");
    let redo = MenuItem::new_with_label("Redo");
    let cut = MenuItem::new_with_label("Cut");
    let copy = MenuItem::new_with_label("Copy");
    let paste = MenuItem::new_with_label("Paste");
    let indent = MenuItem::new_with_label("Indent");
    let unindent = MenuItem::new_with_label("Un-Indent");
    edit_menu.append(&undo);
    edit_menu.append(&redo);
    edit_menu.append(&cut);
    edit_menu.append(&copy);
    edit_menu.append(&paste);
    edit_menu.append(&indent);
    edit_menu.append(&unindent);



    let View = MenuItem::new_with_label("View");

    let view_visual = MenuItem::new_with_label("Visual View");
    let view_code = MenuItem::new_with_label("Code View");
    view_menu.append(&view_code);
    view_menu.append(&view_visual);


    let Board = MenuItem::new_with_label("Board");

    let raspberry_a = MenuItem::new_with_label("Raspberry A");
    let raspberry_b = MenuItem::new_with_label("Raspberry B");
    let raspberry_b_plus = MenuItem::new_with_label("Raspberry B+");
    board_menu.append(&raspberry_a);
    board_menu.append(&raspberry_b);
    board_menu.append(&raspberry_b_plus);


    let About = MenuItem::new_with_label("About");



    File.set_submenu(Some(&file_menu));
    Edit.set_submenu(Some(&edit_menu));
    Board.set_submenu(Some(&board_menu));
    View.set_submenu(Some(&view_menu));

    menu_bar.append(&File);
    menu_bar.append(&Edit);
    menu_bar.append(&View);
    menu_bar.append(&Board);
    menu_bar.append(&About);



    v_box.pack_start(&menu_bar, false, false, 0);

    window.set_title("PI-Studio");
    window.maximize();

    window.add(&v_box);
    window.show_all();


//-----------------EVENT HANDLERS------------------------
    quit.connect_activate(|_| { gtk::main_quit(); });

    About.connect_activate(move |_| {
        let p = AboutDialog::new();
        p.set_website_label(Some("GitHUb"));
        p.set_website(
            Some("http://github.com/amar-laksh/pi-studio-rs")
            );
        p.set_authors(&["Amar Lakshya"]);
        p.set_title("About!");
        p.run();
        p.destroy();
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });


    gtk::main();
}


fn main() {
    gtk_main();
}
