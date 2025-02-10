use http_req::request;
use http_req::uri::Uri;
use std::env;
use std::time::Duration;

fn main() {
    // Get the URL from command-line arguments
    let url = env::args()
        .nth(1)
        .expect("Please provide a URL as an argument");

    // Convert String to &str before passing to Uri::try_from
    let uri = match Uri::try_from(url.as_str()) {
        Ok(uri) => uri,
        Err(e) => {
            eprintln!("Invalid URI: {}", e);
            return;
        }
    };

    let mut body = Vec::new();
    let timeout = Duration::from_millis(1500);

    // Create a request with a timeout
    let mut request = request::Request::new(&uri);
    let response = request.timeout(timeout).send(&mut body);

    match response {
        Ok(resp) => {
            println!("Code: {}", resp.status_code());
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
