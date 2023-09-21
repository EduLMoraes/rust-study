#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::error::LaunchError;
use rocket_contrib::serve::StaticFiles;
//use tera::Template;

#[path = "./modules/controller/router.rs"]
mod router;

fn rocket() -> LaunchError{
    rocket::ignite()
    .mount("/", router::routes())
    .mount("/style", StaticFiles::from("src/modules/templates/style"))
    .mount("/script", StaticFiles::from("src/modules/templates/script"))
    .launch()
}

fn main() {
    rocket();
}