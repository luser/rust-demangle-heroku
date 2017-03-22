#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate itertools;
extern crate rocket;
extern crate rustc_demangle;

use itertools::Itertools;
use rocket::request::Form;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).ok()
}

#[get("/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[derive(FromForm)]
struct Demangle {
    symbols: String,
    hash: Option<bool>,
}

#[post("/demangle", data = "<demangle>")]
fn api_demangle(demangle: Form<Demangle>) -> String {
    let d = demangle.get();
    d.symbols.lines()
        .map(rustc_demangle::demangle)
        .map(|s| if d.hash.unwrap_or(false) { format!("{}", s) } else { format!("{:#}", s) })
        .join("\n") + "\n"
}

fn main() {
    rocket::ignite()
        .mount("/api", routes![api_demangle])
        .mount("/", routes![index, static_files])
        .launch();
}
