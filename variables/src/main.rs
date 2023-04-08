use basics::basics;
use structs::Square;
use cars::{Car, Vechicles};
use shape::Shape;
use std::io;

mod basics;
mod structs;
mod cars;
mod shape;
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

}
