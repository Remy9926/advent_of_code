use reqwest;

const AOC_BASEURL: &str = "https://adventofcode.com";

/// Gets the puzzle input associated with a specific user for AOC 2022 for the specified day
/// and returns it
/// 
/// # Arguments
/// 
/// * `day` - The day's puzzle input that you want to get
/// * `aoc_token` - Your AOC token associated with your account
/// 
/// # Return
/// 
/// The puzzle input for the corresponding user
pub async fn get_input_2022(day: u8, aoc_token: &str) -> String {
    let url = AOC_BASEURL.parse::<reqwest::Url>().unwrap();
    
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

/// Gets the puzzle input associated with a specific user for AOC 2023 for the specified day
/// and returns it
/// 
/// # Arguments
/// 
/// * `day` - The day's puzzle input that you want to get
/// * `aoc_token` - Your AOC token associated with your account
/// 
/// # Return
/// 
/// The puzzle input for the corresponding user
pub async fn get_input_2023(day: u8, aoc_token: &str) -> String {
    let url = AOC_BASEURL.parse::<reqwest::Url>().unwrap();
    
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