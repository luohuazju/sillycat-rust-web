use sqlx::PgPool;
use uuid::Uuid;
use crate::models::seller::Seller;

pub struct SellerDAO;

impl SellerDAO {
    /// Create a new seller in the database
    pub async fn create_seller(pool: &PgPool, name: String, company_name: String) -> Result<Seller, sqlx::Error> {
        sqlx::query_as!(
            Seller,
            r#"
            INSERT INTO sellers (name, company_name)
            VALUES ($1, $2)
            RETURNING id::UUID, name, company_name
            "#,
            name,
            company_name
        )
        .fetch_one(pool)
        .await
    }

    /// Retrieve all sellers from the database
    pub async fn list_sellers(pool: &PgPool) -> Result<Vec<Seller>, sqlx::Error> {
        sqlx::query_as!(
            Seller,
            r#"
            SELECT id::UUID, name, company_name
            FROM sellers
            "#,
        )
        .fetch_all(pool)
        .await
    }

    /// Retrieve a single seller by ID
    pub async fn get_seller(pool: &PgPool, id: Uuid) -> Result<Seller, sqlx::Error> {
        sqlx::query_as!(
            Seller,
            r#"
            SELECT id::UUID, name, company_name
            FROM sellers
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }

    /// Update an existing seller's details
    pub async fn update_seller(pool: &PgPool, id: Uuid, name: String, company_name: String) -> Result<Seller, sqlx::Error> {
        sqlx::query_as!(
            Seller,
            r#"
            UPDATE sellers
            SET name = $1, company_name = $2
            WHERE id = $3
            RETURNING id::UUID, name, company_name
            "#,
            name,
            company_name,
            id
        )
        .fetch_one(pool)
        .await
    }

    /// Delete a seller from the database by ID
    pub async fn delete_seller(pool: &PgPool, id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM sellers
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }
}