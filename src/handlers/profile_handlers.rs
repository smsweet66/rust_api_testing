use actix_web::web;
use diesel::{r2d2::{Pool, ConnectionManager}, SqliteConnection, prelude::*};

use crate::{models::profile_model::*, authentication, schema::profiles};

pub enum ProfileError {
    InvalidToken,
    DatabaseError,
    AlreadyExists
}

impl ProfileError
{
    pub fn message(&self) -> String {
        match self {
            ProfileError::InvalidToken => "Invalid token".to_owned(),
            ProfileError::DatabaseError => "Database error".to_owned(),
            ProfileError::AlreadyExists => "Profile already exists".to_owned()
        }
    }
}

type DB = web::Data<Pool<ConnectionManager<SqliteConnection>>>;

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

pub async fn create_profile_handler(pool: DB, item: web::Json<ProfileUpdate>, token: &str) -> Result<Profile, ProfileError> {
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
        Ok(_) => (),
        Err(_) => return Err(ProfileError::DatabaseError)
    }

    //get newly inserted profile
    let profile = profiles::table
        .filter(profiles::user_id.eq(user))
        .filter(profiles::name.eq(item.name.clone()))
        .first::<Profile>(&mut conn);
        

    match profile {
        Ok(profile) => Ok(profile),
        Err(_) => Err(ProfileError::DatabaseError)
    }
}