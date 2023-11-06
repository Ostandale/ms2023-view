use yup_oauth2::authenticator::Authenticator;
use yup_oauth2::read_service_account_key;

use hyper::client::HttpConnector;
use hyper::Client;
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

pub async fn auth(
    root_path: &str,
) -> Result<
    (
        Authenticator<HttpsConnector<HttpConnector>>,
        Client<HttpsConnector<HttpConnector>>,
    ),
    Box<dyn std::error::Error>,
> {
    let path = format!("{}google_service_acc.json", root_path);
    let credentials_data = read_service_account_key(path).await?;
    let client = Client::builder().build(
        HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .enable_http2()
            .build(),
    );
    let auth =
        yup_oauth2::ServiceAccountAuthenticator::with_client(credentials_data, client.clone())
            .build()
            .await?;
    Ok((auth, client))
}
