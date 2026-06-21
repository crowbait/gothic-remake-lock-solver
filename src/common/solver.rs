use crate::data::{LinkState, LockData};
use std::collections::{HashSet, VecDeque};


const TARGET_SLOT: u8 = 3;
const PLATE_SIZE: u8 = 7;
const MAX_DEPTH: u8 = 100;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    None,
    Left,
    Right,
}
impl Direction {
    pub fn reverse(self) -> Direction {
        match self {
            Direction::None => Direction::None,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
    pub fn position_effect(self) -> i16 {
        match self {
            Direction::None => 0,
            Direction::Left => 1,
            Direction::Right => -1,
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            Direction::None => " - ",
            Direction::Left => "<  ",
            Direction::Right => "  >",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Move {
    pub plate: u8,
    pub direction: Direction,
}

#[derive(Clone, Debug)]
struct QueuedStage {
    pub new_pin_positions: Vec<u8>,
    pub node_index: usize,
    pub depth: usize,
}

#[derive(Clone, Debug)]
struct Node {
    pub parent: Option<usize>,
    pub mv: Option<Move>,
}

pub fn solve(lock: &LockData) -> Option<Vec<Move>> {
    let target_state = vec![TARGET_SLOT; lock.num_plates as usize];

    // state tracking
    let mut visited = HashSet::<u32>::new();
    let mut queue = VecDeque::<QueuedStage>::new();
    let mut nodes = Vec::<Node>::new();

    // prepare first move
    visited.insert(state_key(&lock.pin_positions));
    queue.push_back(QueuedStage {
        new_pin_positions: lock.pin_positions.clone(),
        node_index: 0,
        depth: 0,
    });
    nodes.push(Node {
        parent: None,
        mv: None,
    });

    // Work through queued stages until a solution is found or the queue is empty (= no solution).
    // This loop tests all plates in both direction for "solution found" before moving on to the next level.
    // The next level again tests all plates and directions, only this time taking each possible outcome of the
    // previous stage as the basis ("breadth-first search").
    while let Some(queued) = queue.pop_front() {
        // found solution
        if queued.new_pin_positions == target_state {
            return Some(reconstruct_path(queued.node_index, &nodes));
        }

        if queued.depth >= MAX_DEPTH as usize {
            continue;
        }

        // for all plates ...
        for plate in 0..lock.num_plates {
            // ...check both directions
            for direction in [Direction::Left, Direction::Right] {
                // tries to apply move (and get new position state), if not possible, skip move
                let Some(next_state) =
                    apply_move(lock, &queued.new_pin_positions, plate as usize, direction)
                else {
                    continue;
                };

                let key = state_key(&next_state);

                // if key (= this position) has already been seen, skip move
                if !visited.insert(key) {
                    continue;
                }

                // save this move to nodes list
                let child_node_index = nodes.len();
                nodes.push(Node {
                    parent: Some(queued.node_index),
                    mv: Some(Move { plate, direction }),
                });

                // queue this move as a possible base state for the next search depth
                queue.push_back(QueuedStage {
                    new_pin_positions: next_state,
                    node_index: child_node_index,
                    depth: queued.depth + 1,
                });
            }
        }
    }

    None
}

/** Tries to apply a given move, taking plate links into account, returning the new pin-positions state (or None). */
fn apply_move(
    lock: &LockData,
    pin_positions: &[u8],
    plate: usize,
    direction: Direction,
) -> Option<Vec<u8>> {
    let num_plates = lock.num_plates as usize;

    // vector keeping track of "where did each plate move"
    let mut deltas = vec![Direction::None; num_plates];
    deltas[plate] = direction;

    // apply linked plates to deltas
    for linked_plate in 0..num_plates {
        match lock.links[plate][linked_plate] {
            LinkState::Same => {
                deltas[linked_plate] = direction;
            }
            LinkState::Opposite => {
                deltas[linked_plate] = direction.reverse();
            }
            LinkState::Unlinked => {}
        }
    }

    let mut next_positions = pin_positions.to_vec();

    // get new plate positions
    for plate in 0..num_plates {
        let new_pos = pin_positions[plate] as i16 + deltas[plate].position_effect();

        // if any plate has invalid new state, return
        if new_pos < 0 || new_pos >= PLATE_SIZE as i16 {
            return None;
        }

        next_positions[plate] = new_pos as u8;
    }

    Some(next_positions)
}

/** Traversed a list of nodes backwards, collecting the moves taken. */
fn reconstruct_path(mut node_index: usize, nodes: &[Node]) -> Vec<Move> {
    let mut path = Vec::<Move>::new();

    while let Some(mv) = nodes[node_index].mv.clone() {
        path.push(mv);
        node_index = nodes[node_index].parent.expect("move node without parent");
    }

    path.reverse();
    path
}

/** Turns a variable element count (<= 10) vector of (int <= 7) into a "concatenated" 32-bit integer (3bits / element). */
fn state_key(positions: &[u8]) -> u32 {
    positions
        .iter()
        .fold(0u32, |sum, &cur| (sum << 3) | cur as u32)
}
