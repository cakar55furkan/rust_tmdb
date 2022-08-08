use crate::movie_image;
use crate::movie_image::image_structs::ImageResponse;

pub(crate) async fn get_movie_images(movie_id: i32) ->  ImageResponse {
    //takes the movie id and send images api function
    // puts all the images into same array, with their types written.
    let mut movie_images_request_url = String::from("https://api.themoviedb.org/3/movie/");
    movie_images_request_url.push_str(&*movie_id.to_string());
    movie_images_request_url.push_str("/images?api_key=4e110fd06d91f1f01af5acd9fa42a82d");

    let movie_json: ImageResponse = reqwest::Client::new()
        .get(movie_images_request_url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    movie_json
}
