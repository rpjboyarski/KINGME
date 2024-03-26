extern crate nix;

use nix::sys::signal::{kill, killpg, Signal};
use nix::unistd::{getpgrp, getppid};

// Troll binary to replace legitimate files with
// Prints some text art, then attempts to kill the calling process and parent process
// This won't kill SSH sessions, but it should work on basic netcat reverse shells

fn main() {
    println!("⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⡿⢿⡟⣿⠻⣟⠿⣻⠟⣿⠻⣟⠿⡻⢟⠿⡻⢟⠿⣻⠟⡿⣛⠿⣛⠿⡻⢟⠿⡻⢟⠿⡻⢟⠿⡻⢟⠿⡻⢟⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢿⢧⠹⡜⣣⠞⠤⡛⢬⡓⡥⢛⡔⢫⠜⣣⠝⣎⢳⡙⣎⠳⢥⡛⡴⣩⠞⣡⢏⠵⣋⠞⣱⢋⠞⣱⠩⢎⡑⢪⠐⠡⢊⠶⣹⣻⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⣯⣿⠎⡱⡸⢃⡘⠴⡉⠦⠱⢌⠣⡜⠡⢎⡑⢎⡌⢣⠜⡢⢝⢢⠱⡑⢦⡙⠲⣌⠳⢬⡙⣤⠫⡜⠤⢋⡒⠌⡀⠀⠀⢂⡙⢦⣿⡿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢿⣻⣿⢐⡴⠁⢢⠘⠀⠠⢁⡋⠤⠓⠬⡑⢢⠘⠤⡘⠡⢎⡑⢊⠆⢣⠍⡢⠜⡱⣈⠓⢦⠱⣂⠳⣌⠣⡅⡘⠀⠀⠀⠀⠠⡘⢢⡙⢷⣫⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡗⠚⠀⠌⠀⠀⠀⢀⠂⠜⡠⢉⠆⡑⠄⠣⡀⠅⢃⠢⠌⠡⠊⠔⡨⢑⡘⠤⢡⢋⢆⠳⡌⡱⣈⠱⠐⡀⠀⠀⠀⠀⠡⠀⠅⣞⢧⣛⡾⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠐⠠⠀⠀⠀⠀⣈⠂⠄⠡⠈⠄⠁⡐⠠⢁⠐⡈⠄⡐⢂⠐⠠⠡⢈⠱⡀⢎⡘⢌⢢⠑⢣⢉⠂⠀⠁⠀⠀⠀⠀⢀⠂⠐⡿⣌⣟⣳⢿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡁⠌⠀⠀⠀⠀⠰⠠⠌⠂⠀⠁⠀⠄⠀⠀⠀⠂⠐⠠⠐⠀⠁⢂⠁⠂⠄⡁⠆⡘⠤⢃⠊⡅⢊⠐⠀⠀⠀⠀⠀⠀⠀⠀⢸⣳⡱⢮⡽⣯⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⡌⢡⠃⡜⠠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠁⠂⠐⡀⠆⠌⡀⠃⠌⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⠵⣹⢧⣻⢷⣯⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇⠀⢀⠠⠀⠥⠐⡁⢎⡴⢁⢤⠀⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣯⢻⡵⣫⣞⢿⣞⣿⣿\n⣿⣿⣿⣿⢿⣿⣿⣿⣿⣿⡏⢀⠐⡠⢂⣑⣢⣵⣾⣿⡯⡘⡄⣧⡀⠀⠀⠀⠀⠀⠀⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⢂⣿⣏⡷⣝⣧⡟⣯⣾⣿⣿\n⣿⣿⣿⡿⣿⣻⣿⣿⣿⣿⣿⣿⣿⣷⣿⣿⣿⣿⡿⣋⠕⠠⢃⠸⣿⣦⣀⠀⢀⠠⠁⠄⠈⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢄⣾⢷⣫⢾⣽⣶⣿⣿⢿⣻⣿\n⣿⣿⣿⣿⣿⢿⣷⣿⣿⣿⣿⣿⣿⡿⢿⠛⢏⡑⠆⡐⠈⠄⢣⠐⡈⠛⣿⣿⣶⣦⣭⣦⣭⣶⣴⣤⣦⣐⣀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⢎⣾⢯⡷⣯⣷⣿⢿⡽⣞⣯⢷⣿\n⣿⣿⣿⣿⣾⣿⣿⣯⣿⣿⣿⣿⣿⣇⠢⢉⠄⡒⠌⠰⡁⠌⡀⠆⡑⢢⠠⠙⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡁⠄⠀⠀⠀⠀⠀⠀⢀⠀⡠⣢⣾⣟⣯⣿⣿⣻⠽⣾⣹⡟⣾⢯⣿\n⣿⣿⣿⣿⣽⣿⣷⣿⣿⣿⣿⣿⣿⣿⣷⣌⢆⠱⣨⡑⡰⣈⠔⠠⠐⣡⠂⠥⡁⠆⡌⡙⢫⠟⠻⢿⣿⣿⣿⣿⣷⢀⠀⠀⠀⠀⠀⠐⠂⢀⣴⣿⣿⣿⣿⢻⣮⣽⣻⣳⢯⡽⣯⢿⣽\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⣷⣿⣷⣧⣎⡁⠐⡢⢍⠲⡱⢌⣒⡱⢢⢌⢃⡒⢌⡙⠻⢿⣿⡧⠀⠀⠀⠀⢠⣩⣴⣿⣿⣿⣿⣟⣾⣻⢾⣵⣳⢯⡿⣽⢯⣿⣻\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠉⢽⣿⣿⣿⡅⠂⢧⡘⢆⣣⢼⣩⣗⣮⣜⢮⡑⡌⡑⢢⠘⡽⡷⠀⠀⠀⢠⣾⡿⠿⣿⣿⣿⣿⣽⣯⣟⡾⣽⣻⣞⣷⣻⢾⣯⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠈⣾⣿⣯⣼⡇⢈⠧⣘⢧⣾⡿⠛⣻⣿⣿⣿⣷⣜⡔⢣⠜⡰⠡⠀⠀⢠⣴⣟⣤⣾⣿⣿⣿⣿⣟⣾⣯⣿⢯⣷⢿⣞⣿⣿⣻⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⢿⣤⣻⣿⣿⣿⠇⢈⠶⣩⣾⡟⠀⢸⣿⡿⢻⣿⣿⣿⣿⠢⡝⢢⠁⠀⣴⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣽⣿⣿⣻⣿⢿⣾⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠈⢛⠿⠿⠟⠋⠀⡘⢆⣳⣽⣇⠀⢿⣿⣿⣾⣿⡏⢸⣿⠁⠀⠀⠀⣼⠟⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣽⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣻⣿⣿⣿⣿⣿⣿⣧⠀⠠⠀⠀⠀⢀⠴⣙⢎⣻⣼⢿⣶⣤⣽⣾⣿⣯⠶⡛⠀⠀⠀⠀⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡀⠃⠀⠀⢀⠎⡞⡥⢊⡔⠫⡝⡹⢾⠷⢯⠾⠖⠋⠀⠀⠀⠀⣰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇⠀⠀⠀⠂⠚⠈⠔⠣⢌⠣⡔⢡⢊⡜⡰⢌⡂⢆⡄⣀⡄⣸⣿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢿⡏⠀⠀⠀⠀⠀⠀⠀⠀⠁⠈⠒⣡⢃⢎⡔⡣⢞⡘⢢⠐⡰⢱⠛⣿⣿⣿⢿⣿⣿⣿⣿⡟⣽⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣧⢋⡖⢬⡑⢎⡘⠀⠜⢡⣿⣗⣮⣵⢫⣟⣾⣿⣿⣿⣳⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢤⡚⢦⢏⡜⢢⠍⠂⠀⠀⢠⣿⣿⣿⣿⣿⣿⣿⣾⣟⣿⣿⣿⣻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡱⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠐⠈⠑⠪⣜⡱⢊⠄⠀⠀⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⣦⣀⡀⠀⣀⠐⠀⠀⠀⠀⠀⠀⠀⠀⠱⡍⠆⠀⠀⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⣱⣯⣿⢟⡳⠒⠙⠒⠶⣤⡀⠀⡏⠄⠀⢠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡗⣼⣿⣿⣿⡵⣓⠦⢤⠀⡀⢳⣜⡱⠀⣰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n\nNO PERSISTENCE?\n\n");

    let pgid = getpgrp();

    // Get the parent's PID
    let parent_pid = getppid();

    // Kill the parent process
    kill(parent_pid, Signal::SIGKILL).unwrap();

    // Kill all child processes
    killpg(pgid, Signal::SIGKILL).unwrap();
}