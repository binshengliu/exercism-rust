extern crate num;

use num::Num;

pub struct Triangle<T>
where
    T: Num + PartialOrd + Copy,
{
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Num + PartialOrd + Copy,
{
    pub fn build(sides: [T; 3]) -> Result<Triangle<T>, String> {
        if Triangle::is_valid(&sides) {
            Ok(Triangle { sides })
        } else {
            Err(format!("Invalid input"))
        }
    }

    fn is_valid(sides: &[T; 3]) -> bool {
        sides[0] + sides[1] > sides[2] && sides[0] + sides[2] > sides[1] &&
            sides[1] + sides[2] > sides[0]
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[0] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] || self.sides[0] == self.sides[2] ||
            self.sides[1] == self.sides[2]
    }
}
