use hyper::header::HeaderName;

fn main() {
    let hdr = HeaderName::from_lowercase(b"content-length").unwrap();
}
