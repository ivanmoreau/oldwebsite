#[macro_use] extern crate rocket;
use std::path::{Path, PathBuf};

use rocket_dyn_templates::Template;
use serde::{Serialize, Deserialize};
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use serde_json::Value;

const KEY: &str = "354c2b68ede5786a6b79083ba1";
const SITE: &str = "https://localhost:2368";
const SITE2: &str = "https://ivmoreau.com";

#[get("/static/<file..>")]
async fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/").join(file);
    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()))
}

#[derive(Serialize, Deserialize)]
struct Info {
    _0: Value,
    _1: Value,
    page: i32,
    npage: i32,
    ppage: i32
}

#[get("/?<page>")]
async fn index(page: Option<i32>) -> Template {
    let p = if page.is_none() { 1 } else { page.unwrap() };
    let request_url_body = format!("{}/ghost/api/v3/content/settings/?key={}", SITE, KEY);
    let request_url_post = format!("{}/ghost/api/v3/content/posts/?key={}&fields=slug,title,custom_excerpt&page={}", SITE, KEY, p);
    let body = reqwest::get(request_url_body)
    .await.unwrap()
    .text()
    .await.unwrap();
    println!("{}", body);
    let posts = reqwest::get(request_url_post)
    .await.unwrap()
    .text()
    .await.unwrap();

    // Parse the string of data into serde_json::Value.
    let v0: Value = serde_json::from_str(&body).unwrap();
    let v1: Value = serde_json::from_str(&posts).unwrap();

    let context = Info {
        _0: v0,
        _1: v1,
        page: p,
        npage: p + 1,
        ppage: p - 1,
    };
    Template::render("index", &context)
}

#[get("/post/<slug>")]
async fn post(slug: &str) -> Template {
    let request_url_body = format!("{}/ghost/api/v3/content/settings/?key={}", SITE, KEY);
    let request_url_post = format!("{}/ghost/api/v3/content/posts/slug/{}/?key={}", SITE, slug, KEY);
    let body = reqwest::get(request_url_body)
    .await.unwrap()
    .text()
    .await.unwrap();
    println!("{}", body);
    let posts = reqwest::get(request_url_post)
    .await.unwrap()
    .text()
    .await.unwrap();

    // Parse the string of data into serde_json::Value.
    let v0: Value = serde_json::from_str(&body).unwrap();
    let v1: Value = serde_json::from_str(&posts).unwrap();

    let context = Info {
        _0: v0,
        _1: v1,
        page: 0,
        npage: 0,
        ppage: 0,
    };
    Template::render("post", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Template::fairing())
    .mount("/", routes![index,files,post])
}
