use std::env;

fn main() -> wry::Result<()> {
    if let Some(url) = env::args().nth(1) {
        println!("Opening {}", url);
        bornscreen::spawn_browser(url)
    } else {
        println!("BornScreen requires a URL as a parameter.");
        Ok(())
    }
}
