// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   mod.rs                                             :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: ajaidi <ajaidi@student.42.fr>              +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2024/06/01 13:56:58 by ajaidi            #+#    #+#             //
//   Updated: 2024/06/01 14:03:24 by ajaidi           ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use axum::{http::Request, middleware::Next, response::IntoResponse};

pub async fn manga_exists<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    println!("{:?}", "wayli");
    next.run(req).await
}
