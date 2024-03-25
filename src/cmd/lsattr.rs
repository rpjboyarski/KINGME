use crate::ioctl::ioctl_file_get_flags;
use crate::ui::{print_error, print_status};

pub unsafe fn lsattr(path: String) {
    let flags = ioctl_file_get_flags(path.clone());
    match flags {
        Ok(flag) => {
            if flag == 0 {
                print_status(format!("{path} is mutable"));
            } else if flag == 16 {
                print_status(format!("{path} is immutable"));
            } else {
                print_status(format!("{path} has unknown flags: {}", flag));
            }
        }
        Err(e) => {
            print_error(e);
        }
    }
}