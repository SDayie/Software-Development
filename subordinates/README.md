# Subordinates: Benchmark Interpretation

Both versions produce the same result, and both walk through every employee once to count how many people work under them, so the basic work is the same.

The **1st algorithm (queue)** builds an extra order list as it goes. Every employee is added to that list once, and when the list gets full, Rust sometimes has to make a bigger one and move the old values into a bigger list. This doesn't happen often, but it adds extra time.

The **2nd algorithm (tree)** doesn't build that list. Instead, it uses recursion, meaning the function calls itself for each employee. In the benchmark, the input is a long chain (1 → 2 → 3 → ... → n), so the program has to keep about 200,000 function calls open at once before it can finish. That takes extra time because the computer has to remember all those unfinished calls.

Both algorithms still visit every employee once and calculate the subordinate counts in the same way, so the main work is identical. The difference is that the 1st algorithm (queue) spends a little time resizing its list, while the 2nd algorithm (tree) spends time handling many nested function calls. Both do the same counting — they just manage the work in different ways, but the queue version is faster because it uses simple looping instead of many nested function calls.
