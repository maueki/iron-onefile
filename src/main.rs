#![feature(plugin)]
#![plugin(include_dir)]

extern crate iron;
extern crate router;
extern crate mount;
extern crate conduit_mime_types as mime_types;
extern crate hyper;

#[macro_use] extern crate lazy_static;

use iron::prelude::*;
use iron::status;
use mount::Mount;
use std::collections::HashMap;
use std::path::Path;
use hyper::mime::Mime;

lazy_static! {
    static ref ASSETS: HashMap<String, Vec<u8>> = include_dir!("../assets");
    static ref MIME_TYPES: mime_types::Types = mime_types::Types::new().unwrap();
}

fn request_assets(req: &mut Request) -> IronResult<Response> {
    let path:String = req.url.path().pop().unwrap().to_string();
    match ASSETS.get(&path) {
        Some(b) => {
            let mut res = Response::with((status::Ok, b.as_slice()));

            let mime_str = MIME_TYPES.mime_for_path(Path::new(&path));
            let _ = mime_str.parse().map(|mime: Mime| res.set_mut(mime));
            Ok(res)
        },
        None => {
            Ok(Response::with((status::NotFound)))
        }
    }
}

fn main() {
    let mut mount = Mount::new();

    mount.mount("/assets/", request_assets);

    Iron::new(mount).http("localhost:3000").unwrap();
}
