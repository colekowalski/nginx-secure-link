# nginx-secure-link

[![Tests](https://github.com/colekowalski/nginx-secure-link/actions/workflows/test.yaml/badge.svg)](https://github.com/colekowalski/nginx-secure-link/actions/workflows/test.yaml)
[![License](https://img.shields.io/github/license/colekowalski/nginx-secure-link)](https://github.com/colekowalski/nginx-secure-link/blob/master/LICENSE)

nginx-secure-link is a library to generate and verify checksums for use with Nginx's
[secure_link_module](https://nginx.org/en/docs/http/ngx_http_secure_link_module.html).

## Example
```rust
use nginx_secure_link::SecureLink;

fn main() {
    let slink = SecureLink::new("secureKey");
    let checksum = slink.generate("https://www.google.com/");
    println!("Checksum: {}", checksum);
}
```