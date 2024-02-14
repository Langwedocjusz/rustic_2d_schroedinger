//Utility to make creating potential functions easier
//Currenlty supports defining infinite potential wells
//with shapes defined as union of multiple rectangles and circles

pub enum PotentialValue{
    Finite(f64),
    Infinite,
}

#[derive(Debug)]
pub enum Shape{
    Circle{pos: (f64, f64), rad: f64},
    AABB{min: (f64, f64), max: (f64, f64)},
}

impl Shape{
    pub fn contains(&self, x: f64, y: f64) -> bool {
        match self {
            Shape::Circle{pos, rad} => {
                let dx = x - pos.0;
                let dy = y - pos.1;

                dx*dx + dy*dy < rad*rad
            }

            Shape::AABB{min, max} => {
                let in_x = min.0 < x && x < max.0;
                let in_y = min.1 < y && y < max.1;

                in_x && in_y
            }
        }
    }
}

pub struct InfiniteWell{
    pub shapes: Vec<Shape>,
}

impl InfiniteWell{

    pub fn new() -> InfiniteWell {
        InfiniteWell{shapes: vec![]}
    }

    pub fn add_shape(&mut self, shape: Shape) -> &mut Self {
        self.shapes.push(shape);
        self
    }

    pub fn get_value(&self, x: f64, y: f64) -> PotentialValue {
        let mut value = PotentialValue::Finite(0.0);

            for shape in &self.shapes {
                if shape.contains(x, y) {
                    value = PotentialValue::Infinite;
                }
            }
    
            value
    }
}
