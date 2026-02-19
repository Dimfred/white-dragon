mod app;
mod drag_view;
mod window;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "white-dragon")]
#[command(about = "A macOS drag-and-drop CLI tool")]
pub struct Args {
    /// Files to make draggable
    pub files: Vec<PathBuf>,

    /// Exit after first successful drag
    #[arg(short = 'x', long)]
    pub and_exit: bool,

    /// Show only icons, no text
    #[arg(short = 'i', long)]
    pub icon_only: bool,
}

fn main() {
    let args = Args::parse();

    if args.files.is_empty() {
        eprintln!("Error: No files specified");
        std::process::exit(1);
    }

    // Validate files exist
    let valid_files: Vec<PathBuf> = args
        .files
        .into_iter()
        .filter(|path| {
            if path.exists() {
                true
            } else {
                eprintln!("Warning: File does not exist: {}", path.display());
                false
            }
        })
        .collect();

    if valid_files.is_empty() {
        eprintln!("Error: No valid files to display");
        std::process::exit(1);
    }

    // Run the app
    app::run(valid_files, args.and_exit, args.icon_only);
}
