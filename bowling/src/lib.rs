// use std::rc::{Rc, Weak};
// use std::cell::RefCell;

const MAX_PINS: u32 = 10;
const FRAMES_COUNT: usize = 10;

type Result<T> = std::result::Result<T, String>;

trait FrameStatus {
    fn roll(
        &mut self,
        frame_id: usize,
        index: usize,
        score: u32,
    ) -> Result<Box<FrameStatus>>;
    fn is_finished(&self) -> bool;
    fn get_index_and_count(&self) -> Result<(usize, u32)>;
    fn get_score(&self, score_table: &Vec<u32>) -> Result<u32> {
        if let Ok((index, count)) = self.get_index_and_count() {
            return Ok(
                (&score_table[index..(index + count as usize)]).iter().sum(),
            );
        }
        return Err("".to_string());
    }
}

struct Beginning;
impl FrameStatus for Beginning {
    fn roll(
        &mut self,
        frame_id: usize,
        index: usize,
        score: u32,
    ) -> Result<Box<FrameStatus>> {
        if score == MAX_PINS {
            if frame_id == FRAMES_COUNT - 1 {
                Ok(Box::new(TenthStrike {
                    index: index,
                    throws: 1,
                    second_throw_score: 0,
                }))
            } else {
                Ok(Box::new(Strike { index }))
            }
        } else {
            Ok(Box::new(FirstThrow { index, score }))
        }
    }

    fn is_finished(&self) -> bool {
        false
    }

    fn get_index_and_count(&self) -> Result<(usize, u32)> {
        Err("".to_string())
    }
}

struct FirstThrow {
    index: usize,
    score: u32,
}

impl FrameStatus for FirstThrow {
    fn roll(
        &mut self,
        frame_id: usize,
        _index: usize,
        score: u32,
    ) -> Result<Box<FrameStatus>> {
        if self.score + score > 10 {
            return Err("".to_string());
        }

        if self.score + score == 10 {
            if frame_id == FRAMES_COUNT - 1 {
                Ok(Box::new(TenthSpare {
                    index: self.index,
                    throws: 2,
                }))
            } else {
                Ok(Box::new(Spare { index: self.index }))
            }
        } else {
            Ok(Box::new(Open { index: self.index }))
        }
    }

    fn is_finished(&self) -> bool {
        false
    }

    fn get_index_and_count(&self) -> Result<(usize, u32)> {
        Err("".to_string())
    }
}

struct Open {
    index: usize,
}

impl FrameStatus for Open {
    fn roll(
        &mut self,
        _frame_id: usize,
        _index: usize,
        _score: u32,
    ) -> Result<Box<FrameStatus>> {
        Err("".to_string())
    }

    fn is_finished(&self) -> bool {
        true
    }

    fn get_index_and_count(&self) -> Result<(usize, u32)> {
        Ok((self.index, 2))
    }
}

struct Strike {
    index: usize,
}
impl FrameStatus for Strike {
    fn roll(
        &mut self,
        _frame_id: usize,
        _index: usize,
        _score: u32,
    ) -> Result<Box<FrameStatus>> {
        Err("".to_string())
    }

    fn is_finished(&self) -> bool {
        true
    }

    fn get_index_and_count(&self) -> Result<(usize, u32)> {
        Ok((self.index, 3))
    }
}

struct Spare {
    index: usize,
}

impl FrameStatus for Spare {
    fn roll(
        &mut self,
        _frame_id: usize,
        _index: usize,
        _score: u32,
    ) -> Result<Box<FrameStatus>> {
        Err("".to_string())
    }

    fn is_finished(&self) -> bool {
        true
    }

    fn get_index_and_count(&self) -> Result<(usize, u32)> {
        Ok((self.index, 3))
    }
}

// Implementation for the tenth frame
struct TenthStrike {
    index: usize,
    throws: usize,
    second_throw_score: u32,
}

impl FrameStatus for TenthStrike {
    fn roll(
        &mut self,
        _frame_id: usize,
        _index: usize,
        score: u32,
    ) -> Result<Box<FrameStatus>> {
        if self.throws == 3 {
            return Err("Frame already finished".to_string());
        }

        if self.throws == 2 {
            if self.second_throw_score != MAX_PINS &&
                self.second_throw_score + score > MAX_PINS
            {
                return Err("Impossible rolling".to_string());
            } else {
                return Ok(Box::new(TenthStrike {
                    index: self.index,
                    throws: self.throws + 1,
                    second_throw_score: self.second_throw_score,
                }));
            }
        }

        if self.throws == 1 {
            return Ok(Box::new(TenthStrike {
                index: self.index,
                throws: self.throws + 1,
                second_throw_score: score,
            }));
        }

        return Err("".to_string());
    }

    fn is_finished(&self) -> bool {
        self.throws == 3
    }

    fn get_index_and_count(&self) -> Result<(usize, u32)> {
        Ok((self.index, 3))
    }
}

struct TenthSpare {
    index: usize,
    throws: usize,
}

impl FrameStatus for TenthSpare {
    fn roll(
        &mut self,
        _frame_id: usize,
        _index: usize,
        _score: u32,
    ) -> Result<Box<FrameStatus>> {
        if self.throws >= 3 {
            Err("".to_string())
        } else {
            Ok(Box::new(TenthSpare {
                index: self.index,
                throws: self.throws + 1,
            }))
        }
    }

    fn is_finished(&self) -> bool {
        self.throws == 3
    }

    fn get_index_and_count(&self) -> Result<(usize, u32)> {
        Ok((self.index, 3))
    }
}

pub struct BowlingGame {
    current: Option<Box<FrameStatus>>,
    frames: Vec<Box<FrameStatus>>,
    throws: Vec<u32>,
}

impl BowlingGame {
    pub fn new() -> BowlingGame {
        BowlingGame {
            current: None,
            frames: Vec::with_capacity(FRAMES_COUNT),
            throws: Vec::new(),
        }
    }

    pub fn roll(&mut self, pins: u32) -> Result<()> {
        if pins > MAX_PINS {
            return Err(format!("Invalid pins {}", pins));
        }

        if self.frames.len() < FRAMES_COUNT - 1 {
            self.roll_first_nine(pins)
        } else if self.frames.len() == FRAMES_COUNT - 1 {
            self.roll_tenth(pins)
        } else {
            Err("".to_string())
        }
    }

    fn roll_tenth(&mut self, pins: u32) -> Result<()> {
        let index = self.throws.len();
        self.throws.push(pins);

        let finished = {
            let mut frame = self.current.get_or_insert(Box::new(Beginning {}));
            *frame = frame.roll(9, index, pins)?;
            frame.is_finished()
        };

        if finished {
            self.frames.push(self.current.take().unwrap());
        }

        Ok(())
    }

    fn roll_first_nine(&mut self, pins: u32) -> Result<()> {
        let index = self.throws.len();
        self.throws.push(pins);

        let finished = {
            let frame_id = self.frames.len();
            let mut frame = self.current.get_or_insert(Box::new(Beginning {}));
            *frame = frame.roll(frame_id, index, pins)?;
            frame.is_finished()
        };

        if finished {
            self.frames.push(self.current.take().unwrap());
        }

        Ok(())
    }

    pub fn score(&self) -> Result<u32> {
        println!("frame len {}", self.frames.len());

        if self.frames.len() < FRAMES_COUNT {
            return Err("".to_string());
        }

        let mut score = 0;
        for f in self.frames.iter() {
            let frame_score = f.get_score(&self.throws).unwrap();
            score += frame_score;
        }

        Ok(score)
    }
}
