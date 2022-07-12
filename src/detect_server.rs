use anyhow::Result;

// TODO: use several analysis tactics
/// Detect if a server is running Apache
pub fn is_apache(url: &str) -> Result<bool> {
    let mut address_with_normal_slash = String::from("");
    let mut address_with_url_encoded_slash = String::from("");

    // Detect apache by checking how the URL encoded forward slash
    // characters are treated. Only works when the AllowEncodedSlashes
    // is enabled (it is enabled by default)
    if address_with_normal_slash.ends_with('/') {
        address_with_normal_slash = format!("{}/", url);
        address_with_url_encoded_slash = format!("{}%2f", url);
    } else {
        address_with_normal_slash = format!("{}//", url);
        address_with_url_encoded_slash = format!("{}/%2f", url);
    }

    let normal_slash_body = reqwest::blocking::get(address_with_normal_slash)?;
    let url_encoded_body = reqwest::blocking::get(address_with_url_encoded_slash)?;

    if normal_slash_body.status().is_success()
        && url_encoded_body.status() == reqwest::StatusCode::NOT_FOUND
    {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apache_detection() {
        assert!(is_apache("https://httpd.apache.org").unwrap());
        assert!(!is_apache("https://www.c-sharpcorner.com").unwrap());
    }
}
