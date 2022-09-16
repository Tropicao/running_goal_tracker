use log::debug;
use reqwest::{blocking::Client, header::AUTHORIZATION};
use serde_json::Value;
use std::error::Error;
pub struct Strava {}

impl Strava {
    pub fn get_initial_tokens(
        client_id: &str,
        client_secret: &str,
        code: &str,
    ) -> Result<(String, String), Box<dyn Error>> {
        debug!(
            "Using ID {}, secret {}, code {}",
            client_id, client_secret, code
        );
        let response = Client::new()
            .post("https://www.strava.com/oauth/token")
            .query(&[("client_id", client_id)])
            .query(&[("client_secret", client_secret)])
            .query(&[("grant_type", "authorization_code")])
            .query(&[("code", code)])
            .send()?;
        debug!("Raw response : {:?}", response);

        let reformatted_resp = response.text()?.replace('\\', "");
        debug!("Escaped body : {}", reformatted_resp);
        let v: Value = serde_json::from_str(&reformatted_resp)?;
        debug!("JSON response : {:?}", v);
        Ok((
            v["refresh_token"].as_str().unwrap().to_string(),
            v["access_token"].as_str().unwrap().to_string(),
        ))
    }

    pub fn renew_tokens(
        client_id: &str,
        client_secret: &str,
        refresh_token: &str,
    ) -> Result<(String, String), Box<dyn Error>> {
        debug!(
            "Using ID {}, secret {}, refresh_token {}",
            client_id, client_secret, refresh_token
        );
        let response = Client::new()
            .post("https://www.strava.com/oauth/token")
            .query(&[("client_id", client_id)])
            .query(&[("client_secret", client_secret)])
            .query(&[("grant_type", "refresh_token")])
            .query(&[("refresh_token", refresh_token)])
            .send()?;
        debug!("Raw response : {:?}", response);

        let reformatted_resp = response.text()?.replace('\\', "");
        debug!("Escaped body : {}", reformatted_resp);
        let v: Value = serde_json::from_str(&reformatted_resp)?;
        debug!("JSON response : {:?}", v);
        Ok((
            v["refresh_token"].as_str().unwrap().to_string(),
            v["access_token"].as_str().unwrap().to_string(),
        ))
    }

    pub fn get_logged_athlete(access_token: &str) -> Result<String, Box<dyn Error>> {
        let mut auth = String::from("Bearer ");
        auth.push_str(access_token);
        let response = Client::new()
            .get("https://www.strava.com/api/v3/athlete")
            .header(AUTHORIZATION, &auth)
            .send()?;
        let reformatted_resp = response.text()?.replace('\\', "");
        debug!("Escaped body : {}", reformatted_resp);
        let v: Value = serde_json::from_str(&reformatted_resp)?;
        debug!("JSON response : {:?}", v);
        Ok(v["id"].to_string())
    }

    pub fn get_athlete_stats(access_token: &str, id: &str) -> Result<Value, Box<dyn Error>> {
        let mut auth = String::from("Bearer ");
        auth.push_str(access_token);
        let response = Client::new()
            .get(format!(
                "https://www.strava.com/api/v3/athletes/{}/stats",
                id
            ))
            .header(AUTHORIZATION, &auth)
            .send()?;
        let reformatted_resp = response.text()?.replace('\\', "");
        debug!("Escaped body : {}", reformatted_resp);
        let v: Value = serde_json::from_str(&reformatted_resp)?;
        debug!("JSON response : {:?}", v);
        Ok(v)
    }
}
