use std::path::PathBuf;
use clap::Parser;
use unrot_core::find_broken_symlinks;

fn main() {
    let args = Args::parse();

    // FIXME: Result instead 
    let path = if args.path == PathBuf::from(".") {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    } else {
        args.path
    };

    let broken = find_broken_symlinks(&path);
    for link in broken {
        println!("{link}");
    }
}

#[derive(Parser)]
#[command(name = "unrot")]
struct Args {
    #[arg(short, long)]
    list: bool,

    #[arg(short, long, default_value = ".")]
    path: PathBuf,
}
