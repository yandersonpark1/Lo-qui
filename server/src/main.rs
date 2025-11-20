use std::error::Error;
use sqlx::Row;

struct Message {
    usernam: String,
    content: String,
    created_at: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = env::var("DATABASE_URL").unwrap();
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}
