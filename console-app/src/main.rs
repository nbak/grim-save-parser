mod input;

use input::{get_data, Input};
use parser::util::map_to_json;

fn main() {
    let Input {
        entity_type,
        source,
    } = get_data();
    let result = match map_to_json(&entity_type, &source[..]) {
        Ok(json) => json,
        Err(err) => err.to_string(),
    };
    println!("{}", result);
}
