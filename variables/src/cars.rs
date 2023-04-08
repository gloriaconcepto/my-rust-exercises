pub struct Car<'a>{

pub mpg:&'a u32,
pub color:&'a String,
pub top_speed:&'a u32,
}

pub struct Motor_Cycle<'a>{

    pub mpg:&'a u32,
    pub color:&'a String,
    pub top_speed:&'a u32,
}

pub trait Vechicles<'a>{
     fn set_mpg(&mut self,new_mpg:& 'a u32);
     fn set_color(&mut self,new_color:& 'a String);
    fn set_top_speed(&mut self,new_speed:& 'a u32); 
}

 impl <'a> Vechicles <'a> for Car<'a>{
       fn set_mpg(&mut self,new_mpg:&'a u32){
self.mpg=new_mpg
  }

fn set_color(&mut self,new_color:&'a String){
    self.color=new_color
  }

 fn set_top_speed (&mut self,new_speed:&'a u32){
    self.mpg=new_speed
     }  
}

impl <'a> Vechicles <'a> for Motor_Cycle<'a>{
    fn set_mpg(&mut self,new_mpg:&'a u32){
self.mpg=new_mpg
}

fn set_color(&mut self,new_color:&'a String){
 self.color=new_color
}

fn set_top_speed (&mut self,new_speed:&'a u32){
 self.mpg=new_speed
  }  
}
// impl <'a> Car<'a>{

//   pub fn set_mpg(&mut self,new_mpg:&'a u32){
// self.mpg=new_mpg
//   }

// pub fn set_color(&mut self,new_color:&'a String){
//     self.color=new_color
//   }

// pub fn set_top_speed (&mut self,new_speed:&'a u32){
//     self.mpg=new_speed
//      }  
// }


