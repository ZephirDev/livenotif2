#[macro_use] extern crate rocket;

use dotenvy::dotenv;
use std::env;
use rocket::response::content;
use rocket_sync_db_pools::{database, diesel};

#[database("livenotif_database")]
struct Database(diesel::PgConnection);

#[get("/")]
fn ping() -> content::RawJson<&'static str> {
    content::RawJson("{\
        \"ping\": \"pong\"\
    }")
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL");
    let rocket_databases = env::var("ROCKET_DATABASES");
    if !database_url.is_err() && rocket_databases.is_err() {
        println!("Test");
        let mut str = "{livenotif_database={url=\"".to_owned();
        str.push_str(database_url.expect("Undefined DATABASE_URL env var").as_str());
        str.push_str("\"}}");
        env::set_var("ROCKET_DATABASES", str)
    }

    rocket::build()
        .mount("/ping", routes![ping])
        .attach(Database::fairing())
}
