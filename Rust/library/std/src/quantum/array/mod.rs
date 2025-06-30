//! 量子数组模块
//!
//! 提供量子计算中使用的特殊数据结构，包括：
//! - 多维Arrow数组 (MultiDimArray)
//! - 量子态数组 (QuantumArray)
//! - 量子张量操作

use crate::quantum::{QuantumError, QuantumResult};
use crate::vec::Vec;
use crate::fmt;

pub mod multidim;
pub mod quantum_array;

pub use multidim::MultiDimArray;
pub use quantum_array::QuantumArray;

/// 数组形状描述
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape {
    dimensions: Vec<usize>,
}

impl Shape {
    /// 创建新的形状
    pub fn new(dimensions: Vec<usize>) -> Self {
        Self { dimensions }
    }

    /// 获取维度数量
    pub fn ndim(&self) -> usize {
        self.dimensions.len()
    }

    /// 获取总元素数量
    pub fn size(&self) -> usize {
        self.dimensions.iter().product()
    }

    /// 获取维度切片
    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    /// 检查形状是否有效
    pub fn is_valid(&self) -> bool {
        !self.dimensions.is_empty() && self.dimensions.iter().all(|&d| d > 0)
    }

    /// 检查是否与另一个形状兼容（用于广播）
    pub fn is_compatible_with(&self, other: &Shape) -> bool {
        if self.ndim() != other.ndim() {
            return false;
        }
        
        self.dimensions.iter()
            .zip(other.dimensions.iter())
            .all(|(&a, &b)| a == b || a == 1 || b == 1)
    }

    /// 计算广播后的形状
    pub fn broadcast_with(&self, other: &Shape) -> QuantumResult<Shape> {
        if !self.is_compatible_with(other) {
            return Err(QuantumError::DimensionMismatch {
                expected: self.dimensions.clone(),
                actual: other.dimensions.clone(),
            });
        }

        let result_dims: Vec<usize> = self.dimensions.iter()
            .zip(other.dimensions.iter())
            .map(|(&a, &b)| a.max(b))
            .collect();

        Ok(Shape::new(result_dims))
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Shape({:?})", self.dimensions)
    }
}

/// 数组索引类型
pub type Index = Vec<usize>;

/// 平坦索引转多维索引
pub fn flat_to_multi_index(flat_idx: usize, shape: &Shape) -> QuantumResult<Index> {
    if flat_idx >= shape.size() {
        return Err(QuantumError::DimensionMismatch {
            expected: vec![shape.size()],
            actual: vec![flat_idx],
        });
    }

    let mut multi_idx = Vec::with_capacity(shape.ndim());
    let mut remaining = flat_idx;

    for &dim in shape.dimensions().iter().rev() {
        multi_idx.push(remaining % dim);
        remaining /= dim;
    }

    multi_idx.reverse();
    Ok(multi_idx)
}

/// 多维索引转平坦索引
pub fn multi_to_flat_index(multi_idx: &Index, shape: &Shape) -> QuantumResult<usize> {
    if multi_idx.len() != shape.ndim() {
        return Err(QuantumError::DimensionMismatch {
            expected: vec![shape.ndim()],
            actual: vec![multi_idx.len()],
        });
    }

    let mut flat_idx = 0;
    let mut stride = 1;

    for i in (0..shape.ndim()).rev() {
        if multi_idx[i] >= shape.dimensions()[i] {
            return Err(QuantumError::DimensionMismatch {
                expected: shape.dimensions().to_vec(),
                actual: multi_idx.clone(),
            });
        }
        flat_idx += multi_idx[i] * stride;
        stride *= shape.dimensions()[i];
    }

    Ok(flat_idx)
}

/// 数组元素类型trait
pub trait ArrayElement: Clone + fmt::Debug + PartialEq {
    /// 零值
    fn zero() -> Self;
    /// 单位值
    fn one() -> Self;
    /// 是否为零
    fn is_zero(&self) -> bool;
}

impl ArrayElement for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
    fn is_zero(&self) -> bool { *self == 0.0 }
}

impl ArrayElement for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
    fn is_zero(&self) -> bool { *self == 0.0 }
}

impl ArrayElement for i32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
    fn is_zero(&self) -> bool { *self == 0 }
}

impl ArrayElement for i64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
    fn is_zero(&self) -> bool { *self == 0 }
}

/// 复数类型（简化实现）
#[derive(Debug, Clone, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    pub fn phase(&self) -> f64 {
        self.imag.atan2(self.real)
    }
}

impl ArrayElement for Complex {
    fn zero() -> Self { Complex::new(0.0, 0.0) }
    fn one() -> Self { Complex::new(1.0, 0.0) }
    fn is_zero(&self) -> bool { self.real == 0.0 && self.imag == 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_creation() {
        let shape = Shape::new(vec![2, 3, 4]);
        assert_eq!(shape.ndim(), 3);
        assert_eq!(shape.size(), 24);
        assert!(shape.is_valid());
    }

    #[test]
    fn test_shape_compatibility() {
        let shape1 = Shape::new(vec![2, 3]);
        let shape2 = Shape::new(vec![2, 3]);
        let shape3 = Shape::new(vec![1, 3]);
        
        assert!(shape1.is_compatible_with(&shape2));
        assert!(shape1.is_compatible_with(&shape3));
    }

    #[test]
    fn test_index_conversion() {
        let shape = Shape::new(vec![2, 3]);
        
        // 测试平坦索引转多维索引
        let multi_idx = flat_to_multi_index(4, &shape).unwrap();
        assert_eq!(multi_idx, vec![1, 1]);
        
        // 测试多维索引转平坦索引
        let flat_idx = multi_to_flat_index(&vec![1, 1], &shape).unwrap();
        assert_eq!(flat_idx, 4);
    }

    #[test]
    fn test_array_elements() {
        assert_eq!(f64::zero(), 0.0);
        assert_eq!(f64::one(), 1.0);
        assert!(f64::zero().is_zero());
        
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.magnitude(), 5.0);
    }
}
