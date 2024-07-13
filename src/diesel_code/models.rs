use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::diesel_code::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
}
