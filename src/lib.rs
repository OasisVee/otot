use url::Url;

#[derive(Debug, PartialEq)]
pub enum InputType {
    FullUrl(Url),
    FuzzyPattern(Vec<String>),
}

pub fn classify_input(address: &str) -> InputType {
    if address.contains("://") {
        if let Ok(url) = Url::parse(address) {
            return InputType::FullUrl(url);
        }
    }

    let with_scheme = format!("https://{}", address);
    if let Ok(url) = Url::parse(&with_scheme) {
        // XXX: for now, we're assuming that, if the user didn't input a scheme, we can differentiate between a fuzzy pattern
        //   and a domain that just needs https prepended by the presence of a '.'
        if url.host_str().map_or(false, |h| h.contains('.')) {
            return InputType::FullUrl(url);
        }
    }

    InputType::FuzzyPattern(address.split('/').map(String::from).collect())
}
