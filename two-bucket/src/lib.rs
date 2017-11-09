use std::collections::HashSet;

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
    let initial_state = State::init(capacity_1, capacity_2, *start_bucket);
    let states: HashSet<(u8, u8)> = [(0, 0), (capacity_1, 0), (0, capacity_2)]
        .iter()
        .cloned()
        .collect();

    let (final_state, moves) = if check(initial_state, goal) {
        (initial_state, 0)
    } else {
        proceed(initial_state, states, goal).unwrap()
    };

    to_stats(final_state, goal, moves + 1)
}

fn to_stats(final_state: State, goal: u8, moves: u8) -> BucketStats {
    let (goal_bucket, other_bucket) = if final_state.actual1 == goal {
        (Bucket::One, final_state.actual2)
    } else {
        (Bucket::Two, final_state.actual1)
    };
    BucketStats {
        moves: moves,
        goal_bucket: goal_bucket,
        other_bucket: other_bucket,
    }
}

#[derive(Copy, Clone)]
struct State {
    capacity1: u8,
    actual1: u8,
    capacity2: u8,
    actual2: u8,
}

impl State {
    pub fn init(c1: u8, c2: u8, start_bucket: Bucket) -> State {
        let (a1, a2) = match start_bucket {
            Bucket::One => (c1, 0),
            Bucket::Two => (0, c2),
        };

        State {
            capacity1: c1,
            actual1: a1,
            capacity2: c2,
            actual2: a2,
        }
    }

    pub fn next(&self) -> Vec<State> {
        vec![
            fill_one(*self),
            fill_two(*self),
            pour_one_to_two(*self),
            pour_two_to_one(*self),
            empty_one(*self),
            empty_two(*self),
        ]
    }
}

fn check(state: State, goal: u8) -> bool {
    goal == state.actual1 || goal == state.actual2
}

// See https://math.stackexchange.com/a/649161/479983 for the
// intuition of the breadth-first algorithm.
fn proceed(
    initial_state: State,
    states: HashSet<(u8, u8)>,
    goal: u8,
) -> Option<(State, u8)> {
    let mut states = states;
    let mut current_layer: Vec<State> = vec![initial_state];
    let mut nlayers = 1;
    while !current_layer.is_empty() {
        let mut found: Option<(State, u8)> = None;
        // Using scan instead of inspect to abort the iteration as
        // soon as the solution is found.
        let next_layer: Vec<State> = current_layer
            .iter()
            .flat_map(|&state| state.next())
            .filter(|state| states.insert((state.actual1, state.actual2)))
            .scan(0, |_, state| if check(state, goal) {
                found = Some((state, nlayers));
                None
            } else {
                Some(state)
            })
            .collect();

        if found.is_some() {
            return found;
        }
        nlayers += 1;
        current_layer = next_layer;
    }

    None
}

fn fill_one(mut state: State) -> State {
    state.actual1 = state.capacity1;
    state
}

fn fill_two(mut state: State) -> State {
    state.actual2 = state.capacity2;
    state
}

fn pour_one_to_two(mut state: State) -> State {
    let diff = u8::min(state.actual1, state.capacity2 - state.actual2);
    state.actual1 -= diff;
    state.actual2 += diff;
    state
}

fn pour_two_to_one(mut state: State) -> State {
    let diff = u8::min(state.actual2, state.capacity1 - state.actual1);
    state.actual1 += diff;
    state.actual2 -= diff;
    state
}

fn empty_one(mut state: State) -> State {
    state.actual1 = 0;
    state
}

fn empty_two(mut state: State) -> State {
    state.actual2 = 0;
    state
}
