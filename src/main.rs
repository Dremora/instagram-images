use std::{ffi::OsStr, fs::File, io::BufWriter};

use clap::Parser;
use image::{GenericImageView, ImageEncoder, ImageReader};
use instagram_images::{
    new_dimensions, pad_image, parse_directory_path, resize_image, Cli, Resolution,
};

fn main() {
    let args = Cli::parse();
    let aspect_ratio = args.aspect_ratio;

    let source = parse_directory_path(&args.source).expect("Source directory doesn't exist");
    let source_canonicalized = source
        .canonicalize()
        .expect("Failed to canonicalize source directory");

    let target = parse_directory_path(&args.target).expect("Target directory doesn't exist");
    let target_canonicalized = target
        .canonicalize()
        .expect("Failed to canonicalize target directory");

    if source_canonicalized == target_canonicalized {
        eprintln!("Source and target directories must be different");
        std::process::exit(1);
    }

    let files = source_canonicalized
        .read_dir()
        .expect("Failed to read source directory");
    for file in files {
        let file = file.expect("Failed to read file");
        if file.file_type().expect("Failed to get file type").is_file() {
            let file_path = file.path();
            let extension = file_path.extension().unwrap_or(&OsStr::new(""));
            let extension = extension.to_string_lossy();
            let file_stem = file_path.file_stem().expect("Failed to get file stem");
            let file_stem = file_stem.to_string_lossy();
            if extension == "jpeg" || extension == "jpg" {
                println!("Processing file: {}...", file_path.display());
                let image = ImageReader::open(file.path())
                    .expect("Failed to open image")
                    .decode()
                    .expect("Failed to decode image");

                let resolution = Resolution::from_pair(image.dimensions());
                println!("Resolution: {}", resolution);
                let visible_image_dimensions = new_dimensions(aspect_ratio, &resolution);
                println!("Resizing to: {}", aspect_ratio.to_resolution());
                println!("Visible area: {}", visible_image_dimensions);
                let visible_image_resized = resize_image(&image, &visible_image_dimensions);
                let padded_image =
                    pad_image(visible_image_resized, aspect_ratio, args.pad_color.into());

                let padded_image_path = target_canonicalized
                    .join(format!("{}-{}.{}", file_stem, aspect_ratio, extension));

                let file = File::create(&padded_image_path).expect("Failed to create file");
                let writer = BufWriter::new(file);

                let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(writer, 100);

                encoder
                    .write_image(
                        padded_image.as_bytes(),
                        padded_image.width(),
                        padded_image.height(),
                        padded_image.color().into(),
                    )
                    .expect("Failed to write image");

                println!(
                    "Wrote {}",
                    padded_image_path
                        .canonicalize()
                        .expect("Failed to canonicalize")
                        .to_string_lossy()
                );
            }
        }
    }
}
