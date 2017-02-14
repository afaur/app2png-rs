extern crate icns;
use icns::{IconFamily, IconType};
use std::fs::File;
use std::io::{BufReader, BufWriter};

fn write_icon(path: String, icon: String, save_as: String) -> bool {
  // Read binary data in to a buffer
  let file = BufReader::new(
    File::open(path.clone() + icon.as_str().clone())
    .unwrap()
  );

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
  for (iter_num, &icon_format) in types.iter().enumerate() {
    // TODO: Refactor so that if the first item in the quality levels is
    // not found that we try the next best quality until we get something.
    let default_icon_image = icon_family.get_icon_with_type(icon_format).unwrap();

    // Create a png from the best quality icon
    let default_icon_file = BufWriter::new(
      File::create(save_as.to_string()).unwrap()
    );

    // Save the file locally
    default_icon_image.write_png(
      default_icon_file
    ).unwrap();
  }


  return true;
}

fn main() {
  // Write the system icon
  write_icon(
    "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/".to_string(),
    "GenericApplicationIcon.icns".to_string(),
    "defaultIcon.png".to_string()
  );
}
