mod database;

mod auth_utils;

pub use crate::auth_utils::models::Credentials;
use crate::database::Status;
pub fn authenticate_user(creds: Credentials){
    if let Status::Connected = crate::database::connect_to_database(){
        auth_utils::login(creds);
    }
}