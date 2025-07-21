use std::{process::Command, fs::File, env, path::PathBuf};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::io::Write;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    // Check if "remove" argument is provided
    if args.len() > 1 && args[1] == "remove" {
        return uninstall_aether();
    }

    // Original install logic
    install_aether()
}

fn install_aether() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let releases: Vec<Release> = client.get("https://api.github.com/repos/The-Baremetal/aether2/releases")
        .header("User-Agent", "aether-installer")
        .send()?
        .json()?;
    
    let release = releases.iter()
        .find(|r| r.tag_name != "main")
        .ok_or("No suitable release found")?;

    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    
    if os != "linux" {
        return Err("Only Linux supported.".into());
    }

    let platform_map = [
        (("linux", "x86"), "linux_386"),
        (("linux", "x86_64"), "linux_amd64"),
        (("linux", "aarch64"), "linux_arm64"),
        (("linux", "arm"), "linux_arm"),
    ];
    
    let platform_str = platform_map.iter()
        .find(|((os_key, arch_key), _)| *os_key == os && *arch_key == arch)
        .map(|(_, pat)| *pat)
        .ok_or("Unsupported Linux architecture")?;

    let pkg_exts = ["deb", "rpm", "flatpak"];
    let asset = release.assets.iter()
        .find(|a|
            pkg_exts.iter().any(|ext| a.name.ends_with(ext)) &&
            a.name.contains(platform_str)
        )
        .ok_or("No matching asset found for your Linux architecture")?;

    println!("Downloading {}...", asset.name);
    let resp = client.get(&asset.browser_download_url)
        .header("User-Agent", "aether-installer")
        .send()?;
    
    let bytes = resp.bytes()?;

    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let mut dest = PathBuf::from(home);
    dest.push(&asset.name);

    let mut out = File::create(&dest)?;
    out.write_all(&bytes)?;

    if asset.name.ends_with(".deb") {
        println!("Installing .deb package...");
        let status = Command::new("sudo")
            .arg("dpkg")
            .arg("-i")
            .arg(&dest)
            .status()?;
        if !status.success() {
            return Err("dpkg install failed".into());
        }
    } else if asset.name.ends_with(".rpm") {
        let status = Command::new("sudo")
            .arg("rpm")
            .arg("-i")
            .arg(&dest)
            .status()?;
        if !status.success() {
            return Err("rpm install failed".into());
        }
    } else if asset.name.ends_with(".flatpak") {
        let status = Command::new("flatpak")
            .arg("install")
            .arg("--user")
            .arg("--assumeyes")
            .arg(&dest)
            .status()?;
        if !status.success() {
            return Err("flatpak install failed".into());
        }
    } else {
        return Err("Unsupported package format".into());
    }

    std::fs::remove_file(&dest).ok();

    println!("Aether installed successfully!");
    let package_name = "aether-rs";
    println!("Uninstalling the installer using cargo...");
    let status = Command::new("cargo")
        .arg("uninstall")
        .arg(package_name)
        .status()?;
        
    if status.success() {
        println!("You can now run aether by typing 'aether' in your terminal.");
    } else {
        eprintln!("Failed to uninstall installer via cargo, please uninstall manually.");
    }

    Ok(())
}

fn uninstall_aether() -> Result<(), Box<dyn std::error::Error>> {
    println!("Uninstalling Aether...");
    let mut uninstalled = false;

    if Command::new("which").arg("dpkg").status().is_ok() {
        let status = Command::new("sudo")
            .arg("dpkg")
            .arg("-r")
            .arg("aether")
            .status()?;
        if status.success() {
            println!("Aether has been successfully uninstalled.");
            uninstalled = true;
        }
    }

    if !uninstalled && Command::new("which").arg("rpm").status().is_ok() {
        let status = Command::new("sudo")
            .arg("rpm")
            .arg("-e")
            .arg("aether")
            .status()?;
        if status.success() {
            println!("Aether has been successfully uninstalled.");
            uninstalled = true;
        }
    }

    // Try flatpak
    if !uninstalled && Command::new("which").arg("flatpak").status().is_ok() {
        let status = Command::new("flatpak")
            .arg("uninstall")
            .arg("--user")
            .arg("--assumeyes")
            .arg("org.aether.Aether")
            .status()?;
        if status.success() {
            println!("Aether has been successfully uninstalled.");
            uninstalled = true;
        }
    }

    if !uninstalled {
        println!("Could not automatically uninstall Aether. You may need to remove it manually.");
        println!("Try one of these commands depending on how it was installed:");
        println!("  sudo dpkg -r aether");
        println!("  sudo rpm -e aether");
        println!("  flatpak uninstall --user org.aether.Aether");
    }

    let package_name = "aether-rs";
    let status = Command::new("cargo")
        .arg("uninstall")
        .arg(package_name)
        .status()?;
        
    if status.success() {
        println!("Installer uninstalled successfully.");
    } else {
        eprintln!("Failed to uninstall installer via cargo, please run 'cargo uninstall aether-rs' manually.");
    }
    Ok(())
}