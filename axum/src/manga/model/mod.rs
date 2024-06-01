use self::dto::{Create, Update};

use super::*;

pub struct Model {
    pub uid: String,
    pub name: String,
    pub description: String,
    pub image: String,
    pub manga_type: enums::MangaType,
    pub author: String,
    pub status: enums::MangaStatus,
    pub chapters_count: u128,
}

impl Model {
    pub fn new(create: Create, chapters_count: u128) -> Self {
        Self {
            uid: create.uid,
            name: create.name,
            description: create.description,
            image: create.image,
            manga_type: create.manga_type,
            author: create.author,
            status: create.status,
            chapters_count,
        }
    }

    pub fn update(&mut self, update: Update) {
        if let Some(name) = update.name {
            self.name = name;
        }
        if let Some(description) = update.description {
            self.description = description;
        }
        if let Some(image) = update.image {
            self.image = image;
        }
        if let Some(manga_type) = update.manga_type {
            self.manga_type = manga_type;
        }
        if let Some(author) = update.author {
            self.author = author;
        }
        if let Some(status) = update.status {
            self.status = status;
        }
    }
}
