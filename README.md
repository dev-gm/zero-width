## Zero-Width
A turing-complete programming language using only zero-width unicode characters, inspired by brainfuck and whitespace.
Currently a (possibly incomplete) implementation for an interpreter has been written in rust, with more languages planned.

Specification:

- Cells are on an infinite 1-dimensional plane.
	- The plane starts of with length of 1, but expands as you move left or right farther than before.
- Each cell can hold an unsigned 8-bit integer.
	- If a cell overflows (goes beyond its max capacity), it resets to 0.
	- You can only increment a cell, if you wish to decrement you must increment until the cell resets to 0.

- The instructions are composed of the following zero-width unicode characters:
	- U+180E - move left
	- U+200B - move right
	- U+200C - increment
	- U+200D - input integer from STDIN and add it to current cell
	- U+200D x2 - print current cell to STDOUT
	- U+FEFF - while current cell is not 0, execute until next instance of U+FEFF

- All other characters are considered comments.
