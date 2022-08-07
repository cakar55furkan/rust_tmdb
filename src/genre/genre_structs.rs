use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct genre {
    pub(crate) id: i32,
    pub(crate) name: String,
}

