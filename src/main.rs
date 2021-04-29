#![allow(dead_code, unused_variables)]

mod analyser;
mod scraper;
mod errors;

use analyser::Analyser;

fn main() {
    println!("Hello, Manta!");

    //  TESTING
    let mut analyser = Analyser::new();
    analyser.inspect_html(&String::from("<html><body>Hello, Manta!</body></html>")).unwrap();
}
