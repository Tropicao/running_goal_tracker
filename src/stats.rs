mod secrets;
mod strava;
use log::debug;
use secrets::Secrets;
use std::error::Error;
use std::io::stdin;
use strava::Strava;
use url::Url;
use log::*;

pub struct Stats {
    secrets: Secrets,
}

impl Stats {
    pub fn new(secrets_path: &str) -> Self {
        Stats {
            secrets: Secrets::new(secrets_path),
        }
    }

    pub fn is_configured(&self) -> bool {
        self.secrets.refresh_token().is_some()
    }

    pub fn prompt_user_client_id() -> Result<String, Box<dyn Error>> {
        let mut client_id = String::new();
        info!("Please enter your Strava client ID:");
        stdin().read_line(&mut client_id)?;
        client_id.pop();
        Ok(client_id)
    }

    pub fn prompt_user_client_secret() -> Result<String, Box<dyn Error>> {
        let mut client_secret = String::new();
        info!("No credentials configured. Please go to your Strava account, in Settings -> My API Application, and create an application");
        info!("Then enter asked credentials below");
        info!("Please enter your Strava client secret:");
        stdin().read_line(&mut client_secret)?;
        client_secret.pop();
        Ok(client_secret)
    }

    pub fn prompt_code(client_id: &str) -> Result<String, Box<dyn Error>> {
        info!("Please open this URL and validate authorization for your Strava application to interact with your Strava account:");
        info!("https://www.strava.com/oauth/authorize?client_id={}&response_type=code&redirect_uri=http://localhost&approval_prompt=force", client_id);
        info!("Once authorized, please paste here the URL you have been redirected to :");
        let mut redirected_url = String::new();
        stdin().read_line(&mut redirected_url)?;
        redirected_url.pop();
        let parsed_url = Url::parse(&redirected_url)?;
        Ok(parsed_url
            .query_pairs()
            .find_map(|(k, v)| if k == "code" { Some(v) } else { None })
            .ok_or("Can not extract code from redirected URL")?
            .to_string())
    }

    pub fn configure_secrets(
        &mut self,
        client_id_cli: Option<&str>,
        client_secret_cli: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        let client_id = if let Some(x) = client_id_cli {
            String::from(x.trim())
        } else {
            Self::prompt_user_client_id()?
        };

        let code = Self::prompt_code(&client_id)?;

        let client_secret = if let Some(x) = client_secret_cli {
            String::from(x.trim())
        } else {
            Self::prompt_user_client_secret()?
        };

        let (refresh_token, access_token) =
            Strava::get_initial_tokens(&client_id, &client_secret, &code)?;
        // We are now sure that passed credentials are valid, store them
        self.secrets.set_client_id(client_id);
        self.secrets.set_client_secret(client_secret);
        self.secrets.set_refresh_token(refresh_token);
        self.secrets.set_access_token(access_token);
        self.secrets.store()
    }

    pub fn update_secrets(&mut self) -> Result<(), Box<dyn Error>> {
        let (refresh_token, access_token) = Strava::renew_tokens(
            self.secrets.client_id().as_ref().unwrap(),
            self.secrets.client_secret().as_ref().unwrap(),
            self.secrets.refresh_token().as_ref().unwrap(),
        )?;
        self.secrets.set_refresh_token(refresh_token);
        self.secrets.set_access_token(access_token);
        self.secrets.store()
    }

    pub fn get_running_yearly_stat(&self) -> Result<u32, Box<dyn Error>> {
        debug!("Fetching athlete ID...");
        let access_token = self.secrets.access_token().as_ref().unwrap();
        debug!("Using access token {}", access_token);
        let id = Strava::get_logged_athlete(access_token)?;
        debug!("Athlete ID is {}", id);
        let stats = Strava::get_athlete_stats(access_token, &id)?;
        Ok(stats["ytd_run_totals"]["distance"]
            .to_string()
            .parse::<u32>()
            .unwrap()
            / 1000)
    }
}
