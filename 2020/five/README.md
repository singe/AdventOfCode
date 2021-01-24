I don't know why but I was quite enamoured with this challenge.

I solved it in three different ways:

1) A simple algorithm that divides length by two repeatedly, and if there is a B or R then add the length to the base to get the row and column.

2) Doh, it's binary, just replace the F & L with 0 and the B & R with 1. But that's slower than (1) hmm.

3) I couldn't stop thinking about it and realised first that since we're multiplying the row by 8, it's actually just a single binary number. Second, that we could more efficiently convert the string to binary because the 3rd LSB on the various characters can tell us if we need a 1 or 0. This was the fastest.

Then I made a C and x86_64 asm version.
