use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = env::var("SECRET")?;
    println!("secret: {}", secret);
    Ok(())
}
