# Dangling Pointer in Rust Vector

This repository demonstrates a common error in Rust involving dangling pointers when working with vectors.  The `bug.rs` file contains code that creates a reference to a vector element, then modifies the vector in a way that invalidates that reference.  Attempting to access the invalid reference leads to undefined behavior (likely a panic or crash).  The solution is shown in `bugSolution.rs` which illustrates safe practices to prevent this error.

**How to reproduce:**
1. Clone this repository.
2. Navigate to the repository's directory.
3. Run `rustc bug.rs && ./bug` (expect a crash or undefined behavior).
4. Run `rustc bugSolution.rs && ./bugSolution` (expect a correct output).

This example highlights the importance of understanding Rust's ownership and borrowing system to write safe and reliable code.