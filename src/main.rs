use image::{GenericImageView};

fn main() {
    // Load the image
    let img = image::open("pastel_de_nata.png").expect("Failed to open image");
    // let img = image::open("fries.png").expect("Failed to open image");

    // desired width for the ASCII art output
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

    // Define the ASCII character set, from light to dark (adjusting for aspect ratio)
    let ascii_chars = vec![
        ' ', '.', ',', ':', ';', '+', '*', '%', '#', '@', // More compact characters
        'M', 'W', 'N', 'X', '$' // Wider characters to cover more space
    ];

    // Get the dimensions of the resized image
    let (width, height) = gray_img.dimensions();

    // Create a vector to hold the ASCII art lines
    let mut ascii_art = Vec::new();

    for y in 0..height {
        let mut line = String::new();
        for x in 0..width {
            // Get the pixel's brightness (grayscale value)
            let pixel = gray_img.get_pixel(x, y);
            let brightness = pixel[0]; // grayscale value

            // Normalize the brightness to the range of the ASCII characters
            let idx = ((brightness as f32 / 255.0) * (ascii_chars.len() - 1) as f32).round() as usize;
            let ascii_char = ascii_chars[idx];

            // Add two ASCII characters for each pixel to adjust for aspect ratio
            line.push(ascii_char);
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
