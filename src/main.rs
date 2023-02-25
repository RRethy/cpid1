use std::process::Command;

fn main() {
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();

    // handle_signals().await;

    Command::new(&args[1])
        .args(&args[2..])
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on process");
    Ok(())
}
