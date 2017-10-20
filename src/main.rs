#![feature(plugin)]
#![plugin(include_dir_bytes)]

extern crate iron;
extern crate mount;

#[macro_use] extern crate embed_staticfile;

use iron::prelude::*;
use mount::Mount;

fn main() {
    let mut mount = Mount::new();

    mount.mount("/assets/", embed_staticfile!("assets"));

    Iron::new(mount).http("localhost:3000").unwrap();
}
