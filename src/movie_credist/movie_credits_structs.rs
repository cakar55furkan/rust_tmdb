use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct cast_member {
    pub(crate) adult: bool,
    pub(crate) gender: i16,
    pub(crate) id: i32,
    pub(crate) known_for_department: String,
    pub(crate) name: String,
    pub(crate) original_name: String,
    pub(crate) popularity: f32,
    pub(crate) profile_path: Option<String>,
    pub(crate) cast_id: i32,
    pub(crate) character: Option<String>,
    pub(crate) credit_id: Option<String>,
    pub(crate) order: i32
}
#[derive(Debug, Deserialize)]
pub struct crew_member {
    pub(crate) adult: bool,
    pub(crate) gender: i16,
    pub(crate) id: i32,
    pub(crate) known_for_department: Option<String>,
    pub(crate) name: String,
    pub(crate) original_name: String,
    pub(crate) popularity: f32,
    pub(crate) profile_path: Option<String>,
    pub(crate) credit_id: Option<String>,
    pub(crate) department: Option<String>,
    pub(crate) job: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct movie_credits{
    pub(crate) id:i32,
    pub(crate) cast: Vec<cast_member>,
    pub(crate) crew: Vec<crew_member>
}