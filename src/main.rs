#[macro_use] extern crate rocket;

extern crate dotenv;
use dotenv::dotenv;
use rocket::{State, Route};
use rocket::fs::{FileServer, relative};
use rocket::http::ContentType;
use rocket::request::FromRequest;
use rocket::response::content::RawXml;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket::response::{Responder, Response, content};
use log::LevelFilter;
use log::{error, info};
use log4rs;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::{
    roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger,
};
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::Config;
use log4rs::encode::pattern::PatternEncoder;
use std::collections::HashMap;
use std::{env};
use reqwest::{Client, Url};
use serde_json::{from_str, Value};
use rocket::form::{Form};
use rocket::form::FromFormField;
use urlencoding::encode;

use chrono::Utc;
use chrono::format::strftime::StrftimeItems;









//routes



#[catch(404)]
async fn not_found() -> Template {
    


    Template::render("errors/404", context! {
  
    })
}


//routes


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let log_line_pattern = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {m}{n}";

    let trigger_size = byte_unit::n_mb_bytes!(30) as u64;
    let trigger = Box::new(SizeTrigger::new(trigger_size));

    let roller_pattern = "logs/arch/log_{}.gz";
    let roller_count = 5;
    let roller_base = 1;
    let roller = Box::new(
        FixedWindowRoller::builder()
            .base(roller_base)
            .build(roller_pattern, roller_count)
            .unwrap(),
    );

    let _compound_policy = Box::new(CompoundPolicy::new(trigger, roller));

    let step_ap = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_line_pattern)))
        .build("logs/log/log.log", _compound_policy)
        .unwrap();

    //let stdout = ConsoleAppender::builder().build();

    let config = Config::builder()
        //.appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("step_ap", Box::new(step_ap)))
        .logger(
            Logger::builder()
                .appender("step_ap")
                .build("step", LevelFilter::Trace),
        )

        .build(
            Root::builder()
                .appender("step_ap")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();


    dotenv().ok();
    info!("---------------ENV----------------");
    for (key, value) in env::vars() {
        info!("{key}: {value}");
    }
    info!("---------------ENV----------------");

    let _rocket = rocket::build()
        .mount("/", routes![])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .register("/", catchers![not_found])
        //.attach(Template::fairing(|_| {})) 
        .launch()
        .await?;


        info!("{:?}", rocket::routes!());

    Ok(())
}


