use crate::movie_movie_id::get_movie_structs::MovieMovieId;

pub(crate) async fn get_movie_details(movie_tmdb_id: String) -> MovieMovieId {
    let mut movie_url = String::from("https://api.themoviedb.org/3/movie/");
    movie_url.push_str(&*movie_tmdb_id.replace(" ", "%20"));
    movie_url.push_str("&?api_key=4e110fd06d91f1f01af5acd9fa42a82d&language=en-US");

    println!("!!!!!-----1!!!!!!{}", movie_url);
    let get_movie_result: MovieMovieId = reqwest::Client::new()
        .get(movie_url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    get_movie_result
}
