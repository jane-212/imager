use sqlx::{
    Pool,
    MySql,
    FromRow,
    mysql::MySqlRow, Row,
};

use crate::utils::error::{
    IError,
    IResult,
};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Image {
    url: String,
}

impl<'a> FromRow<'a, MySqlRow> for Image {
    fn from_row(row: &'a MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            url: row.try_get("url")?
        })
    }
}

pub async fn get_all_images(pool: &Pool<MySql>) -> IResult<Vec<Image>> {
    let sql = "select url from image";

    let images: Vec<Image> = sqlx::query_as::<_, Image>(sql)
        .fetch_all(pool)
        .await
        .map_err(|_| IError::Unknown)?;

    Ok(images)
}
