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
    #[error("Failed to list hosts: {0}")]
    FailedList(#[from] sqlx::Error),

    #[error("Failed to add host: {0}, error: {1}")]
    FailedAdd(String, sqlx::Error),
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
    .map_err(HostError::FailedList)?;

    Ok(hosts)
}

pub async fn add(pool: &PgPool, host: &NewHost) -> Result<Uuid, HostError> {
    let rec = sqlx::query!(
        r#"
INSERT INTO hosts (name, address, port, status, host_user, password)
VALUES ( $1, $2, $3, $4, $5, $6 )
RETURNING id
"#,
        host.name,
        host.address,
        host.port,
        Status::Down as Status,
        host.host_user,
        host.password
    )
    .fetch_one(pool)
    .await
    .map_err(|e| HostError::FailedAdd(host.name.to_owned(), e))?;

    Ok(rec.id)
}
