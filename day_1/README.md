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

so the task is:
- for each line figure out the value
- sum all the values

I've done enough leetcode to feel like a 2-pointer solution will work for this but not enough to know if its the best method. jist of a plan then is to start on either end of the string, iterate forwards and stop iterating one pointer if its numeric, or if its equal to the other pointer. not sure what we'd do in the case where there are no numeric characters so for now i'll choose to make the assumption that there will always be at least one.
