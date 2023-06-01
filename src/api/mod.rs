//! Module that contains the main [iplookupapi] struct

use std::sync::Arc;
use reqwest::Client;
use crate::error::IplookupapiError;
use crate::{error, models, utils};
use crate::utils::baseline::construct_base_url;

/// Settings struct that contains the api key
#[derive(Debug, Clone)]
pub struct Settings {
    api_key: String,
}

/// The main struct of the crate giving access to the iplookupapi.
/// Create a new instance of the struct with your api key as parameter.
#[derive(Debug, Clone)]
pub struct Iplookupapi {
    client: Client,
    settings: Arc<Settings>,
}

impl<'a> Iplookupapi {
    /// Creates a new instance of the Iplookupapi struct by passing your api key as
    /// function parameter.
    pub fn new(api_key: &'a str) -> Result<Self, IplookupapiError> {
        let settings = std::sync::Arc::new(Settings {
            api_key: String::from(api_key),
        });
        let client = utils::baseline::construct_client(None, &settings)?;
        Ok(Self { client, settings })
    }

    pub async fn status(
        &self,
    ) -> Result<models::DetailsResponse, error::IplookupapiError> {
        let mut url = construct_base_url(&self.settings.api_key, Some("status"))?;
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::IplookupapiError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::IplookupapiError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::IplookupapiError::ResponseParsingError { body: res_body })
    }

    pub async fn info(
        &self,
        ip: &'a str,
        language: &'a str,
    ) -> Result<models::DetailsResponse, error::IplookupapiError> {
        let mut url = construct_base_url(&self.settings.api_key, Some("latest"))?;
        url.query_pairs_mut()
            .append_pair("ip", ip)
            .append_pair("language", language);
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::IplookupapiError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::IplookupapiError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::IplookupapiError::ResponseParsingError { body: res_body })
    }
}
