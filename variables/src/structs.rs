pub struct Square{
    pub width:u32,
    pub height:u32
}

impl Square{
    pub fn area_cal(&self)->u32{
      self.width * self.height
    }
    pub fn what_width(&self)->u32{
    self.width
    }
}