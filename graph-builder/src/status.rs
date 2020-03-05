//! Status service.

use crate::graph::State;
use actix_web::HttpResponse;
use failure::Fallible;

/// Landing Page.
///
pub async fn landing() -> Fallible<HttpResponse> {
    let body = "
<html>
  <head><title>Graph Status</title></head>
  <body>
    <ul>
      <li><a href=\"liveness\">liveness</a></li>
      <li><a href=\"metrics\">metrics</a></li>
      <li><a href=\"readiness\">readiness</a></li>
    </ul>
  </body>
</html>";

    Ok(HttpResponse::Ok()
       .content_type("text/html; charset=utf-8")
       .body(body))
}

/// Expose liveness status.
///
/// Status:
///  * Live (200 code): The upstream scrape loop thread is running
///  * Not Live (503 code): everything else.
pub async fn serve_liveness(app_data: actix_web::web::Data<State>) -> Fallible<HttpResponse> {
    let resp = if app_data.is_live() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::ServiceUnavailable().finish()
    };

    Ok(resp)
}

/// Expose readiness status.
///
/// Status:
///  * Ready (200 code): a JSON graph as the result of a successful scrape is available.
///  * Not Ready (503 code): no JSON graph available yet.
pub async fn serve_readiness(app_data: actix_web::web::Data<State>) -> Fallible<HttpResponse> {
    let resp = if app_data.is_ready() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::ServiceUnavailable().finish()
    };

    Ok(resp)
}
