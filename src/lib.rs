#![feature(custom_derive,plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

extern crate rand;

extern crate rocket;
extern crate rocket_contrib;

extern crate r2d2;
extern crate r2d2_diesel;

extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use diesel::pg::PgConnection;

use r2d2::{Config,Pool};
use r2d2_diesel::{ConnectionManager};

use std::env;

pub mod database {
    use super::{Pool,ConnectionManager,PgConnection};

    pub type DBConnection = PgConnection;
    pub type DBPool = Pool<ConnectionManager<DBConnection>>;
}

pub mod schema {
    table! {
        campaigns (id) {
            id -> Int8,
            title -> Varchar,
            description -> Nullable<Varchar>,
            start_date -> Timestamp,
            end_date -> Nullable<Timestamp>,
            click_url -> Varchar,
        }
    }
}

pub mod api {
    use serde_json::Value;

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
}

pub mod models {
    use super::chrono::NaiveDateTime;
    use super::schema::campaigns;

    #[derive(Clone, Deserialize, Queryable, Serialize)]
    pub struct Campaign {
        id: i64,
        title: String,
        description: Option<String>,
        start_date: NaiveDateTime,
        end_date: Option<NaiveDateTime>,
        click_url: String
    }

    #[derive(Insertable)]
    #[table_name="campaigns"]
    pub struct NewCampaign {
        title: String,
        description: Option<String>,
        start_date: NaiveDateTime,
        end_date: Option<NaiveDateTime>,
        click_url: String,
    }
}

pub mod queries {
    use super::database::DBConnection;
    use super::models::Campaign;
    use super::diesel::prelude::*;
    use super::diesel::expression::dsl::now;
    use super::rand::{thread_rng,Rng};
    use super::schema::campaigns::dsl::{campaigns,start_date as start};

    pub struct CampaignQueries;

    impl CampaignQueries {
        // returns a vector of `limit` random active campaigns
        pub fn random_set(conn: &DBConnection, limit: usize) -> Vec<Campaign> {
            let active_campaigns_qry = campaigns
                .filter(start.lt(now))
            //            .filter(end_date.gt(now))
                .load::<Campaign>(conn);

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

                    camps
                },
                Err(_) => vec![]
            }
        }
    }
}


pub mod handlers {

    use super::database::DBPool;
    use super::api::APIRoot;
    use super::models::Campaign;
    use super::queries::CampaignQueries;
    use super::rocket::State;
    use super::rocket_contrib::{Json,Template};

    #[derive(FromForm)]
    struct CampaignsParams {
        l: Option<usize>
    }

    const DEFAULT_LIMIT: usize = 5;

    // API endpoints

    fn get_limit(pars: CampaignsParams) -> usize {
        if let Some(l) = pars.l {
            l
        } else {
            DEFAULT_LIMIT
        }
    }

    #[get("/campaigns")]
    fn get_campaigns(pool: State<DBPool>) -> Json<Vec<Campaign>> {
        let ref conn = *pool.clone().get().unwrap();
        Json(CampaignQueries::random_set(&conn, DEFAULT_LIMIT))
    }

    #[get("/campaigns?<pars>", rank = 2)]
    fn get_campaigns_with_pars(pool: State<DBPool>, pars: CampaignsParams) -> Json<Vec<Campaign>> {
        let ref conn = *pool.clone().get().unwrap();
        Json(CampaignQueries::random_set(&conn, get_limit(pars)))
    }

    #[get("/")]
    fn api_root<'a>() -> Json<APIRoot<'a>> {
        Json(APIRoot::new())
    }

    #[get("/campaigns.js", format = "application/javascript")]
    fn campaigns_script(pool: State<DBPool>) -> Template {
        let ref conn = *pool.clone().get().unwrap();

        let context = json!({
            "campaigns": CampaignQueries::random_set(conn, DEFAULT_LIMIT)
        });
        Template::render("script", &context)
    }

    #[get("/campaigns.js?<pars>", format = "application/javascript")]
    fn campaigns_script_with_pars(pool: State<DBPool>, pars: CampaignsParams) -> Template {
        let ref conn = *pool.clone().get().unwrap();

        let context = json!({
            "campaigns": CampaignQueries::random_set(conn, get_limit(pars))
        });
        Template::render("script", &context)
    }
}


pub fn init_pool() -> database::DBPool {
    let url = env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set");

    let config = Config::default();

    let manager = ConnectionManager::<PgConnection>::new(url.as_str());

    Pool::new(config, manager)
        .expect("Could not create database connection pool")    
}
