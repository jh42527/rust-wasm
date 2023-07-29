use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode, Server};
use hyper::header::*;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::result::Result;
use std::io::Cursor;

use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;

async fn grayscale(req: Request<Body>) -> Result<Response<Body>, anyhow::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /grayscale such as: `curl http://localhost:9005/grayscale -X POST --data-binary '@flower.png'`",
        ))),

        (&Method::POST, "/grayscale") => {
            println!("process grayscale ...");

            let image_data = hyper::body::to_bytes(req.into_body()).await?;
            
            let detected = image::guess_format(&image_data);
            if detected.is_err() {
                return Ok(Response::new(Body::from("Unknown image format")));
            }
            
            let image_format_detected = detected.unwrap();

            let mut img = image::load_from_memory(&image_data).unwrap();

            img = img.grayscale();

            let mut buf: Vec<u8> = Vec::new();

            img.write_to(&mut Cursor::new(&mut buf), Png).unwrap();

            let encoded_img = encode(&buf);

            let response = Response::builder()
                .header("Content-Type", "image/png")
                .body(Body::from(encoded_img))
                .unwrap();
            
            Ok(response)
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 9005));
    let make_svc = make_service_fn(|_| {
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                grayscale(req)
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}