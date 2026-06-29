# Two Sets: Benchmark Interpretation

Both versions get to the same answer, and both build their sets by pushing numbers into new lists, so they both use a similar approach.

The **first one (greedy)** looks at every number from n down to 1 and asks, "Does this fit?" before deciding. That means it goes through all n numbers to build set1, so it spends more time checking numbers before adding them.

The **second one (two-pointer pairing)** skips all that asking. It already knows 1 goes with n, 2 goes with n−1, so it just adds those pairs straight into set1. It uses fewer loops to build the first set because it already knows which numbers belong together.

Both algorithms still go through all the numbers to build set2, so that part takes the same amount of work. The greedy version is slower because it checks every number before deciding whether to use it, while the pairing version is faster because it already knows which numbers to pick. This doesn't mean the greedy method is worse — it just does more checking to reach the same result.
