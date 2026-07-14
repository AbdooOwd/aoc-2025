# Notes for Day 1 Puzzle of Advent of Code

- The mechanism can point at values from `0` to `99`
- `L` & `R` are for "left" & "right" rotations of the mechanism<br/>
  `L` means lower values (counter-clockwise)<br/>
  `R` means greater values (clockwise)
- `L` when pointing at `0` makes the new value `99`,<br/>
  and `R` when pointing at `99` makes the new value `0`
- **The password is actually the number of times `0` is pointer at
  after each rotation**
- **The mechanism starts pointed at `50`**

> Puzzle status: **DONE**!
> Answer was `1150`

## Part 2

- The password is actually how many times the dial points at `0`
  --whether during a rotation or after a rotation.
