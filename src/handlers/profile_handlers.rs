use actix_web::web;
use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection, prelude::*};

use crate::{models::profile_model::*, authentication, schema::profiles};

pub enum ProfileError {
    InvalidToken,
    DatabaseError,
    AlreadyExists,
    NotFound
}

impl ProfileError
{
    pub fn message(&self) -> String {
        match self {
            ProfileError::InvalidToken => "Invalid token".to_owned(),
            ProfileError::DatabaseError => "Database error".to_owned(),
            ProfileError::AlreadyExists => "Profile already exists".to_owned(),
            ProfileError::NotFound => "Profile not found".to_owned()
        }
    }
}

type DB = web::Data<Pool<ConnectionManager<PgConnection>>>;

pub async fn get_profiles_handler(pool: DB, token: &str) -> Result<Vec<Profile>, ProfileError> {
    let mut conn = pool.get().unwrap();
    let user = authentication::validate_token(token);
    let user = match user {
        Ok(user) => user,
        Err(_) => return Err(ProfileError::InvalidToken)
    };

    let profiles = profiles::table
        .filter(profiles::user_id.eq(user))
        .load::<Profile>(&mut conn);

    match profiles {
        Ok(profiles) => Ok(profiles),
        Err(_) => Err(ProfileError::DatabaseError)
    }
}

pub async fn create_profile_handler(pool: DB, item: web::Json<ProfileUpdate>, token: &str) -> Result<(), ProfileError> {
    let mut conn = pool.get().unwrap();
    let user = authentication::validate_token(token);
    let user = match user {
        Ok(user) => user,
        Err(_) => return Err(ProfileError::InvalidToken)
    };

    let new_profile = ProfileNew {
        user_id: user,
        name: item.name.clone(),
        body_sizes: item.body_sizes.clone()
    };

    //check if profile already exists
    let profile = profiles::table
        .filter(profiles::user_id.eq(user))
        .filter(profiles::name.eq(item.name.clone()))
        .first::<Profile>(&mut conn);

    match profile {
        Ok(_) => return Err(ProfileError::AlreadyExists),
        Err(_) => ()
    }

    let result = diesel::insert_into(profiles::table)
        .values(&new_profile)
        .execute(&mut conn);

    match result {
        Ok(_) => Ok(()),
        Err(_) => return Err(ProfileError::DatabaseError)
    }
}

pub async fn update_profile_handler(pool: DB, item: web::Json<ProfileUpdate>, id: i32, token: &str) -> Result<(), ProfileError> {
    let mut conn = pool.get().unwrap();
    let user = authentication::validate_token(token);
    let user = match user {
        Ok(user) => user,
        Err(_) => return Err(ProfileError::InvalidToken)
    };

    //check if profile with name already exists and is not the same profile
    let profile = profiles::table
        .filter(profiles::user_id.eq(user))
        .filter(profiles::name.eq(item.name.clone()))
        .filter(profiles::id.ne(id))
        .first::<Profile>(&mut conn);

    match profile {
        Ok(_) => return Err(ProfileError::AlreadyExists),
        Err(diesel::result::Error::NotFound) => (),
        Err(_) => return Err(ProfileError::DatabaseError)
    }

    let result = diesel::update(profiles::table)
        .filter(profiles::id.eq(id))
        .filter(profiles::user_id.eq(user))
        .set((
            profiles::name.eq(item.name.clone()),
            profiles::body_sizes.eq(item.body_sizes.clone())
        ))
        .execute(&mut conn);

    match result {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::NotFound) => return Err(ProfileError::NotFound),
        Err(_) => return Err(ProfileError::DatabaseError)
    }
}

pub async fn delete_profile_handler(pool: DB, id: i32, token: &str) -> Result<(), ProfileError> {
    let mut conn = pool.get().unwrap();
    let user = authentication::validate_token(token);
    let user = match user {
        Ok(user) => user,
        Err(_) => return Err(ProfileError::InvalidToken)
    };

    let result = diesel::delete(profiles::table)
        .filter(profiles::id.eq(id))
        .filter(profiles::user_id.eq(user))
        .execute(&mut conn);

    match result {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::NotFound) => return Err(ProfileError::NotFound),
        Err(_) => return Err(ProfileError::DatabaseError)
    }
}