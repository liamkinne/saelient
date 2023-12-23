use saelient::prelude::*;
use saelient::slot::SAEaa01;

fn main() {
    let slot = SAEaa01::new(20.0).unwrap();

    // When the `std` feature is enabled, you can `Display` the value of a slot with it's units.
    println!("Output: {}", slot);
}
