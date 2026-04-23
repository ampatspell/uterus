#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Forward,
    Backward,
    Stop,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Limit {
    Forward,
    Backward,
}

pub fn get_inverse(limit: Limit) -> Direction {
    if limit == Limit::Forward {
        return Direction::Backward;
    }
    Direction::Forward
}
