use exmpl_md5;

#[test]
fn empty_data() {
    let x = "";
    let hash = "d41d8cd98f00b204e9800998ecf8427e";
    println!("exmpl_digest: {}", exmpl_md5::md5(x));
    assert_eq!(hash, exmpl_md5::md5(x));
}

#[test]
fn b1_data() {
    let x = "1";
    let hash = "c4ca4238a0b923820dcc509a6f75849b";
    println!("exmpl_digest: {}", exmpl_md5::md5(x));
    assert_eq!(hash, exmpl_md5::md5(x));
}

#[test]
fn b2_data() {
    let x = "12";
    let hash = "c20ad4d76fe97759aa27a0c99bff6710";
    println!("exmpl_digest: {}", exmpl_md5::md5(x));
    assert_eq!(hash, exmpl_md5::md5(x));
}

#[test]
fn b3_data() {
    let x = "123";
    let hash = "202cb962ac59075b964b07152d234b70";
    println!("exmpl_digest: {}", exmpl_md5::md5(x));
    assert_eq!(hash, exmpl_md5::md5(x));
}

#[test]
fn hello_world_data() {
    let x = "Hello, world!";
    let hash = "6cd3556deb0da54bca060b4c39479839";
    assert_eq!(hash, exmpl_md5::md5(x));
}

#[test]
fn n6_hello_world_data() {
    let x = "Hello, world! Hello, world! Hello, world! Hello, world! Hello, world! Hello, world!";
    let hash = "3a80ffdf9d199af6a32a2d325f83d928";
    assert_eq!(hash, exmpl_md5::md5(x));
}
