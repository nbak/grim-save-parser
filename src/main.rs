#![feature(array_try_from_fn)]

use hyper::body::Buf;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use parser::Readable;
use serde::Serialize;
use std::io::Read;
use std::net::SocketAddr;

use crate::parser::{
    character::CharacterReader, formulas::FormulasReader, stash::StashFileReader, Reader,
};

mod parser;
mod util;

fn map_to_str<T, U>(reader: Box<&mut dyn Reader<Source = T, Item = U>>) -> util::Result<String>
where
    T: Read,
    U: Readable + Serialize,
{
    let obj = reader.read()?;
    match serde_json::to_string(&obj) {
        Ok(res) => Ok(res),
        Err(e) => Err(util::CustomError::new(e.to_string())),
    }
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let (parts, body) = req.into_parts();
    let whole_body = hyper::body::to_bytes(body).await?;
    println!("{}", whole_body.len());
    let result = match (parts.method, parts.uri.path()) {
        (Method::POST, "/character") => {
            map_to_str(Box::new(&mut CharacterReader::new(whole_body.reader())))
        }
        (Method::POST, "/stash") => {
            map_to_str(Box::new(&mut StashFileReader::new(whole_body.reader())))
        }
        (Method::POST, "/formulas") => {
            map_to_str(Box::new(&mut FormulasReader::new(whole_body.reader())))
        }
        _ => util::Result::Err(util::CustomError::new("no method".to_owned())),
    };

    match result {
        Ok(s) => Ok(Response::new(Body::from(s))),
        Err(e) => Ok(Response::builder()
            .status(404)
            .body(Body::from(e.to_string()))
            .unwrap()),
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_service = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
