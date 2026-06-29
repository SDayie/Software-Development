use subordinates::{solve_queue, solve_tree};
// pulls in both algorithms from the library so we can test them directly

#[test]
fn test_example_from_problem() {
    let bosses = vec![1, 1, 2, 3];
    let expected = vec![0, 4, 1, 1, 0, 0];
    assert_eq!(solve_queue(5, &bosses), expected);
    assert_eq!(solve_tree(5, &bosses), expected);
}

#[test]
fn test_single_employee() {
    let bosses: Vec<usize> = vec![];
    assert_eq!(solve_queue(1, &bosses)[1], 0);
    assert_eq!(solve_tree(1, &bosses)[1], 0);
}

#[test]
fn test_two_employees() {
    let bosses = vec![1];
    assert_eq!(solve_queue(2, &bosses), vec![0, 1, 0]);
    assert_eq!(solve_tree(2, &bosses), vec![0, 1, 0]);
}

#[test]
fn test_chain_tree() {
    // a straight chain: 1 -> 2 -> 3 -> 4 -> 5
    let bosses = vec![1, 2, 3, 4];
    let expected = vec![0, 4, 3, 2, 1, 0];
    assert_eq!(solve_queue(5, &bosses), expected);
    assert_eq!(solve_tree(5, &bosses), expected);
}

#[test]
fn test_star_tree() {
    // one boss, everyone else reports straight to them
    let bosses = vec![1, 1, 1, 1];
    let expected = vec![0, 4, 0, 0, 0, 0];
    assert_eq!(solve_queue(5, &bosses), expected);
    assert_eq!(solve_tree(5, &bosses), expected);
}

#[test]
fn test_two_level_balanced() {
    let bosses = vec![1, 1, 2, 2, 3, 3];
    let expected = vec![0, 6, 2, 2, 0, 0, 0, 0];
    assert_eq!(solve_queue(7, &bosses), expected);
    assert_eq!(solve_tree(7, &bosses), expected);
}

#[test]
fn test_large_chain_n200000() {
    // stress test at max size, also checks recursion doesn't blow the stack
    let builder = std::thread::Builder::new().stack_size(64 * 1024 * 1024);
    let handle = builder
        .spawn(|| {
            let n = 200_000;
            let bosses: Vec<usize> = (1..n).collect();
            let queue_result = solve_queue(n, &bosses);
            let tree_result = solve_tree(n, &bosses);
            assert_eq!(queue_result[1], n - 1);
            assert_eq!(queue_result[n], 0);
            assert_eq!(tree_result[1], n - 1);
            assert_eq!(tree_result[n], 0);
        })
        .unwrap();
    handle.join().unwrap();
}

#[test]
fn test_queue_and_tree_always_agree() {
    let cases: Vec<(usize, Vec<usize>)> = vec![
        (6, vec![1, 1, 2, 2, 3]),
        (4, vec![1, 2, 3]),
        (4, vec![1, 1, 1]),
        (8, vec![1, 1, 2, 2, 3, 3, 4]),
    ];

    for (n, bosses) in cases {
        let queue_result = solve_queue(n, &bosses);
        let tree_result = solve_tree(n, &bosses);
        assert_eq!(queue_result, tree_result, "disagreement for n={n}");
    }
}
