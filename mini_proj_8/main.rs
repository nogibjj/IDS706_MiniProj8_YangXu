use mini_proj_8::{connect_db, create_tables_if_not_exists, clear_table, insert_data_from_csv, complex_query};

#[tokio::main]
async fn main() {
    let pool = connect_db();
    create_tables_if_not_exists(&pool).await;
    clear_table(&pool).await;
    insert_data_from_csv(&pool, "dataset_sample.csv").await;
    complex_query(&pool).await;
}
