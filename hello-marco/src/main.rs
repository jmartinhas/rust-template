use clap::Parser; // Add this line

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Martin Helmhout", about = "A MP game")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Martin Helmhout")]
    Play {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Play { name }) => {
            let result: String = hello_marco::marco_polo(&name);
            println!("{}", result);
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}
