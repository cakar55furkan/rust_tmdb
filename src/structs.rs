use serde::Deserialize;
use std::cmp::Ordering;
use serde_json::Value::Bool;

#[derive(Debug, Deserialize)]
pub struct MovieMovieId {
    pub(crate) backdrop_path: String,
    pub(crate) adult: bool,
    pub(crate) budget: i64,
    //genres
    pub(crate) id: i32,
    pub(crate) imdb_id: String,
    pub(crate) original_language: String,
    pub(crate) original_title: String,
    pub(crate) overview: String,
    pub(crate) popularity: f32,
    pub(crate) poster_path:String,
    // prod_companies
    // prod_countries
    pub(crate) release_date:String,
    pub(crate) runtime:i16,
    //spoken_languages
    pub(crate) tagline: Option<String>,
    pub(crate) title: String,
    pub(crate) vote_average: Option<f32>,
    pub(crate) vote_count: Option<i32>
}

#[derive(Debug, Deserialize)]
pub struct SearchMovie {
    pub(crate) results: Vec<SearchMovieResultObject>,
}

#[derive(Debug, Deserialize)]
pub struct SearchMovieResultObject {
    pub(crate) id: i32,
    pub(crate) original_title: String,
    pub(crate) release_date: Option<String>,
    pub(crate) popularity: f32,
    pub(crate) vote_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct MovieGetImageImageObject{
    pub(crate) aspect_ratio:f32,
    pub(crate) width:i32,
    pub(crate) height:i32,
    pub(crate) vote_average:f32,
    pub(crate) file_path: String,
}

#[derive(Debug, Deserialize)]
pub struct MovieGetImage{
    pub(crate) id: i32,
    pub(crate) backdrops :Vec<MovieGetImageImageObject>,
    pub(crate) logos :Vec<MovieGetImageImageObject>,
    pub(crate) posters :Vec<MovieGetImageImageObject>,
}


