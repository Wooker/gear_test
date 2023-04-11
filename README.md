# gear_test

This code implements a basic function to split some generic computational work between threads. The split occurs only if the input length exceeds a certain threshold. If the computational work (i.e., the input length) is shorter than this threshold, no splitting occurs, and no threads are created.

Inputs:

1. A Vec of type T
2. A function fn(&T) -> R

Returns:

1. A Vec of type R with the same length as the first input argument.
   
## Testing
```
cargo run
```
```
cargo test
```
