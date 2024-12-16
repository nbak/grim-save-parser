use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::body::{Buf, Bytes};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use parser::{
    parser::{CharacterReader, FormulasReader, Reader, StashFileReader},
    util::{self, map_to_json},
};

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

async fn handle(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let result = match (req.method(), req.uri().path()) {
        (&Method::POST, "/character") => {
            let whole_body = req.collect().await?.to_bytes();
            map_to_json(Box::new(&mut CharacterReader::new(whole_body.reader())))
        }
        (&Method::POST, "/stash") => {
            let whole_body = req.collect().await?.to_bytes();
            map_to_json(Box::new(&mut StashFileReader::new(whole_body.reader())))
        }
        (&Method::POST, "/formulas") => {
            let whole_body = req.collect().await?.to_bytes();
            map_to_json(Box::new(&mut FormulasReader::new(whole_body.reader())))
        }
        _ => util::Result::Err(util::CustomError::new("no method".to_owned())),
    };

    match result {
        Ok(s) => Ok(Response::new(full(s))),
        Err(e) => Ok(Response::builder()
            .status(404)
            .body(full(e.to_string()))
            .unwrap()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (tcp, _) = listener.accept().await?;
        let io = TokioIo::new(tcp);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
