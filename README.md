# Boolean Formula Equivalence Checker
This is a WIP command-line application used to determine whether or not an arbitrary number of Boolean expressions are logically equivalent.

Currently, this is done through comparison by truth table; the expressions are parsed into an AST and will compute the result for each of the 2<sup>n</sup> cases. 
To mitigate this runtime cost, the program uses multithreading to split up the workload: 2<sup>n</sup> threads are spawned - one for each permutation. (Should also test to see if the overhead of spawning 2<sup>n</sup> threads is worth it for reasonably sized cases - it may just be better to have threads per expression instead)

## Plans for the future:
- Use a Reduced Ordered Binary Decision Digram (ROBDD) representation to achieve a better big-O runtime
    - This could also allow users to modify the expressions in place
- Migrate to a web app: use this as the backend and eventually build a React front-end to go with it