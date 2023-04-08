pub fn basics(){
// question_one();
// question_two();
// let mut text:String="Hello".to_owned();
// concat_string(&mut text);
// let num:i32=26;
// control_flow(num);
let mut vec_data=vec![1,3,5,7];
let result= question_vec(&mut vec_data);
print!("{}",result);
vec_data.push(15);
println!("vector {:?} ",vec_data);
let num:i8=4;
let ans=add_two(&num);
println!("add two{}",ans);
println!("number{}",num);
}
// Create a function that takes one argument called val that is of type Vec with the values: 1,3,5,7. 
// Inside of this function check the first value of the vector and see if it is equal to one. 
// If the value is equal to one, then return true, otherwise return false. 
// Next add the value 15 to the vector. 
// Print out the vector to confirm your result
fn question_vec(val:&mut Vec<i32>)->bool{
 let first_num=val[0];
 if(first_num==1){
  return true;
 }else{
  return false;
 } 
}
fn add_two(num: &i8)->i8{
  num + 2
}
fn question_one(){
    let val1:i32=5;
    let val2:i32=2;
    let ans=val1 % val2;
    println!("the modulus between {} and {} is {}",val1 ,val2,ans)
}

fn question_two(){
  let mut data=vec![2,4,6,8,10];
  for num in data.iter(){
    println!("iterate {}",num)
  }
  data.pop();
  data.push(12);

  println!("new modified data {:?}",data)
}

fn concat_string( word:&mut String){
let base_string:&str="World";
word.push_str(base_string);

println!(" concat string {}",word)
}

fn control_flow(num:i32){
    match num{
        x if x==1 =>println!("Value is one"),
        x if x>50=>println!("Greater than 50"),
        x  if x<25=>println!("Less than 25"),
        x  if x>25 && x<50=>println!("greater than 25 but less than 50"),
        _ =>println!("none of the value")
    }
}

