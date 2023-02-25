use clap::Parser;

#[derive(Parser)]
#[command(about = "An init process purpose-built for containers")]
struct Cli {
    command: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("command: {:?}", cli.command);
}
