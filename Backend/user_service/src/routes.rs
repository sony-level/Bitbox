use rocket::Route;

/**
 * Récupérer les routes de l'API
 * @return les routes de l'API
 * @example
    * routes![
    *     create_user,
    *     get_user,
    *     update_user,
    *     delete_user
    * ]
 */
pub fn routes() -> Vec<Route> {
    routes![
        crate::handlers::create_user
       // crate::handlers::get_user,
        //crate::handlers::update_user,
        //crate::handlers::delete_user
    ]
}
