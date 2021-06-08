use id_contact_proto::{AuthResult, StartCommRequest, StartCommResponse};
use rocket::{State, fairing::AdHoc, get, launch, post, routes};
use rocket_contrib::{database, databases::postgres, json::Json};
use serde::{Serialize, Deserialize};

use crate::{config::Config, error::Error};

mod comm;
mod config;
mod error;

#[database("session")]
pub struct SessionDBConn(postgres::Client);

#[get("/clean_db")]
async fn clean_db(
    db: SessionDBConn,
) -> Result<(), Error> {
    comm::clean_db(&db).await
}

#[post("/start_communication", data = "<request>")]
async fn start(
    request: Json<StartCommRequest>,
    config: State<'_, Config>,
    db: SessionDBConn,
) -> Result<Json<StartCommResponse>, Error> {
    let (dtmf, resultcode) = comm::create_session(&db, &config, &request.purpose).await?;

    if let Some(auth_result) = &request.auth_result {
        comm::report_result(&db, &config, &resultcode, auth_result).await?;
        Ok(Json(StartCommResponse{
            client_url: format!("tel:{},{}", config.phonenumber(), dtmf),
            attr_url: None,
        }))
    } else {
        Ok(Json(StartCommResponse{
            client_url: format!("tel:{},{}", config.phonenumber(), dtmf),
            attr_url: Some(format!("{}/session_result/{}", config.internal_url(), resultcode)),
        }))
    }
}

#[post("/session_result/<resultcode>", data = "<auth_result>")]
async fn report_result(
    resultcode: String,
    auth_result: String,
    config: State<'_, Config>,
    db: SessionDBConn,
) -> Result<(), Error> {
    comm::report_result(&db, &config, &resultcode, &auth_result).await?;
    Ok(())
}

#[derive(Debug, Deserialize)]
struct PhoneLink {
    dtmf: String,
    sessionid: String,
}

#[post("/link_phone", data = "<request>")]
async fn link_phone(
    request: Json<PhoneLink>,
    db: SessionDBConn,
) -> Result<(), Error> {
    comm::link_phone_session(&db, &request.dtmf, &request.sessionid).await?;
    Ok(())
}

#[derive(Debug, Serialize)]
struct SessionInfo {
    purpose: String,
    auth_result: Option<AuthResult>,
}

#[get("/session_info/<sessionid>")]
async fn session_info(
    sessionid: String,
    config: State<'_, Config>,
    db: SessionDBConn,
) -> Result<Json<SessionInfo>, Error> {
    let (purpose, auth_result) = comm::get_session_info(&db, &config, &sessionid).await?;
    Ok(Json(SessionInfo{
        purpose: purpose,
        auth_result: auth_result,
    }))
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![
            start,
            report_result,
            link_phone,
            session_info,
            clean_db,
        ])
        .attach(SessionDBConn::fairing())
        .attach(AdHoc::config::<Config>())
}
