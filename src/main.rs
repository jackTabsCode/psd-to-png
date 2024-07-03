use anyhow::Context;
use clap::Parser;
use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder};
use psd::Psd;
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    path: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let current_dir = args.path.unwrap_or_else(|| PathBuf::from("."));
    let walk = WalkDir::new(current_dir).into_iter().filter_map(|e| e.ok());

    for entry in walk {
        if entry.path().extension().and_then(|s| s.to_str()) == Some("psd") {
            match run_file(entry.path()) {
                Ok(_) => println!("Converted {:?}", entry.path()),
                Err(e) => eprintln!("Failed to convert {:?}: {}", entry.path(), e),
            }
        }
    }

    Ok(())
}

fn run_file(path: &Path) -> anyhow::Result<()> {
    let data = fs::read(path).context("Failed to read PSD file")?;

    let png = psd_to_png(data).context("Failed to convert PSD to PNG")?;
    let output_path = path.with_extension("png");

    fs::write(output_path, png).context("Failed to write PNG file")?;

    Ok(())
}

fn psd_to_png(data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let psd = Psd::from_bytes(&data).context("Failed to parse PSD file")?;

    let width = psd.width();
    let height = psd.height();

    let pixels = psd.rgba();

    let mut png = Vec::new();
    PngEncoder::new(&mut png)
        .write_image(&pixels, width, height, ExtendedColorType::Rgba8)
        .context("Failed to encode PNG")?;

    Ok(png)
}
