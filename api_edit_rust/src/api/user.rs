use rocket::http::Status;
use rocket::request::{ Outcome, Request, FromRequest};

pub struct AuthUser {
    pub user: String
}

#[derive(Debug)]
pub enum AuthUserError {
    Missing,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = AuthUserError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("x-preferred-username") {
            None => Outcome::Failure((Status::BadRequest, AuthUserError::Missing)),
            Some(user) => Outcome::Success(AuthUser{ user: user.to_string() }),
        }
    }
}
