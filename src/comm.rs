use id_contact_comm_common::error::Error;
use id_contact_proto::AuthResult;

use crate::{SessionDBConn, config::Config};

pub async fn clean_db(db: &SessionDBConn) -> Result<(), Error> {
    db.run(move |c| c.execute("DELETE FROM session WHERE lastActivity < now() - INTERVAL '1 hour'", &[])).await?;
    Ok(())
}

pub async fn create_session(db: &SessionDBConn, config: &Config, purpose: &str) -> Result<(String, String), Error> {
    loop {
        let dtmf = config.generate_dtmf();
        let result = config.generate_resultcode();

        let dtmf_copy = dtmf.clone();
        let result_copy = result.clone();
        let purpose_copy = purpose.to_string();
        let n = db.run(move |c| c.execute("INSERT INTO session (dtmfcode, resultcode, purpose, lastActivity) VALUES ($1, $2, $3, now())", &[&dtmf_copy, &result_copy, &purpose_copy])).await?;
        if n == 1 {
            return Ok((dtmf, result))
        }
    }
}

pub async fn report_result(db: &SessionDBConn, config: &Config, resultcode: &str, jwt: &str) -> Result<(), Error> {
    id_contact_jwt::decrypt_and_verify_auth_result(&jwt, config.validator(), config.decrypter())?;

    let resultcode_copy = resultcode.to_string();
    let jwt_copy = jwt.to_string();
    let n = db.run(move |c| c.execute("UPDATE session SET attr_jwt = $1, lastActivity = now() WHERE attr_jwt IS NULL AND resultcode = $2", &[&jwt_copy, &resultcode_copy])).await?;
    if n == 1 {
        Ok(())
    } else {
        Err(Error::NotFound)
    }
}

pub async fn link_phone_session(db: &SessionDBConn, dtmfcode: &str, sessionid: &str) -> Result<(), Error> {
    let dtmf_copy = dtmfcode.to_string();
    let session_copy = sessionid.to_string();

    let n = db.run(move |c| c.execute("UPDATE session SET sessionid = $1, lastActivity = now() WHERE sessionid IS NULL AND dtmfcode = $2", &[&session_copy, &dtmf_copy])).await?;
    if n == 1 {
        Ok(())
    } else {
        Err(Error::NotFound)
    }
}

pub async fn get_session_info(db: &SessionDBConn, config: &Config, sessionid: &str) -> Result<(String, Option<AuthResult>), Error> {
    let sessionid_copy = sessionid.to_string();
    let (purpose, jwt) = db.run(move |c| -> Result<(String, Option<String>), Error> {
        let rows = c.query("SELECT purpose, attr_jwt FROM session WHERE sessionid = $1", &[&sessionid_copy])?;
        if rows.len() != 1 {
            Err(Error::NotFound)
        } else {
            Ok((rows[0].get("purpose"), rows[0].try_get("attr_jwt")?))
        }
    }).await?;

    if let Some(jwt) = jwt {
        Ok((purpose, Some(id_contact_jwt::decrypt_and_verify_auth_result(&jwt, config.validator(), config.decrypter())?)))
    } else {
        Ok((purpose, None))
    }
}
