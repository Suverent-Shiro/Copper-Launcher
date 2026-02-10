// Copper Launcher is a GTK based launcher for Minecraft written in Rust.

// Copper Launcher is licensed under GNU General Public License v3.0.
// Copyright (c) 2026 Suverent_Shiro
//
// See LICENSE file for more details.


use gtk::prelude::*;
use gtk::{Application, glib};
use std::{fs, path::PathBuf};

mod ui {
    pub mod launcher_ui;
}
mod instances {
    pub mod instance;
}
mod minecraft {
    pub mod api;
    pub mod game_launch;
}

const APP_ID: &str = "com.github.suverent-shiro.CopperLauncher";

fn main() -> glib::ExitCode {
    println!("Lauching Copper Launcher");

    launcher_home();


    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| ui::launcher_ui::build_ui(app));
    app.connect_startup(|app| ui::launcher_ui::on_startup(app));

    app.run()
}

// This function creates launchers home directory
// On linux it will create directory at /home/{user}/.copper-launcher
// On linux directory will be hidden by default
fn launcher_home() {
    let home_dir = glib::home_dir();
    
    let mut base_path = PathBuf::from(home_dir);
            base_path.push(".copper-launcher");

    // Creating launcher home directory
    // Launcher home directory - .copper-launcher
    fs::DirBuilder::new()
        .recursive(true)
        .create(&base_path)
        .expect("Failed to create launcher directory");

    // Instances directory - .copper-launcher/instances
    let mut instances_path = base_path.clone();
            instances_path.push("instances");
    fs::DirBuilder::new()
        .recursive(true)
        .create(&instances_path)
        .expect("Failed to create instances directory");

    // Versions directory - .copper-launcher/versions
    let mut versions_path = base_path.clone();
            versions_path.push("versions");
    fs::DirBuilder::new()
        .recursive(true)
        .create(&versions_path)
        .expect("Failed to create versions directory");

    // Assets directory - .copper-launcher/assets    
    let mut assets_path = base_path.clone();
            assets_path.push("assets");
    fs::DirBuilder::new()
        .recursive(true)
        .create(&assets_path)
        .expect("Failed to create assets directory");
}
