extern crate dotenv;
extern crate rust_campaigns_server;
extern crate clap;
extern crate chrono;
extern crate diesel;

use clap::{Arg, App};

use chrono::naive::NaiveDateTime;

use rust_campaigns_server::{init_pool};
use rust_campaigns_server::models::{Campaign,NewCampaign};
use rust_campaigns_server::schema::campaigns;

use diesel::LoadDsl;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let matches = App::new("Add Campaign")
                             .version("0.1.0")
                             .author("Florian Gilcher <florian.gilcher@asquera.de>")
                             .about("Adds a campaign to the campaign database")
                             .arg(Arg::with_name("title")
                                  .short("t")
                                  .long("title")
                                  .value_name("TITLE")
                                  .help("The title of the campaign")
                                  .takes_value(true))
                             .arg(Arg::with_name("description")
                                  .short("d")
                                  .long("description")
                                  .value_name("DESCRIPTION")
                                  .help("The description of the campaign")
                                  .takes_value(true))
                             .arg(Arg::with_name("click_url")
                                  .short("u")
                                  .long("click-url")
                                  .value_name("CLICK_URL")
                                  .help("The url of the campaign")
                                  .takes_value(true))
                             .arg(Arg::with_name("start_date")
                                  .short("s")
                                  .long("start-date")
                                  .value_name("START_DATE")
                                  .help("The start date of the campaign")
                                  .takes_value(true))
                             .arg(Arg::with_name("end_date")
                                  .short("e")
                                  .long("end-date")
                                  .value_name("END_DATE")
                                  .help("The end date of the campaign")
                                  .takes_value(true))
                             .get_matches();

  let campaign = NewCampaign {
    title: matches.value_of("title").expect("Title needs to be given").to_string(),
    description: matches.value_of("description").map(String::from),
    start_date: NaiveDateTime::parse_from_str(matches.value_of("start_date").expect("start_date must be given"), "%Y-%m-%d %H:%M:%S").unwrap(),
    end_date: matches.value_of("end_date").map(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap()),
    click_url: matches.value_of("click_url").expect("Click url must be given").to_string()
  };

  let pool = init_pool();

  let ref conn = *pool.clone().get().unwrap();

  let _: Campaign = diesel::insert(&campaign).into(campaigns::table)
      .get_result(conn)
      .expect("Error saving new Campaign");
}
