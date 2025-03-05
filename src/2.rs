  fn main() {
    let mut x = 5;
    let y = 10;
  
    println!("{}", add(x, y));
  
    x = sub(x, y);
    println!("{}", x);
  
    x = mul(x, y);
    println!("{}", x);
  
    x = div(x, y);
    println!("{}", x);
  }
  
  fn add(a: i32, b: i32) -> i32 {
      a + b
  }
  
  fn sub(a: i32, b: i32) -> i32 {
      a - b
  }
  
  fn mul(a: i32, b: i32) -> i32 {
      a * b
  }
  
  fn div(a: i32, b: i32) -> i32 {
      a / b
  }
  