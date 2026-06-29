# Concert Tickets : Benchmark Interpretation

Both versions solve the problem the same way — for each customer, they choose the most expensive ticket that still fits the budget, sell it, and remove it from availability. So the core logic is the same; the difference is only how the data is stored and accessed.

The first algorithm (segment tree) first sorts all ticket prices and builds a complete binary tree in a single array. Each node stores the minimum value in its range, which helps guide the search for the best affordable ticket. The lookup process moves toward the right whenever possible, so it effectively finds the largest ticket within the budget. After each sale, it updates the tree along the path back to the root. This requires an initial setup cost, but afterwards queries and updates are very fast and benefit from a compact, cache-friendly layout.

The second algorithm (BTreeMap) stores prices in a balanced tree with counts for duplicates. For each customer, it directly finds the largest key ≤ budget using the ordered map, then decreases the count or removes the entry if needed. Each operation involves navigating tree nodes in memory, which adds overhead compared to the array-based segment tree.

So the trade-off is simple: the first algorithm builds a fast, tightly packed structure for efficient repeated queries, while the second is easier to implement but has higher per-operation overhead due to pointer-based traversal. The first algorithm (segment tree) is the fastest, thanks to its compact, cache-friendly array layout, while the second (BTreeMap) is slower due to its pointer-based traversal (lookup process).
