use anyhow::Result;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use serde_derive::Deserialize;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use futures_util::future::join;
use std::convert::Infallible;

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default="default_port")]
    port: u16,
    #[serde(default="default_second_port")]
    second_port: u16,
}

fn default_port() -> u16 {
    8080
}

fn default_second_port() -> u16 {
    8888
}

// Sample handler
async fn index(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Iterate request header
    for (hkey, hval) in req.headers().iter() {
        let hname = hkey.as_str().to_string().to_uppercase();
        info!("Request Header: {}: {}", hname, hval.to_str().unwrap_or(""));
    }

    // Set response body
    let body = match std::env::var("RESPONSE") {
        Ok(val) => val,
        Err(_) => "Hello, OpenShift!\n".to_string(),
    };
    info!("RESPONSE: {}", body);

    let response = Response::builder()
                    .status(200)
                    .header("X-Foo", "value")
                    .header("Set-Cookie", "foo=bar")
                    .body(Body::from(body))
                    .unwrap();

    // Return response
    Ok(response)
}


#[tokio::main]
async fn main() -> Result<()> {
    // Initial logging setting
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Get OS Environment
    let config = envy::from_env::<Config>()?;
    info!("Port={}", config.port);
    info!("Second Port={}", config.second_port);

    let addr1 = ([0, 0, 0, 0], config.port).into();
    let addr2 = ([0, 0, 0, 0], config.second_port).into();

    let srv1 = Server::bind(&addr1).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(index))
    }));

    let srv2 = Server::bind(&addr2).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(index))
    }));

    info!("Listening on http://{}", addr1);
    info!("Listening on http://{}", addr2);

    let _ret = join(srv1, srv2).await;

    Ok(())
}
