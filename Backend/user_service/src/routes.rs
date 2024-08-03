use rocket::Route;

/**
 * Récupérer les routes de l'API
 * @return les routes de l'API
 * @example
    * routes![
    *     create_user,
    *     get_users,
    *     get_users_by_id,
    *     update_user,
    *     delete_user
    * ]
 */
pub fn routes() -> Vec<Route> {
    routes![
        crate::handlers::get_users,
        crate::handlers::get_users_by_id,
        crate::handlers::update_user,
        crate::handlers::delete_user
    ]
}
