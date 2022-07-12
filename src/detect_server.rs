use anyhow::Result;

// TODO: use several analysis tactics
/// Detect if a server is running Apache
pub fn is_apache(address: &str) -> Result<bool> {
    let mut formated_address_with_slash = String::from("");
    let mut formated_address_without_slash = String::from("");

    // Detect apache by checking how the URL encoded forward slash
    // characters are treated. Only works when the AllowEncodedSlashes
    // is enabled (it is enabled by default)
    if address.ends_with('/') {
        formated_address_with_slash = format!("{}/", address);
    } else {
        formated_address_with_slash = format!("{}//", address);
    }

    if address.ends_with('/') {
        formated_address_without_slash = format!("{}%2f", address);
    } else {
        formated_address_without_slash = format!("{}/%2f", address);
    }

    let body = reqwest::blocking::get(formated_address_with_slash)?;
    let body2 = reqwest::blocking::get(formated_address_without_slash)?;

    if body.status().is_success() && body2.status() == reqwest::StatusCode::NOT_FOUND {
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
