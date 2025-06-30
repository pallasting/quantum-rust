# Quantum Rust 示例

## 基础使用

```rust
fn main() {
    println!("Hello, Quantum World!");
}
```

## 量子数据结构

```rust
use quantum::prelude::*;

fn main() -> QuantumResult<()> {
    let mut qvec = QuantumVec::new();
    qvec.quantum_push(42)?;
    Ok(())
}
```

## 量子算法

```rust
use quantum::algorithms::*;

fn main() -> QuantumResult<()> {
    let data = vec![1, 2, 3, 4, 5];
    let result = quantum_fft(&data)?;
    println!("QFT: {:?}", result);
    Ok(())
}
```
