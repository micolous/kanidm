use kanidm_client::KanidmClient;

#[kanidmd_testkit::test]
async fn test_https_middleware_headers(rsclient: KanidmClient) {
    // We need to do manual reqwests here.
    let addr = rsclient.get_url();

    // here we test the /ui/ endpoint which should have the headers
    let response = match reqwest::get(format!("{}/ui", &addr)).await {
        Ok(value) => value,
        Err(error) => {
            panic!("Failed to query {:?} : {:#?}", addr, error);
        }
    };
    eprintln!("response: {:#?}", response);
    assert_eq!(response.status(), 200);
    eprintln!(
        "csp headers: {:#?}",
        response.headers().get("content-security-policy")
    );
    assert_ne!(response.headers().get("content-security-policy"), None);

    // here we test the /ui/login endpoint which should have the headers
    let response = match reqwest::get(format!("{}/ui/login", &addr)).await {
        Ok(value) => value,
        Err(error) => {
            panic!("Failed to query {:?} : {:#?}", addr, error);
        }
    };
    eprintln!("response: {:#?}", response);
    assert_eq!(response.status(), 200);

    eprintln!(
        "csp headers: {:#?}",
        response.headers().get("content-security-policy")
    );
    assert_ne!(response.headers().get("content-security-policy"), None);
}
