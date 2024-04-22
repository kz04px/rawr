#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Score(pub i32, pub i32);

impl Score {
    #[must_use]
    pub fn mg(&self) -> i32 {
        self.0
    }

    #[must_use]
    pub fn eg(&self) -> i32 {
        self.1
    }
}

impl Default for Score {
    #[must_use]
    fn default() -> Self {
        Score(0, 0)
    }
}

impl std::ops::Add for Score {
    type Output = Self;

    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Score {
    type Output = Self;

    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Mul<i32> for Score {
    type Output = Self;

    #[must_use]
    fn mul(self, rhs: i32) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl std::ops::Mul<Score> for i32 {
    type Output = Score;

    #[must_use]
    fn mul(self, rhs: Score) -> Score {
        Score(self * rhs.0, self * rhs.1)
    }
}

impl std::ops::AddAssign for Score {
    #[must_use]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl std::ops::SubAssign for Score {
    #[must_use]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

#[cfg(test)]
mod tests {
    use crate::search::score::Score;

    #[test]
    fn default() {
        assert_eq!(Score::default(), Score(0, 0));
    }

    #[test]
    fn mg_eg() {
        assert_eq!(Score::default().mg(), 0);
        assert_eq!(Score::default().eg(), 0);
        assert_eq!(Score(1, 2).mg(), 1);
        assert_eq!(Score(1, 2).eg(), 2);
    }

    #[test]
    fn add() {
        assert_eq!(Score(1, 2) + Score(3, 4), Score(4, 6));
        assert_eq!(Score(-10, -5) + Score(3, 4), Score(-7, -1));
    }

    #[test]
    fn sub() {
        assert_eq!(Score(1, 2) - Score(4, 3), Score(-3, -1));
        assert_eq!(Score(-10, -5) - Score(4, 3), Score(-14, -8));
    }

    #[test]
    fn mul() {
        assert_eq!(Score(1, 2) * 2, Score(2, 4));
        assert_eq!(2 * Score(1, 2), Score(2, 4));
        assert_eq!(Score(-1, -2) * 2, Score(-2, -4));
        assert_eq!(2 * Score(-1, -2), Score(-2, -4));
    }

    #[test]
    fn add_assign() {
        let mut score = Score(1, 2);
        score += Score(3, 4);
        assert_eq!(score, Score(4, 6));
    }

    #[test]
    fn sub_assign() {
        let mut score = Score(1, 2);
        score -= Score(4, 3);
        assert_eq!(score, Score(-3, -1));
    }
}
