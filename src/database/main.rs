use std::env;
use mysql::Pool;
use mysql::prelude::Queryable;

fn main() {
    dotenv::dotenv().ok();
    create_connection();
}


pub fn create_connection() -> Result<(), mysql::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = Pool::new(database_url).expect("Could not create pool");
    let mut conn = pool.get_conn().expect("Could not connect");
    conn.query_drop(
        "CREATE TABLE account (
            id int not null,
            email VARCHAR(255) not null,
            firebase_id VARCHAR(255)
        )")?;
    Ok(())
}
