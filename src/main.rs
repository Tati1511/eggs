//use std;

use std::io;

fn main() {
  println!("Count the eggs!");
  println!("Please imput number of guests");
  let mut guests = String::new();
  // this reads line from user
  io::stdin()
     .read_line( &mut guests)
     .expect("Failed to read line");
     //println!("You invited:{} guests",guests);

     println!("Please imput number of eggs per guests");
     let mut eggs_guests = String::new();
     io::stdin()
        .read_line( &mut eggs_guests)
        .expect("Failed to read line");
    
        //println!("You invited:{} guests",guests);
//y --eggs per person
  //let y = 2;
  // this converts string to text
  let guests : i32 = guests.trim().parse().unwrap();
  let eggs_guests : i32 = eggs_guests.trim().parse().unwrap();
  let c  = guests * eggs_guests;

  
  println!("Number of eggs needed {}", c);
  println!("eggs are delicious and awesome!");

}