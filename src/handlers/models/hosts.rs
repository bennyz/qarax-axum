use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Host {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub port: i32,
    pub status: Status,
    pub host_user: String,

    #[serde(skip_deserializing)]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewHost {
    pub name: String,
    pub address: String,
    pub port: i32,
    pub host_user: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Type, EnumString, Display)]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "varchar")]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Unknown,
    Down,
    Installing,
    Initializing,
    Up,
}

#[derive(Error, Debug)]
pub enum HostError {
    #[error("Failed to list hosts")]
    DatabaseListError,
}

pub async fn list(pool: &PgPool) -> Result<Vec<Host>, HostError> {
    let hosts = sqlx::query_as!(
        Host,
        r#"
SELECT id, name, address, port, status as "status: _", host_user, password FROM hosts
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|_| HostError::DatabaseListError)?;

    Ok(hosts)
}
