use crate::hal::Hal;

pub fn build_hal() -> Hal {
    println!("Building Crawl HAL...");
    let hal = Hal::new();
    return hal;
}
