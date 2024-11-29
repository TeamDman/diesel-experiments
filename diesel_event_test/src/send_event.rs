use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url).expect("Error connecting to database");

    let event_name = "test_notifications";
    let payload = r#"{"key": "value"}"#;

    // Construct the query with event name and payload
    let notify_query = format!("NOTIFY {}, '{}'", event_name, payload);
    diesel::sql_query(notify_query)
        .execute(&mut connection)
        .expect("Failed to send event");

    println!("Event sent: {}", event_name);
}
