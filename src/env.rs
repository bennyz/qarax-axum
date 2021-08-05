use sqlx::postgres::PgPool;

#[derive(Clone, Debug)]
pub struct Environment {
    pool: PgPool,
}

impl Environment {
    pub async fn new(pool: PgPool) -> anyhow::Result<Self> {
        Ok(Self { pool })
    }

    pub fn db(&self) -> &PgPool {
        &self.pool
    }
}
