use jsonwebtoken::encode;
use crate::create_connection;

struct Claims {
    sub: String,
    company: String
}

pub fn login(email: String, password: String) -> String {
    let mut conn = create_connection().expect("Couldn't create connection");
    "hello".to_string()
}
