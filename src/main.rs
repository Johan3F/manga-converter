mod convert;
mod extract;
mod models;

use std::{
    env::current_dir,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "manga converter")]
#[command(version = "0.5")]
#[command(about = "Converts .pbz files into a manga pdf", long_about = None)]
struct Cli {
    #[arg(short('d') , long , default_value = get_default_destination().into_os_string())]
    destination: PathBuf,

    #[arg(short('f'), long)]
    file: PathBuf,
}

fn get_default_destination() -> PathBuf {
    let mut path = current_dir().unwrap();
    path.push("output");
    path
}

fn main() {
    let args = Cli::parse();

    ensure_destination(&args.destination)
        .expect("unable to ensure that the destination folder exists");

    process(&args.file, &args.destination).expect("unable to process file");
}

fn process(file_path: &Path, destination_folder: &Path) -> Result<()> {
    println!("Processing: {:?}", file_path);
    let operation_folder = tempfile::tempdir()?;

    let images = extract::extract(file_path, operation_folder.path())?;
    convert::convert_to_pdf(file_path, destination_folder, images)?;

    Ok(())
}

fn ensure_destination(destination_folder: &Path) -> Result<()> {
    if !destination_folder.exists() {
        create_dir_all(destination_folder)?;
    }

    if !destination_folder.is_dir() {
        bail!("the destination folder is not a folder");
    }

    Ok(())
}
