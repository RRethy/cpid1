use std::process::Command;
use tokio::signal::unix::{signal, SignalKind};

async fn handle_signals() {
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    let mut sigalarm = signal(SignalKind::alarm()).unwrap();
    let mut sighup = signal(SignalKind::hangup()).unwrap();
    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = sigterm.recv() => println!("SIGTERM received"),
                _ = sigalarm.recv() => println!("SIGALRM received"),
                _ = sighup.recv() => println!("SIGHUP received"),
                _ = sigint.recv() => println!("SIGINT received"),
            }
        }
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();

    handle_signals().await;

    Command::new(&args[1])
        .args(&args[2..])
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on process");
    Ok(())
}
