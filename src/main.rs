// use openssl::rsa::Rsa;
use webauthn_authenticator_rs::prelude::Url;

fn main() {
    // let _rsa = Rsa::generate(2048).unwrap();
    let url = Url::parse("http://foo.com").unwrap();
}
