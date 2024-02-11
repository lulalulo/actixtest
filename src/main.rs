use actix_web::{error, get middleware::Logger, App, HttpServer, Result};
user derive_more::{Display, Error};
use log::info;

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {} ", name)]
pub struct MyError {
    name: &'static str,
}

//use Default implementation for error_response() method
impl error::ResponseError for MyError {}



