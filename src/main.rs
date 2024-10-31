use std::fs::read;

use image::MemoryImage;

mod image;

fn main() {
    match read("yaASM.bin") {
        Ok(memory_image_bytes) => match MemoryImage::read(&memory_image_bytes) {
            Ok(mut memory_image) => {
                for module in memory_image.modules() {
                    for sector in module.sectors() {
                        for syllable in sector.syllables() {
                            for (idx, word) in syllable.words().iter().enumerate() {
                                print!("{:04x} ", word.value());

                                if (idx != 0) && ((idx + 1) % 16 == 0) {
                                    println!();
                                }
                            }
                            println!();
                            println!();
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading memory image");
            }
        },
        Err(e) => {
            eprintln!("{e}");
        }
    }
}
