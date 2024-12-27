use chrono::NaiveDate;
use derive_builder::Builder;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
//#[serde(rename_all = "camelCase")] 不符合postgres还有我们api文档的命名规范
pub struct Book {
    pub id: i32,
    pub cover_url: String,
    pub title: String,
    pub author: Option<String>,
    pub rating: Option<f64>,
    pub description: Option<String>,
    pub added_date: NaiveDate,
    pub extension: String,
}

#[derive(Insertable, Builder)]
#[diesel(table_name = crate::schema::books)]
pub struct NewBook {
    pub title: String,
    pub author: Option<String>,
    pub description: Option<String>,
    #[builder(default = "chrono::Local::now().naive_local().date()")]
    pub added_date: NaiveDate,
    pub extension: String,
}
