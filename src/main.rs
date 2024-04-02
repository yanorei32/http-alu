pub mod alus;

use axum::{
    http::{header::CACHE_CONTROL, method::Method, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::any,
    Router,
};

use tower_http::cors::{Any, CorsLayer};

fn enc_u16(v: u16) -> String {
    format!("{v:016b}")
}

fn dec_u16(s: &[u8]) -> Result<u16, ()> {
    let mut n = 0;

    for b in s {
        n <<= 1;
        match b {
            b'0' => {}
            b'1' => n |= 1,
            _ => return Err(()),
        }
    }

    Ok(n)
}

async fn nandgame(headers: HeaderMap, method: Method) -> impl IntoResponse {
    let Ok(i) = dec_u16(method.as_str().as_bytes()) else {
        return (
            StatusCode::BAD_REQUEST,
            HeaderMap::new(),
            "Failed to read Instruction".to_string(),
        );
    };

    let Some(Ok(a)) = headers.get("x-a").map(|v| dec_u16(v.as_bytes())) else {
        return (
            StatusCode::BAD_REQUEST,
            HeaderMap::new(),
            "Failed to read A Register".to_string(),
        );
    };

    let Some(Ok(d)) = headers.get("x-d").map(|v| dec_u16(v.as_bytes())) else {
        return (
            StatusCode::BAD_REQUEST,
            HeaderMap::new(),
            "Failed to read D Register".to_string(),
        );
    };

    let Some(Ok(star_a)) = headers.get("x-*a").map(|v| dec_u16(v.as_bytes())) else {
        return (
            StatusCode::BAD_REQUEST,
            HeaderMap::new(),
            "Failed to read *A Register".to_string(),
        );
    };

    use alus::nandgame::*;
    let i = Instruction::from_bits_truncate(i);

    let input = Input { i, a, d, star_a };
    let output = alus::nandgame::alu(&input);

    println!("{input:?} {output:?}");

    let mut headers = HeaderMap::new();

    headers.insert(
        CACHE_CONTROL,
        HeaderValue::from_str("public, max-age=31536000, s-maxage=31536000, immutable").unwrap(),
    );

    headers.insert(
        "x-j",
        HeaderValue::from_str(&output.jump.to_string()).unwrap(),
    );

    return (StatusCode::OK, headers, enc_u16(output.r));
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/nandgame", any(nandgame)).layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_private_network(true),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
