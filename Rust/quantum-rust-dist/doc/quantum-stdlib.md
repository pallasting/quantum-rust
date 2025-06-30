# Quantum Standard Library

The Quantum Standard Library provides quantum-enhanced versions of common
data structures and algorithms.

## Modules

### quantum::vec
Arrow-optimized vectors with quantum parallel operations.

### quantum::map
Quantum hash maps with entangled key-value relationships.

### quantum::algorithms
Quantum computing algorithms including FFT, search, and optimization.

### quantum::ml
Quantum machine learning primitives.

## Examples

```rust
use quantum::prelude::*;

let mut qvec = QuantumVec::new();
qvec.push(42);
let doubled = qvec.quantum_map(|x| x * 2);
```
