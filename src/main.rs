use std::env;
use reqwest::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get url
    println!("Cheat.sh Helper");
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let first_arg = args.get(0).unwrap().to_owned();

    // Send http request and print result
    let url = "https://cheat.sh/".to_string() + &first_arg;
    let client = Client::new();
    let resp = client.get(url).header("User-Agent", "curl/7.79.1").send()?.text()?;
    println!("{}", resp);
    Ok(())
}