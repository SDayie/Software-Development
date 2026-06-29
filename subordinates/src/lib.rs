// ============================================================
// ALGORITHM 1: QUEUE
// ============================================================
// builds the order vector by looping over it while it's still growing

pub fn solve_queue(n: usize, input_numbers: &[usize]) -> Vec<usize> {
// everything below this line is my original code, untouched

let mut children: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
// children - a vector that holds the subordinates under each employee, it's a vector of vectors(the family tree)
for employee in 2..=n {
let boss: usize = input_numbers[employee - 2];
children[boss].push(employee);
    }
// for employee in 2..=n - we start here from value 2, beacuse the 1st value is always the general boss
// push the subordinates we find into the children value
let mut order: Vec<usize> = Vec::new();
order.push(1);
//creates an empty vector called order, that can change
// start with 1-the boss
let mut index = 0;
// points the position in the order vector
while index < order.len() {
let current_employee = order[index];
// the while loop, loops over the order variable to check if we still have employees left
for &child in &children[current_employee] {
order.push(child);
        }
//go through all direct workers under the current employee
index += 1;
    }
// index += 1 : it tells the computer to move to the next employee    
let mut subordinates: Vec<usize> = vec![0; n + 1];
// it creates a vector to store subordinates for each and every employee
for &employee in order.iter().rev() {
for &child in &children[employee] {
subordinates[employee] += 1 + subordinates[child];
        }
    }
// for &employee in order.iter().rev() = reverse the list of employees(from eg 1-6 to 6-1) to count the children under each employee
//subordinates[employee] += 1 + subordinates[child] = it starts at zero, and if it finds a subordinate it increases by 1, until all subordinates are counted for the current employee
subordinates
// return the finished vector instead of printing it, so main can call this twice for the benchmark
}

// ============================================================
// ALGORITHM 2: TREE (recursive)
// ============================================================
// instead of a vector we loop over, this one calls itself and uses rust's own call stack

fn count_subordinates_recursive(
    employee: usize,
    children: &Vec<Vec<usize>>,
    subordinates: &mut Vec<usize>,
) -> usize {
// employee - who we're counting for. children - the tree, read only. subordinates - the answer vector we fill in

    let mut total = 0;
    for &child in &children[employee] {
        total += 1 + count_subordinates_recursive(child, children, subordinates);
    }
    // for every child, add 1 for the child plus everyone under them - found by calling this function again

    subordinates[employee] = total;
    // save the total once we know it

    total
    // hand it back up to whoever called us
}

pub fn solve_tree(n: usize, input_numbers: &[usize]) -> Vec<usize> {
// same job as solve_queue, but recursive instead

let mut children: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
// same family tree setup as algorithm 1
for employee in 2..=n {
let boss: usize = input_numbers[employee - 2];
children[boss].push(employee);
    }
// employee 1 is always the boss so we skip it and start from 2

    let mut subordinates: Vec<usize> = vec![0; n + 1];

    if n >= 1 {
        count_subordinates_recursive(1, &children, &mut subordinates);
    }
    // start the recursion at employee 1, it fills in everyone else on the way back up

    subordinates
}
