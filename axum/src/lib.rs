use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing, Json, Router};
use serde::{Deserialize, Serialize};
use std::{error::Error, net::SocketAddr};

pub mod manga {
    use super::*;

    mod dto {
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

        #[derive(Default, Debug, Serialize, Deserialize)]
        pub struct Create {
            pub uid: String,
            pub name: String,
            pub description: String,
            pub image: String,
            pub manga_type: MangaType,
            pub author: String,
            pub status: MangaStatus,
        }

        #[derive(Default, Debug, Serialize, Deserialize)]
        pub struct Update {
            pub name: Option<String>,
            pub description: Option<String>,
            pub image: Option<String>,
            pub manga_type: Option<MangaType>,
            pub author: Option<String>,
            pub status: Option<MangaStatus>,
        }
    }

    use dto::{Create, MangaStatus, MangaType, Update};

    pub struct Model {
        pub uid: String,
        pub name: String,
        pub description: String,
        pub image: String,
        pub manga_type: MangaType,
        pub author: String,
        pub status: MangaStatus,
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

    pub struct Controller;
    impl Controller {
        pub async fn create(Json(dto): Json<Create>) -> impl IntoResponse {
            Json(dto)
        }

        pub async fn get_all() -> impl IntoResponse {
            // Placeholder: Return an empty list for now
        }

        pub async fn get_one(Path(uid): Path<String>) -> impl IntoResponse {
            uid
        }

        pub async fn update(Path(uid): Path<String>, Json(dto): Json<Update>) -> impl IntoResponse {
            // Placeholder: Update logic not implemented yet
            Json(dto)
        }

        pub async fn delete_one(Path(uid): Path<String>) -> impl IntoResponse {
            (StatusCode::NO_CONTENT, Json(uid))
        }

        pub async fn delete_all() -> impl IntoResponse {
            // Placeholder: Delete all logic not implemented yet
            StatusCode::NO_CONTENT
        }
    }
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let router = Router::new()
        .route(
            "/manga",
            routing::post(manga::Controller::create)
                .get(manga::Controller::get_all)
                .delete(manga::Controller::delete_all),
        )
        .route(
            "/manga/:id",
            routing::get(manga::Controller::get_one)
                .delete(manga::Controller::delete_one)
                .patch(manga::Controller::update),
        );

    let address = "0.0.0.0:3000".parse::<SocketAddr>()?;

    println!("Listening on {}", address);
    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}
