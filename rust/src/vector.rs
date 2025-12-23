pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn add(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn subtract(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    pub fn multiply(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
    pub fn divide(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn normalize(&self) -> Vec2 {
        let length = self.length();
        Vec2 {
            x: self.x / length,
            y: self.y / length,
        }
    }
    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
    pub fn cross(&self, other: &Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }
    pub fn angle(&self, other: &Vec2) -> f32 {
        self.dot(other) / (self.length() * other.length())
    }
    pub fn distance(&self, other: &Vec2) -> f32 {
        (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)
    }
    pub fn lerp(&self, other: &Vec2, t: f32) -> Vec2 {
        Vec2 {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }
    pub fn nlerp(&self, other: &Vec2, t: f32) -> Vec2 {
        self.lerp(other, t).normalize()
    }
    pub fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
    pub fn from_string(string: &str) -> Self {
        let parts = string.split(",").collect::<Vec<&str>>();
        Self {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
        }
    }
    pub fn from_vec2(vec2: &Vec2) -> Self {
        Self { x: vec2.x, y: vec2.y }
    }
}