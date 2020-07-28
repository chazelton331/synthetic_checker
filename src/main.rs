use hyper_tls::HttpsConnector;
use hyper::Client;

#[tokio::main]
async fn get_example() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get("https://www.stash.com".parse()?).await?;
    assert_eq!(res.status(), 200);

    Ok(())
}

fn main() {
    get_example();
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn todo_test() {
        assert_eq!(1 + 1, 2);
    }
}
