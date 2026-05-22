// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Contributors to Eclipse OpenSOVD (see CONTRIBUTORS)
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache License Version 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0

use aide::axum::ApiRouter;
use axum::{
    body::Body,
    extract::Path,
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use cda_sovd::dynamic_router::DynamicRouter;
use rust_embed::RustEmbed;

/// The compiled `cda-ui/dist/` tree, embedded at compile time.
#[derive(RustEmbed, Clone)]
#[folder = "../cda-ui/dist"]
struct UiAssets;

/// Looks up an embedded asset by path, falling back to a case-insensitive
/// search. The fallback is needed because the CDA request middleware lowercases
/// all URI paths, while Vite's content-hashed filenames contain uppercase chars.
fn get_asset(path: &str) -> Option<rust_embed::EmbeddedFile> {
    if let Some(file) = UiAssets::get(path) {
        return Some(file);
    }
    let lower = path.to_lowercase();
    UiAssets::iter()
        .find(|name| name.to_lowercase() == lower)
        .and_then(|name| UiAssets::get(&name))
}

/// Adds the `/ui` and `/ui/*path` routes to the dynamic router.
///
/// All requests are served from the embedded `cda-ui/dist/` assets.
/// Unknown paths fall back to `index.html` so the Vue SPA can handle
/// client-side routing.
pub async fn add_ui_routes(dynamic_router: &DynamicRouter) {
    let router = ApiRouter::new()
        .route("/ui", axum::routing::get(serve_index))
        .route("/ui/{*path}", axum::routing::get(serve_path));
    dynamic_router.merge_routes(router).await;
}

async fn serve_index() -> impl IntoResponse {
    serve_asset("index.html")
}

async fn serve_path(Path(path): Path<String>) -> impl IntoResponse {
    // Serve the exact asset; fall back to index.html for SPA navigation routes.
    if get_asset(&path).is_some() {
        serve_asset(&path)
    } else {
        serve_asset("index.html")
    }
}

fn serve_asset(path: &str) -> impl IntoResponse + use<> {
    match get_asset(path) {
        Some(file) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            match Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(Body::from(file.data.into_owned()))
            {
                Ok(resp) => resp.into_response(),
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
