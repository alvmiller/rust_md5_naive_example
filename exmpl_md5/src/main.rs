/*
https://dev.to/mdimovich/build-md5-from-scratch-with-rust-563e
https://github.com/mdimovich/rusty_md5/
https://github.com/RustCrypto/hashes/tree/master/md5
https://stackoverflow.com/questions/65036641/md5-hash-in-rust
https://docs.rs/md-5/latest/md5/#usage
https://mojoauth.com/hashing/hmac-md5-in-rust/

sudo snap install rustup --classic
rustup default stable
cargo new exmpl_md5
cargo run
cargo test
cargo bench
*/

fn main() {
    //let s = "Hello, world!";
    //let md5s = "6cd3556deb0da54bca060b4c39479839";
    let s = "Hello, world! Hello, world! Hello, world! Hello, world! Hello, world! Hello, world! Hello, world!";
    let md5s = "cecaddde2f3f4cbe32521e17fd07c864";
    println!("Input data: {}", s);
    println!("MD5 expected: {}", md5s);

    let exmpl_digest = exmpl_md5::md5(s);
    let md5_digest = md5::compute(s);
    println!("exmpl_digest: {}", exmpl_digest);
    println!("md5_digest: {:x}", md5_digest);
    assert_eq!(format!("{:x}", md5_digest), exmpl_digest);
    assert_eq!(md5s, exmpl_digest);
}
