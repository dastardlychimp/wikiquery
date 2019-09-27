use wikiquery::responses::Query;

use http;
use hyper;
use tokio;
use serde_json;
use lazy_static;
use hyper_alpn;

use lazy_static::{lazy_static as lazy};
use hyper::{Client, Response, Body, error::Error};
use hyper_alpn::AlpnConnector;
use tokio::runtime::Runtime;

lazy! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref CLIENT: Client<AlpnConnector> = {
        let mut builder = Client::builder();
        builder.http2_only(true);

        builder.build(AlpnConnector::new())
    };
}

pub async fn send_query(uri: http::Uri) -> Result<Response<Body>, Error>
{
    CLIENT.get(uri).await
}

pub fn send_successful_query(uri: http::Uri) -> Query
{
    let body_fut = async {
        let resp = send_query(uri).await.unwrap();
        let body = resp.into_body();
        body_to_string(body).await
    };

    let body = RUNTIME.block_on(body_fut);

    println!("body: {:?}", &body);

    serde_json::from_str(&body).unwrap()
}

pub async fn body_to_string(mut body: Body) -> String
{
    let mut string = String::new();

    while let Some(chunk) = body.next().await {
        string.push_str(std::str::from_utf8(&*chunk.unwrap()).unwrap());
    }

    string
}