use std::io;



fn main() {
    println!("Hello there, convert Celcius degrees to Farenheits shall we");
    println!("Please input a number");
   

   let mut temp_input = String::new();

   io::stdin()
      .read_line(&mut temp_input)
      .expect("Ooops please input a number");
 

   let temp_input: f64 = temp_input.trim().parse().expect("Please enter a valid number");  
   let final_temp = (temp_input * 9.0 / 5.0) + 32.0;


  println!("The final temperature is: {:.2} Farenheits", final_temp);    
}
