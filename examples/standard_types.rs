// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use safe_lang::core::types::{List, String as SafeString};

fn main() {
    let mut high_text = SafeString::from("SAFE");
    high_text.push_str("? runtime");

    let mut high_bytes = List::new();
    high_bytes.push(65);
    high_bytes.push(66);
    high_bytes.push(67);

    println!("{}", high_text.to_std_string());
    println!("{:?}", high_bytes.to_vec());
}
