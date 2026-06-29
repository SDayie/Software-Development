use increasing_array::{increasing_array_v1, increasing_array_v2};
// pulls in both algorithms straight from lib.rs, so we can test the real code directly

#[test]
fn example_from_problem_statement() {
    // matches the example from the task: 3 2 5 1 7 -> 5
    let original = vec![3, 2, 5, 1, 7];

    let mut copy_for_v1 = original.clone();
    // copy because v1 changes the array it's given
    assert_eq!(increasing_array_v1(&mut copy_for_v1), 5);

    assert_eq!(increasing_array_v2(&original), 5);
}

#[test]
fn already_increasing_needs_zero_moves() {
    let original = vec![1, 2, 3, 4, 5];

    let mut copy_for_v1 = original.clone();
    assert_eq!(increasing_array_v1(&mut copy_for_v1), 0);
    assert_eq!(increasing_array_v2(&original), 0);
}

#[test]
fn single_element_array() {
    let original = vec![42];

    let mut copy_for_v1 = original.clone();
    assert_eq!(increasing_array_v1(&mut copy_for_v1), 0);
    assert_eq!(increasing_array_v2(&original), 0);
}

#[test]
fn all_same_value() {
    let original = vec![7, 7, 7, 7];

    let mut copy_for_v1 = original.clone();
    assert_eq!(increasing_array_v1(&mut copy_for_v1), 0);
    assert_eq!(increasing_array_v2(&original), 0);
}

#[test]
fn strictly_decreasing_array() {
    // 4->5 (1), 3->5 (2), 2->5 (3), 1->5 (4), total = 10
    let original = vec![5, 4, 3, 2, 1];
    let expected = 10;

    let mut copy_for_v1 = original.clone();
    assert_eq!(increasing_array_v1(&mut copy_for_v1), expected);
    assert_eq!(increasing_array_v2(&original), expected);
}

#[test]
fn both_algorithms_agree_on_random_looking_data() {
    // checks v1 and v2 land on the same number, even on bigger messier data
    let original: Vec<i64> = vec![10, 1, 100, 2, 99, 3, 98, 4, 97, 5, 200, 1, 1, 1, 50];

    let mut copy_for_v1 = original.clone();
    let result_v1 = increasing_array_v1(&mut copy_for_v1);
    let result_v2 = increasing_array_v2(&original);

    assert_eq!(result_v1, result_v2);
}
