# 2017 Sky Christmas Competition

> find `n` which produces the sum of divisors over `x` 

Sum of Divisors, Prime Factorisation, Highly Abundant Numbers

# The Challenge

## Problem

Leeds Dock has had a super productive year with many big wins, to reward the staff Renee has comissioned Santa's Elves to deliver gifts by hand to each desk.

For the sake of this exercise assume there are an infinite number of employees at Leeds Dock who sit at desks numbered sequentially: 
```1, 2, 3, 4, 5, and so on.```

Each Elf is assigned a number, too, and delivers presents to desks based on that number:

* The first Elf (number 1) delivers presents to every desk: ```1, 2, 3, 4, 5, ....```
* The second Elf (number 2) delivers presents to every second desk: ```2, 4, 6, 8, 10, ....```
* Elf number 3 delivers presents to every third desk: ```3, 6, 9, 12, 15, ....```

There are infinitely many Elves, numbered starting with 1. 

Each Elf delivers presents equal to ten times his or her number at the desks they make deliveries to.

So, the first nine desks end up like this:

```
Desk 1 got 10 presents.
Desk 2 got 30 presents.
Desk 3 got 40 presents.
Desk 4 got 70 presents.
Desk 5 got 60 presents.
Desk 6 got 120 presents.
Desk 7 got 80 presents.
Desk 8 got 150 presents.
Desk 9 got 130 presents.
```

The first desk gets 10 presents: it is visited only by Elf 1, which delivers 1 * 10 = 10 presents. 
The fourth desk gets 70 presents, because it is visited by Elves 1, 2, and 4, for a total of 10 + 20 + 40 = 70 presents.

## Question

What is the lowest desk number of the desk to get at least 50000000 presents?
