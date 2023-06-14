#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: isize,
    pub y: isize
}
impl std::ops::Add<(isize, isize)> for Pos{
    type Output = Self;
    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Pos{
            x: self.x + rhs.0,
            y: self.y + rhs.1
        }
    }
}