use crate::structs::MovieGetImage;

pub async fn download_all_images_by_id(movie_id:&str) -> MovieGetImage {
    let mut url = String::from("https://api.themoviedb.org/3/movie/");
    url.push_str(movie_id);
    url.push_str("/images?api_key=4e110fd06d91f1f01af5acd9fa42a82d");

    let movie_get_images: MovieGetImage = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    return movie_get_images
}