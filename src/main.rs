use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web::{web, App, Error, HttpResponse, HttpServer, cookie::Key};

async fn index(session: Session) -> Result<HttpResponse, Error> {
    // access session data
    if let Some(count) = session.get::<i32>("counter")? {
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }

    Ok(HttpResponse::Ok().body(format!(
        "Count is {:?}!",
        session.get::<i32>("counter")?.unwrap()
    )))
}
