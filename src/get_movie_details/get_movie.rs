use crate::get_movie_structs::MovieMovieId;

pub(crate) async fn movie_movie_id(search_query: String) -> MovieMovieId {
    let mut movie_url = String::from("https://api.themoviedb.org/3/search/movie?api_key=4e110fd06d91f1f01af5acd9fa42a82d&language=en-US&query=");
    movie_url.push_str(&*search_query.replace(" ", "%20"));
    movie_url.push_str("&page=1&include_adult=false");
    let search_results: SearchMovie = reqwest::Client::new()
        .get(movie_url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    search_results
}
