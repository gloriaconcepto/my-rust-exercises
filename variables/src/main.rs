use basics::basics;
use structs::Square;
use cars::{Car, Vechicles};
use shape::Shape;
use std::io;

use crate::difference_sqr::{compu_sqr_sum, sqr_sum_total, multiple_0f_five_or_three};

mod basics;
mod structs;
mod cars;
mod shape;
mod difference_sqr;
fn main() {
  //basics()
let squ_one=Square{width:2,height:4};
let squ_two=Square{width:2,height:3};
// println!("square-one {}",squ_one.area_cal());
// println!("square-two {}",squ_two.area_cal());
// println!("square-one {}",squ_one.what_width());
// println!("square-two {}",squ_two.what_width())
let car_color=String::from("RED");
let mut car=Car{mpg:&3,color:&car_color,top_speed:&2};
println!("car color {}",car.color);
// println!("car speed {}",car.top_speed);
// println!("car mpg {}",car.mpg);
let  new_color=String::from("Blue");
let  new_mp:u32=23;
let  new_top_speed:u32=900;
 car.set_color(&new_color);
// car.set_mpg(&new_mp);
// car.set_top_speed(&new_top_speed);
 println!("car color {}",car.color);
// println!("car speed {}",car.top_speed);
// println!("car mpg {}",car.mpg);
let triangle=Shape::triangle;
// println!("triangle {}",triangle.corners())
println!("Please a vlaid input number");
   let mut input_num=String::new();
   io::stdin().read_line(&mut input_num).expect("Failed to read input");
   let number:u32=input_num.trim().parse().expect("invalid input");
   let sqr_num=compu_sqr_sum(number
  );
 let sqr_num_2=sqr_sum_total(number);
  println!("{} {}",sqr_num,sqr_num_2);

let answer=sqr_num_2-sqr_num;


let multiple=multiple_0f_five_or_three(number
);
println!("the answer is {}",multiple);

}
