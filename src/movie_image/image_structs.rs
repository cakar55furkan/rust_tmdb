use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct movie_image {
    pub(crate) aspect_ratio: f32,
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) file_path:String,
    pub(crate) vote_count: i32,
    pub(crate) vote_average: f32,
    pub(crate) iso_639_1: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImageResponse {
    pub(crate) id: i32,
    pub(crate) backdrops:Vec<movie_image>,
    pub(crate) logos: Vec<movie_image>,
    pub(crate) posters: Vec<movie_image>
}
