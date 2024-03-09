use clap::Parser;
use cli::Cli;
use fs_extra::{copy_items, dir::CopyOptions};

mod cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Create(args) => {
            let options = CopyOptions::new();

            let args_path = args.from_path.to_str().expect("Something went wrong");
            if args.verbose {
                println!(
                    "Copying from directory {} to {}",
                    args_path,
                    args.dest_path.to_str().expect("Something went wrong")
                );
            }
            let from_path = vec![args_path];

            let _ = copy_items(&from_path, args.dest_path, &options);
        }
    }
}
