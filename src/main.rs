use std::env;

fn main() -> wry::Result<()> {
    let url = env::args()
        .nth(1)
        .expect("The app requires a URL parameter!");
    println!("Opening {}", url);
    bornscreen::spawn_browser(url)
}
