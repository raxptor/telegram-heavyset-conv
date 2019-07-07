use super::conv;
use std::fs;

#[test]
fn test() {        
    let data = fs::read_to_string("data/heavyset-export.csv").expect("could not open file!");
    conv::convert(&data).expect("Could not converte");
}
