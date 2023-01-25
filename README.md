# Boolean Formula Equivalence Checker
This is a WIP command-line application used to determine whether or not an arbitrary number of Boolean expressions are logically equivalent. I'm also using this to learn Rust!

Currently, equivalence is checked through comparison by truth table; the expressions are parsed into an AST and will compute the result for each of the 2<sup>n</sup> cases. 
To mitigate this runtime cost, the program uses multithreading to split up the workload: 2<sup>n</sup> threads are spawned - one for each permutation. 

As a back-of-the-envelope test, running with multithreading provides a significant speedup. With an artificial 5ms delay on each operation, the real time spent on an arbitrary run with n=8 is 32 seconds; the same parameters with multithreading takes 0.234 seconds. That being said, the sequential execution occasionally performs better when run without the artificial delay, most likely due to the overhead of spawning 2<sup>n</sup> threads. It may be worthwhile to consider creating threads per expression instead to see if that offers a better speedup; or perhaps a threadpool implementation would help mitigate that overhead.

## Plans for the future:
- Use a Reduced Ordered Binary Decision Digram (ROBDD) representation to achieve a better asymptotic bound
    - This could also allow users to modify the expressions in place
- Migrate to a web app: use this as the backend and eventually build a React front-end to go with it