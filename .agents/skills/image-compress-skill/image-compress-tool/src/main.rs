use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use std::io::BufWriter;
use imagequant::RGBA;
use image::imageops::FilterType;
use std::ffi::OsStr;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input image file or URL
    #[arg(short, long)]
    input: String,

    /// Output image file (optional, defaults to input_optimized.png or .webp based on context)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Quality (0-100) for lossy compression (imagequant) or WebP quality
    #[arg(short, long, default_value = "80")]
    quality: u8,

    /// Max width for resizing (aspect ratio preserved)
    #[arg(long)]
    width: Option<u32>,

    /// Max height for resizing (aspect ratio preserved)
    #[arg(long)]
    height: Option<u32>,
}

fn download_image(url: &str) -> Result<tempfile::NamedTempFile> {
    println!("Downloading image from {}...", url);
    let mut response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        anyhow::bail!("Failed to download image: status code {}", response.status());
    }
    
    // We try to guess extension from url or content-type, but simple is to use a safe default
    // Image crate guesses format from content if we use Reader, but image::open depends on extension.
    // Let's force a .png suffix for temp file to hint image crate, or use image::io::Reader
    let mut temp_file = tempfile::Builder::new()
        .suffix(".png") // Hack: Assume most inputs work if we trick it, or use format guessing
        .tempfile()?;
    
    response.copy_to(temp_file.as_file_mut())?;
    Ok(temp_file)
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Check if input is a URL or a file path
    let (input_path, temp_file_guard) = if args.input.starts_with("http://") || args.input.starts_with("https://") {
        let temp = download_image(&args.input)?;
        // Keep temp file alive until function ends
        (temp.path().to_path_buf(), Some(temp))
    } else {
        (PathBuf::from(&args.input), None)
    };

    let mut output_path = args.output.unwrap_or_else(|| {
        let mut p = if let Some(ref _guard) = temp_file_guard {
             PathBuf::from("downloaded_optimized")
        } else {
             input_path.clone()
        };
        
        if let Some(stem) = p.file_stem() {
            let mut new_stem = stem.to_os_string();
             if temp_file_guard.is_none() {
                 new_stem.push("_optimized");
             }
            p.set_file_name(new_stem);
        }
        
        if p.extension().is_none() {
             p.set_extension("png");
        }
        p
    });

    // Check if output is WebP
    let is_webp = output_path.extension()
        .and_then(OsStr::to_str)
        .map(|s| s.eq_ignore_ascii_case("webp"))
        .unwrap_or(false);

    println!("Reading image from {:?}", input_path);
    
    // Use Reader to guess format content instead of relying on extension for temp files
    let img = image::io::Reader::open(&input_path)
        .context("Failed to open image file")?
        .with_guessed_format()
        .context("Failed to guess image format")?
        .decode()
        .context("Failed to decode image")?;

    // Resize if requested
    let mut img = img; // Make mutable for resize
    if args.width.is_some() || args.height.is_some() {
        let current_width = img.width();
        let current_height = img.height();
        
        let target_width = args.width.unwrap_or(current_width);
        let target_height = args.height.unwrap_or(current_height);

        if target_width < current_width || target_height < current_height {
            println!("Resizing to {}x{} (preserve aspect ratio)...", target_width, target_height);
            img = img.resize(target_width, target_height, FilterType::Lanczos3);
        }
    }

    if is_webp {
        println!("Saving as WebP to {:?} (Quality: {})", output_path, args.quality);
        
        let encoder = webp::Encoder::from_image(&img)
            .map_err(|e| anyhow::anyhow!("Failed to create webp encoder: {}", e))?;
        
        let webp_data = encoder.encode(args.quality as f32);
        
        let mut file = std::fs::File::create(&output_path)?;
        file.write_all(&webp_data)?;
    } else {
        // PNG Path
        let img = img.to_rgba8();
        let width = img.width();
        let height = img.height();
        let pixels = img.into_raw();

        println!("Quantizing image (quality: {})...", args.quality);
        let mut attributes = imagequant::new();
        attributes.set_quality(0, args.quality)?;
        
        let pixels_rgba: Vec<RGBA> = pixels.chunks(4)
            .map(|c| RGBA { r: c[0], g: c[1], b: c[2], a: c[3] })
            .collect();

        let mut image = attributes.new_image(pixels_rgba, width as usize, height as usize, 0.0)?;
        
        let mut quantization_result = attributes.quantize(&mut image)?;
        let (palette, pixels_indices) = quantization_result.remapped(&mut image)?;

        println!("Encoding intermediate PNG...");
        let mut png_buffer = Vec::new();
        {
            let w = BufWriter::new(&mut png_buffer);
            let mut encoder = png::Encoder::new(w, width, height);
            encoder.set_color(png::ColorType::Indexed);
            encoder.set_depth(png::BitDepth::Eight);
            
            let mut rgb_palette = Vec::new();
            let mut alpha_palette = Vec::new();
            let mut has_alpha = false;
            
            for color in palette {
                rgb_palette.push(color.r);
                rgb_palette.push(color.g);
                rgb_palette.push(color.b);
                alpha_palette.push(color.a);
                if color.a != 255 {
                    has_alpha = true;
                }
            }
            
            encoder.set_palette(rgb_palette);
            if has_alpha {
                encoder.set_trns(alpha_palette);
            }
            
            let mut writer = encoder.write_header()?;
            writer.write_image_data(&pixels_indices)?;
        }

        println!("Optimizing PNG (oxipng)...");
        let options = oxipng::Options::from_preset(2); 
        let optimized_data = oxipng::optimize_from_memory(&png_buffer, &options)?;

        println!("Saving to {:?}", output_path);
        std::fs::write(output_path, optimized_data)?;
    }

    println!("Done!");
    Ok(())
}