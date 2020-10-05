use mysql::*;
use serde::{Deserialize, Serialize};
use crate::create_connection;
use mysql::prelude::Queryable;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub firebase_id: String,
}

impl Account {
    pub fn new(email: String, firebase_id: String) -> Account {
        let mut conn = create_connection().unwrap();
        let insert = conn.query_drop(format!("INSERT INTO account (email, firebase_id) VALUES ({}, {})", email, firebase_id));
        if let Ok(insertion) = insert {
            if let Ok(query) = conn.query_first(format!("SELECT id, email, firebase_id FROM account WHERE email = {} AND firebase_id = {}", email, firebase_id)) {
                query.map(| account: Option<Account> | {
                    Account {
                        id,
                        email,
                        firebase_id,
                    }
                }).unwrap()
            } else {
                false
            }
        };
    }
}
