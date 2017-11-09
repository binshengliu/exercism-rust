use std::collections::HashSet;
use std::cmp::Ordering::*;

#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> BucketStats {
    let actions: Vec<Box<fn(State) -> State>> = vec![
        Box::new(fill),
        Box::new(pour_one_to_two),
        Box::new(pour_two_to_one),
        Box::new(empty_one),
        Box::new(empty_two),
    ];
    let state = State {
        capacity1: capacity_1,
        actual1: 0,
        capacity2: capacity_2,
        actual2: 0,
        fill: *start_bucket,
    };
    let mut solver = Solver {
        states: HashSet::new(),
        moves: 0,
        goal: goal,
    };
    println!("Init: {:?}", state);
    let result = solver.proceed(state, &actions).unwrap();
    let (goal_bucket, other_bucket) = if result.actual1 == goal {
        (Bucket::One, result.actual2)
    } else {
        (Bucket::Two, result.actual1)
    };
    BucketStats {
        moves: solver.moves,
        goal_bucket: goal_bucket,
        other_bucket: other_bucket,
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct State {
    capacity1: u8,
    actual1: u8,
    capacity2: u8,
    actual2: u8,
    fill: Bucket,
}


fn check(goal: u8, state: State) -> bool {
    goal == state.actual1 || goal == state.actual2
}

struct Solver {
    states: HashSet<State>,
    moves: u8,
    goal: u8,
}

impl Solver {
    pub fn proceed(
        &mut self,
        state: State,
        actions: &Vec<Box<fn(State) -> State>>,
    ) -> Option<State> {
        for action in actions.iter() {
            let new = action(state);

            if self.states.contains(&new) {
                println!("Repeated state. Skip.");
                continue;
            }

            self.moves += 1;
            self.states.insert(new);
            if check(self.goal, new) {
                return Some(new);
            }
            if let Some(result) = self.proceed(new, actions) {
                return Some(result);
            }
            self.moves -= 1;
        }

        None
    }
}



fn fill(mut state: State) -> State {
    match state.fill {
        Bucket::One => state.actual1 = state.capacity1,
        Bucket::Two => state.actual2 = state.capacity2,
    };
    println!("After Fill {:?}", state);
    state
}

fn pour_one_to_two(mut state: State) -> State {
    let diff = state.capacity2 - state.actual2;
    match state.actual1.cmp(&diff) {
        Less | Equal => {
            state.actual2 += state.actual1;
            state.actual1 = 0;
        }
        Greater => {
            state.actual2 += diff;
            state.actual1 -= diff;
        }
    }
    println!("After Pour 1 -> 2 {:?}", state);
    state
}

fn pour_two_to_one(mut state: State) -> State {
    let diff = state.capacity1 - state.actual1;
    match state.actual2.cmp(&diff) {
        Less | Equal => {
            state.actual1 += state.actual2;
            state.actual2 = 0;
        }
        Greater => {
            state.actual1 += diff;
            state.actual2 -= diff;
        }
    }
    println!("After Pour 2 -> 1 {:?}", state);
    state
}

fn empty_one(mut state: State) -> State {
    state.actual1 = 0;
    println!("After Empty 1 {:?}", state);
    state
}

fn empty_two(mut state: State) -> State {
    state.actual2 = 0;
    println!("After Empty 2 {:?}", state);
    state
}
