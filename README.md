# Advent Of Code 2022 - My Solutions in Rust

## Prerequisites
Install Rust: https://www.rust-lang.org/tools/install.  
Tested with `rustc 1.65.0`.

## Running
Run tests with `cargo test`.  
Run benchmarks with `cargo bench`.

## Solution Descriptions

⌛ = Time complexity of solution
📦 = Space complexity of solution (not including the input string)

### TEMPLATE

TODO

> **Part 1**: TODO  
&ensp;&ensp;`⌛O(n)` | `📦O(1)`, where TODO  
> **Part 2**: TODO  
&ensp;&ensp;`⌛O(n)` | `📦O(1)`, where TODO

### Day 1

For both parts we need to sum each continuous run of integers, seperated by blank lines. If only there was a `split` function on an `Iter` like there is on a `String` - I could have just written one 🤔.

> **Part 1**: Track the largest sum and return it.  
&ensp;&ensp;`⌛O(n)` | `📦O(1)`, where n is the number of individual rations.  
> **Part 2**: Calculate all sums and find the top 3 at the end of a sorted array. The time and space complexity could be improved by just tracking the top 3 sums.  
&ensp;&ensp;`⌛O(n·log(n))` | `📦O(n)`, where n is the number of individual rations.

### Day 2

For both parts we can calculate the scores of each round by a simple lookup table of 6 items. I maybe went a bit overboard with trying to design some nice structs for an alternate solution which turned out to be a rather complicated abstraction.

The performance could be further improved by avoiding string handling and looking at the individual bytes which are always at a fixed offset from the last round.

> **Part 1**: Perform a lookup for the score on each line and sum them.  
&ensp;&ensp;`⌛O(n)` | `📦O(1)`, where n is the number of rounds.  
> **Part 2**: Perform a lookup for the score on each line and sum them.  
&ensp;&ensp;`⌛O(n)` | `📦O(1)`, where n is the number of rounds.

### Day 3

Sets are the easiest way to solve this and lead to the most readable code, but performance is better (at least for *this* specific use case) when performing naieve comparisons with an early return.

> **Part 1 (Solve 1)**: Split each rucksack into a set and perform an intersection to find the common element.  
&ensp;&ensp;`⌛O(m·n)` | `📦O(m)`, where n is the number of rucksacks and m is the size of each rucksack.  
> **Part 1 (Solve 2)**: Check the cartesian product of both sides of the rucksack to find a duplicate tuple.  
    While the complexity of this algorithim is quadratic, the small input size makes this ~4x faster than the overhead of creating and comparing sets.  
&ensp;&ensp;`⌛O(m^2·n)` | `📦O(1)`, where n is the number of rucksacks and m is the size of each rucksack.  
> **Part 2**: Chunk into groups of 3 elves (rucksacks) and find the single item (char) present in all 3.  
    We check the rucksacks smallest-to-largest to reduce the search space and quickly cull false possibilites.  
&ensp;&ensp;`⌛O(m^3·n)` | `📦O(1)`, where n is the number of rucksacks and m is the size of each rucksack.`

### Day 4

The most complicated part here is parsing each line into 4 numbers so we can perform comparisons on them. 6 lines - I wonder if we can do better?

> **Part 1**: For each pair, parse the 4 values and check if either range is fully contained within the other.  
&ensp;&ensp;`⌛O(n)` | `📦O(1)`, where n is the number of elf pairs.  
> **Part 2**: For each pair, parse the 4 values and check if the two ranges overlap.  
&ensp;&ensp;`⌛O(n)` | `📦O(1)`, where n is the number of elf pairs.

### Day 5

Split the input into two parts - the crate diagram and a list of instructions. We parse the crate diagram into a 2d array of vectors and use the inner ones as stacks as we only ever touch the top elements in a single operation. The instructions are parsed in a fixed format and then applied to the stacks to move elements.

We could make this more efficient by avoiding the temporary array in Part 2. If only `get_many_mut` was in Stable.

> **Part 1 (Solve 1)**: As above, while ensuring that multiple crates moved in a single command end up on the *opposite* order on the destination stack.  
&ensp;&ensp;`⌛O(s + i)` | `📦O(s)`, where s is the number of squares in the crate diagram and i is the number of instructions.  
> **Part 2 (Solve 1)**: As above, while ensuring that multiple crates moved in a single command end up on the *same* order on the destination stack.  
&ensp;&ensp;`⌛O(s + i)` | `📦O(s)`, where s is the number of squares in the crate diagram and i is the number of instructions.

### Day 6

This day is about finding runs of 4 and 14 unique characters in a string, as well as the index of this run. My second solution for Part 2 was the most interesting an runs in 60% of the time as the first.

> **Part 1**: Compare all characters in a 4-wide sliding window over the input (6 comparisons) and return the index of the found with unique characters.  
&ensp;&ensp;`⌛O(n·m^2)` | `📦O(1)`, where n is the length of the input and m is the length of the run to find.  
> **Part 2 (Solve 1)**: Iterate over each character in each 14-char sliding window and track if it was already found in this window using a fixed-size (26 element) array used as a lookup table. Return the index of the window with no duplicates.  
&ensp;&ensp;`⌛O(n·m)` | `📦O(1)`, where n is the length of the input and m is the length of the run to find.  
> **Part 2 (Solve 2)**: Slide the window across the input, but only handle two the two chars on the edges on each slide/iteration. We use lookup to track how many times we've seen each character and a counter so we know when we have no duplicates in our window.  
&ensp;&ensp;`⌛O(n)` | `📦O(1)`, where n is the length of the input and m is the length of the run to find.

### Day 7

The directory traveral commands in our input are the result of a depth-first search of an arbitrary directory tree. Using this assumption, we can simplify the traversal tracking logic to only consider when we actually change directories (the names don't matter at all) and the size of files in a directory.

> **Part 1**: Sum all directories which contain files (directly, or indirectly in child directories) with a combined size of <= 100,000.  
&ensp;&ensp;`⌛O(n)` | `📦O(m)`, where n is the number of directories we have to traverse, and m is the depth of the directory tree.  
> **Part 2**: Calculate and store the sizes of all directories in an array. Then we determine how much additional space we need (based on the size of the root directory and known constants) and proceed to find the smallest single directory which is larger that this.  
&ensp;&ensp;`⌛O(n)` | `📦O(n)`, where n is the number of directories we have to traverse.

### Day 8

This day is about finding adjacent trees and distances from other trees. We start by parsing the input into a 2d array of heights (stored as bytes) and then proceed to cast rays to find visibilities and scores.

An alternate solution for Part 1 (and the most obvious) would have been very similar to part 2 - alongside the scary complexity.

My solution for Part 2 has scary complexity due to casting 4 rays from each and every tree. It feels like there should be a better solution... Maybe for each tree we could store the distance to a tree of each possible height in each direction, then cast a ray from each cardinal direction to calculate the values and a final pass to calculate the top scenic score 🤔. If that worked, the time complexity would be reduced to `O(4n + n) = O(n)`.

> **Part 1**: Start with all trees marked as hidden, then cast rays from the edges along all 4 cardinal directions and mark the visible trees based on their height and if they are obsured by taller trees along the ray's path.  
&ensp;&ensp;`⌛O(n)` | `📦O(n)`, where n is the number of trees  
> **Part 2**: For each tree on the grid we cast a ray in each of the 4 cardinal directions to calculate it's scenic score.  
&ensp;&ensp;`⌛O(n^3)` | `📦O(n)`, where n is the number of trees

### Day 9

Simulating rope! We track the 2D positions of each rope knot and update them on each movement.

We simulate every single step of movement and every square moved but I haven't found a way to avoid this - especially for the 10-knot rope in Part 2.

> **Part 1**: Simulate the head movements and check if the tail is still adjacent. If not we move the tail to the last head position.  
&ensp;&ensp;`⌛O(n)` | `📦O(1)`, where n is the number of head movements.  
> **Part 2**: Simulate the head movements and iteratively update the tail positions if they are too far from the head. We use a clamped vector (-1,1 for x and y) to move each component adjacent to it's parent again.  
&ensp;&ensp;`⌛O(n·m)` | `📦O(1)`, where n is the number of head movements and m is the tail length.

### Day 10

We create an iterator which consumes opcodes and outputs the value of the `x` register at each cycle (it also takes into account noops and multi-cycle instructions). We then simply consume this iterator to perform the calculations we need at each cycle/time step.

> **Part 1**: Calculate the signal strength from the nth elements (cycles) of the iterator.  
&ensp;&ensp;`⌛O(n)` | `📦O(1)`, where n is the number of commands.  
> **Part 2**: Calculate the pixel value on each cycle based on the distance from the center of the sprite (`x` register value) and output them formatted in a 40x6 grid.  
&ensp;&ensp;`⌛O(n)` | `📦O(n)`, where n is the number of commands/cycles/pixels.

### Day 11

For this one I hardcoded the monkey definitions for both the example and the input to avoid parsing them 😅. We loop over the monkeys in each round to process which monkeys their items should end up with.

This gets interesting in Part 2, as there is no longer a safely of dividing each item's worry level by 3. This results in overflows of even a 64-bit `usize` type as we run 10000 rounds (Not surprising, given the 4th monkey squares their items every round).

The key is that the only property of each item's value we actually care about is the divisibility check that lets each monkey determine who to throw to. If we modulo each item by the LCM of all divisors we test against, the result will remain unchanged but we avoid overflow.

For a larger number of rounds we can also check if there are any repetitions and if at some point the monkey states start to repeat themselves. **Part 2 (Solve 2)** shows an implementation where we keep track of the number of monkey inspections on each round as well as a hash of the monkey's state. Once we find a round state that we've seen before, we know this will continue to infinity and we can look back in the history to calculate the expected number of inspections of each monkey by the 10000th round.

Possible improvements include:
 + Using an `i32` instead of a `usize` for state. We actually only need to worry about overflow in the `x*x` operations, so we could just perform the multiply there and convert back to `i32` after the modulo.
 + Using a better squaring-modulo algorithm to avoid overflow with `i32` data types entirely.
 + ~~Calculate the *actual* LCM (least-common-multiple) of all divisors to use for reducing the size of each item. Right now we naievely multiply them all together.~~ Actually, this won't help, as the divisors are all prime numbers so multiplying them all together *is* the LCM. 

> **Part 1**: Run 20 iterations and return the product of the top-2 inspect counts.  
&ensp;&ensp;`⌛O(n·m)` | `📦O(n)`, where n is the number of monkeys and m is the number of rounds.  
> **Part 2 (Solve 1)**: Run 10000 iterations and return the product of the top-2 inspect counts.  
&ensp;&ensp;`⌛O(n·m)` | `📦O(n·m)`, where n is the number of monkeys and m is the number of rounds.  
> **Part 2 (Solve 2)**: Run 10000 iterations and return the product of the top-2 inspect counts. We also check for and skip cycles in the monkey states to avoid similating all 10000 rounds.  
&ensp;&ensp;`⌛O(n·m)` | `📦O(n·m))`, where n is the number of monkeys and m is the number of rounds.

### Day 12

We're tasked to find the length of the shortest route between two points on map considering the traversal requirmements between squares of different heights. We use a simplified version of Dijkstra's algorithm that considers all edges to have the same weight (i.e. moving from one square to any valid adjacent square is always a distance of 1).

Possible improvements include:        
 + Make this solution truly work for any input in Part 2 by also allowing it to still find the marked starting point if it's actually the closest of all level `a` map squares.

> **Part 1**: Find the shortest path from the marked starting point to the marked goal.  
&ensp;&ensp;`⌛O(n)` | `📦O(n)`, where n is the number of map squares.  
> **Part 2**: Find the shortest path from the marked goal to the closest square at level `a`. The algorithm  is essentially reversed in direction from Part 1.  
&ensp;&ensp;`⌛O(n)` | `📦O(n)`, where n is the number of map squares.

### Day 13

This problem involves sorting string packets which consist of nested lists and integer.

Possible improvements include:
 + Use a more efficient packet comparison algorithm that doesn't require any allocations. This would avoid adding `m` to the space complexity and also result in a sizable speedup.

> **Part 1**: Compare each pair of packets and sum the indices of pairs in the correct order.  
&ensp;&ensp;`⌛O(n·m)` | `📦O(m)`, where n is the number of packets and m is the length of each packet.  
> **Part 2 (Solve 1)**: Sort all individual packets into the correct order and multiply the indices of two specific packets in the resulting sorted list.  
&ensp;&ensp;`⌛O((n·m) * log(n·m))` | `📦O(n+m))`, where n is the number of packets and m is the length of each packet.  
> **Part 2 (Solve 2)**: For each of the two divider packets, we count the number of input packets that would come before it (`2n` comparisons) and therefore allow us to find it's effective index to save us from sorting the entire list.  
&ensp;&ensp;`⌛O(n*m)` | `📦O(n+m))`, where n is the number of packets and m is the length of each packet.

### Day 14

This was a fun one about simulating falling sand. I initially used a `HashMap<Position2D>` to store a sparse representation of cave tiles blocked by rocks and sand, but after benchmarking, an 2D array of `Grid<bool>` the simulated cave area is both faster (1/10 the time) and takes less memory (`> 22k * 16 bytes` down to `~50k * 1 byte`). The utilisation of the cave space ends up being around 50% which isn't worth the overhead of a sparse data structure in this instance.

For Part 2 Solve 2, I improved the falling sand algorithm by taking into account that the next block of sand, dropped from the same place, will mostly adhere to the path of the previous block. We store each sand block movement in a stack (to represent the travel path of the previous block) and start our seach for the resting position of the next block above the end position of the previous one.

> **Part 1**: Simulate sand until a block moves down past the lowest level (highest `y`) of rock formations.  
&ensp;&ensp;`⌛O(n^3)` | `📦O(n)`, where n is the size of the simulated area.  
> **Part 2 (Solve 1)**: Simulate sand until it gets high enough to block the sand spout.  
&ensp;&ensp;`⌛O(n^3)` | `📦O(n)`, where n is the size of the simulated area.  
> **Part 2 (Solve 2)**: Simulate sand until it gets high enough to block the sand spout. We also use a cache to avoid simulating the entire sand fall path each time.  
&ensp;&ensp;`⌛O(n^2 · log(n))` | `📦O(n)`, where n is the size of the simulated area.

### Day 15

For Part 1, we are tasked with finding the total number of points on a single row that are overlapped from diamonds centered on each probe and sized by their nearest beacon. This is simple enough to do with intervals (and a library) to avoid double-counting overlaps.

For Part 2, we can take the same idea but instead find which row of the 4,000,000 row search space has a gap between two intervals. My current solution takes ~7 seconds to run in release mode as it runs 4m times 😱. It was quick to implement after Part 1, but it definitely is not efficient.

Possible improvements for Part 2 would involve a better algorithm or further profiling to find quick wins in the current one.

> **Part 1**: Find the size of the interval on row `x` based on overlapping ranges.  
&ensp;&ensp;`⌛O(n)` | `📦O(1)`, where n is the number of probes  
> **Part 2**: Find the row with a disjoint interval formed by overlapping ranges.  
&ensp;&ensp;`⌛O(n·m)` | `📦O(1)`, where n is the number of probes and m is the y search range

### Day 16

This one took me a long time to get working for both parts. I ended up reading about other people's approaches before I got a working solution, though I didn't copy any code and re-implemented everything myself.

1. Parse the input into a directed graph.
2. Create matrix to find the distance of each valve to all other valves.
3. Find all valves that we would care about travelling to (has a flow rate).

For Part 1, we then perform a depth-first search across all possible orderings of useful valves to see which results in the highest pressure release after 30 minutes. I originally tried a greedy algorithm, but it wasn't quite optimal. 

For Part 2, we know that 2 independent actors will take 2 disjoint paths through the available valves (where a path includes a valve that is turned on), as we can't turn on the same valve twice. We build on the algorithm in Part 1, but instead return all subsets of possible paths along with their total pressure release. The solution must then be the maximim sum of the pressure releases of two of these disjoint paths. We can efficiently find this pair by sorting the list of paths by total pressure release and starting a `N^2` comparison with early exit.

> **Part 1**: Find the path which results in the highest pressure release, assuming 1 actor.  
&ensp;&ensp;`⌛O(n!)` | `📦O(n)`, where n is the number of valves.  
> **Part 2**: Find the path which results in the highest pressure release, assuming 2 actors.  
&ensp;&ensp;`⌛O(n! · n)` | `📦O(n!)`, where n is the number of valves.

### Day 17

Simulating this problem is like Tetris, except we never make a complete row (I checked).

My solution for Part 1 is a simple simulation with minimal optimisations.

In Part 2 we can't feasibly simulate or store 1 trillion rocks so we have to make some optimisations. The pattern of rock formations eventually repeats after the gas movements provided as an input start cycling. Once we have identified the cycle parameters, we can extrapolate the tower height at any point without simulating it.

To identify the cycle point, we first optimise the data structure holding the rocks to drop old rows that won't be useful again (+ a margin). Then we hash this after each rock and find the point where it looks the same.

Possible future optimisations include:
  + Representing each row as a byte (instead of a vector of bool) and each rock shape as an array of bytes. We could then use bitshift operations to simulate the gas jets pushing rocks left and right, and binary OR to stamp dropped rocks into existing formations.

> **Part 1**: Find the height of the tower after 2022 rocks.  
&ensp;&ensp;`⌛O(n)` | `📦O(n)`, where n is the number of rocks to drop.  
> **Part 2**: Find the height of the tower after 1 Trillion rocks.  
&ensp;&ensp;`⌛O(n)` | `📦O(n)`, where n is the number of rocks to drop.

### Day 18

This challenge is abotu finding the surface area of a 3d grid of cubes.

For Part 1 we simply iterate over each cube and check all 6 neighbors. Each missing neighbor adds 1 to the surface area.

For Part 2 we only want to find the exterior area and exclude any air pockets inside. We do this by running a flood-fill algorithm starting from the outside which counts each exposed face it finds. Since it will never find a path to any interior pockets, this will give us the exterior surface area. Of note is that we also have a 1-wide margin in all dimensions around the object to allow the algorithm to correctly count any faces on the edges.

> **Part 1**: Count empty neighbors of all cubes.  
&ensp;&ensp;`⌛O(n)` | `📦O(m)`, where n is the number of cubes and m is the dimensions of the droplet.  
> **Part 2**: Flood-fill from outside to find exterior surfaces.  
&ensp;&ensp;`⌛O(m)` | `📦O(m)`, where m is the dimensions of the droplet.

...

### Day 25

A conversion betwen numerical systems. Standard Base-10 and some weird Base-5 *thing*.

The place values of each digit are as follows.
```
2 = 2
1 = 1
0 = 0
- = -1
= = -2
```

> **Part 1**: Convert from SNAFU to decimal, sum all numbers, and convert the sum from decimal to SNAFU.  
&ensp;&ensp;`⌛O(n)` | `📦O(1)`, where TODO  
> ~~**Part 2**~~: TODO: Requires all other puzzles to be solved.  
~~&ensp;&ensp;`⌛O(n)` | `📦O(1)`, where TODO~~