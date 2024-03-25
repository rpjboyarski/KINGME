use nix::unistd::Uid;
use crate::ioctl::{ioctl_file_get_flags, ioctl_file_set_flags};
use crate::ui::{print_error, print_status, print_success};

pub unsafe fn file_lock(path: String) {
    match ioctl_file_get_flags(path.clone()) {
        Ok(flag) => {
            if flag == 16 {
                print_status(format!("{path} is already immutable"));
                return;
            }
        }
        Err(e) => {
            print_error(e);
            return;
        }
    }

    if !Uid::effective().is_root() {
        print_error("Locking a file requires root privileges!");
        return;
    }

    match ioctl_file_set_flags(path.clone(), 16) {
        Ok(_) => print_success(format!("{path} is now immutable")),
        Err(e) => print_error(e),
    };
}