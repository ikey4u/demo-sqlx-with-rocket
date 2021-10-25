use std::str::FromStr;

use tokio::time::{sleep, Duration};
use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
use rocket::form::{Form, FromForm};
use rocket::{self, Build, routes};

async fn insert() {
    let dboptions = SqliteConnectOptions::from_str("sqlite://test.db").unwrap().create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .connect_with(dboptions).await.unwrap();
    sqlx::query("CREATE TABLE IF NOT EXISTS demotbl(id INTEGER PRIMARY KEY, quantity INTEGER)")
        .execute(&pool).await.unwrap();
    let mut i = 0;
    loop {
        sqlx::query("INSERT OR IGNORE INTO demotbl VALUES ($1, $2)").bind(i).bind(i.to_string() + "_str")
            .execute(&pool).await.unwrap();
        i += 1;
        sleep(Duration::from_secs(5)).await;
    }
}

#[derive(FromForm)]
struct Task {
    name: String,
}

#[rocket::post("/todo", data = "<task>")]
fn new(task: Form<Task>) -> String {
    format!("you send me task {}", task.name)
}

pub async fn rocket() -> rocket::Rocket<Build> {
    rocket::build()
        .mount("/", routes![new])
}

#[tokio::main]
async fn main() {
    futures::join!(insert(), rocket());
    // futures::join!(insert());
}
