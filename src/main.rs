fn main() {
    let url = std::env::args()
        .nth(1)
        .expect("Please provide a URL as an argument");

    let mut body = Vec::new();
    let response = http_req::request::get(&url, &mut body);

    match response {
        Ok(resp) => {
            println!("Code: {}", resp.status_code());
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
