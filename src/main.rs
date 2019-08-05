#![feature(proc_macro_hygiene, decl_macro)]

use myrias::{router, Config};
use rocket::routes;

fn main() {
    std::env::set_var("ROCKET_CLI_COLORS", "off");
    let config = Config::from_file("Config.toml");

    rocket::ignite()
        .manage(config)
        .mount(
            "/",
            routes![
                router::containers::index,
                router::cleanup::index,
                router::eval::index,
                router::create_container::index
            ],
        )
        .launch();
}
