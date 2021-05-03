Here are some notes on the more interesting puzzles from this year, which was the first one I
tackled.

## Day 10: Monitoring station

https://adventofcode.com/2019/day/10

### Part one

This was maybe the first really interesting problem. I have been attempting to approach all puzzles
in a pure way, so I was keen to find a perfect solution. To do this I would need to avoid using
floating point operations, which famously should not be compared for equality under most
circumstances. I also suspected that there might be some deliberate traps around this.

To compare the angle of two points I instead convert a relative position, a vector (x, y), into
another vector (x1, y1), by dividing each coordinate by the greatest common divisor of both values,
along with the value of the greatest common divisor. All points along the same line will share the
same value for (x1, y1), and the other value gives us the relative distance along that line. The
magnitude of these vectors and distances will not be consistent, but this doesn't matter for our
purposes.

Once we can do this operation we just need to iterate through and find the best option.

### Part two

The algorithm built for part one gives us a good base for this part as well. We need to add the
ability to sort the directional vectors in the right order. Again, I wanted to do this in a "pure"
way, so I came up with a simple algorithm to compare these directly. First we classify each one
into 4 directions, up, down, left and right, and the 4 sets of angles in between. I define these in
the order the puzzle requires, which means I can rely on rust's derived ordering for ordering the
classification itself. I added an extra one for zero, but I could also have just panicked if a zero
vector was evaluated in this way.

I then needed to order directional vectors in the diagonal categories. By scaling the vector so
that one coordinate matches, the other can be compared directly, since we are looking at two points
on the same line. To do this, I multiply each coordinate by the opposite coordinate in the other
vector. We only do this once, but to illustrate why this works we can look at what the full vectors
would be. If our vectors to be compared are (x0, y0) and (x1, y1), then we multiply the left vector
by y1 to get (x0·y1, y0·y1), and we multiply the right vector by y0 to get (x1·y0, y1·y0). We can
easily see that the y value for both these is the same, so comparing the resulting x coordinate
alone gives us the same ordering. Better yet, this works for all four quadrants, since negative
values invert the ordering and combine to cancel out in exactly the right way.

It would be an easy trap to simply sort the asteroids based on their magnitude followed by the
direction, but this is not correct. For example, some directions may have their first asteroid at a
distance which evaluates to magnitude 1, and another may have the first with magnitude 2. Both of
these should appear in the first rotation, so they should be treated the same. Instead, we collect
the asteroids in each direction into a list, then use the index in that list as the first part of
our search key.

## Day 12: The n-body problem

https://adventofcode.com/2019/day/12

### Part one

This part was relatively simple. Just encode the rules and run the simulation.

### Part two

This was the first problem which stumped me for a while, rather than making constant progress.

The second part becomes a lot more interesting, because the number of iterations required may be
very large. I ran it this way anyway to start with, but indeed found that it was taking a long
time. Obviously we need to find some trick to optimize the process.

Each moon's position is updated on every iteration, and every one contributes to every value the
next time round, so it seems we have to run it every time. After some more consideration I realised
that each component of the position evolved independently - for example the next value of a moon's
x only depends on all the x values from the previous iteration, but not the y or z.

We can now idependently evaluate how many iterations it will take for each coordinate to start
repeating, and also how frequently it will repeat. Once we have this information, we can work out
how many iterations it will take for them to all repeat together. I use the lowest common multiple
for this, which gives the right answer for the puzzle. I think this is only guaranteed to be
correct if the first value to be repeated is the initial value, and it looks like this is the case
for this data, so I just added a check to make sure this is true.

## Day 13: Care package

https://adventofcode.com/2019/day/13

### Part one

Part one was quite easy, just keep track of which character is printed where on the screen and then
count the number of blocks at the end.

### Part two

I decided to have some fun now, and make some use of terminal support for unicode, and found
characters for the ball "⚽", and blocks "📦". This produces some really nice output! I did create
an interface to control the paddle manually, but it turns out this is really difficult because the
paddle is tiny. Obviously it would be quite difficult to break such a large number of blocks this
way.

Instead, it should be fairly easy to control the paddle automatically, by setting the joystick
based on the relative positions of the paddle and the ball. This does indeed work, and with a nice
delay of one tenth of a second between updates we can see the game playing itself. It's obviously
going to take a long time at this speed, so I made it run a lot faster, and we get the right
answer.

## Day 14: Space stoichiometry

https://adventofcode.com/2019/day/14

### Part one

This is an interesting problem. The reactions form an acyclic graph, so we can find an order to
evaluate them which guarantees we only need to perform each one once. I can use the same algorithm
I used before for a dependency injection framework. I start with a list containing only the
starting chemical, ORE, then build up the list by adding all output chemicals from reactions which
have all their inputs already listed.

We then need to reverse this order, because the initially known quantity is the final output. We
can then work through the rections backwards, summing the amount of each chemical required, until
we have a final figure for the amount of ORE needed.

### Part two

Having come up with such a thorough solution for Part one, the second part is quite easy. We just
need to use the right value for the FUEL output, instead of hard-coding it to 1.

## Day 15: Oxygen system

https://adventofcode.com/2019/day/15

### Part one

This was a fun puzzle to solve. Path finding is fairly simple if you understand the algorithms. By
using a breadth-first search we can completely ignore the number of steps taken, since our search
is guaranteed to find the shortest path (or one of them) before any longer paths.

### Part two

My optimal solution to part one is not going to work any more. This time I needed to discover the
full map. This was a relatively small modification, however. Once we have this information we can
use basically the same algorithm we used to walk the map to fill it with oxygen. This time we need
to count the steps, of course, since this is the answer the puzzle asks for.

## Day 16: Flawed frequency transmission

https://adventofcode.com/2019/day/16

### Part one

Part one is relatively simple, we just need to implement the algorithm as described in the puzzle
description.

### Part two

This was the second puzzle which stumped me for a while. Fortunately after leaving it to tick over
in the back of my mind for a while a solution came to me.

Although I suspected it would be too slow, I first used the same method to try and find the answer.
This was indeed much too slow, so I was going to have to find a better solution. I next reasoned
that due to the presence of many zeros in the pattern, not all calculations would be relevant to a
single answer, so I implemented a recursive evaluator. This was also too slow, although I neglected
to implement caching to prevent evaluating the same position twice. Perhaps this would have been
enough?

The solution I came up with still takes a long time, even in an efficient language, so I strongly
suspect there is a better way, but it still gives me an answer in about 90 seconds. I spent a long
time staring at the patterns generated for each output digit, and looking for a relationship
between them. The only pattern which really stood out was that the second half of the output
consisted entirely of a number of zeros, followed by a number of ones. These, at least, were easy
to calculate, by taking the sum of all digits in a trailing range of the input. This can easily be
optimised by processing the list in reverse order.

This doesn't work for the first half, since we start to see extra ranges after the first series of
ones in the pattern. thought for a long time that there would be some trick to make it work, and I
think this may well have been the case if the patterns weren't offset by one. Eventually I gave up
on this approach, and instead decided to iteratively add and remove trailing sums at different
points until I got the right answer. This worked, after some considerable execution time, but if I
had been using an interpreted language, I think this would probably have been impractical.

## Day 17: Set and forget

https://adventofcode.com/2019/day/17

## Part one

Yet again, the first part was fairly simple. Just collect the output, count the number of adjacent
positions with scaffold for each position in the input, carefuly avoiding those on the edges, and
we get the correct answer.

## Part two

The second part is somewhat more difficult, and this is the third problem which stumped me for a
while.

I started out assuming that the robot would have to walk in one direction as far as it could before
turning. I approached this as a compression problem, looking for repeating patterns in the input
and replacing them with function calls. I could work out the exact number of characters saved
easily enough, but the optimal solution for all three functions might not include the longest
pattern which could fit in one function, so we also need to consider less optimal candidates for
each function, at least for the first two. This was fast enough to run to completion, but it didn't
find a solution. I now know that this should have found a solution if the algorithm was correct, so
obviously I made a mistake somewhere. Perhaps the time taken would have been too long if I had done
this correctly.

When I didn't find a solution, I considered that maybe I should try turning at different points.
This would give us many different full sequences to try and compress. This was too slow to give me
an answer in a reasonable amount of time, so I gave up on this approach.

After stewing on the problem for a while I realised that I had failed to consider an important part
of the puzzle as described. The main function could not include any direct commands, only function
calls. With this information, we can drastically reduce the search space. We know with certainly
that one of the functions matches a prefix of the full command sequence, and that the same one or
another matches a suffix. We can decide that function A will match the start, and iterate through
all prefixes which are in the range allowed by the function size limit.

We do the same for functions B and C in a loop, moving through the full command sequence whenever
we detect a sequence of commands which matches a function we have defined, building up a list of
the matched functions' names as our main sequence. When we don't have a match, we try defining the
next function based on prefixes of the remaining commands, again. I think it's possible to
construct a pattern which wouldn't match this way, if there were certain types of partially
repeating patterns in the input, but it turns out this gives us an answer and it runs quickly.

## Day 18: Many-worlds interpretation

https://adventofcode.com/2019/day/18

### Part one

This was a great puzzle, and the complexity took me by surprise.

As usual, I followed a "pure" approach, and avoided making any assumptions that I didn't think were
obviously sound. I model the full state of the labyrinth, including the position of the
key-collector, and iterate through the reachable keys to form a set of next steps. I evaluate these
in a loop until I have searched through all possible key orders.

To my surprise, this didn't look like it would complete in a reasonable time. In fact, even the
third example was taking far too long. I added caching, to short-circuit the evaluation if two
orders resulted in an equivalent number of steps to reach the same board state. I was careful to
avoid a possible trap, which was to only consider the keys here, since the character position is
also important.

This was still pretty slow, although I think it would have completed. I added an extra optimisation
step before searching for a solution, by pre-calculating the distances between the starting point,
all the doors and all the keys, and iterating through these instead of taking individual steps.

The final solution takes a respectable half of one second to complete.

### Part two

Due to my diligence in solving the first part of this problem, modelling the state of the whole
board, including the position of the key-collector, this was fairly trivial. I simply needed to
track four positions, instead of one, and consider the next moves for each of these while iterating
through my search for the solution.

I also had some fun with this, rendering the puzzle nicely using double-widh unicode, 256-colour
support in the terminal, and also shading anywhere that was a dead-end with no keys to visualise
the problem more clearly.

The solution is still found quickly, taking a little over half of one second on my machine.

## Day 19: Tractor beam

https://adventofcode.com/2019/day/19

### Part one

This was super easy. Just run the program 2500 times and add up the matches.

### Part two

This one was going to take more computing power if we checked every possible point. Instead I only
tested one out of every 100 points in both directions, starting at (99, 99). If this is false then
we know that none of the 10,000 points in the square from (0, 0) to (99, 99) can possibly be the
top-left corner of a match, since they would all include (99, 99) in their range.

If we find one of these points does match, then we need to check those points. We can reduce the
work we have to do further by checking points spaced 10 apart, and applying the same logic to each
10x10 grid. When testing an individual origin point, we can speed things up by testing the four
corners first, since these are the most likely to be false assuming that the beam has the kind of
shape described in the puzzle.

Finally, I use a cache to prevent evaluating the same point twice, since many of the squares we
test will overlap. I also start by only evaluating the first 100x100 square, and increasing the
dimensions of the search by 100 until a match is found. Caching ensures that we don't need any
special logic to make this fast, although it could probably be slightly quicker.

The puzzle description is vague about the definition of "closest", but I sort the found points by
the sum of the squares of the coordinates. Since the geometric distance would be the square root of
this value, which is an operation that preserves the order, this should genuinely find the closest
point from those it looks at.

Since the search pattern is a square, whereas equidistant points form a circle, we may find a point
which is closer to a 45° angle on one iteration of our search size, when a closer point at an angle
which is further from 45° exists in a subsequent iteration. Since the puzzle description was vague
about how to calculate the distance I assumed that there should be no ambiguity here, and indeed
the answer my code produced was accepted.

This runs in a little over one second on my machine.

## Day 20: Donut maze

https://adventofcode.com/2019/day/20

### Part one

This is fairly similar to day 17, except that we have a little more complexity parsing the input.
Because we have to consider portals as well as regular steps, and I had already come up with logic
to precalculate the paths through a maze, I did the same here rather than try and calculate it with
individual steps.

### Part two

Again, having implemented part one thoroughly, the changes needed here were quite small. Along with
each path I need to model the layer changes, using -1 for outer portals which take you to an outer
layer, 0 for paths through the maze between portals, and 1 for inner portals which take you to an
inner layer. When evaluating paths I need to track the layer changes, prevent passing through an
outer portal from the outer layer, and only consider solutions arriving at the end point in the
outer layer.

## Day 21: Springdroid Adventure

https://adventofcode.com/2019/day/21

### Part one

To start with I built an engine which would generate a 16 bit programme ID to a specific jump
register value, with each bit in the programme ID corresponding to its own bitwise value of the
sensor registers. For example, programme 0000100000100010 has a 1 bit in positions 1, 5 and 11,
which are 1, 101 and 1011 in binary, so correspond to the coming up ground looking like "...#",
".#.#" and "#.##".

This programme would produce a sequence of tests for each positive pattern. Each sequence starts by
setting T to zero with something like "NOT A T, AND A T", ORing all the registers which were not
set in the pattern, inverting T, then ANDing all the registers which should be clear. This would
then be ORed into J to combine the patterns. Unfortunately, I had not read the puzzle description
properly, because it turned out there was a limit of 15 instructions. This was clearly not going to
work.

I then turned to a more brute-force approach. I had an idea that the most logical way to write a
programme for this limited architecture would involve setting T to some known value, ANDing and
ORing it with various sensor registers, perhaps inverting it at some point, then combining it into
J with an AND or an OR, and then repeating this process for an arbitrary number of expressions.

I built an iterator over programmes which looked like this, which would specifically never repeat a
register in an expression, with the option to limit the number of registers in each expression, and
with a configurable number of expressions. I can't remember exactly how many there were, but it was
in the region of 1-2k expressions which would then be combined a set number of times. A single
expression didn't seem to work, but with two I managed to solve the problem in a reasonable time.

### Part two

This was the fourth puzzle to cause me considerable difficulty, with many periods where I made no
progress at all.

Given my brute force attempt which worked (albeitly slowly) for part one, I tried the same approach
again. It soon became apparent that this wasn't going to work this any more...

I knew that there was probably a less computationally-expensive way to approach this, ie to write
the springbot machine myself based on knowledge of the kind of patterns I would see in the
programme output, but I wanted to tackle these puzzles with the idea that the only knowledge used
was what was present in the puzzle description, rather than write code based on information gleaned
from the input value. I was already interested in genetic search algorithms, but had never actually
used this technique, so this seemed like a good way to go.

I needed to evaluate each candidate, but I still had some serious constraints on execution time -
running the intcode programme every time was unlikely to be efficient. Also, I needed a "fitness"
function for my programme candidates, but it was difficult to get a fitness function which returned
anything but true/false using the intcode programme itself. I could try and find a memory location
which described the distance travelled, but I felt this was not in the spirit of the puzzle as
described.

The virtual machine implemented by the intcode programme is trivially simple, so the obvious way to
make this more efficient, given that I am using a very efficient language, was to implement the
same architecture myself. This is done in the emulate function, which executes a springdroid
programme once, and the simulate function, which tests a programme against a sample stretch of
ground and checks if we make it to the end. I can pull these samples from the programme output when
a run fails, building up a collection of samples I can use to ensure I never fail twice in the same
place. Of course, I "compile" the assembly into my own structs, to make this as efficient as
possible. Now I could create a fitness function based on a number of ground samples which would
check how many of those samples were passed without issue. I added a secondary fitness score based
on the programme length, to prefer shorter programmes so long as they succeed with the same number
of samples.

I made my code start with an empty list of candidate programmes and ground samples, but I had a few
problems, so I experimented with adding some samples I had already gathered, and with a basic
candidate "NOT A J", ie jump just before any hole. After tweaking my parameters a bit I found I
could get rid of these and remain true to the spirit of the problem. I also added "rayon" as a
dependency to get a bit more speed out.

I'm really happy with my solution. This frequently completes in less than a minute on my machine,
which is fine for the purposes of solving the puzzle. I notice that sometimes it seems to get
stuck, so perhaps I fall in to some kind of "local minimum", but I have enough randomness that I
think the search will always make progress eventually. In any case, I can just interrupt the
programme and start it again.

## Day 22: Slam shuffle

https://adventofcode.com/2019/day/22

### Part one

Part one doesn't pose any major challenges. I just needed to parse the input, and apply the changes
listed.

### Part two

Part two was great fun, and I'm proud to say that I didn't get stuck, although it did take me a
considerable number of iterations to complete. I modified my code from part one to operate on a
list of positions for each card, instead of a list of cards in each position. I created a helper
function to translate between the two representations, so that I could use the same data for my
unit tests, taken from the examples in the puzzle description. Finally I built a function to
reverse a list of operations, given a specific deck size.

At one point I think that my tests were passing, even though I had not included the translation
between the two representations at the start or end, nor had I reversed the order of the
operations. I am not entirely sure what state the code was in, but I have a feeling there is some
kind of symmetry here which could be used to solve this puzzle without modifying the code to the
same extent...

In any case, I pushed forward and made the tests apply the correct logic. I noticed that the new
operations were basically combinations of add and multiply with modulo of the deck size.
Technically the "cut" operation is subtraction instead of addition. While subtraction under modulo
can be converted to positive addition, this is not possible without knowing what the modulo is.
Since my representation closely modeled the problem as described, with the list of operations not
including any knowledge of the desk size, I had to keep the signed argument to my "cut" operation,
but I still kept it named as addition, and implemented it as addition in my new code. This was a
confusing bug for a while, and I went through a lot of debugging before finding where the problem
lay.

I also ran into at least one overflow bug, which was frustrating because I knew this was a
potential trap and had taken steps to avoid it. The reason I ran into the issue anyway is because I
was modifying code from part one, which was written when I knew the deck size would be small and so
I wouldn't have to deal with this problem. Unfortunately, while modifying my code I didn't apply
the right logic everywhere, and this required some more fiddly debugging to weed out. The lesson I
will take away from this is that if requirements like this change, it may be better to rewrite a
unit rather than modify it, to ensure that you are using the correct assumptions. Of course, before
you do this you should refactor the API to the existing code, and write plenty of tests - if the
API is in a good place then the tests themselves should not need to be rewritten.

It took a few optimisation steps to make this run quickly. I knew that all these operations under
modulo could be collapsed down into a single add and a single multiply. I created a function to
look at a list of operations and produce an equivalent list with only two operations, first
multiply, then add. At one point I had to get a pen and some paper to work out how to combine
operations, although all I ended up writing was "(x + a) × b = x·b + a·b".

Even with this optimisation, the number of iterations was too high to run this in a loop. I already
had my general-purpose optimisation function. I decided to apply the operation in a loop until the
number of remaining operations was a multiple of 1024, divide it by this number, then optimize the
list of operations the same number of times. By repeating this process until the remaining
operations reached zero, I quickly came up with the correct solution.

## Day 24: Planet of discord

https://adventofcode.com/2019/day/24

### Part one

This was pretty a pretty easy cellular automaton implementation - I used to play around with these
as a teenager so there was no particular challenge for this part.

Anticipating another optimisation challenge in part two, I created a super-efficient implementation
storing the state of the entire bit in a u32. Having completed part two, this was probably not
necessary, but it didn't add much complexity so I am happy with my code.

### Part two

This should have been fairly simple, but I ran into a very annoying bug which took me far too long
to track down. When modifying my code from part one, I accidentally deleted the logic which applied
the rule for adding bugs to squares. I re-did this from memory, and tested for 2 and 3 instead of 1
and 2, as stated in the problem description. I think this is the rule from Conway's game of life,
so it looks like my previous experience worked against me this time.

My solution still didn't add up, so I printed the final board state and counted the bugs. Oddly,
the number of bugs was correct, but the sum was giving me a different figure for some reason. I
quickly realised that I was applying my logic to the middle square, but neglecting to ignore this
when counting bugs. My display function did have a special case for the middle square, which is why
counting them manually gave the right answer. With this fixed, I got the correct solution.

## Day 25: Cryostasis

https://adventofcode.com/2019/day/25

My solution to the final puzzle was dramatically over-engineered. That said, I enjoy implementing
this kind of complex state machine, so I am happy with what I produced. I did end up compromising
on my "pure" approach, by hard-coding the names of two items to avoid picking up, namely the "giant
electromagnet" and the "infinite loop". I could have come up with code to detect both of these, but
it seemed to me that I already had to include so much of the programme output in my game-playing
engine that it wasn't worth worrying about this.

In retrospect, an ideal approach would have been to create an interactive version of this, along
with two extra features. The first would allow you to restore the machine to a previous state. This
could be used after picking up an item which should be avoided. The second would iterate through
all combinations of carried items and trying to move in a given direction until the move was
successful, which would of course be used to easily pass the security checkpoint, having collected
all the items.
