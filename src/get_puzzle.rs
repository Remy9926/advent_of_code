async fn get_input_2022(day: u8, aoc_token: &str) -> String {
    let url = "https://adventofcode.com".parse::<reqwest::Url>().unwrap();
    
    let jar = reqwest::cookie::Jar::default();
    jar.add_cookie_str(aoc_token, &url);
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(jar.into())
        .build()
        .unwrap();
    
    let api_url = format!("{}2022/day/{day}/input", url.to_string(), day = day);
    let req = client.get(api_url);
    
    let resp = req.send()
        .await.expect("Failed to get response")
        .text()
        .await;

    resp.unwrap()
}

async fn get_input_2023(day: u8, aoc_token: &str) -> String {
    let url = "https://adventofcode.com".parse::<reqwest::Url>().unwrap();
    
    let jar = reqwest::cookie::Jar::default();
    jar.add_cookie_str(aoc_token, &url);
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(jar.into())
        .build()
        .unwrap();
    
    let api_url = format!("{}2023/day/{day}/input", url.to_string(), day = day);
    let req = client.get(api_url);
    
    let resp = req.send()
        .await.expect("Failed to get response")
        .text()
        .await;

    resp.unwrap()
}