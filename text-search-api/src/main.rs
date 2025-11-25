#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};

#[get("/home")]
fn hello() -> &'static str {
    "Hello from Rocket API"
}

fn rocket() -> Rocket<Build> {
    // izinkan origin dari dev server Vue
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:5173",
        "http://127.0.0.1:5173",
    ]);

    let cors = CorsOptions {
        allowed_origins,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS");

    rocket::build()
        .mount("/api", routes![hello])
        .attach(cors)
}

#[launch]
fn launch() -> _ {
    rocket()
}
