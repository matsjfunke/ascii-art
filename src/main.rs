use image::GenericImageView;
use image::imageops::FilterType;

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let mut img = image::open("fries.png").expect("Failed to open image");

    // The dimensions method returns the image's width and height.
    let dimensions = img.dimensions();
    println!("Dimensions: {:?}", dimensions);

    // The color method returns the image's `ColorType`.
    println!("Color type: {:?}", img.color());

    // The width method returns the width of the image.
    let width = img.width();
    println!("Width: {}", width);

    // The height method returns the height of the image.
    let height = img.height();
    println!("Height: {}", height);

    // Iterate over all pixels
    // for (x, y, pixel) in img.pixels() {
    //     println!("Pixel at ({}, {}): {:?}", x, y, pixel);
    //     // Limit the output to avoid overwhelming the console
    //     if x > 10 && y > 10 {
    //         break;
    //     }
    // }

    // Get a specific pixel
    let pixel = img.get_pixel(0, 0);
    println!("Pixel at (0, 0): {:?}", pixel);

    // Create a cropped image
    let cropped_img = img.crop(0, 0, 100, 100);
    println!("Cropped image dimensions: {:?}", cropped_img.dimensions());

    // Create a thumbnail
    let thumbnail = img.thumbnail(100, 100);
    println!("Thumbnail dimensions: {:?}", thumbnail.dimensions());

    // Resize the image
    let resized_img = img.resize(200, 200, FilterType::Nearest);
    println!("Resized image dimensions: {:?}", resized_img.dimensions());

    // Convert the image to grayscale
    let grayscale_img = img.grayscale();
    println!("Grayscale image dimensions: {:?}", grayscale_img.dimensions());

    // Invert the colors of the image
    let mut img_copy = img.clone();
    img_copy.invert();
    println!("Inverted image dimensions: {:?}", img_copy.dimensions());
}
