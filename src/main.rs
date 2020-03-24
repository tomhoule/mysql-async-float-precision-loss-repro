use mysql_async::prelude::*;

#[tokio::main]
async fn main() {
    let conn = mysql_async::Conn::from_url(std::env::var("DB_URL").unwrap())
        .await
        .unwrap();

    let result = conn.prep_exec("SELECT ?", (6.4 as f32,)).await.unwrap();

    result
        .map_and_drop(|row| {
            assert_eq!(
                row.get(0),
                Some(mysql_async::Value::Float(6.400000095367432))
            )
        })
        .await
        .unwrap();
}
