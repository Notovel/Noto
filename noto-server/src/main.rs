extern crate rocket;

use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket::uri;
use rocket::{get, launch, routes};
use rocket_dyn_templates::{Template, context};
use std::io;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[get("/<name>")]
fn index(name: Option<&str>) -> Template {
    Template::render("editor", context! { user_name: name })
}

#[get("/")]
fn index_red() -> Redirect {
    Redirect::to(uri!("/login"))
}

#[get("/login")]
fn login() -> Template {
    Template::render("login", context! { server_name: "Noto" })
}

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

fn get_exec_path() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    Ok(dir)
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    let exec_dir = get_exec_path().expect("Server should have a path!");
    env::set_current_dir(exec_dir).expect("Could not change path to server directory");
    let wasm_pkg_dir = match env::var("VEL_WEB_DIR") {
        Ok(res) => res,
        Err(_) => "static".to_string(),
    };
    let wasm_pkg_dir = Path::new(wasm_pkg_dir.as_str());

    if !wasm_pkg_dir.exists() {
        panic!("❌ Did not find any folder for the static files!");
    }

    println!("🚀 Starting Rocket server...");
    println!("📦 WASM files served from: {}", wasm_pkg_dir.display());

    let css_dir = Path::new("sites/css");
    if !css_dir.exists() {
        panic!("❌ Did not find css folder!");
    }
    let js_dir = Path::new("sites/js");
    if !js_dir.exists() {
        panic!("❌ Did not find js folder!");
    }

    match env::var("ROCKET_CONFIG") {
        Ok(_) => {}
        Err(_) => {
            println!("Rocket not configured. Setting now!");
            let mut rocket_toml = PathBuf::from("Rocket.toml");
            rocket_toml = match rocket_toml.canonicalize() {
                Ok(res) => res,
                Err(e) => {
                    println!(
                        "An error occured while setting the config for Rocket: {}",
                        e
                    );
                    let config = get_exec_path().unwrap().join(rocket_toml);
                    if !config.exists() {
                        fs::create_dir_all(&config).unwrap();
                    }
                    config
                }
            };
            unsafe { env::set_var("ROCKET_CONFIG", rocket_toml.to_string_lossy().to_string()) };
        }
    };

    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, index_red, login, health])
        .mount("/res/css", FileServer::from(css_dir))
        .mount("/res/js", FileServer::from(js_dir))
        .mount("/res/wasm", FileServer::from(wasm_pkg_dir))
}
