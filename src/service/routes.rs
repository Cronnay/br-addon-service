use actix_web::{HttpRequest, Responder, HttpResponse};
use std::env;
use crate::create_connection;
use mysql::prelude::Queryable;
use crate::models::Account;

pub struct AppRoutes {}

impl AppRoutes {
    pub async fn index(_req: HttpRequest) -> impl Responder {
        let mut connection = create_connection().unwrap();
        let get_accounts: Vec<Account> = connection.query_map("SELECT id, email, firebase_id FROM account",
        |(id, email, firebase_id)| {
            Account { id, email, firebase_id }
        }).unwrap();
        HttpResponse::Ok().json(get_accounts)
    }
}
