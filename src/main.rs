#![feature(proc_macro_hygiene, decl_macro)]

use myrias::{router, Config};
use rocket::{catchers, routes};

fn main() {
    std::env::set_var("ROCKET_CLI_COLORS", "off");
    let config = Config::from_file("Config.toml");

    rocket::ignite()
        .manage(config)
        .register(catchers![
            router::not_found::index,
            router::gateway_timeout::index,
            router::internal_server_error::index
        ])
        .mount(
            "/",
            routes![
                router::languages::index,
                router::create_container::index,
                router::eval::index,
                router::containers::index,
                router::cleanup::index,
            ],
        )
        .launch();
}
