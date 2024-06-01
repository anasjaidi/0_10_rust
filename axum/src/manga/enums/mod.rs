use super::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MangaType {
    Shonen,
    Seinen,
    Shojo,
    Josei,
    Kodomo,
    Other(String),
}

impl Default for MangaType {
    fn default() -> Self {
        MangaType::Other("Unknown".to_string())
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MangaStatus {
    #[default]
    Ongoing,
    Completed,
    Hiatus,
    Cancelled,
}
