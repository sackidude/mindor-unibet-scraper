pub struct Response {}

pub async fn get(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    Ok(Response {})
}
