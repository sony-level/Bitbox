use domain::models::UserRole;
use rocket::data::Outcome;

pub struct TrainerGuard;
#[async_trait]
impl<'r> FromRequest<'r> for TrainerGuard {
    type Error = Status;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user = match AuthenticatedUser::from_request(request).await {
            Outcome::Success(user) => user,
            Outcome::Error(e) => return Outcome::Error(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        if user.role == UserRole::Trainer {
            Outcome::Success(TrainerGuard)
        } else {
            Outcome::Error(Status::Unauthorized)
        }
    }
}