use mysql_async::prelude::*;

#[tokio::main]
async fn main() {
    let conn = mysql_async::Conn::from_url(std::env::var("DB_URL").unwrap())
        .await
        .unwrap();

    let conn = conn
        .drop_exec(
            "CREATE TABLE IF NOT EXISTS float_test (id INT AUTO_INCREMENT PRIMARY KEY, num float4)",
            (),
        )
        .await
        .unwrap();

    let value: f64 = 6.4;
    let conn = conn
        .drop_exec("INSERT INTO float_test (num) VALUES (?)", (value,))
        .await
        .unwrap();

    let result = conn
        .prep_exec("SELECT num FROM float_test", ())
        .await
        .unwrap();

    result
        .for_each_and_drop(|row| {
            dbg!("new row");
            assert_eq!(
                row.get(0),
                Some(mysql_async::Value::Float(6.400000095367432))
            );
            dbg!("it's a match");
        })
        .await
        .unwrap();
}
