// ============================================================
// Tests for Tower of Hanoi 
// ============================================================
// These tests are like a referee checking the rules:
//   1. The right NUMBER of moves (always 2^n - 1)
//   2. Every move is LEGAL (no big disk on a small disk)
//   3. At the END all disks are on stack 3 (the right stack)
//   4. Both algorithms give IDENTICAL results
// ============================================================

use hanoi::{solve_iterative, solve_recursive};

fn verify_solution(n: u8, moves: &[(u8, u8)]) {
    // Check count: Tower of Hanoi always takes exactly 2^n - 1 moves.
    let expected_count = if n == 0 { 0 } else { (1u64 << n) - 1 } as usize;
    assert_eq!(
        moves.len(),
        expected_count,
        "Wrong number of moves for n={n}: got {}, expected {expected_count}",
        moves.len()
    );

    // Simulate the moves on actual stacks, checking legality as we go.
    // Start: all disks on stack 1 (index 0), biggest at the bottom.
    let mut stacks: [Vec<u8>; 3] = [vec![], vec![], vec![]];
    for disk in (1..=n).rev() {
        stacks[0].push(disk); // disk n is biggest, disk 1 is smallest
    }

    for (step, &(from, to)) in moves.iter().enumerate() {
        // Stacks are labelled 1-3 in moves but 0-2 in our array
        let f = (from - 1) as usize;
        let t = (to - 1) as usize;

        // The source stack must not be empty
        assert!(
            !stacks[f].is_empty(),
            "Step {step}: tried to move from empty stack {from}"
        );

        let moving_disk = *stacks[f].last().unwrap();

        // The destination must not have a smaller disk on top
        if let Some(&top) = stacks[t].last() {
            assert!(
                moving_disk < top,
                "Step {step}: illegal move — disk {moving_disk} on top of disk {top} (stack {to})"
            );
        }

        // Perform the move
        stacks[f].pop();
        stacks[t].push(moving_disk);
    }

    // After all moves, stacks 1 and 2 (indices 0 and 1) must be empty,
    // and stack 3 (index 2) must hold all disks in order (biggest at bottom).
    assert!(stacks[0].is_empty(), "Stack 1 not empty after solving n={n}");
    assert!(stacks[1].is_empty(), "Stack 2 not empty after solving n={n}");

    let expected_final: Vec<u8> = (1..=n).rev().collect(); // [n, n-1, ..., 1]
    assert_eq!(
        stacks[2], expected_final,
        "Stack 3 has wrong disks after solving n={n}"
    );
}

// ============================================================
// Test: n=0  (edge case — no disks, nothing to do)
// ============================================================
#[test]
fn test_zero_disks_recursive() {
    let moves = solve_recursive(0);
    assert!(moves.is_empty(), "Expected no moves for n=0");
}

#[test]
fn test_zero_disks_iterative() {
    let moves = solve_iterative(0);
    assert!(moves.is_empty(), "Expected no moves for n=0");
}

// ============================================================
// Test: n=1  (smallest real case: 1 move)
// ============================================================
#[test]
fn test_one_disk_recursive() {
    let moves = solve_recursive(1);
    verify_solution(1, &moves);
    // The single move must be stack 1 → stack 3
    assert_eq!(moves, vec![(1, 3)], "Wrong move for n=1 (recursive)");
}

#[test]
fn test_one_disk_iterative() {
    let moves = solve_iterative(1);
    verify_solution(1, &moves);
    assert_eq!(moves, vec![(1, 3)], "Wrong move for n=1 (iterative)");
}

// ============================================================
// Test: n=2  (the example from the problem: 3 moves)
// ============================================================
#[test]
fn test_two_disks_recursive() {
    let moves = solve_recursive(2);
    verify_solution(2, &moves);
    // Must match the exact example output
    assert_eq!(
        moves,
        vec![(1, 2), (1, 3), (2, 3)],
        "Wrong moves for n=2 (recursive)"
    );
}

#[test]
fn test_two_disks_iterative() {
    let moves = solve_iterative(2);
    verify_solution(2, &moves);
    assert_eq!(
        moves,
        vec![(1, 2), (1, 3), (2, 3)],
        "Wrong moves for n=2 (iterative)"
    );
}

// ============================================================
// Test: n=3  (7 moves)
// ============================================================
#[test]
fn test_three_disks_recursive() {
    let moves = solve_recursive(3);
    verify_solution(3, &moves);
}

#[test]
fn test_three_disks_iterative() {
    let moves = solve_iterative(3);
    verify_solution(3, &moves);
}

// ============================================================
// Test: all n from 1 to 16 (the full allowed range)
// ============================================================
#[test]
fn test_all_sizes_recursive() {
    for n in 1..=16 {
        let moves = solve_recursive(n);
        verify_solution(n, &moves);
    }
}

#[test]
fn test_all_sizes_iterative() {
    for n in 1..=16 {
        let moves = solve_iterative(n);
        verify_solution(n, &moves);
    }
}

// ============================================================
// Test: both algorithms produce IDENTICAL move sequences
// ============================================================
// If two different methods find the same shortest path,
// that's very strong evidence both are correct!
#[test]
fn test_algorithms_agree_for_all_sizes() {
    for n in 0..=16 {
        let rec = solve_recursive(n);
        let itr = solve_iterative(n);
        assert_eq!(
            rec, itr,
            "Algorithms disagree for n={n}:\n  recursive={rec:?}\n  iterative={itr:?}"
        );
    }
}

// ============================================================
// Test: move counts match the 2^n - 1 formula exactly
// ============================================================
#[test]
fn test_move_counts() {
    for n in 0u8..=16 {
        let expected = if n == 0 { 0 } else { (1u64 << n) - 1 } as usize;
        assert_eq!(solve_recursive(n).len(), expected, "Wrong count (recursive) n={n}");
        assert_eq!(solve_iterative(n).len(), expected, "Wrong count (iterative) n={n}");
    }
}
