pub fn is_armstrong_number(num: u32) -> bool {
    let s = format!("{num}");
    let l = s.len();
    s.chars()
        .map(|c| c.to_digit(10).unwrap().pow(l as u32))
        .try_fold(0u32, u32::checked_add)
        == Some(num)
}


fn main() {
    let inputs: Vec<u32> = vec![0u32, 5u32, 10u32, 153u32, 100u32];
    let mut out: Vec<String> = Vec::new();
    for &x in inputs.iter() {
        out.push(if is_armstrong_number(x) { "true".to_string() } else { "false".to_string() });
    }
    println!("{{\"out\": [{}]}}", out.join(","));
}
