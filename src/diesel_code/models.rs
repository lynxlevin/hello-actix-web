use crate::diesel_code::schema::todos;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub description: &'a str,
}
