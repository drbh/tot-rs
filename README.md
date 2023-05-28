tot-rs

A small/and unfinished exploration of the [Tree of Thoughts: Deliberate Problem Solving with Large Language Models](https://arxiv.org/pdf/2305.10601.pdf) paper in Rust and with the Google PALM API.

As shown below the bison API isn't very good at solving the problem or simple math in general, but it's still fun to play with.

```bash
cargo run
#    Compiling tot-rs v0.1.0 (/Users/drbh/Projects/tot-rs)
#     Finished dev [unoptimized + debuginfo] target(s) in 1.10s
#      Running `target/debug/tot-rs`
# [
#     "4 9 10 13",
# ]
# REQUEST_COUNTER: 1

# 4 + 9 = 13 (unused: 10 13) (new: 13) (left: 13 10 13)
# 4 * 9 = 36 (unused: 10 13) (new: 36) (left: 36 10 13)
# 10 / 4 = 2.5 (unused: 9 13) (new: 2.5) (left: 2.5 9 13)
# 13 - 10 = 3 (unused: 4 9) (new: 3) (left: 3 4 9)


# ======== step 0 ========
#   Proposals:
#     0: 4 + 9 = 13 (unused: 10 13) (new: 13) (left: 13 10 13)
#     1: 4 * 9 = 36 (unused: 10 13) (new: 36) (left: 36 10 13)
#     2: 10 / 4 = 2.5 (unused: 9 13) (new: 2.5) (left: 2.5 9 13)
#     3: 13 - 10 = 3 (unused: 4 9) (new: 3) (left: 3 4 9)
# REQUEST_COUNTER: 2

# 13 + 10 + 13 = 36
# (13 - 10) * 13 = 3 * 13 = 39
# 13 10 13 are all too big
# impossible

# REQUEST_COUNTER: 3

# 36 + 10 + 13 = 59
# 36 - 10 = 26
# 10 * 13 = 130
# 13 / 10 = 1.3
# impossible
# 10 10 10
# 10 + 10 + 10 = 30
# (10 - 10) * 10 = 0
# 10 10 10 are all too big
# impossible

# REQUEST_COUNTER: 4

# 2.5 + 9 + 13 = 24
# sure

# REQUEST_COUNTER: 5

# 3 + 4 + 9 = 16
# (9 - 4) * 3 = 5 * 3 = 15
# I cannot obtain 24 now, but numbers are within a reasonable range
# likely
# 10 10 10
# 10 + 10 + 10 = 30
# (10 - 10) * 10 = 0
# 10 10 10 are all too big
# impossible

#   Values:
#     0: 0.001
#     1: 0.002
#     2: 20
#     3: 1.001
#   Selected:
#     0: 10 / 4 = 2.5 (unused: 9 13) (new: 2.5) (left: 2.5 9 13)
#     1: 13 - 10 = 3 (unused: 4 9) (new: 3) (left: 3 4 9)
# request counter: 5
```
