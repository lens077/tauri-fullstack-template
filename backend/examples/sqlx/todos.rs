//! SQLx CRUD example. Use DATABASE_URL with either SQLite or Postgres.
use sqlx::{FromRow, Row};

#[derive(Debug, FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

pub async fn create(pool: &sqlx::AnyPool, title: &str) -> sqlx::Result<Todo> {
    let row = sqlx::query("INSERT INTO todos (title) VALUES (?) RETURNING id, title, completed")
        .bind(title).fetch_one(pool).await?;
    Ok(Todo { id: row.try_get("id")?, title: row.try_get("title")?, completed: row.try_get("completed")? })
}

pub async fn list(pool: &sqlx::AnyPool) -> sqlx::Result<Vec<Todo>> {
    sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos ORDER BY created_at DESC")
        .fetch_all(pool).await
}

pub async fn update(pool: &sqlx::AnyPool, id: i64, completed: bool) -> sqlx::Result<u64> {
    Ok(sqlx::query("UPDATE todos SET completed = ? WHERE id = ?").bind(completed).bind(id).execute(pool).await?.rows_affected())
}

pub async fn delete(pool: &sqlx::AnyPool, id: i64) -> sqlx::Result<u64> {
    Ok(sqlx::query("DELETE FROM todos WHERE id = ?").bind(id).execute(pool).await?.rows_affected())
}
