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

    let inferred_scheme = if address.contains(':') {
        "http"
    } else {
        "https"
    };

    let with_scheme = format!("{}://{}", inferred_scheme, address);
    if let Ok(url) = Url::parse(&with_scheme) {
        // XXX: for now, we're assuming that, if the user didn't input a scheme, we can differentiate between a fuzzy pattern
        //   and a domain that just needs https prepended by the presence of a '.'
        if url.host_str().map_or(false, |h| h.contains('.')) || url.port().is_some() {
            return InputType::FullUrl(url);
        }
    }

    InputType::FuzzyPattern(
        address
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .collect(),
    )
}
