# Poker VRF
A solution for the activity as a part of the Polkadot Blockchain Academy, Cambridge 2022 Cohort.
The solution basically simulates a texas holden poker game utilising [Verifiable Random Functions](https://en.wikipedia.org/wiki/Verifiable_random_function).

Navigate to `poker.rs` to see the code.

# Limitations
* Card counting is not possible at the moment, duplicates are allowed
* Card are represented as integers 1 -> 52, the winner is deduced by calculating points from from both hands.
* Hence, duplicates are allowed