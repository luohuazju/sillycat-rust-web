use sqlx::PgPool;
use uuid::Uuid;
use crate::models::customer::Customer;

pub struct CustomerDAO;

impl CustomerDAO {
    pub async fn create_customer(pool: &PgPool, name: String, email: String) -> Result<Customer, sqlx::Error> {
        sqlx::query_as!(
            Customer,
            r#"
            INSERT INTO customers(name, email)
            VALUES ($1, $2)
            RETURNING id::UUID, name, email
            "#,
            name,
            email
        )
        .fetch_one(pool)
        .await
    }

    pub async fn list_customers(pool: &PgPool) -> Result<Vec<Customer>, sqlx::Error>{
        sqlx::query_as!(
            Customer,
            r#"
            SELECT id::UUID, name, email
            FROM customers
            "#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn get_customer(pool: &PgPool, id: Uuid) -> Result<Customer, sqlx::Error>{
        sqlx::query_as!(
            Customer,
            r#"
            SELECT id::UUID, name, email
            FROM customers
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update_customer(pool: &PgPool, id: Uuid, name: String, email: String) -> Result<Customer, sqlx::Error> {
        sqlx::query_as!(
            Customer,
            r#"
            UPDATE customers
            SET name = $1, email = $2
            WHERE id = $3
            RETURNING id::UUID, name, email
            "#,
            name,
            email,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete_customer(pool: &PgPool, id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM customers
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

}
