# Quantum Rust API 参考

本文档提供 Quantum Rust 所有 API 的详细参考。

## 📋 模块概览

- [`quantum::prelude`](#quantumprelude) - 常用类型和函数
- [`quantum::vec`](#quantumvec) - 量子向量
- [`quantum::matrix`](#quantummatrix) - 量子矩阵  
- [`quantum::map`](#quantummap) - 量子哈希表
- [`quantum::algorithms`](#quantumalgorithms) - 量子算法
- [`quantum::ml`](#quantumml) - 量子机器学习
- [`quantum::arrow`](#quantumarrow) - Arrow 数据结构
- [`quantum::fft`](#quantumfft) - 量子傅里叶变换

## `quantum::prelude`

预导入模块，包含最常用的类型和函数。

```rust
use quantum::prelude::*;
```

### 重新导出

- `QuantumVec<T>` - 量子向量
- `QuantumMatrix<T>` - 量子矩阵
- `QuantumHashMap<K, V>` - 量子哈希表
- `QuantumResult<T>` - 量子操作结果类型
- `QuantumError` - 量子错误类型

## `quantum::vec`

量子增强的向量类型，提供高性能的并行操作。

### `QuantumVec<T>`

```rust
pub struct QuantumVec<T> {
    // 内部实现
}
```

#### 构造函数

```rust
impl<T> QuantumVec<T> {
    /// 创建新的空量子向量
    pub fn new() -> Self
    
    /// 创建指定容量的量子向量
    pub fn with_capacity(capacity: usize) -> Self
    
    /// 从普通向量创建量子向量
    pub fn from_vec(vec: Vec<T>) -> Self
    
    /// 创建超位置状态的向量
    pub fn superposition(size: usize) -> Self
    where T: Default + Clone
}
```

#### 基础操作

```rust
impl<T> QuantumVec<T> {
    /// 添加元素到向量末尾
    pub fn quantum_push(&mut self, value: T) -> QuantumResult<()>
    
    /// 移除并返回最后一个元素
    pub fn quantum_pop(&mut self) -> Option<T>
    
    /// 获取指定索引的元素
    pub fn quantum_get(&self, index: usize) -> Option<&T>
    
    /// 获取向量长度
    pub fn len(&self) -> usize
    
    /// 检查是否为空
    pub fn is_empty(&self) -> bool
    
    /// 清空向量
    pub fn clear(&mut self)
}
```

#### 量子并行操作

```rust
impl<T: Send + Sync> QuantumVec<T> {
    /// 量子并行映射
    pub fn quantum_map<U, F>(&self, f: F) -> QuantumResult<QuantumVec<U>>
    where
        F: Fn(&T) -> U + Send + Sync,
        U: Send + Sync,
    
    /// 量子并行过滤
    pub fn quantum_filter<F>(&self, predicate: F) -> QuantumResult<QuantumVec<T>>
    where
        F: Fn(&T) -> bool + Send + Sync,
        T: Clone,
    
    /// 量子并行归约
    pub fn quantum_reduce<F>(&self, identity: T, op: F) -> QuantumResult<T>
    where
        F: Fn(T, &T) -> T + Send + Sync,
        T: Clone,
    
    /// 量子并行查找
    pub fn quantum_find<F>(&self, predicate: F) -> QuantumResult<Option<&T>>
    where
        F: Fn(&T) -> bool + Send + Sync,
}
```

#### 示例

```rust
use quantum::prelude::*;

fn main() -> QuantumResult<()> {
    let mut qvec = QuantumVec::new();
    
    // 添加元素
    for i in 0..1000 {
        qvec.quantum_push(i)?;
    }
    
    // 量子并行映射
    let squared = qvec.quantum_map(|&x| x * x)?;
    
    // 量子并行过滤
    let evens = qvec.quantum_filter(|&x| x % 2 == 0)?;
    
    // 量子并行归约
    let sum = qvec.quantum_reduce(0, |acc, &x| acc + x)?;
    
    println!("Sum: {}", sum);
    Ok(())
}
```

## `quantum::matrix`

高性能量子矩阵实现。

### `QuantumMatrix<T>`

```rust
pub struct QuantumMatrix<T> {
    // 内部实现
}
```

#### 构造函数

```rust
impl<T> QuantumMatrix<T> {
    /// 创建指定大小的矩阵
    pub fn new(rows: usize, cols: usize) -> Self
    where T: Default + Clone
    
    /// 创建零矩阵
    pub fn zeros(rows: usize, cols: usize) -> Self
    where T: Default + Clone
    
    /// 创建单位矩阵
    pub fn identity(size: usize) -> Self
    where T: Default + Clone + From<i32>
    
    /// 从二维向量创建矩阵
    pub fn from_vec(data: Vec<Vec<T>>) -> QuantumResult<Self>
}
```

#### 基础操作

```rust
impl<T> QuantumMatrix<T> {
    /// 获取元素
    pub fn get(&self, row: usize, col: usize) -> Option<&T>
    
    /// 设置元素
    pub fn set(&mut self, row: usize, col: usize, value: T) -> QuantumResult<()>
    
    /// 获取行数
    pub fn rows(&self) -> usize
    
    /// 获取列数
    pub fn cols(&self) -> usize
    
    /// 获取形状
    pub fn shape(&self) -> (usize, usize)
}
```

#### 量子矩阵运算

```rust
impl<T> QuantumMatrix<T>
where
    T: Clone + Default + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Send + Sync,
{
    /// 量子矩阵乘法
    pub fn quantum_multiply(&self, other: &Self) -> QuantumResult<Self>
    
    /// 量子转置
    pub fn quantum_transpose(&self) -> QuantumResult<Self>
    
    /// 量子行列式计算
    pub fn quantum_determinant(&self) -> QuantumResult<T>
    where T: std::ops::Sub<Output = T>
    
    /// 量子逆矩阵
    pub fn quantum_inverse(&self) -> QuantumResult<Self>
    where T: std::ops::Div<Output = T> + std::ops::Sub<Output = T> + PartialEq
}
```

## `quantum::algorithms`

量子算法实现。

### 量子搜索

```rust
/// 量子搜索算法 - O(√n) 复杂度
pub fn quantum_search<T, F>(data: &[T], predicate: F) -> QuantumResult<Option<usize>>
where
    T: Send + Sync,
    F: Fn(&T) -> bool + Send + Sync,

/// 量子二分搜索
pub fn quantum_binary_search<T>(data: &[T], target: &T) -> QuantumResult<Option<usize>>
where
    T: Ord + Send + Sync,
```

### 量子排序

```rust
/// 量子快速排序
pub fn quantum_quicksort<T>(data: &mut [T]) -> QuantumResult<()>
where
    T: Ord + Send + Sync,

/// 量子归并排序
pub fn quantum_mergesort<T>(data: &mut [T]) -> QuantumResult<()>
where
    T: Ord + Clone + Send + Sync,
```

### 量子优化

```rust
/// 量子退火优化
pub fn quantum_annealing<T, F>(
    initial: T,
    cost_function: F,
    max_iterations: usize,
) -> QuantumResult<T>
where
    T: Clone + Send + Sync,
    F: Fn(&T) -> f64 + Send + Sync,

/// 量子梯度下降
pub fn quantum_gradient_descent<F>(
    initial: f64,
    gradient: F,
    learning_rate: f64,
    iterations: usize,
) -> QuantumResult<f64>
where
    F: Fn(f64) -> f64 + Send + Sync,
```

## `quantum::fft`

量子傅里叶变换实现。

### 函数

```rust
/// 量子快速傅里叶变换
pub fn quantum_fft(input: &[f64]) -> QuantumResult<Vec<Complex64>>

/// 量子逆傅里叶变换
pub fn quantum_ifft(input: &[Complex64]) -> QuantumResult<Vec<f64>>

/// 实数量子FFT
pub fn quantum_rfft(input: &[f64]) -> QuantumResult<Vec<Complex64>>

/// 二维量子FFT
pub fn quantum_fft2d(input: &[Vec<f64>]) -> QuantumResult<Vec<Vec<Complex64>>>
```

### 示例

```rust
use quantum::fft::*;

fn main() -> QuantumResult<()> {
    let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    
    // 量子FFT
    let fft_result = quantum_fft(&signal)?;
    println!("FFT result: {:?}", fft_result);
    
    // 量子逆FFT
    let ifft_result = quantum_ifft(&fft_result)?;
    println!("IFFT result: {:?}", ifft_result);
    
    Ok(())
}
```

## `quantum::ml`

量子机器学习算法。

### 量子主成分分析

```rust
/// 量子PCA
pub fn quantum_pca(
    data: &[Vec<f64>],
    components: usize,
) -> QuantumResult<Vec<Vec<f64>>>
```

### 量子聚类

```rust
/// 量子K-means聚类
pub fn quantum_kmeans(
    data: &[Vec<f64>],
    k: usize,
    max_iterations: usize,
) -> QuantumResult<Vec<usize>>

/// 量子层次聚类
pub fn quantum_hierarchical_clustering(
    data: &[Vec<f64>],
    linkage: LinkageMethod,
) -> QuantumResult<ClusterTree>
```

### 量子神经网络

```rust
pub struct QuantumNeuralNetwork {
    // 内部实现
}

impl QuantumNeuralNetwork {
    /// 创建新的量子神经网络
    pub fn new(layers: Vec<usize>) -> Self
    
    /// 量子训练
    pub fn quantum_train(
        &mut self,
        data: &[Vec<f64>],
        labels: &[f64],
        epochs: usize,
    ) -> QuantumResult<TrainingResult>
    
    /// 量子预测
    pub fn quantum_predict(&self, input: &[f64]) -> QuantumResult<f64>
}
```

## `quantum::arrow`

Arrow 数据结构集成。

### `ArrowTable`

```rust
pub struct ArrowTable {
    // 内部实现
}

impl ArrowTable {
    /// 创建新表
    pub fn new() -> Self
    
    /// 添加列
    pub fn add_column(&mut self, name: &str, data_type: ArrowDataType) -> QuantumResult<()>
    
    /// 插入行
    pub fn insert_row(&mut self, row: Vec<ArrowValue>) -> QuantumResult<()>
    
    /// 向量化过滤
    pub fn filter(&self, condition: &str) -> QuantumResult<ArrowTable>
    
    /// 向量化聚合
    pub fn aggregate(&self, column: &str, agg: ArrowAggregation) -> QuantumResult<f64>
    
    /// 零拷贝切片
    pub fn slice(&self, start: usize, length: usize) -> QuantumResult<ArrowTableSlice>
}
```

## 错误处理

### `QuantumError`

```rust
#[derive(Debug)]
pub enum QuantumError {
    /// 量子状态错误
    QuantumStateError(String),
    
    /// 维度不匹配
    DimensionMismatch { expected: usize, actual: usize },
    
    /// 内存不足
    OutOfMemory,
    
    /// 算法收敛失败
    ConvergenceFailure,
    
    /// Arrow 格式错误
    ArrowError(String),
    
    /// 集成错误
    IntegrationError(String),
}
```

### `QuantumResult<T>`

```rust
pub type QuantumResult<T> = Result<T, QuantumError>;
```

## 性能提示

### 编译器提示

```rust
// 启用量子优化
#[quantum_optimize]
fn my_function() {
    // 函数体
}

// 使用量子并行
#[quantum_parallel]
fn parallel_computation() {
    // 并行计算
}

// Arrow 优化
#[arrow_optimize]
fn data_processing() {
    // 数据处理
}
```

### 性能宏

```rust
use quantum::macros::*;

// 量子向量宏
let qvec = quantum_vec![1, 2, 3, 4, 5];

// 量子矩阵宏
let qmat = quantum_matrix![
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
];

// 量子FFT宏
let fft_result = quantum_fft!(signal);
```

---

**📚 这只是 API 的一部分！**

完整的 API 文档请访问：
- [在线文档](https://docs.quantum-rust.org)
- [本地文档](./generated-docs/index.html)

生成本地文档：
```bash
quantum-doc --generate-docs
```
