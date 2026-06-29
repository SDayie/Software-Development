// ALGORITHM 1
pub fn increasing_array_v1(numbers: &mut Vec<i64>) -> i64 {
    let mut moves: i64 = 0;
    // use let mut - because it can change and grow. const means it can never change and remains constant
    // moves - holds the values when the numbers when the numbers are decreasing
    let n = numbers.len();
    for i in 1..n {
        if numbers[i] < numbers[i - 1] {
            moves += numbers[i - 1] - numbers[i];
            numbers[i] = numbers[i - 1];
        }
    }
    // for i in 1..n     n holds the array [3 2 5 1 7]
    // for i(index), compare i = 2 to (i-1)=3
    // moves : 3-2 = 1; so store the differncere (number 1);
    // if it's increasing it ignores it; if it's decreasing, subtract the values and add it to moves
    // index[1] and index[3]
    //[3, 2, 5, 1, 7]
    // [3, 3, 5, 5, 7].  numbers[i] = numbers [i - 1];
    // difference/ moves = 2 -> 3 and 1 -> 5. = 5
    moves
}
// ALGORITHM 2: dynamic array version, builds a brand new array instead of changing the old one
pub fn increasing_array_v2(numbers: &[i64]) -> i64 {
    let n = numbers.len();
    let mut adjusted: Vec<i64> = Vec::new();
    // Vec::new() - starts empty, grows bigger by itself as we add to it. that's a dynamic array
    let mut moves: i64 = 0;
    // moves - same as algorithm 1, holds the total
    for i in 0..n {
        if i == 0 {
            adjusted.push(numbers[i]);
            // first number never needs lifting, nothing comes before it
        } else {
            let prev = *adjusted.last().unwrap();
            // .last() - grabs the last number we already added to adjusted
            if numbers[i] < prev {
                moves += prev - numbers[i];
                adjusted.push(prev);
                // push the lifted-up number instead of the original one
            } else {
                adjusted.push(numbers[i]);
                // already big enough, push it as is
            }
        }
    }
    // [3, 2, 5, 1, 7] stays untouched, adjusted grows into [3, 3, 5, 5, 7] on its own
    moves
}
