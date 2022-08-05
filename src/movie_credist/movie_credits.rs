use crate::movie_credist::movie_credits_structs::movie_credits;

pub(crate) async fn get_movie_credits(movie_id: String) ->  movie_credits{
    ///   https://api.themoviedb.org/3/movie/1930/credits?api_key=4e110fd06d91f1f01af5acd9fa42a82d&language=en-US
    let mut movie_url = String::from("https://api.themoviedb.org/3/movie/");
    movie_url.push_str(&*movie_id);
    movie_url.push_str("/credits?api_key=4e110fd06d91f1f01af5acd9fa42a82d&language=en-US");

    let search_results: movie_credits = reqwest::Client::new()
        .get(movie_url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    search_results
}
