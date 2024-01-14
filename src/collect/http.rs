use hmac::Mac;
use reqwest::Request;
use reqwest::Response;
use reqwest_middleware::ClientBuilder;
use reqwest_middleware::ClientWithMiddleware;
use reqwest_middleware::Middleware;
use reqwest_middleware::Next;
use reqwest_middleware::Result;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use task_local_extensions::Extensions;

pub fn default_client() -> ClientWithMiddleware {
  let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
  ClientBuilder::new(reqwest::Client::new())
    .with(RetryTransientMiddleware::new_with_policy(retry_policy))
    .build()
}

pub struct SignatureMiddleware;

impl SignatureMiddleware {
  const SIGNATURE_HEADER_KEY: &'static str = "X-CMDB-Signature";

  fn sign(bytes: &[u8], secret_key: &[u8]) -> Result<String> {
    let mut mac: hmac::Hmac<sha1::Sha1> = hmac::Mac::new_from_slice(secret_key)
      .unwrap_or_else(|e| panic!("HMAC can not take key of any size: {}", e));
    mac.update(bytes);
    let result = mac.finalize();
    Ok(hex::encode(result.into_bytes()))
  }
}

#[async_trait::async_trait]
impl Middleware for SignatureMiddleware {
  async fn handle(
    &self,
    mut request: Request,
    extensions: &mut Extensions,
    next: Next<'_>,
  ) -> Result<Response> {
    let body = request.body();

    if let Some(body) = body {
      let signature = Self::sign(
        body.as_bytes().unwrap(),
        "b35057912c5def0e848b14fb".as_bytes(),
      )?;
      request.headers_mut().insert(
        Self::SIGNATURE_HEADER_KEY,
        format!("SHA1={}", signature)
          .parse()
          .map_err(reqwest_middleware::Error::middleware)?,
      );
    }

    let headers = request.headers_mut();
    headers.insert(
      reqwest::header::USER_AGENT.as_str(),
      "CMDB Agent/reqwest client".parse().unwrap(),
    );
    headers.insert(
      reqwest::header::CONTENT_TYPE.as_str(),
      "application/json".parse().unwrap(),
    );

    next.run(request, extensions).await
  }
}
