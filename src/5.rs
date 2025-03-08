use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let x = 50;
    let y = 32;
    println!("{}", (x + y) / 2);
    Ok(())
}
