#![windows_subsystem = "windows"]

fn main() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }

    /*
    let icon_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("rust-logo.png");
    app.set_icon_from_file(icon_path.to_str().unwrap())?;
    */

    let icon = include_bytes!("rust-logo.png");
    app.set_icon_from_buffer(icon, 32, 32).unwrap();

    app.add_menu_item("Print a thing", |_| {
        println!("Printing a thing!");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Add Menu Item", |window| {
        window.add_menu_item("Delete item", |window| {
            window.remove_menu_item(3);
            Ok::<_, systray::Error>(())
        })?;
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Waiting on message!");
    app.wait_for_message()?;
    Ok(())
}

// #[cfg(not(target_os = "windows"))]
// fn main() {
//     panic!("Not implemented on this platform!");
// }
