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

use rand::Rng;

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
    let sql = "select count(*) from image";

    let row = sqlx::query(sql)
        .fetch_one(pool)
        .await
        .map_err(|_| IError::Database)?;

    let total: i32 = row.get(0);

    let mut rng = rand::thread_rng();

    let begin = rng.gen_range(0..total);

    let sql = "select url from image where id > ? limit 10";

    let images: Vec<Image> = sqlx::query_as::<_, Image>(sql)
        .bind(begin)
        .fetch_all(pool)
        .await
        .map_err(|_| IError::Database)?;

    Ok(images)
}
