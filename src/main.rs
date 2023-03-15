use openssl::rsa::Rsa;

fn main() {
    let _rsa = Rsa::generate(2048).unwrap();
}
