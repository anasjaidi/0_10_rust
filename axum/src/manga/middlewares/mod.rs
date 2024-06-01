// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   mod.rs                                             :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: ajaidi <ajaidi@student.42.fr>              +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2024/06/01 13:56:58 by ajaidi            #+#    #+#             //
//   Updated: 2024/06/01 14:17:08 by ajaidi           ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use std::error::Error;

use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

pub async fn manga_exists<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, StatusCode> {
    let headers = req.headers();
    let header = headers
        .get("Content-Type")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .to_string();

    let extensions = req.extensions_mut();
    extensions.insert(header);
    Ok(next.run(req).await)
}
