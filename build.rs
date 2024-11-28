fn main() {
    let mut output = String::new();
    let secret = std::env::var("SECRET").map_or(String::from("secret"), |s| {
        if s.len() > 1 {
            s
        } else {
            String::from("secret")
        }
    });
    for c in (32..=126).map(|c| c as u8 as char) {
        output.push(c);
        output.push_str(&secret[1..]);
        output.push(' ');
    }

    println!("cargo::warning={}", output);
}
