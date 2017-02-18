extern crate image;
extern crate icns;

use icns::{IconFamily, IconType};
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufWriter};

// sips -s format tiff tamil.tiff --out .bmp

fn tiff_to_png(source: &'static str, output: &'static str) -> bool {
  let img = image::open( &Path::new(source) ).unwrap();
  let ref mut fout = File::create( &Path::new(output) ).unwrap();
  let _ = img.save( fout, image::PNG ).unwrap();
  return true;
}

fn icon_to_png(source: &'static str, output: &'static str) -> bool {
  // Read binary data in to a buffer
  let file = BufReader::new( File::open(source).unwrap() );

  // Load an icon family from an ICNS file.
  let icon_family = IconFamily::read(file).unwrap();

  // Possible quality levels
  let types = vec![
    IconType::RGBA32_512x512_2x, IconType::RGBA32_512x512, IconType::RGBA32_256x256_2x,
    IconType::RGBA32_256x256, IconType::RGBA32_128x128_2x, IconType::RGB24_128x128,
    IconType::RGBA32_32x32_2x, IconType::RGB24_32x32, IconType::RGBA32_16x16_2x,
    IconType::RGB24_16x16
  ];

  // Get the best quality icon
  for (_, &icon_format) in types.iter().enumerate() {
    // TODO: Refactor so that if the first item in the quality levels is
    // not found that we try the next best quality until we get something.
    let icon = icon_family.get_icon_with_type(icon_format);
    match icon {
        Ok(default_icon_image) => {
          // Create a png from the best quality icon
          let default_icon_file = BufWriter::new( File::create(output).unwrap() );

          // Save the file locally
          default_icon_image.write_png(default_icon_file).unwrap();
          break;
        },
        Err(_) => continue,
    }

  }
  return true;
}

fn main() {
  // Write the system icon
  icon_to_png(
    "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/GenericApplicationIcon.icns",
    "defaultIcon.png"
  );
  tiff_to_png(
    "/System/Library/Input Methods/TamilIM.app/Contents/Resources/Tamil.tiff",
    "meow.png"
  );
}
