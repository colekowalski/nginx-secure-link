use base64;
use md5;

pub struct SecureLink<'a> {
    key: &'a str,
}

impl SecureLink<'_> {
    pub fn new(key: &str) -> SecureLink {
        return SecureLink { key };
    }

    pub fn generate(self, url: &str) -> String {
        let digest = md5::compute(format!("{} {}", url, self.key));
        return base64::encode_config(digest.as_slice(), base64::URL_SAFE_NO_PAD);
    }

    pub fn verify(self, url: &str, key: &str) -> bool {
        return key == self.generate(url);
    }
}

#[cfg(test)]
mod tests {
    use super::SecureLink;

    #[test]
    fn test_key_generation() {
        let slink = SecureLink::new("testkey");
        let key = slink.generate("https://www.google.com/");
        assert_eq!(key, "HY4On4h_ZipVaOa0uOHtOg");
    }

    #[test]
    fn test_key_verification() {
        let slink = SecureLink::new("testkey");
        assert_eq!(
            slink.verify("https://www.google.com/", "HY4On4h_ZipVaOa0uOHtOg"),
            true
        );
    }

    #[test]
    fn test_bad_key_verification() {
        let slink = SecureLink::new("testkey");
        assert_eq!(slink.verify("https://www.google.com/", "invalidKey"), false);
    }
}
