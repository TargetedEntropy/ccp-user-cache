use std::fmt;
// holy crap we have eror.. functions!?
// what is this 1999
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

// Response used in JSON 
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}
impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    HashingError,
    InvalidHashFormat,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvided,
    PermissionDenied,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl Into<String> for ErrorMessage {
    fn into(self) -> String {
        self.to_string()
    }
}


impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::ServerError => "Server Error, try again later".to_string(),
            ErrorMessage::WrongCredentials => "Invalid Email and/or password".to_string(),
            ErrorMessage::EmailExist => "Email is in use, try again".to_string(),
            ErrorMessage::UserNoLongerExist => {
                "User no longer exists for this token".to_string()
            }
            ErrorMessage::EmptyPassword => "No Empty Passwords".to_string(),
            ErrorMessage::HashingError => "Couldn't Hash your password, wtf did you enter".to_string(),
            ErrorMessage::InvalidHashFormat => "Invalid password format".to_string(),
            ErrorMessage::ExceededMaxPasswordLength(max_length) => {
                format!("Password must not be more than {} characters", max_length)
            }
            ErrorMessage::InvalidToken => "Auth token is invalid or expired".to_string(),
            ErrorMessage::TokenNotProvided => {
                "You are not logged in, please provide token".to_string()
            }
            ErrorMessage::PermissionDenied => {
                "You are not allowed to perform this action".to_string()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: u16,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: u16) -> Self {
        HttpError {
            message: message.into(),
            status,
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 500,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 400,
        }
    }

    pub fn unique_constraint_voilation(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 409,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 401,
        }
    }

    pub fn into_http_response(self) -> HttpResponse {
        match self.status {
            400 => HttpResponse::BadRequest().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            401 => HttpResponse::Unauthorized().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            409 => HttpResponse::Conflict().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            500 => HttpResponse::InternalServerError().json(Response {
                status: "error",
                message: self.message.into(),
            }),
            _ => {
                eprintln!(
                    "Warning: Missing pattern match. Converted status code {} to 500.",
                    self.status
                );

                HttpResponse::InternalServerError().json(Response {
                    status: "error",
                    message: ErrorMessage::ServerError.into(),
                })
            }
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: message: {}, status: {}",
            self.message, self.status
        )
    }
}

impl std::error::Error for HttpError {}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let cloned = self.clone();
        cloned.into_http_response()
    }
}
