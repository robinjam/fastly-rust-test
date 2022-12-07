use fastly::{Error, Request, Response};

#[fastly::main]
fn main(mut request: Request) -> Result<Response, Error> {
    let mut query: Vec<(String, String)> = request.get_query()?;
    query.retain(|(key, _)| !key.to_lowercase().starts_with("utm_"));
    query.sort_unstable();
    request.set_query(&query)?;

    let response = request.send("origin")?;

    Ok(response)
}
