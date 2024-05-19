// Example api/foo.rs
use vercel_runtime::{Body, Error, Request, Response, StatusCode, bundled_api};

#[bundled_api]
pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::Text("Route is /api/foo".into()))?)
}
