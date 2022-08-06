use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct cast {
    pub(crate) adult: Option<bool>,
    pub(crate) also_known_as: Option<Vec<String>>, // not in DB
    pub(crate) biography: Option<String>,
    pub(crate) birthday:Option<String>,
    pub(crate) deathday:Option<String>,
    pub(crate) gender:  Option<i16>,
    pub(crate) homepage: Option<String>,
    pub(crate) id: Option<i32>,
    pub(crate) imdb_id: Option<String>,
    pub(crate) known_for_department: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) place_of_birth: Option<String>,
    pub(crate) popularity: Option<f32>,
    pub(crate) profile_path: Option<String>
}