mod v1;
mod v2;

fn main() {
    // Test trading engine v1
    let _v1 = v1::test::testv1();
    // match v1 {
    //     Ok(_) => println!("v1 test passed"),
    //     Err(e) => eprintln!("v1 test failed: {}", e),
    // }

    // Test trading engine v2
    v2::test::testv2();
}
