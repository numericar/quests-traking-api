use chrono::NaiveDateTime;
use diesel::{prelude::{Identifiable, Insertable, Queryable}, Selectable};

use crate::infrastructure::postgres::schema::adventurers;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = adventurers)]
pub struct AdventurerEntity {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = adventurers)]
pub struct RegisterAdventurerEntity {
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}