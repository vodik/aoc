# Advent Of Code 2021

## Day 1

**822ns / 615ns**

TODO

## Day 2

**4.96µs / 5.15µs**

TODO

## Day 3

**1.23µs / 25.70µs**

TODO

## Day 4

**6.15µs / 10.129µs**

Highly tuned, there are several tricks deployed here to get this
performance.

Firstly, I don't run all the games at the same time, rather I consider
the boards one by one and run them individually. When I detect that a
game has been won, I note its score and on which turn it won (the
generation).

That framing reduced the puzzle to a simple min/max problem. The
solution to the first part is the score generated in the fewest turns
and score generated in the most turns for the second part.

To further optimize part one, I can track the lowest number of turns
known to complete a card and use that as to bound the simulation. If
we hit the limit without winning, we know we're not interested in the
card and can stop trying.

### Simulating Cards

When simulating cards, I first build a lookup map of numbers on the
card to their position using an array. Since I know the maximum number
that can be on a card is 99, it can efficiently be stored as an array
(`[Option<NonZerou32>; 100]`).

I track the board's mutable state separately as a bitmap
(`u32`). Storing it separately performs better for reasons I don't
completely understand yet.

## Day 5

**278.56µs / 479.30µs**

TODO
