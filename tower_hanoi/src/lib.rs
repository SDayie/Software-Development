// lib.rs — re-exports the two solvers so tests can reach them.


// ============================================================
// ALGORITHM 1: RECURSIVE
// ============================================================

fn tower_of_hanoi(
    n: u8,
    source_pole: u8,
    auxiliary_pole: u8,
    destination_pole: u8,
    moves: &mut Vec<(u8, u8)>,
) {
    if n == 1 {
        moves.push((source_pole, destination_pole));
        return;
    }
    tower_of_hanoi(n - 1, source_pole, destination_pole, auxiliary_pole, moves);
    moves.push((source_pole, destination_pole));
    tower_of_hanoi(n - 1, auxiliary_pole, source_pole, destination_pole, moves);
}

pub fn solve_recursive(n: u8) -> Vec<(u8, u8)> {
    let mut moves = Vec::new();
    if n > 0 {
        tower_of_hanoi(n, 1, 2, 3, &mut moves);
    }
    moves
}

// ============================================================
// ALGORITHM 2: ITERATIVE
// ============================================================

pub fn solve_iterative(n: u8) -> Vec<(u8, u8)> {
    if n == 0 {
        return Vec::new();
    }
    let mut stacks: [Vec<u8>; 3] = [vec![], vec![], vec![]];
    for disk in (1..=n).rev() {
        stacks[0].push(disk);
    }
    let mut moves: Vec<(u8, u8)> = Vec::new();
    let total_moves = (1u64 << n) - 1;
    let mut smallest_disk_pos: usize = 0;

    for step in 0..total_moves {
        if step % 2 == 0 {
            let next_pos = if n % 2 == 1 {
                (smallest_disk_pos + 2) % 3
            } else {
                (smallest_disk_pos + 1) % 3
            };
            let disk = stacks[smallest_disk_pos].pop().unwrap();
            stacks[next_pos].push(disk);
            moves.push((smallest_disk_pos as u8 + 1, next_pos as u8 + 1));
            smallest_disk_pos = next_pos;
        } else {
            let candidates: Vec<usize> = (0..3).filter(|&s| s != smallest_disk_pos).collect();
            let s0 = candidates[0];
            let s1 = candidates[1];
            let (from, to) = match (stacks[s0].last(), stacks[s1].last()) {
                (None, _) => (s1, s0),
                (_, None) => (s0, s1),
                (Some(&a), Some(&b)) if a < b => (s0, s1),
                _ => (s1, s0),
            };
            let disk = stacks[from].pop().unwrap();
            stacks[to].push(disk);
            moves.push((from as u8 + 1, to as u8 + 1));
        }
    }
    moves
}
