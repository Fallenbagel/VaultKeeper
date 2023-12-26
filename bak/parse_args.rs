use clap::Parser;

/// Backs up a Vaultwarden instance
#[derive(Parser, Debug)]
#[command(
    name = "Vaultwarden Backup",
    author = "Fallenbagel <fallenbagel@github.com>",
    version = "1.0",
    about = "Backs up a Vaultwarden instance"
)]
#[command(
    help_template = "{name}\n{author-with-newline}{about-section}\n{usage-heading} {usage}\n\n{all-args} {tab}"
)]
struct Args {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,

    /// Creates a config file with default values
    #[arg(short, long, value_name = "FILE", conflicts_with = "config")]
    default_config: bool,
}

pub fn parse_args() {
    println!("{:?}", args);

    if args.default_config {
        println!("Creating config file");
    }
}
