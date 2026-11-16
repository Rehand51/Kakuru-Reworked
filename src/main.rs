mod calculators;

use askama::Template;
use axum::{
    extract::{Form, Path},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use calculators::{build_page, navigation, HomeTemplate};
use std::{collections::HashMap, net::SocketAddr};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/:slug", get(calculator).post(calculate))
        .nest_service("/static", ServeDir::new("static"));

    let address = std::env::var("KAKURU_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_owned())
        .parse::<SocketAddr>()
        .expect("KAKURU_ADDRESS must be a valid socket address");

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind the KAKURU server");
    println!("KAKURU is listening on http://{address}");

    axum::serve(listener, app)
        .await
        .expect("KAKURU server failed");
}

async fn home() -> Response {
    render(HomeTemplate {
        nav: navigation(None),
    })
}

async fn calculator(Path(slug): Path<String>) -> Response {
    match build_page(&slug, None) {
        Some(Ok(page)) => render(page),
        Some(Err(message)) => (StatusCode::BAD_REQUEST, message).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn calculate(
    Path(slug): Path<String>,
    Form(form): Form<HashMap<String, String>>,
) -> Response {
    match build_page(&slug, Some(&form)) {
        Some(Ok(page)) => render(page),
        Some(Err(message)) => (StatusCode::BAD_REQUEST, message).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

fn render<T: Template>(template: T) -> Response {
    match template.render() {
        Ok(page) => Html(page).into_response(),
        Err(error) => {
            eprintln!("Failed to render page template: {error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
