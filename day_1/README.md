# part 1
--------
the challenge for part one is as follows:

"""
The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?
"""

in order to solve this, I've opted to use a 2 pointer algorithm like all good leetcode noobs would. the jist of the alg is as follows:

for each string in inputs:
    1. set left pointer to 0, set right pointer to length of string
    WHILE left <= right AND (string[left] not numeric) AND (string[right] not numeric):
        1. if string left not numeric, left += 1
        2. if string right not numeric, right -= 1

from there, we can just concat string[left] and string[right] into one string, convert to int and we're golden. This was my first time ever using rust so the code is pretty jank but glad it at least runs and happy with the runtime for now - im sure it couldve been made to be much faster but for now I'll take it

# part 2
--------
oh my goodness fuck part 2. the challenge is updated as follows:
"""
Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?
"""

The real catch here (and there is unfortunately a catch) is that strings who share a character should both be converted i.e.
"oneight" should be treated as "18" NOT "1ight" or indeed "on8". this was bullshit as none of the examples covered it :( :( :(

to solve this, i simply needed a string cleaning function to wrap the input to my 2 pointer algorithm from part 1. the solution i went with was to wrap all string digits with itself like:

1 -> one1one
2 -> two2two

etc...

this is super goofy but it means that any shared letters between digits are preserved during the replacement. im certain there is a better solution to this problem and that im currently a laughing stock for this goofy ass solution but it worked and my hairline wasnt completely ruined by it so I'll take it tbh.

overall ive enjoyed learning some rust! but id like to focus on being a bit more rust native over the next few days - this code worked but for instance didnt utilise any packages etc...