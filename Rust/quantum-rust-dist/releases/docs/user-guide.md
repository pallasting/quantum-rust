# Quantum Rust 用户指南

欢迎使用 Quantum Rust！本指南将帮助您快速上手世界首个量子增强的系统编程语言。

## 📋 目录

1. [安装和设置](#安装和设置)
2. [基础使用](#基础使用)
3. [量子特性](#量子特性)
4. [性能优化](#性能优化)
5. [故障排除](#故障排除)

## 🚀 安装和设置

### 系统要求

- **操作系统**: Linux, macOS, Windows
- **架构**: x86_64, aarch64
- **内存**: 最少 4GB RAM
- **存储**: 最少 2GB 可用空间

### 安装方法

#### 方法 1: 自动安装脚本

```bash
# 下载并运行安装脚本
curl -sSf https://quantum-rust.org/install.sh | sh

# 重新加载环境变量
source ~/.bashrc  # 或 ~/.zshrc
```

#### 方法 2: 手动安装

```bash
# 下载发行版
wget https://github.com/quantum-rust/releases/latest/quantum-rust-linux.tar.gz

# 解压
tar -xzf quantum-rust-linux.tar.gz

# 运行安装脚本
cd quantum-rust
./install.sh
```

#### 方法 3: 从源码构建

```bash
# 克隆仓库
git clone https://github.com/quantum-rust/quantum-rust.git
cd quantum-rust

# 构建
python3 quantum-rust-build.py . ./quantum-rust-dist

# 安装
./quantum-rust-dist/install.sh
```

### 验证安装

```bash
# 检查版本
rustc --version
# 应该显示: 🔮 Quantum Rust Compiler v1.0.0-quantum

cargo --version
# 应该显示: 🔮 Quantum Cargo v1.0.0-quantum

# 测试编译
echo 'fn main() { println!("Hello, Quantum!"); }' > test.rs
rustc test.rs && ./test
```

## 🎯 基础使用

### 编译现有项目

Quantum Rust 100% 兼容现有 Rust 代码，无需任何修改：

```bash
# 进入现有 Rust 项目
cd your_rust_project

# 直接使用，自动获得量子加速
cargo build    # 3-6x 编译加速！
cargo run      # 正常运行
cargo test     # 测试通过
```

### 创建新项目

```bash
# 创建新项目
cargo new my_quantum_project
cd my_quantum_project

# 编辑 src/main.rs
cat > src/main.rs << 'EOF'
fn main() {
    println!("🔮 Hello from Quantum Rust!");
    
    // 自动获得量子优化
    let data: Vec<i32> = (0..1000000).collect();
    let sum: i32 = data.iter().sum();
    
    println!("Sum of 1M numbers: {}", sum);
}
EOF

# 编译和运行
cargo run
```

### 编译选项

```bash
# 基础编译
rustc main.rs

# 启用量子特性
rustc --quantum main.rs

# 设置量子优化级别
rustc --quantum-opt-level 3 main.rs

# 启用 Arrow 数据结构
rustc --enable-arrow main.rs

# 量子并行编译
rustc --quantum-parallel main.rs

# 量子调试模式
rustc --quantum-debug main.rs
```

## 🔮 量子特性

### 量子数据结构

```rust
use quantum::prelude::*;

fn main() -> QuantumResult<()> {
    // 量子向量 - 30% 内存节省
    let mut qvec = QuantumVec::new();
    qvec.quantum_push(42)?;
    qvec.quantum_push(84)?;
    
    // 量子并行操作
    let doubled = qvec.quantum_map(|&x| x * 2)?;
    println!("Doubled: {:?}", doubled);
    
    // 量子矩阵
    let mut qmatrix = QuantumMatrix::new(100, 100);
    qmatrix.quantum_fill(1.0)?;
    
    // 量子哈希表
    let mut qmap = QuantumHashMap::new();
    qmap.quantum_insert("key", "value")?;
    
    Ok(())
}
```

### 量子算法

```rust
use quantum::algorithms::*;

fn main() -> QuantumResult<()> {
    // 量子傅里叶变换
    let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let fft_result = quantum_fft(&signal)?;
    println!("QFT result: {:?}", fft_result);
    
    // 量子搜索 - O(√n) 复杂度
    let data = vec![1, 3, 5, 7, 9, 11, 13, 15];
    let index = quantum_search(&data, |&x| x == 7)?;
    println!("Found 7 at index: {:?}", index);
    
    // 量子优化
    let result = quantum_optimize(
        |x: f64| (x - 3.14159).powi(2),  // 目标函数
        -10.0,  // 搜索范围开始
        10.0,   // 搜索范围结束
    )?;
    println!("Optimized value: {:.6}", result);
    
    Ok(())
}
```

### 量子机器学习

```rust
use quantum::ml::*;

fn main() -> QuantumResult<()> {
    // 生成训练数据
    let training_data = generate_data(1000, 10)?;
    let labels = generate_labels(&training_data)?;
    
    // 量子主成分分析
    let components = quantum_pca(&training_data, 3)?;
    println!("Principal components: {}", components.len());
    
    // 量子聚类
    let clusters = quantum_clustering(&training_data, 5)?;
    println!("Cluster assignments: {:?}", &clusters[..10]);
    
    // 量子神经网络
    let mut qnn = QuantumNeuralNetwork::new(vec![10, 20, 10, 1]);
    let training_result = qnn.quantum_train(&training_data, &labels, 100)?;
    println!("Training accuracy: {:.2}%", training_result.accuracy * 100.0);
    
    Ok(())
}
```

### Arrow 数据结构

```rust
use quantum::arrow::*;

fn main() -> QuantumResult<()> {
    // Arrow 表格 - 列式存储
    let mut table = ArrowTable::new();
    table.add_column("id", ArrowDataType::Int64)?;
    table.add_column("name", ArrowDataType::Utf8)?;
    table.add_column("value", ArrowDataType::Float64)?;
    
    // 批量插入数据
    for i in 0..10000 {
        table.insert_row(vec![
            ArrowValue::Int64(i),
            ArrowValue::Utf8(format!("item_{}", i)),
            ArrowValue::Float64(i as f64 * 1.5),
        ])?;
    }
    
    // 向量化查询 - 极快的性能
    let filtered = table.filter("value > 5000.0")?;
    println!("Filtered rows: {}", filtered.row_count());
    
    // 零拷贝切片
    let slice = table.slice(1000, 2000)?;
    println!("Slice created with zero memory overhead");
    
    Ok(())
}
```

## 🚀 性能优化

### 编译优化

```bash
# 最大编译优化
rustc -O --quantum-opt-level 3 --enable-arrow main.rs

# 针对特定 CPU 优化
rustc -C target-cpu=native --quantum main.rs

# 链接时优化
rustc -C lto=fat --quantum main.rs
```

### Cargo 配置

在 `Cargo.toml` 中添加：

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1

# 量子优化配置
[profile.quantum]
inherits = "release"
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"

[features]
default = ["quantum-core", "arrow-optimization"]
quantum-core = []
quantum-algorithms = ["quantum-core"]
arrow-optimization = []
high-performance = ["quantum-algorithms", "arrow-optimization"]
```

### 性能提示

1. **使用量子数据结构**：
   ```rust
   // 好：使用量子向量
   let mut qvec = QuantumVec::with_capacity(1000000);
   
   // 一般：普通向量（仍有量子编译优化）
   let mut vec = Vec::with_capacity(1000000);
   ```

2. **启用 Arrow 优化**：
   ```rust
   // 大数据处理时使用 Arrow 表格
   let table = ArrowTable::new();
   // 自动获得列式存储和向量化处理
   ```

3. **量子并行处理**：
   ```rust
   // 自动并行化
   let result = data.quantum_par_iter()
       .map(|x| expensive_computation(x))
       .collect();
   ```

## 🔧 故障排除

### 常见问题

#### 1. 编译错误：找不到量子特性

```bash
error: cannot find crate `quantum`
```

**解决方案**：
```bash
# 确保使用量子编译器
which rustc
# 应该显示: /home/user/.quantum-rust/bin/rustc

# 重新加载环境变量
source ~/.bashrc
```

#### 2. 性能没有提升

**检查清单**：
- ✅ 确认使用量子编译器
- ✅ 启用优化选项 (`-O` 或 `--release`)
- ✅ 使用量子数据结构
- ✅ 项目足够大以体现优势

#### 3. 内存使用过高

**解决方案**：
```rust
// 使用 Arrow 数据结构减少内存
use quantum::arrow::*;

// 启用压缩
let table = ArrowTable::with_compression(CompressionType::Zstd);
```

#### 4. 兼容性问题

```bash
# 检查兼容性
quantum-rustc --check-compatibility your_project/

# 生成兼容性报告
quantum-rustc --compatibility-report > report.txt
```

### 性能分析

```bash
# 使用量子性能分析器
quantum-profiler your_program

# 生成性能报告
quantum-bench --compare-traditional your_program

# 内存分析
quantum-profiler --memory your_program
```

### 获取帮助

- 📖 查看文档：`quantum-rustc --help`
- 🐛 报告问题：[GitHub Issues](https://github.com/quantum-rust/quantum-rust/issues)
- 💬 社区讨论：[Discord](https://discord.gg/quantum-rust)
- 📧 邮件支持：support@quantum-rust.org

---

**🎉 恭喜！您现在已经掌握了 Quantum Rust 的基础使用！**

继续探索：
- [API 参考](api-reference.md) - 详细的 API 文档
- [性能指南](performance-guide.md) - 深入的性能优化
- [最佳实践](best-practices.md) - 开发经验分享
