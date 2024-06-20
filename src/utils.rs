#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Add<IVec2> for IVec2 {
    type Output = IVec2;
    fn add(self, rhs: IVec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign<IVec2> for IVec2 {
    fn add_assign(&mut self, rhs: IVec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
