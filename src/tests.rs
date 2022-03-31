#[cfg(test)]
mod tests {
    use scraper::{Html, Selector};
    #[test]
    fn it_works() {
        let fragment = Html::parse_fragment(
            r#"
            <div>
                <p>Hello, world!</p>
                <p>Goodbye, world!</p>
            </div>"#);
        let selector = Selector::parse("p").unwrap();
        let paragraphs = fragment.select(&selector);
        let mut v = vec![];
        for p in paragraphs {
            for text in p.text() {
                v.push(text);
            }
        }
        assert_eq!(v, vec!["Hello, world!", "Goodbye, world!"]);
    }
    #[test]
    fn ajax_test() {
        let url = "gogoplay5.com/embedplus?id=MTA0NTE4&token=mKYid9T_98sbXhnS2IFMmA&expires=1648704533";
        println!("ajax: {}", crate::decode::decode::get_ajax(url));
    }

}