#![feature(custom_derive,plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

extern crate dotenv;

extern crate rand;

extern crate rocket;
extern crate rocket_contrib;

extern crate r2d2;
extern crate r2d2_diesel;

extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use diesel::pg::PgConnection;

use dotenv::dotenv;

use r2d2::{Config,Pool};
use r2d2_diesel::{ConnectionManager};

use std::env;



mod database {
    use super::{Pool,ConnectionManager,PgConnection};

    pub type DBPool = Pool<ConnectionManager<PgConnection>>;    
}


mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}


mod models {
    use super::chrono::NaiveDateTime;
    use super::serde_json::Value;

    #[derive(Serialize)]
    pub struct APIRoot<'a> {
        title: &'a str,
        links: Value
    }

    impl<'a> APIRoot<'a> {
        pub fn new() -> APIRoot<'a> {
            APIRoot {
                title: "Rust Campaigns Server API v1",
                links: json!({
                    "campaigns": "/api/v1/campaigns"
                })
            }
        }
    }

    #[derive(Clone, Deserialize, Queryable, Serialize)]
    pub struct Campaign {
        id: i64,
        title: String,
        description: Option<String>,
        start_date: NaiveDateTime,
        end_date: Option<NaiveDateTime>,
        click_url: String
    }
}


mod handlers {
    use super::database::DBPool;
    use super::diesel::prelude::*;
    use super::diesel::expression::dsl::now;
    use super::models::{APIRoot,Campaign};
    use super::rand::{thread_rng,Rng};
    use super::rocket::State;
    use super::rocket_contrib::Json;
    use super::schema::campaigns::dsl::*;

    // API endpoints

    #[get("/campaigns")]
    fn get_campaigns(pool: State<DBPool>) -> Json<Vec<Campaign>> {
        let ref conn = *pool.clone().get().unwrap();

        let active_campaigns_qry = campaigns
            .filter(start_date.lt(now))
//            .filter(end_date.gt(now))
            .load::<Campaign>(conn);

        let limit = 5;

        match active_campaigns_qry {
            Ok(mut active_campaigns) => {
                let camps: Vec<Campaign> = {
                    // shuffle the campaigns
                    let mut rng = thread_rng();
                    rng.shuffle(&mut active_campaigns);

                    if active_campaigns.len() > limit {
                        // pick `limit` campaigns
                        (active_campaigns[..limit]).to_vec()
                    } else {
                        active_campaigns
                    }
                };

                Json(
                    camps
                )
            },
            Err(_) => Json(vec![])
        }
    }

    #[get("/")]
    fn api_root<'a>() -> Json<APIRoot<'a>> {
        Json(APIRoot::new())
    }
}


fn init_pool() -> database::DBPool {
    let url = env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set");

    let config = Config::default();

    let manager = ConnectionManager::<PgConnection>::new(url.as_str());

    Pool::new(config, manager)
        .expect("Could not create database connection pool")    
}

fn main() {
    dotenv().unwrap();

    let pool: database::DBPool = init_pool();

    rocket::ignite()
        .manage(pool)
        .mount("/api/v1/", routes![handlers::api_root,
                            handlers::get_campaigns])
        .launch();
}
