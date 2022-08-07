use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct image {
    pub(crate) aspect_ratio: f32,
    pub(crate) name: String,
}

