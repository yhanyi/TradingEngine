mod v1;
mod v2;
mod v3;

#[tokio::main]
async fn main() {
    // Test trading engine v1
    // let _v1 = v1::test::testv1();
    // match v1 {
    //     Ok(_) => println!("v1 test passed"),
    //     Err(e) => eprintln!("v1 test failed: {}", e),
    // }

    // Test trading engine v2
    // v2::test::testv2();

    // Test trading engine v3
    println!("Starting Trading Engine...");
    v3::test::testv3().await;
    println!("Trading Engine shut down.");
}
