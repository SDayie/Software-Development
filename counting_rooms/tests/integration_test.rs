// now that there's a real lib.rs, we can just import from the crate by
// name instead of pulling main.rs in with the #[path] trick
use counting_rooms::{count_rooms_flood_fill, count_rooms_union_find};

fn build_grid(rows: &[&str]) -> Vec<Vec<u8>> {
    rows.iter().map(|row| row.as_bytes().to_vec()).collect()
    // turns each text row into a Vec<u8>, same format the algorithms expect
}

#[test]
fn example_from_the_problem_statement() {
    // the example from the task, expected answer is 3
    let rows = [
        "########",
        "#..#...#",
        "####.#.#",
        "#..#...#",
        "########",
    ];
    let grid = build_grid(&rows);
    let n = rows.len();
    let m = rows[0].len();

    assert_eq!(count_rooms_flood_fill(&grid, n, m), 3);
    assert_eq!(count_rooms_union_find(&grid, n, m), 3);
}

#[test]
fn entire_map_is_walls_means_zero_rooms() {
    let rows = ["###", "###", "###"];
    let grid = build_grid(&rows);
    let n = rows.len();
    let m = rows[0].len();

    assert_eq!(count_rooms_flood_fill(&grid, n, m), 0);
    assert_eq!(count_rooms_union_find(&grid, n, m), 0);
}

#[test]
fn entire_map_is_floor_means_one_giant_room() {
    let rows = ["....", "....", "...."];
    let grid = build_grid(&rows);
    let n = rows.len();
    let m = rows[0].len();

    assert_eq!(count_rooms_flood_fill(&grid, n, m), 1);
    assert_eq!(count_rooms_union_find(&grid, n, m), 1);
}

#[test]
fn single_floor_square_surrounded_by_walls() {
    let rows = ["###", "#.#", "###"];
    let grid = build_grid(&rows);
    let n = rows.len();
    let m = rows[0].len();

    assert_eq!(count_rooms_flood_fill(&grid, n, m), 1);
    assert_eq!(count_rooms_union_find(&grid, n, m), 1);
}

#[test]
fn diagonal_floor_squares_do_not_count_as_connected() {
    // diagonal touches don't connect, only up/down/left/right - so this is 2 separate rooms
    let rows = [".#", "#."];
    let grid = build_grid(&rows);
    let n = rows.len();
    let m = rows[0].len();

    assert_eq!(count_rooms_flood_fill(&grid, n, m), 2);
    assert_eq!(count_rooms_union_find(&grid, n, m), 2);
}

#[test]
fn several_separate_rooms_of_different_shapes() {
    // hand-built map, counted by hand to be 3 rooms
    let rows = [
        "###########",
        "#.....#...#",
        "#.....#.#.#",
        "#.....#...#",
        "###.#######",
        "#...#.....#",
        "#####.....#",
        "#####.....#",
    ];
    let grid = build_grid(&rows);
    let n = rows.len();
    let m = rows[0].len();

    let flood_answer = count_rooms_flood_fill(&grid, n, m);
    let union_find_answer = count_rooms_union_find(&grid, n, m);

    assert_eq!(flood_answer, union_find_answer);
    assert_eq!(flood_answer, 3);
}

#[test]
fn both_algorithms_agree_on_a_larger_generated_maze() {
    // same checkerboard maze shape as the benchmark, just smaller
    let n = 21;
    let m = 21;
    let mut grid = vec![vec![b'.'; m]; n];
    for r in 0..n {
        for c in 0..m {
            if r % 2 == 1 && c % 2 == 1 {
                grid[r][c] = b'#';
            }
        }
    }

    let flood_answer = count_rooms_flood_fill(&grid, n, m);
    let union_find_answer = count_rooms_union_find(&grid, n, m);
    assert_eq!(flood_answer, union_find_answer);
}
