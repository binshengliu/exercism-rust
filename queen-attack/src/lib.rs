pub struct ChessPosition {
    x: i8,
    y: i8,
}

impl ChessPosition {
    pub fn new(x: i8, y: i8) -> Result<ChessPosition, ()> {
        if x < 0 || x > 7 || y < 0 || y > 7 {
            return Err(());
        }

        Ok(ChessPosition { x: x, y: y })
    }
}

pub struct Queen {
    pos: ChessPosition,
}

impl Queen {
    pub fn new(pos: ChessPosition) -> Queen {
        Queen { pos: pos }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.pos.x == other.pos.x || self.pos.y == other.pos.y {
            return true;
        }

        if (self.pos.x - other.pos.x).abs() == (self.pos.y - other.pos.y).abs() {
            return true;
        }

        false
    }
}
