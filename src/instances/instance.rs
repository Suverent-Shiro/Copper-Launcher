// This is where Minecraft instances are managed.

// Copper Launcher is licensed under GNU General Public License v3.0.
// Copyright (c) 2026 Suverent_Shiro
//
// See LICENSE file for more details.


use gtk::{glib};
use std::{fs, path::PathBuf};


pub fn instance_create(instance_name: &str) {
    let home_dir = glib::home_dir();
    
    // Build the instance path
    let mut instance_dir_path = PathBuf::from(home_dir);
            instance_dir_path.push(".copper-launcher");
            instance_dir_path.push("instances");
            instance_dir_path.push(instance_name);
    
    // Create instance directory
    fs::DirBuilder::new()
        .recursive(true)
        .create(&instance_dir_path)
        .expect("Failed to create instance");
    
    // Instance directory - Minecraft game directory
    let mut mc_instance_dir_path = instance_dir_path.clone();
            mc_instance_dir_path.push("minecraft");
    
    fs::DirBuilder::new()
        .recursive(true)
        .create(&mc_instance_dir_path)
        .expect("Failed to create minecraft directory");
    
    // Instance directory - Minecraft - saves
    let mut mc_save_instance_dir_path = mc_instance_dir_path.clone();
            mc_save_instance_dir_path.push("saves");
    
    fs::DirBuilder::new()
        .recursive(true)
        .create(&mc_save_instance_dir_path)
        .expect("Failed to create saves directory");
    
    // Instance directory - Minecraft - resourcepacks
    let mut mc_txt_instance_dir_path = mc_instance_dir_path.clone();
            mc_txt_instance_dir_path.push("resourcepacks");
    
    fs::DirBuilder::new()
        .recursive(true)
        .create(&mc_txt_instance_dir_path)
        .expect("Failed to create resourcepacks directory");
}

// Work in progress
pub fn instance_remove(instance_name: &str) {
     let home_dir = glib::home_dir();
    
    // Build the instance path
    let mut instance_dir_path = PathBuf::from(home_dir);
            instance_dir_path.push(".copper-launcher");
            instance_dir_path.push("instances");
            instance_dir_path.push(instance_name);   
}
