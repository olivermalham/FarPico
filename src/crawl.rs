// use crate::hal::{Gpio, HalComponent, HalFuncs};
// use serde::Serialize;
//
// #[derive(Serialize)]
// pub struct CrawlHal {
//
// }
//
// impl HalFuncs for CrawlHal {
//
//     fn to_json(&self) -> String {
//         // Build JSON state string here
//         serde_json::to_string(self).ok().unwrap()
//     }
//
//     fn dispatch(&self, action: &str) {
//         todo!()
//     }
// }
//
// fn build_hal() -> CrawlHal {
//     println!("Building Crawl HAL...");
//     CrawlHal {}
// }
