// This is where Minecraft related API's are handled.

// Copper Launcher is licensed under GNU General Public License v3.0.
// Copyright (c) 2026 Suverent_Shiro
//
// See LICENSE file for more details.


use serde::Deserialize;


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

pub fn download_minecraft_version() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let manifest = fetch_minecraft_versions()?;
    
    let versions: Vec<String> = manifest
        .versions
        .into_iter()
        .map(|v| v.id)
        .collect();
    
    Ok(versions)
}