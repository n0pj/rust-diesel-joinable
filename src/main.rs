#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod schema;

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable, Debug)]
struct Post {
    id: i32,
    content: String,
    user_id: i32,
}

#[derive(Queryable, Debug)]
struct User {
    id: i32,
    name: String,
}

fn main() {
    dotenv().ok();
    // let url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    // let builder = mysql::OptsBuilder::from_opts(mysql::Opts::from_url(&url).unwrap());
    // let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default())).unwrap();
    // let _conn = pool.get_conn().unwrap();

    // diesel の場合
    // establish_connection();

    use schema::{post, user};

    let query = post::table
        .inner_join(user::table)
        .select((post::all_columns, user::all_columns));

    let query = query.order(user::id.asc());

    let result = query.load::<(Post, User)>(&establish_connection());

    match result {
        Ok(posts) => {
            for post in posts {
                println!("{:?}", post);
            }
        }
        Err(err) => println!("Error: {:?}", err),
    }

    println!("Successfully connected to PlanetScale!");
}
