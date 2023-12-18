# Tips

Every year, it's easy to forget the approaches you've used in to similar problems in the past, or the pitfalls you've encountered.
Here's some reminders from 2023, that might serve you well in 2024.

## Intervals

See [Day 5](./src/bin/05.rs) where we needed to deal with a very large set of numbers being passed through each stage. This would have
blown out tracking each number, but were able to narrow the solution down by calculating the intersecting intervals on the number line
at each stage instead.

## Area inside a polygon

See [Day 10](./src/bin/10.rs) and [Day 18](./src/bin/18.rs). Both solutions needed to calculate the area inside a closed path. My initial
attempt used a flood fill each time (and in day 10, this needed to be sophisticated to deal with adjacent pipes that didn't matter to an
area algorithm). That might be great for visualisation, but is unnecessary to calculate the numerical value.

The algorithm for the area is simple: for each vertex v2 and prior vertex v1, sum (v2.x + v1.x) * (v2.y - v1.y). Finally take the absolute
value of of the area, add the perimeter to it, divide it by two and add 1 for the four outermost corners not considered in the perimeter.

In both cases, because the edges mapped out whole 1x1 squares, the area encompasses half of the perimeter (as it is taken from the middle
of each square). This is why half of the perimeter is added to the area, along with one extra for the corners.

## Dynamic Programming & Memoization

See [Day 12](./src/bin/12.rs). In this case there was a calculation at the end of the pattern that would always be the same when it
was encountered when searching from the front, so was a good candidate for memoization.

## Lowest Common Multiple

See [Day 8](./src/bin/08.rs). LCM and GCD often come into play with large numbers. LCM helped detect when a series of cycles would align at
the same point. I had extended discussion about the validity of this problem as it relied on certain input properties - a reminder to keep
an eye out for this potential.

## Detecting cycles

In [Day 8](./src/bin/08.rs) (as above), it was noted that a cycle occurred, after the initial stop,
which permitted LCM to be used as the interval was consistent.

In [Day 14](./src/bin/14.rs), a more deliberate cycle needed to be detected to solve the second part, as it requested 1000000000 iterations
which is unlikely to complete by brute force. Here, we captured each unique moment in time until a repeated iteration occurred, and knowing
that the next iteration is always deterministic it would be a cycle and could be used to extrapolate the result.

Note to store many iterations of a complex type, a fast hashing algorithm needed to be used.

## Pathfinding & Dijkstra's algorithm

See [Day 17](./src/bin/17.rs). On the surface a classic pathfinding problem from start to finish with a weight on each node, but the
constraint to not move more than a certain distance in one direction made a unique constraint. This didn't just adjust the weight of that
movement, but introduced new alternatives depending on the arrival square and direction.

This is a reminder that it's important to capture all the unique properites of the graph with dijkstra - in this case the solution was to add
direction to the node, and to add one vertex for each possible distance in that direction (and then not allowing that direction to be used again
from the smaller distances). You can think of this being a little like converting the 2d graph into 3d space where the z is based on alternative
pathways.

Note that Dijkstra can be simplified if you don't need to backtrack to the origin by not keeping the "came from" map. I also mistakenly used a
regular priority queue at the start which returned the maximum element - it must return the one with the minimum priority. This can be achieved
with a DoublePriorityQueue, but there are other implementations that can be used (e.g. BinaryHeap) by either inserting a negative cost to reverse
the order, or by providing an alternate `Ord` / `PartialOrd` implementation.

## Tracking Direction

See [Day 16](./src/bin/16.rs) and [Day 17](./src/bin/17.rs). As above, note that when direction is relevant to the puzzle, it often needs to be
tracked. In both puzzles, this impacted the way you could test the solution. For Day 16, cycles existed in the grid, but you could not simply
track visited squares as there were multiple active beams and some crossed over. Rather than tracking all visited squares per beam and detecting
identical mirror locations, it was easiest to track "visited by any beam in the same direction" as it would then follow the same path.

## Other Rust Tips

### Watch out for casting (Correctness)

In [Day 8](./src/bin/08.rs), I got caught up when my submission didn't work, even though the algorithm and test was correct. This turned out to be
that I was casting to `as u32` to return from the solution, as is the default in the template, but the result was larger than that. While add/subtract
will be checked, casting will not. There now appears to be a warning in template [README](./README.md#common-pitfalls) about this!

### FxHashMap / FxHashSet (Performance)

`HashMap` provides a secure default hashing algorithm, but it can be quite slow, especially for simple keys including integers as often used in AOC.
A simple alternative is `rustc_hash::FxHashMap::default()`, which uses a faster hashing algorithm, typically halving the time taken on hash-intensive
solutions.

The only one that didn't benefit from this was [Day 14](./src/bin/14.rs), but I expect this was because I'd already generated a fast hash from the
vector using `seahash`.

### Cached keys (Performance)

See [Day 7](./src/bin/07.rs) which was initially quite slow to execute because a vector was being sorted by a key that had some calculation to complete
each time (in this case it needed to do a frequency distribution across five cards and determine the hand). By using a cached key, it ensured that the
hand was only generated once for a given hand.

### Iterators are fast, you may not need bitwise operations (Performance)

See [Day 13](./src/bin/13.rs) in which I thought the initial iterator based solution could be optimised by not allocating a vector and instead packing
it in two a bit array. The steps to create the array and then manipulate it ended up being slower than the original iterator every time, and had a size
limitation.

I haven't explored further alternatives on this (e.g. `bitarray` crate), but suspect that continuing to use vectors with pre-allocation and reuse is
likely to be the fastest solution as these are heavily optimised in compiled Rust.
