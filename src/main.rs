use std::process::Command;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("command: {:?}", args);
    Command::new(&args[1])
        .args(&args[2..])
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on process");
}
