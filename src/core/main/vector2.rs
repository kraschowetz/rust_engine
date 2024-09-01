#[allow(dead_code)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

#[allow(dead_code)]
impl Vector2 {
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(&self) -> Vector2 {
        let len: f32 = self.len();
        let v: Vector2 = Vector2 {
            x: self.x / len,
            y: self.y / len
        };

        return v
    }

    pub fn distance_to(&self, other: &Vector2) -> f32{
        (
            (self.x * self.x - other.x * other.x) + 
            (self.y * self.y - other.y * other.y) 
        ).sqrt()
    }
}

impl std::ops::Add<Vector2> for Vector2 {
    type Output = Self;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl std::ops::Sub<Vector2> for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl std::ops::Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl std::ops::Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl std::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}


//TODO:
//format :)
//copying & cloning :)
