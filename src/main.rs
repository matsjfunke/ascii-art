use image::{GenericImageView, Pixel};

fn main() {
    // Load the image
    // let img = image::open("fries.png").expect("Failed to open image");
    let img = image::open("pastel_de_nata.png").expect("Failed to open image");
    
    // Determine the desired width for the ASCII art output
    let desired_width = 80;

    // Calculate the scaling factor to maintain aspect ratio
    let (width, height) = img.dimensions();
    let aspect_ratio = height as f32 / width as f32;
    println!("aspect ratio : {}", aspect_ratio);
    let scaled_height = (desired_width as f32 * aspect_ratio).round() as u32;

    // Resize the image while maintaining aspect ratio
    let resized_img = img.resize_exact(desired_width, scaled_height, image::imageops::FilterType::Lanczos3);

    // Convert the resized image to grayscale
    let gray_img = resized_img.grayscale();

    // Define the ASCII character set, from light to dark
    let ascii_chars = vec![' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

    // Get the dimensions of the image
    let (width, height) = gray_img.dimensions();

    // Create a vector to hold the ASCII art lines
    let mut ascii_art = Vec::new();

    for y in 0..height {
        let mut line = String::new();
        for x in 0..width {
            // Get the pixel's brightness (luma)
            let pixel = gray_img.get_pixel(x, y);
            let luma = pixel.to_luma().0[0];

            // Normalize the brightness to the range of the ASCII characters
            let idx = (luma as f32 / 255.0 * (ascii_chars.len() - 1) as f32).round() as usize;
            let ascii_char = ascii_chars[idx];

            // Add the character to the line
            line.push(ascii_char);
        }
        // Add the line to the ASCII art
        ascii_art.push(line);
    }

    // Print the ASCII art
    for line in ascii_art {
        println!("{}", line);
    }
}
