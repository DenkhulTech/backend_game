// use crate::models::item::Item;
// use sqlx::PgPool;

// pub async fn get_all_items(pool: &PgPool) -> Result<Vec<Item>, sqlx::Error> {
//    let items = sqlx::query_as::<_, Item>("SELECT id, name, type FROM items")
//     .fetch_all(pool)
//     .await?;

//     Ok(items)
// }
