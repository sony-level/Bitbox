use rocket::async_trait;
use domain::models::UserRole;
use rocket::outcome::Outcome;
use rocket::{Request, request};
use rocket::http::Status;
use rocket::request::FromRequest;
use utoipa::Modify;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use auth_service::guard::AuthenticatedUser;


pub struct TrainerGuard;

#[async_trait]
impl<'r> FromRequest<'r> for TrainerGuard {
  type Error = ();
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error>  {
       let user = match AuthenticatedUser::from_request(request).await {
           Outcome::Success(user) => user,
          Outcome::Error(_e) => return Outcome::Error((Status::Unauthorized, ())),
            Outcome::Forward(_) => return Outcome::Forward(Default::default()),
        };

        if user.role == UserRole::Trainer {
           Outcome::Success(TrainerGuard)
        } else {
            return Outcome::Error((Status::Unauthorized, ()));
        }
    }
}


pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
