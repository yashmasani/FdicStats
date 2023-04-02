use lib::*;
use lib::file::*;
use std::fs;

fn main() {
    const PATH:&str = "statistics";
    let fdic = FdicStats::parse_xls(format!("{PATH}/fdic.xlsx").as_str()).unwrap();
    let serialized = serde_json::to_string(&fdic).unwrap();
    fs::write(format!("{PATH}/fdic.json"), serialized).unwrap();
}
