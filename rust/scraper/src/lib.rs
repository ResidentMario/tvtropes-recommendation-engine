// TODO: https://diesel.rs/guides/getting-started/

// This #[macro_use] call is apparently required to get the diesel macros that schema.rs relies
// on into scope. I will get to understanding macros one day, I promise!
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;
use diesel::sqlite::SqliteConnection;
// This prelude import is required b/c it's where diesel keeps its traits. Remember, in order to
// use a method on a trait associated with a struct, that trait must be in scope in the calling
// file. In this case, SqliteConnection is a struct, and a Connection trait granting an establish
// method is a trait on that struct. I suspect this is a common way of getting traits into scope;
// when I was working on JWT parser, I found myself having to import the traits by hand, despite
// not expecting an end user to know anything about them! Yes, this is much cleaner.
use diesel::prelude::*;

pub fn init_db() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
}