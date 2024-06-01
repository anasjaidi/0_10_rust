use super::*;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Create {
    pub uid: String,
    pub name: String,
    pub description: String,
    pub image: String,
    pub manga_type: enums::MangaType,
    pub author: String,
    pub status: enums::MangaStatus,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Update {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub manga_type: Option<enums::MangaType>,
    pub author: Option<String>,
    pub status: Option<enums::MangaStatus>,
}
