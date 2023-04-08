pub enum Shape{
    triangle, square, pentagon, octagon
}

impl Shape {
    pub fn corners(self)->u8{
        match self{
            Shape::triangle=>3,
            Shape::square=>4,
            Shape::pentagon=>5,
            Shape::octagon=>6,
        }
        
    }
}