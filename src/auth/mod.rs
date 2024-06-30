extern crate bcrypt;
extern crate diesel;

use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::PgConnection;

use crate::db::{
    models::{NewUser, User},
    schema::users::dsl::*,
};

pub fn create_user<'a>(
    conn: &PgConnection,
    user_name: &'a str,
    e_mail: &'a str,
    password: &'a str,
) -> Result<usize, diesel::result::Error> {
    let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");

    let new_user = NewUser {
        username: user_name,
        email: e_mail,
        password_hash: &hashed_password,
    };

    diesel::insert_into(users).values(new_user).execute(conn)
}

pub fn authenticate_user(
    conn: &PgConnection,
    input_username: &str,
    input_password: &str,
) -> Result<Option<User>, diesel::result::Error> {
    let user_result = users
        .filter(username.eq(input_username))
        .first::<User>(conn)
        .optional()?;

    if let Some(user) = user_result {
        if verify(input_password, &user.password_hash).unwrap_or(false) {
            return Ok(Some(user));
        }
    }

    Ok(None)
}
