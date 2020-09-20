use std::fmt::Debug;
use std::string::String;
use diesel::prelude::*;
use diesel::result;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

// Local
use crate::schema::*;
use crate::DbConn;

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub dob: Option<String>,
}

impl User {
    pub fn get_user(conn: DbConn, id: &str) -> result::QueryResult<User> {
        users::table.find(id).first::<User>(&*conn)
    }

    pub fn add_user(conn: DbConn, user: &User) -> result::QueryResult<usize> {
        diesel::insert_into(users::table)
            .values(user)
            .execute(&*conn)
    }
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "categories"]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: Option<String>
}

impl Category {
    pub fn all(conn: DbConn) -> result::QueryResult<Vec<Category>> {
        categories::table.load::<Category>(&*conn)
    }

    pub fn add<'a>(conn: DbConn, name: &'a str, description: Option<&'a str>) -> result::QueryResult<usize> {
        let value = (categories::name.eq(name.to_owned()), categories::description.eq(description.map(|v| v.to_owned())));

        diesel::insert_into(categories::table).values(value).execute(&*conn)
    }
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "statuses"]
pub struct Status {
    pub id: String,
    pub name: String,
    pub description: Option<String>
}

impl Status {
    pub fn all(conn: DbConn) -> result::QueryResult<Vec<Status>> {
        statuses::table.load::<Status>(&*conn)
    }

    pub fn add<'a>(conn: DbConn, name: &'a str, description: Option<&'a str>) -> result::QueryResult<usize> {
        let value = (statuses::name.eq(name.to_owned()), statuses::description.eq(description.map(|v| v.to_owned())));

        diesel::insert_into(statuses::table).values(value).execute(&*conn)
    }
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "activities"]
pub struct Activity {
    pub id: i32,
    pub user_id: String,
    pub name: String,
    pub short_description: Option<String>,
    pub start_date: i32,
    pub end_date: i32,
    pub category_id: String,
    pub status_id: String,
}

impl Activity {
    pub fn user_all<'a>(conn: DbConn, user_id: &'a str) -> result::QueryResult<Vec<Activity>> {
        activities::table.filter(activities::user_id.eq(user_id)).load::<Activity>(&*conn)
    }
}
