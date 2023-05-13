
pub fn compu_sqr_sum(num:u32)->u32{

let mut result:u32=0;
   for i in 1..=num{
      result=i*i+result
   }
result
}

pub fn sqr_sum_total(num:u32)->u32{

    let mut sum:u32=0;
       for i in 1..=num{
         sum+=i
       }
       sum.pow(2)

    }

pub fn multiple_0f_five_or_three(mut num:u32)->u32{
   
     let mut sum:u32=0;
      while num !=0{
        if num %3==0 || num%5==0{
           sum=sum+num;
        
        }
        num=num-1;
       
      }
     
   sum
        }
        enum Return {
            U32(u32),
            String(String),
        }
       

  pub fn total_production(hour:u32,speed:u32)->u32{
   let num_car_per_hr:u32=221;
   let car_per_speed:u32=num_car_per_hr*speed;
   let total_car:u32=car_per_speed*hour;
   match speed {
       1..=4=>{
    
     (100/100)*total_car
       },
       5..=8=>{
        (90/100)*total_car
       },
       9..=10=>{
        (70/100)*total_car
       },
       _ => 0
   }

   } 

  pub fn Cars_produced_per_minutes(hour:u32,speed:u32)->u32{
    let num_car_per_min:u32=221/60;
    let car_per_speed:u32=num_car_per_min*speed;
    let total_car:u32=car_per_speed*hour;
    match speed {
        1..=4=>{
     
      (100/100)*total_car
        },
        5..=8=>{
         (90/100)*total_car
        },
        9..=10=>{
         (70/100)*total_car
        },
        _ => 0
    }
   }

  pub fn is_palindrome(word: String) -> bool {
    let rev_word = word.chars().rev().collect::<String>();
    word == rev_word
}

pub fn pythagorean_triple(a:i64,b:i64,c:i64)->bool{
    if(a<b&&a<c&&b<c&&(a+b+c==1000)){
        return true;
    }else {
        return  false;
    }
    
}

pub fn can_see_movie(age: i32, permission: bool) -> bool {
 let min_age_17:i32=17;
 let min_age_13:i32=13; 
  if min_age_17>=age{
    return true;
  }else if min_age_13>=age&&permission==true {
      return true;
  }else{
    return false;
  }
    
}
