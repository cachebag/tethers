use clap::Parser;
use std::path::PathBuf;
use unrot_core::{find_broken_symlinks, find_candidates};

fn main() {
    let args = Args::parse();

    // FIXME: Result instead
    let path = if args.path.as_os_str() == "." {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    } else {
        args.path
    };

    let broken = find_broken_symlinks(&path);

    if args.list {
        for link in &broken {
            println!("{link}");
        }
        return;
    }

    for link in &broken {
        println!("{link}");
        let candidates = find_candidates(link, &path);
        if candidates.is_empty() {
            println!("  no candidates found");
        } else {
            for (i, candidate) in candidates.iter().enumerate() {
                println!("  [{}] {}", i + 1, candidate.display());
            }
        }
        println!();
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
