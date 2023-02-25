use std::process::Command;
use tokio::signal::unix::{signal, SignalKind};

fn wait_for_child() {
    loop {
        match nix::sys::wait::waitpid(
            nix::unistd::Pid::from_raw(-1),
            Some(nix::sys::wait::WaitPidFlag::WNOHANG),
        ) {
            Ok(ws) => {
                // we loop until no state changes are detected
                if let nix::sys::wait::WaitStatus::StillAlive = ws {
                    break;
                }
            }
            Err(e) => {
                panic!("waitpid failed: {}", e);
            }
        }
    }
}

async fn handle_signals() {
    // Represents the SIGALRM signal.
    //
    // On Unix systems this signal is sent when a real-time timer has expired.
    // By default, the process is terminated by this signal.
    let mut sigalarm = signal(SignalKind::alarm()).unwrap();

    // Represents the SIGCHLD signal.
    //
    // On Unix systems this signal is sent when the status of a child process
    // has changed. By default, this signal is ignored.
    let mut sigchld = signal(SignalKind::child()).unwrap();

    // Represents the SIGHUP signal.
    //
    // On Unix systems this signal is sent when the terminal is disconnected.
    // By default, the process is terminated by this signal.
    let mut sighup = signal(SignalKind::hangup()).unwrap();

    // Represents the SIGINT signal.
    //
    // On Unix systems this signal is sent to interrupt a program.
    // By default, the process is terminated by this signal.
    let mut sigint = signal(SignalKind::interrupt()).unwrap();

    // Represents the SIGIO signal.
    //
    // On Unix systems this signal is sent when I/O operations are possible
    // on some file descriptor. By default, this signal is ignored.
    let mut sigio = signal(SignalKind::io()).unwrap();

    // Represents the SIGPIPE signal.
    //
    // On Unix systems this signal is sent when the process attempts to write
    // to a pipe which has no reader. By default, the process is terminated by
    // this signal.
    let mut sigpipe = signal(SignalKind::pipe()).unwrap();

    // Represents the SIGQUIT signal.
    //
    // On Unix systems this signal is sent to issue a shutdown of the
    // process, after which the OS will dump the process core.
    // By default, the process is terminated by this signal.
    let mut sigquit = signal(SignalKind::quit()).unwrap();

    // Represents the SIGTERM signal.
    //
    // On Unix systems this signal is sent to issue a shutdown of the
    // process. By default, the process is terminated by this signal.
    let mut sigterm = signal(SignalKind::terminate()).unwrap();

    // Represents the SIGUSR1 signal.
    //
    // On Unix systems this is a user defined signal.
    // By default, the process is terminated by this signal.
    let mut sigusr1 = signal(SignalKind::user_defined1()).unwrap();

    // Represents the SIGUSR2 signal.
    //
    // On Unix systems this is a user defined signal.
    // By default, the process is terminated by this signal.
    let mut sigusr2 = signal(SignalKind::user_defined2()).unwrap();

    // Represents the SIGWINCH signal.
    //
    // On Unix systems this signal is sent when the terminal window is resized.
    // By default, this signal is ignored.
    let mut sigwinch = signal(SignalKind::window_change()).unwrap();

    tokio::spawn(async move {
        loop {
            let sig = tokio::select! {
                _ = sigchld.recv() => {
                    wait_for_child();

                    Some(nix::sys::signal::Signal::SIGCHLD)
                }
                _ = sigalarm.recv() => {
                    Some(nix::sys::signal::Signal::SIGALRM)
                }
                _ = sighup.recv() => {
                    Some(nix::sys::signal::Signal::SIGHUP)
                }
                _ = sigint.recv() => {
                    Some(nix::sys::signal::Signal::SIGINT)
                }
                _ = sigio.recv() => {
                    Some(nix::sys::signal::Signal::SIGIO)
                }
                _ = sigpipe.recv() => {
                    Some(nix::sys::signal::Signal::SIGPIPE)
                }
                _ = sigquit.recv() => {
                    Some(nix::sys::signal::Signal::SIGQUIT)
                }
                _ = sigterm.recv() => {
                    Some(nix::sys::signal::Signal::SIGTERM)
                }
                _ = sigusr1.recv() => {
                    Some(nix::sys::signal::Signal::SIGUSR1)
                }
                _ = sigusr2.recv() => {
                    Some(nix::sys::signal::Signal::SIGUSR2)
                }
                _ = sigwinch.recv() => {
                    Some(nix::sys::signal::Signal::SIGWINCH)
                }
            };

            if let Some(sig) = sig {
                nix::sys::signal::kill(nix::unistd::Pid::from_raw(-1), sig)
                    .expect("failed to send signal");
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
