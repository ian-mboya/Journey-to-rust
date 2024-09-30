

// in this i explored mutability and immutability in rust


fn main() {
  let x = 5;
  println!("The value of x is: {x}");

  let mut x = 56;
  println!("The value is x: {x}");

  x = 76;
  println!("The value of x: {x}");



  let mut y = 4;
  println!("The value of y is: {y}");
 
  y = 32;

  println!("The value of y is: {y}");
 

 const DAY_MINUTES: u32 = 60 * 24;

 println!("Minutes in a day are: {DAY_MINUTES} minutes");


}



