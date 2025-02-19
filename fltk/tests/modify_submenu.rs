use fltk::{enums::*, prelude::*, *};

fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let mut menubar = menu::MenuBar::new(0, 0, 400, 40, "rew");
    menubar.add("File/New\t", Shortcut::None, menu::MenuFlag::Normal, |_| {});
    menubar.add(
        "File/Open\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        |_| {},
    );
    let idx = menubar.add(
        "File/Recent",
        Shortcut::None,
        menu::MenuFlag::Submenu,
        |_| {},
    );
    menubar.add(
        "File/Recent/First\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        |_| {},
    );
    menubar.add(
        "File/Recent/Second\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        |_| {},
    );
    menubar.add(
        "File/Quit\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        |_| {},
    );
    let mut btn1 = button::Button::new(160, 150, 80, 30, "Modify 1");
    let mut btn2 = button::Button::new(160, 200, 80, 30, "Modify 2");
    let mut clear = button::Button::new(160, 250, 80, 30, "Clear");
    win.end();
    win.show();

    btn1.set_callback({
        let menubar = menubar.clone();
        move |_| {
            if let Some(mut item) = menubar.find_item("File/Recent") {
                item.add(
                    "Recent/Third",
                    Shortcut::None,
                    menu::MenuFlag::Normal,
                    |_| {},
                );
                item.add(
                    "Recent/Fourth",
                    Shortcut::None,
                    menu::MenuFlag::Normal,
                    |_| {},
                );
            }
        }
    });

    btn2.set_callback({
        let mut menubar = menubar.clone();
        move |_| {
            menubar.add(
                "File/Recent/Fifth\t",
                Shortcut::None,
                menu::MenuFlag::Normal,
                |_| {},
            );
            menubar.add(
                "File/Recent/Sixth\t",
                Shortcut::None,
                menu::MenuFlag::Normal,
                |_| {},
            );
        }
    });

    clear.set_callback(move |_| {
        menubar.clear_submenu(idx).unwrap();
    });

    a.run().unwrap();
}
