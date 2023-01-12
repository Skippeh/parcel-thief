use http::Response;

pub fn into_raw_http(response: Response<String>) -> String {
    let mut result = String::with_capacity(128);

    result.push_str(format!("{:?} {}\r\n", response.version(), response.status()).as_str());

    for (name, value) in response.headers() {
        result.push_str(format!("{}: {}\r\n", name, value.to_str().unwrap()).as_str());
    }

    result.push_str("\r\n");

    let body = response.into_body();

    if !body.is_empty() {
        result.push_str(&body);
    }

    result
}
