use dialog::DialogBox;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn init() {
    if !is_installed() {
        let choice = dialog::Question::new(
            "PolyMC is not installed.\n\nWould you like to open Discover to install it now?",
        )
        .title("PolyMC not installed")
        .show()
        .expect("Could not display dialog box");

        if choice == dialog::Choice::Yes {
            Command::new("plasma-discover")
                .arg("--application")
                .arg("appstream:org.polymc.PolyMC")
                .spawn()
                .expect("Could not launch plasma-discover")
                .wait()
                .expect("Failed to wait for plasma-discover");
        }

        eprintln!("Please install PolyMC from Discover, and restart ssm_minecraft");
        std::process::exit(1);
    }
}

#[allow(dead_code)]
fn prepare_profiles() {
    // PolyMC paths
    let poly_dir = get_dir();
    let config_dir = poly_dir.join(".var/app/org.polymc.PolyMC/data/PolyMC");
    let profiles_dir = config_dir.join("profiles");
    let instances_dir = config_dir.join("instances");

    // Create PolyMC directories if they don't exist
    fs::create_dir_all(&profiles_dir).expect("Could not create PolyMC profiles directory");
    fs::create_dir_all(&instances_dir).expect("Could not create PolyMC instances directory");
}

fn is_installed() -> bool {
    let output = Command::new("flatpak")
        .arg("list")
        .output()
        .expect("Could not get list of flatpak installed packages");

    String::from_utf8_lossy(&output.stdout).contains("org.polymc.PolyMC")
}

#[allow(dead_code)]
fn get_dir() -> PathBuf {
    let output = Command::new("flatpak")
        .arg("info")
        .arg("-l")
        .arg("org.polymc.PolyMC")
        .output()
        .expect("Could not get PolyMC package info");

    PathBuf::from(String::from_utf8_lossy(&output.stdout).trim())
}
