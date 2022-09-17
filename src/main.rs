use reqwest::blocking::Client;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Cheat.sh Helper");

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let url = "https://cheat.sh/".to_string() + &args[1];
        let client = Client::new();
        let resp = client
            .get(url)
            .header("User-Agent", "curl/7.79.1")
            .send()?
            .text()?;
        println!("{}", resp);
    } else {
        eprintln!("Unable to find bash command\n");
        println!("Usage:\nch [command]\nExample:\nch ls\nch cp");
    }

    Ok(())
}
