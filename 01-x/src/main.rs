extern crate reqwest;

fn main() {
    match fetch_text("https://www.rust-lang.org") {
        Err(e) => println!("{}", e),
        Ok(result) => println!("{}", result.chars().count())

    }
}

fn fetch_text(url: &str) -> Result<String, reqwest::Error> {
    reqwest::get(url)?.text()
}

#[test]
fn it_fetches_text() {
    let url = "https://www.rust-lang.org";
    let _ = fetch_text(url).and_then(|res| {
        assert_eq!(16191, res.chars().count());
        Ok(())
    });
}