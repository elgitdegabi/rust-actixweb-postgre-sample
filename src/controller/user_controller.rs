use actix_web::{get, post, web::Json, web::Path, Responder};

use crate::config::constants::*;
use crate::model::health::Health;
use crate::model::user::UserUpsert;
use crate::service::user_service::*;

#[get("/health")]
pub async fn map_health() -> impl Responder {
    Json(Health {
        status: String::from(SERVER_RUNNING_STATUS),
    })
}

#[get("/users")]
pub async fn map_get_users() -> impl Responder {
    Json(get_users())
}

#[get("/user/{userid}")]
pub async fn map_get_user(user_id: Path<i64>) -> impl Responder {
    Json(get_user(*user_id))
}

#[post("/user/add")]
pub async fn map_add_user(user: Json<UserUpsert>) -> impl Responder {
    Json(add_user(user.0))
}

#[post("/user/update/{userid}")]
pub async fn map_update_user(user_id: Path<i64>, user: Json<UserUpsert>) -> impl Responder {
    Json(update_user(*user_id, user.0))
}

#[post("/user/delete/{userid}")]
pub async fn map_delete_user(user_id: Path<i64>) -> impl Responder {
    Json(delete_user(*user_id))
}

/**
 * Unit test cases
 */
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Scenario:
     * Executes config_endpoints with valid API server
     * Expectation:
     * A set of endpoints should be created
     */
    #[test]
    fn when_config_endpoints_with_api_server_should_add_endpoints() {
        // TODO to be implemented
        assert_eq!(true, true);
    }
}
