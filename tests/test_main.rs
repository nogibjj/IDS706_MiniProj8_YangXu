use mini_proj_8::{clear_table, connect_db, create_tables_if_not_exists, insert_data_from_csv}; // Import necessary functions
use mysql_async::prelude::Queryable;

#[tokio::test]
async fn test_table_creation() {
    let pool = connect_db();
    create_tables_if_not_exists(&pool).await;

    let mut conn = pool.get_conn().await.unwrap();
    let result_mini: Vec<String> = conn.query("SHOW TABLES LIKE 'week6_mini';").await.unwrap();
    assert!(!result_mini.is_empty(), "Table week6_mini not found");

    let result_discounts: Vec<String> = conn
        .query("SHOW TABLES LIKE 'week6_mini_discounts';")
        .await
        .unwrap();
    assert!(
        !result_discounts.is_empty(),
        "Table week6_mini_discounts not found"
    );
}

#[tokio::test]
async fn test_data_insertion() {
    let pool = connect_db();
    clear_table(&pool).await;
    insert_data_from_csv(&pool, "dataset_sample.csv").await;

    let mut conn = pool.get_conn().await.unwrap();
    let count: (i64,) = conn
        .query_first("SELECT COUNT(*) FROM week6_mini;")
        .await
        .unwrap()
        .expect("Expected a result from COUNT query");
    assert_eq!(count.0, 9, "Data insertion from CSV failed");
}

#[tokio::test]
async fn test_complex_query() {
    let pool = connect_db();
    let mut conn = pool.get_conn().await.unwrap();
    let results: Vec<(String, String, f64, i32, Option<f64>, f64)> = conn
        .query(
            r"SELECT w.Date, w.Product, w.Price, w.Quantity, d.Discount,
               (w.Price * w.Quantity) * IFNULL(1 - d.Discount, 1) AS Total_Revenue
        FROM week6_mini w
        LEFT JOIN week6_mini_discounts d ON w.Product = d.Product
        ORDER BY Total_Revenue DESC",
        )
        .await
        .unwrap();

    assert!(!results.is_empty(), "No results from the complex query");
}
