# Advent Of Code 2022 - My Solutions in Rust

## Prerequisites
Install Rust: https://www.rust-lang.org/tools/install.  
Tested with `rustc 1.65.0`.

## Running
Run tests with `cargo test`.  
Run benchmarks with `cargo bench`.

## Solution Descriptions

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

TODO

+ **Part 1**: TODO  
    `⌛O(n)` | `📦O(1)`, where TODO
+ **Part 2**: TODO  
    `⌛O(n)` | `📦O(1)`, where TODO

### TEMPLATE

TODO

+ **Part 1**: TODO  
    `⌛O(n)` | `📦O(1)`, where TODO
+ **Part 2**: TODO  
    `⌛O(n)` | `📦O(1)`, where TODO