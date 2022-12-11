# Advent Of Code 2022 - My Solutions in Rust

## Prerequisites
Install Rust: https://www.rust-lang.org/tools/install.  
Tested with `rustc 1.65.0`.

## Running
Run tests with `cargo test`.  
Run benchmarks with `cargo bench`.

## Solution Descriptions

### TEMPLATE

TODO

+ **Part 1**: TODO  
    `⌛O(n)` | `📦O(1)`, where TODO
+ **Part 2**: TODO  
    `⌛O(n)` | `📦O(1)`, where TODO


### Day 1

For both parts we need to sum each continuous run of integers, seperated by blank lines. If only there was a `split` function on an `Iter` like there is on a `String` - I could have just written one 🤔.

+ **Part 1**: Track the largest sum and return it.  
    `⌛O(n)` | `📦O(1)`, where n is the number of individual rations.
+ **Part 2**: Calculate all sums and find the top 3 at the end of a sorted array.  
    The time and space complexity could be improved by just tracking the top 3 sums.  
    `⌛O(n·log(n))` | `📦O(n)`, where n is the number of individual rations.

### Day 2

For both parts we can calculate the scores of each round by a simple lookup table of 6 items. I maybe went a bit overboard with trying to design some nice structs for an alternate solution which turned out to be a rather complicated abstraction.

The performance could be further improved by avoiding string handling and looking at the individual bytes which are always at a fixed offset from the last round.

+ **Part 1**: Perform a lookup for the score on each line and sum them.  
    `⌛O(n)` | `📦O(1)`, where n is the number of rounds.
+ **Part 2**: Perform a lookup for the score on each line and sum them.  
    `⌛O(n)` | `📦O(1)`, where n is the number of rounds.

### Day 3

Sets are the easiest way to solve this and lead to the most readable code, but performance is better (at least for *this* specific use case) when performing naieve comparisons with an early return.

+ **Part 1 (Solve 1)**: Split each rucksack into a set and perform an intersection to find the common element.  
    `⌛O(m·n)` | `📦O(m)`, where n is the number of rucksacks and m is the size of each rucksack.
+ **Part 1 (Solve 2)**: Check the cartesian product of both sides of the rucksack to find a duplicate tuple.  
    While the complexity of this algorithim is quadratic, the small input size makes this ~4x faster than the overhead of creating and comparing sets.  
    `⌛O(m^2·n)` | `📦O(1)`, where n is the number of rucksacks and m is the size of each rucksack.
+ **Part 2**: Chunk into groups of 3 elves (rucksacks) and find the single item (char) present in all 3.  
    We check the rucksacks smallest-to-largest to reduce the search space and quickly cull false possibilites.  
    `⌛O(m^3·n)` | `📦O(1)`, where n is the number of rucksacks and m is the size of each rucksack.`

### Day 4

The most complicated part here is parsing each line into 4 numbers so we can perform comparisons on them. 6 lines - I wonder if we can do better?

+ **Part 1**: For each pair, parse the 4 values and check if either range is fully contained within the other.  
    `⌛O(n)` | `📦O(1)`, where n is the number of elf pairs.  
+ **Part 2**: For each pair, parse the 4 values and check if the two ranges overlap.  
    `⌛O(n)` | `📦O(1)`, where n is the number of elf pairs.

### Day 5

Split the input into two parts - the crate diagram and a list of instructions. We parse the crate diagram into a 2d array of vectors and use the inner ones as stacks as we only ever touch the top elements in a single operation. The instructions are parsed in a fixed format and then applied to the stacks to move elements.

We could make this more efficient by avoiding the temporary array in Part 2. If only `get_many_mut` was in Stable.

+ **Part 1 (Solve 1)**: As above, while ensuring that multiple crates moved in a single command end up on the *opposite* order on the destination stack.  
    `⌛O(s + i)` | `📦O(s)`, where s is the number of squares in the crate diagram and i is the number of instructions.
+ **Part 2 (Solve 1)**: As above, while ensuring that multiple crates moved in a single command end up on the *same* order on the destination stack.  
    `⌛O(s + i)` | `📦O(s)`, where s is the number of squares in the crate diagram and i is the number of instructions.

### Day 6

This day is about finding runs of 4 and 14 unique characters in a string, as well as the index of this run. My second solution for Part 2 was the most interesting an runs in 60% of the time as the first.

+ **Part 1**: Compare all characters in a 4-wide sliding window over the input (6 comparisons) and return the index of the found with unique characters.  
    `⌛O(n·m^2)` | `📦O(1)`, where n is the length of the input and m is the length of the run to find.
+ **Part 2 (Solve 1)**: Iterate over each character in each 14-char sliding window and track if it was already found in this window using a fixed-size (26 element) array used as a lookup table. Return the index of the window with no duplicates.  
    `⌛O(n·m)` | `📦O(1)`, where n is the length of the input and m is the length of the run to find.
+ **Part 2 (Solve 2)**: Slide the window across the input, but only handle two the two chars on the edges on each slide/iteration. We use lookup to track how many times we've seen each character and a counter so we know when we have no duplicates in our window.  
    `⌛O(n)` | `📦O(1)`, where n is the length of the input and m is the length of the run to find.

### Day 7

The directory traveral commands in our input are the result of a depth-first search of an arbitrary directory tree. Using this assumption, we can simplify the traversal tracking logic to only consider when we actually change directories (the names don't matter at all) and the size of files in a directory.

+ **Part 1**: Sum all directories which contain files (directly, or indirectly in child directories) with a combined size of <= 100,000.
    `⌛O(n)` | `📦O(m)`, where n is the number of directories we have to traverse, and m is the depth of the directory tree.
+ **Part 2**: Calculate and store the sizes of all directories in an array. Then we determine how much additional space we need (based on the size of the root directory and known constants) and proceed to find the smallest single directory which is larger that this.
    `⌛O(n)` | `📦O(n)`, where n is the number of directories we have to traverse.

### Day 8

This day is about finding adjacent trees and distances from other trees. We start by parsing the input into a 2d array of heights (stored as bytes) and then proceed to cast rays to find visibilities and scores.

An alternate solution for Part 1 (and the most obvious) would have been very similar to part 2 - alongside the scary complexity.

My solution for Part 2 has scary complexity due to casting 4 rays from each and every tree. It feels like there should be a better solution... Maybe for each tree we could store the distance to a tree of each possible height in each direction, then cast a ray from each cardinal direction to calculate the values and a final pass to calculate the top scenic score 🤔. If that worked, the time complexity would be reduced to `O(4n + n) = O(n)`.

+ **Part 1**: Start with all trees marked as hidden, then cast rays from the edges along all 4 cardinal directions and mark the visible trees based on their height and if they are obsured by taller trees along the ray's path.  
    `⌛O(n)` | `📦O(n)`, where n is the number of trees
+ **Part 2**: For each tree on the grid we cast a ray in each of the 4 cardinal directions to calculate it's scenic score.  
    `⌛O(n^3)` | `📦O(n)`, where n is the number of trees

### Day 9

Simulating rope! We track the 2D positions of each rope knot and update them on each movement.

We simulate every single step of movement and every square moved but I haven't found a way to avoid this - especially for the 10-knot rope in Part 2.

+ **Part 1**: Simulate the head movements and check if the tail is still adjacent. If not we move the tail to the last head position.  
    `⌛O(n)` | `📦O(1)`, where n is the number of head movements.
+ **Part 2**: Simulate the head movements and iteratively update the tail positions if they are too far from the head. We use a clamped vector (-1,1 for x and y) to move each component adjacent to it's parent again.  
    `⌛O(n·m)` | `📦O(1)`, where n is the number of head movements and m is the tail length.

### Day 10

TODO

+ **Part 1**: TODO  
    `⌛O(n)` | `📦O(1)`, where TODO
+ **Part 2**: TODO  
    `⌛O(n)` | `📦O(1)`, where TODO