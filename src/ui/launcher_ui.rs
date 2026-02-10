// Here is all code related to Launcher's UI.

// Copper Launcher is licensed under GNU General Public License v3.0.
// Copyright (c) 2026 Suverent_Shiro
// 
// See LICENSE file for more details.

// TODO:
//  - System that detects if launcher is first run, if yes than it will auto open special instance creation window (Special, for first time)


use crate::instances::instance;
use crate::minecraft::api::get_release_versions;

use std::path::PathBuf;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib, gdk, StringList};
use glib::{clone};


// App logo, currently just copper ingot item lol
static LOGO_SVG: &[u8] = include_bytes!("Copper_Ingot.svg");

pub fn build_ui(app: &Application) {
    // Play button
    let play_button = gtk::Button::builder()
        .label("Launch game")
        .build();

        play_button.connect_clicked(|_| {
            println!("Play button: Click!")
        });

    // Username field   
    let username_input_field_title = gtk::Label::default();
    username_input_field_title.set_markup("Enter your username");

    let username_input_field = gtk::Entry::new();
    let username_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .build();
    username_box.append(&username_input_field_title);
    username_box.append(&username_input_field);

    username_input_field.set_placeholder_text(Some("Player"));

    // Label
    let label = gtk::Label::builder()
        .label("Copper Launcher")
        .vexpand(true)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .css_classes(["large-title"])
        .build();

    // Instance selector
    let instance_dropdown_options = StringList::new(&["Instance 0", "Instance 1"]);
        
    let instance_dropdown = gtk::DropDown::new(Some(instance_dropdown_options), None::<gtk::Expression>);
        instance_dropdown.set_selected(0);

            instance_dropdown.connect_selected_notify(|dd| {
                println!("Instance selector. Selected: {}", dd.selected())
            });

    // Containers - For nice layout
    let main_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Fill)
        .hexpand(true)
        .vexpand(true)
        .build();

    let pushdownpls = gtk::Box::new(gtk::Orientation::Vertical, 0);
    pushdownpls.set_vexpand(true);

    let bottom_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .halign(gtk::Align::Fill)
        .spacing(24)
        .margin_start(24)
        .margin_end(24)
        .margin_bottom(24)
        .build();

    bottom_container.append(&username_box);

    let pushbuttonrightpls = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    pushbuttonrightpls.set_hexpand(true);
    
    bottom_container.append(&pushbuttonrightpls);
    bottom_container.append(&instance_dropdown);
    bottom_container.append(&play_button);

    main_container.append(&label);
    main_container.append(&pushdownpls);
    main_container.append(&bottom_container);

    // Create main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Copper Launcher")
        .default_width(1280)
        .default_height(720)
        .resizable(false)
        .show_menubar(true)
        .child(&main_container)
        .build();

    // Present window
    window.present();
}

pub fn on_startup(app: &gtk::Application) {
    // Create launcher logo texture
    let bytes = glib::Bytes::from_static(LOGO_SVG);
    let logo = gdk::Texture::from_bytes(&bytes).expect("Copper_Ingot.svg to load");

    // Create about action that shows dialog
    let about = gio::ActionEntry::builder("about")
        .activate(move |app: &gtk::Application, _, _| {
            // Get the active window
            if let Some(window) = app.active_window() {
                let dialog = gtk::AboutDialog::builder()
                    .transient_for(&window)
                    .modal(true)
                    .program_name("About Copper Launcher")
                    .version("0.1.0")
                    .website("https:// github.com/Suverent-Shiro")
                    .license_type(gtk::License::Gpl30)
                    .authors(["Suverent-Shiro"])
                    .logo(&logo)
                    .build();

                dialog.present();
            }
        })
        .build();

    let quit = gio::ActionEntry::builder("quit")
        .activate(|app: &gtk::Application, _, _| app.quit())
        .build();


// New instance window
    // Create button
        let create_button = gtk::Button::builder()
            .label("Create instance")
            .build();

    // Search bar
    // Used for searching minecraft versions
        let search_button = gtk::ToggleButton::new();
            search_button.set_icon_name("system-search-symbolic");

        let search_bar = gtk::SearchBar::builder()
            .valign(gtk::Align::Start)
            .build();

        let entry = gtk::SearchEntry::new();
            entry.set_hexpand(true);
            search_bar.set_child(Some(&entry));

        search_bar.connect_entry(&entry);
        search_button.bind_property("active", &search_bar, "search-mode-enabled")
            .bidirectional()
            .build();

        let label = gtk::Label::builder()
            .vexpand(true)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .css_classes(["large-title"])
            .build();

        entry.connect_search_started(clone!(
            #[weak]
            search_button,
            move |_| {
                search_button.set_active(true);
            }
        ));

        entry.connect_stop_search(clone!(
            #[weak]
            search_button,
            move |_| {
                search_button.set_active(false);
            }
        ));

        entry.connect_search_changed(clone!(
            #[weak]
            label,
            move |entry| {
                if entry.text() != "" {
                    label.set_text(&entry.text());
                } else {
                    label.set_text("Type to start search");
                }
            }
        ));

    // Instance name input bar
        let instance_input_field_title = gtk::Label::default();
            instance_input_field_title.set_markup("Enter instance name");

        let instance_input_field = gtk::Entry::new();
        let instance_name_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(6)
            .build();
        instance_name_box.append(&instance_input_field_title);
        instance_name_box.append(&instance_input_field);    
        
        instance_input_field.set_placeholder_text(Some("Minecraft instance"));
        instance_input_field.set_max_length(24); // Maximum 24 characters for instance name
        instance_input_field.connect_changed(clone!(
            #[weak]
            create_button,
            move |entry| {
                let text = entry.text();
                let len = text.len();
        
                // Check if length is between 3 and 16 characters
                if len >= 3 && len <= 16 {
                    create_button.set_sensitive(true); // Enables button
                } else {
                    create_button.set_sensitive(false); // Disables button
                }
            }
        ));

        instance_input_field.set_max_length(24);
        
        //Here create button gets the name that user inputs into the input bar 
        create_button.connect_clicked(clone!(
            #[weak]
            instance_input_field,
             move |_| {
                let instance_name = instance_input_field.text();
                
                instance::instance_create(&instance_name);
                println!("Instance name: {}", instance_name);
            }
        ));

    // Version selector
    // Used for selecting Minecraft versions
        
        let ver_versions = match get_release_versions() {
            Ok(versions) => {
                let version_strings: Vec<&str> = versions.iter().map(|s| s.as_str()).collect();
                    StringList::new(&version_strings)
            }
            Err(e) => {
                eprintln!("Failed to fetch Minecraft versions: {}", e);
                // Fallback to some default versions
                StringList::new(&["1.21.11", "1.21.1", "1.12.2"])
            }
        };

        let ver_dropdown = gtk::DropDown::new(Some(ver_versions), None::<gtk::Expression>);
            ver_dropdown.set_selected(0);

            ver_dropdown.connect_selected_notify(|vdd| {
                println!("Version selector dropdown. Selected: {}", vdd.selected())
            });
        


    // Creating instance window
        
    
    // Containers - For nice layout
        let main_container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Fill)
            .valign(gtk::Align::Fill)
            .hexpand(true)
            .vexpand(true)
            .build();

        // main_container.append(&search_bar); // Add search bar to container

        let pushdownpls = gtk::Box::new(gtk::Orientation::Vertical, 0);
            pushdownpls.set_vexpand(true);

        let bottom_container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Fill)
            .spacing(24)
            .margin_start(24)
            .margin_end(24)
            .margin_bottom(24)
            .build();

        // bottom_container.append(&search_button);
        bottom_container.append(&ver_dropdown);

        let pushbuttonrightpls = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            pushbuttonrightpls.set_hexpand(true);

        bottom_container.append(&pushbuttonrightpls);
        bottom_container.append(&instance_name_box);
        bottom_container.append(&create_button);


        main_container.append(&label);
        main_container.append(&pushdownpls);
        main_container.append(&bottom_container);


    

// Menu bar
// Menu bar - Creating new instance window 
    let new_instance_action = gio::ActionEntry::builder("new_instance")
        .activate(move |app: &gtk::Application, _, _| {
            if let Some(window) = app.active_window() {
                let new_instance_window = ApplicationWindow::builder()
                    .transient_for(&window)
                    .application(app)
                   .title("Creating an instance - Copper Launcher")
                   .default_width(1160)
                   .default_height(580)
                   .resizable(false)
                    .child(&main_container)
                    .build();
            
            // Connect the create button to close the window
            create_button.connect_clicked(clone!(
                #[weak]
                instance_input_field,
                #[weak]
                new_instance_window,
                move |_| {
                    let instance_name = instance_input_field.text();
                    
                        if instance_name.len() < 3 || instance_name.len() > 16 {
                            println!("Invalid instance name length");
                            return;
                        }

                        instance::instance_create(&instance_name);

                        new_instance_window.close();
                    }
                ));

                new_instance_window.present();
            }
        })
        .build();

// Menu bar - Open instances directory
    let instance_open_dir = gio::ActionEntry::builder("files_instance")
        .activate(|app: &gtk::Application, _, _| {
            if let Some(window) = app.active_window() {
                // Get instances directory
                let home_dir = glib::home_dir();
                let mut instances_path = PathBuf::from(home_dir);
                        instances_path.push(".copper-launcher");
                        instances_path.push("instances");
            
                let file = gio::File::for_path(&instances_path);
                let launcher = gtk::FileLauncher::new(Some(&file));
            
                launcher.launch(
                    Some(&window),
                    None::<&gio::Cancellable>,
                    |result| {
                        if let Err(e) = result {
                            eprintln!("Failed to open directory: {}", e);
                    }
                    }
                );
            }
        })
        .build();

    
// Menu bar - GitHub repo / Opens URL in browser
    let github_repo = gio::ActionEntry::builder("github_repo")
        .activate(|app: &gtk::Application, _, _| {
           if let Some(window) = app.active_window() {
                let launcher = gtk::UriLauncher::new("https://github.com/Suverent-Shiro/CopperLauncher");
                launcher.launch(
                    Some(&window),
                    None::<&gio::Cancellable>,
                    |result| {
                        if let Err(e) = result {
                            eprintln!("Failed to open URL: {}", e);
                    }
                }
            );
        }
    })
    .build();


// Menu bar entries
    app.add_action_entries([about, quit, github_repo, new_instance_action, instance_open_dir]); // Entries for menu bar (I keep forgeting about it)
    
    let menubar = {
        let file_menu = {
            // Instance stuff
            let new_instance = gio::MenuItem::new(Some("Create new instance"), Some("app.new_instance")); //Instance creation window
            
            let files_instance = gio::MenuItem::new(Some("Open instances directory"), Some("app.files_instance")); //Opens instances directory in file manager
            
            // Other
            let quit_menu_item = gio::MenuItem::new(Some("Quit"), Some("app.quit")); //Quits

            let open_preferences = gio::MenuItem::new(Some("Preferences"), Some("app.open_preferences"));

            // Stuff for bar
            let file_menu = gio::Menu::new();
            file_menu.append_item(&new_instance);
            file_menu.append_item(&files_instance);
            file_menu.append_item(&open_preferences);
            file_menu.append_item(&quit_menu_item);
 
            file_menu
        };

        let about_menu = {
            // About project stuff
            let about_menu_item = gio::MenuItem::new(Some("About"), Some("app.about")); //About app window

            let about_github = gio::MenuItem::new(Some("Github"), Some("app.github_repo")); //Github repo URL

            // Stuff for bar
            let about_menu = gio::Menu::new();
            about_menu.append_item(&about_github);
            about_menu.append_item(&about_menu_item);

            about_menu
        };

        let menubar = gio::Menu::new();
        menubar.append_submenu(Some("File"), &file_menu);
        menubar.append_submenu(Some("About"), &about_menu);

        menubar
    };

    app.set_menubar(Some(&menubar));
}