// This is where Minecraft related API's are handled.

// Copper Launcher is licensed under GNU General Public License v3.0.
// Copyright (c) 2026 Suverent_Shiro
//
// See LICENSE file for more details.


use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Write, copy};
use std::path::PathBuf;


#[derive(Debug, Deserialize)]
pub struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Debug, Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize)]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
}

// Detailed version info (from the version-specific JSON)
#[derive(Debug, Deserialize, Serialize)]
pub struct VersionDetails {
    pub id: String,
    pub downloads: Downloads,
    pub libraries: Vec<Library>,
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndex,
    pub assets: String,
    #[serde(rename = "mainClass")]
    pub main_class: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Downloads {
    pub client: DownloadInfo,
    pub server: Option<DownloadInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadInfo {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Library {
    pub name: String,
    pub downloads: LibraryDownloads,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LibraryDownloads {
    pub artifact: Option<DownloadInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
    #[serde(rename = "totalSize")]
    pub total_size: u64,
}


pub fn fetch_minecraft_versions() -> Result<VersionManifest, Box<dyn std::error::Error>> {
    let url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
    
    let response = reqwest::blocking::get(url)?;
    let manifest: VersionManifest = response.json()?;
    
    Ok(manifest)
}

// Only gets release versions
pub fn get_release_versions() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let manifest = fetch_minecraft_versions()?;
    
    let releases: Vec<String> = manifest
        .versions
        .into_iter()
        .filter(|v| v.version_type == "release")
        .map(|v| v.id)
        .collect();
    
    Ok(releases)
}

// Gets all version types
pub fn get_all_versions() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let manifest = fetch_minecraft_versions()?;
    
    let versions: Vec<String> = manifest
        .versions
        .into_iter()
        .map(|v| v.id)
        .collect();
    
    Ok(versions)
}

// Get the URL for a specific version's details
pub fn get_version_url(version_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let manifest = fetch_minecraft_versions()?;
    
    let version = manifest
        .versions
        .into_iter()
        .find(|v| v.id == version_id)
        .ok_or("Version not found")?;
    
    Ok(version.url)
}

// Fetch detailed version information
pub fn fetch_version_details(version_id: &str) -> Result<VersionDetails, Box<dyn std::error::Error>> {
    let version_url = get_version_url(version_id)?;
    let response = reqwest::blocking::get(&version_url)?;
    let details: VersionDetails = response.json()?;
    Ok(details)
}

// Download a file from URL to a path
fn download_file(url: &str, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let response = reqwest::blocking::get(url)?;
    let mut file = File::create(path)?;
    let content = response.bytes()?;
    file.write_all(&content)?;
    
    Ok(())
}

// Download Minecraft client JAR
pub fn download_minecraft_client(version_id: &str, instance_path: &PathBuf,) -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching version details for {}...", version_id);
    let details = fetch_version_details(version_id)?;
    
    // Create versions directory
    let mut versions_dir = instance_path.clone();
            versions_dir.push("minecraft");
            versions_dir.push("versions");
            versions_dir.push(version_id);
    fs::create_dir_all(&versions_dir)?;
    
    // Download client JAR
    let mut client_jar_path = versions_dir.clone();
            client_jar_path.push(format!("{}.jar", version_id));
    
    println!("Downloading client JAR...");
    download_file(&details.downloads.client.url, &client_jar_path)?;
    
    // Save version JSON
    let mut version_json_path = versions_dir.clone();
            version_json_path.push(format!("{}.json", version_id));
    
    let version_json = serde_json::to_string_pretty(&details)?;
    fs::write(&version_json_path, version_json)?;
    
    println!("Client downloaded successfully!");
    Ok(())
}

// Download libraries
pub fn download_libraries(version_id: &str, instance_path: &PathBuf,) -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching libraries for {}...", version_id);
    let details = fetch_version_details(version_id)?;
    
    let mut libraries_dir = instance_path.clone();
            libraries_dir.push("minecraft");
            libraries_dir.push("libraries");
    
    println!("Found {} libraries to download", details.libraries.len());
    
    for (i, library) in details.libraries.iter().enumerate() {
        if let Some(artifact) = &library.downloads.artifact {
            // Parse library name to create directory structure
            // Format: "com.mojang:authlib:1.5.25" -> "com/mojang/authlib/1.5.25/authlib-1.5.25.jar"
            let parts: Vec<&str> = library.name.split(':').collect();
            if parts.len() == 3 {
                let group = parts[0].replace('.', "/");
                let name = parts[1];
                let version = parts[2];
                
                let mut lib_path = libraries_dir.clone();
                        lib_path.push(&group);
                        lib_path.push(name);
                        lib_path.push(version);
                        lib_path.push(format!("{}-{}.jar", name, version));
                
                println!("Downloading library {}/{}: {}", i + 1, details.libraries.len(), library.name);
                download_file(&artifact.url, &lib_path)?;
            }
        }
    }
    
    println!("Libraries downloaded successfully!");
    Ok(())
}

// Download assets
pub fn download_assets(version_id: &str, instance_path: &PathBuf,) -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching assets for {}...", version_id);
    let details = fetch_version_details(version_id)?;
    
    // Download asset index
    let mut assets_dir = instance_path.clone();
            assets_dir.push("minecraft");
            assets_dir.push("assets");
    
    let mut indexes_dir = assets_dir.clone();
            indexes_dir.push("indexes");
    fs::create_dir_all(&indexes_dir)?;
    
    let mut index_path = indexes_dir.clone();
            index_path.push(format!("{}.json", details.asset_index.id));
    
    println!("Downloading asset index...");
    download_file(&details.asset_index.url, &index_path)?;
    
    // Parse asset index to download individual assets
    let index_content = fs::read_to_string(&index_path)?;
    let asset_index: serde_json::Value = serde_json::from_str(&index_content)?;
    
    if let Some(objects) = asset_index.get("objects").and_then(|o| o.as_object()) {
        let total_assets = objects.len();
        println!("Found {} assets to download", total_assets);
        
        for (i, (_name, asset_info)) in objects.iter().enumerate() {
            if let Some(hash) = asset_info.get("hash").and_then(|h| h.as_str()) {
                let prefix = &hash[0..2];
                let asset_url = format!("https://resources.download.minecraft.net/{}/{}", prefix, hash);
                
                let mut asset_path = assets_dir.clone();
                asset_path.push("objects");
                asset_path.push(prefix);
                asset_path.push(hash);
                
                // Only download if not already exists
                if !asset_path.exists() {
                    if i % 100 == 0 {
                        println!("Downloading assets: {}/{}", i, total_assets);
                    }
                    download_file(&asset_url, &asset_path)?;
                }
            }
        }
    }
    
    println!("Assets downloaded successfully!");
    Ok(())
}

// Main function to download everything for a version
pub fn setup_minecraft_version(version_id: &str, instance_path: &PathBuf, download_assets: bool,) -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up Minecraft {} in instance...", version_id);
    
    download_minecraft_client(version_id, instance_path)?;
    download_libraries(version_id, instance_path)?;
    
    if download_assets {
        self::download_assets(version_id, instance_path)?;
    }
    
    println!("Minecraft {} setup complete!", version_id);
    Ok(())
}