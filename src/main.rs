use image::{DynamicImage, GenericImageView, Pixel, RgbImage};
use std::error::Error;

const END_MARKER: &str = "EOF"; // Utilisation d'une chaîne spécifique comme marqueur de fin

fn encode_message_in_image(image_path: &str, output_path: &str, message: &str) -> Result<(), Box<dyn Error>> {
    let img = image::open(image_path)?;
    let mut img = img.to_rgb8(); // Convertir en image RGB mutable
    let (width, height) = img.dimensions();

    // Convertir le message en bits avec un marqueur de fin
    let full_message = format!("{}{}", message, END_MARKER);
    let message_bits = full_message.bytes().flat_map(|byte| {
        (0..8).rev().map(move |i| (byte >> i) & 1)
    }).collect::<Vec<_>>();

    if message_bits.len() > (width * height * 3) as usize {
        return Err("Message is too long to fit in the image".into());
    }

    // Encodage des bits dans les pixels de l'image
    for (i, pixel) in img.pixels_mut().enumerate() {
        for (j, channel) in pixel.0.iter_mut().take(3).enumerate() {
            if let Some(bit) = message_bits.get(i * 3 + j) {
                *channel = (*channel & !1) | bit;
            }
        }
    }

    img.save(output_path)?;
    Ok(())
}

fn decode_message_from_image(image_path: &str) -> Result<String, Box<dyn Error>> {
    let img = image::open(image_path)?;
    let img = img.to_rgb8(); // Convertir en image RGB
    let mut bits = Vec::new();

    for pixel in img.pixels() {
        for channel in pixel.0.iter().take(3) {
            bits.push(channel & 1);
        }
    }

    let bytes = bits.chunks(8).map(|chunk| {
        chunk.iter().enumerate().fold(0, |acc, (i, &bit)| acc | (bit << (7 - i)))
    }).collect::<Vec<_>>();

    let decoded_message = String::from_utf8_lossy(&bytes);
    if let Some(end_pos) = decoded_message.find(END_MARKER) {
        return Ok(decoded_message[..end_pos].to_string());
    }

    Err("Failed to find the end marker in the decoded message".into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_image = "prendre-soin_duree-vie-chat.jpg";
    let output_image = "prendre-soin_duree-vie-chat_modifi.jpg";
    let message = "Hello, world! EOF";

    encode_message_in_image(input_image, output_image, message)?;
    println!("Message hidden successfully in {}", output_image);

    let decoded_message = decode_message_from_image(output_image)?;
    println!("Decoded message: {}", decoded_message);

    Ok(())
}
