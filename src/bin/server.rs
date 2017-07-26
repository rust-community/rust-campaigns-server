#![feature(custom_derive,plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rust_campaigns_server;
extern crate dotenv;

use dotenv::dotenv;

use rust_campaigns_server::{handlers,database};
use rust_campaigns_server::init_pool;

fn main() {
    dotenv().ok();

    let pool: database::DBPool = init_pool();

    rocket::ignite()
        .manage(pool)
        .attach(rocket_contrib::Template::fairing())
        .mount("/api/v1/", routes![handlers::api_root,
                                   handlers::get_campaigns,
                                   handlers::get_campaigns_with_pars])
        .mount("", routes![handlers::campaigns_script,
                           handlers::campaigns_script_with_pars])
        .launch();
}
