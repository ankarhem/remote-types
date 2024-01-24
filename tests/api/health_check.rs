use reqwest::StatusCode;

use remote_types::*;

#[tokio::test]
async fn healthcheck_works() {
    let addr = spawn_app();

    let response = reqwest::get(format!("http://{addr}/__healthcheck"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
