use crate::structs::MovieMovieId;
use image::{load_from_memory, DynamicImage, ImageDecoder, ImageError};

pub(crate) async fn download_image(image_location: &str, folder_path: &str, width: &str) {
    /// Takes a image id of TMDB and saves it to given location!
    // TODO: add width functionality!

    let pre_url = "https://image.tmdb.org/t/p/original/";
    let mut image_url = pre_url.to_owned();
    image_url.push_str(image_location);

    let img_bytes = reqwest::Client::new()
        .get(image_url)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    let backdrop = load_from_memory(&img_bytes).unwrap();

    match backdrop.save(String::from(folder_path) + image_location) {
        Ok(something) => something,
        Err(error) => println!("Error occured during saving image: {}", error),
    };
}
