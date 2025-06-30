//! 量子态数组实现
//!
//! 提供量子计算中的量子态表示和操作

use super::{Complex, ArrayElement};
use crate::quantum::{QuantumError, QuantumResult};
use crate::vec::Vec;
use crate::fmt;

/// 量子态数组
///
/// 表示量子系统的状态向量，支持量子门操作和测量
#[derive(Debug, Clone)]
pub struct QuantumArray {
    /// 量子态振幅
    amplitudes: Vec<Complex>,
    /// 量子比特数量
    num_qubits: usize,
}

impl QuantumArray {
    /// 创建新的量子态数组
    ///
    /// # 参数
    /// - `num_qubits`: 量子比特数量
    ///
    /// # 示例
    /// ```rust
    /// use std::quantum::array::QuantumArray;
    ///
    /// // 创建2量子比特系统，初始化为|00⟩态
    /// let qarray = QuantumArray::new(2)?;
    /// ```
    pub fn new(num_qubits: usize) -> QuantumResult<Self> {
        if num_qubits == 0 {
            return Err(QuantumError::InvalidQuantumState {
                reason: "量子比特数量必须大于0".to_string(),
            });
        }

        if num_qubits > 64 {
            return Err(QuantumError::InvalidQuantumState {
                reason: "量子比特数量不能超过64".to_string(),
            });
        }

        let state_size = 1 << num_qubits; // 2^num_qubits
        let mut amplitudes = vec![Complex::zero(); state_size];
        amplitudes[0] = Complex::one(); // 初始化为|0...0⟩态

        Ok(Self {
            amplitudes,
            num_qubits,
        })
    }

    /// 从振幅向量创建量子态
    pub fn from_amplitudes(amplitudes: Vec<Complex>) -> QuantumResult<Self> {
        if amplitudes.is_empty() {
            return Err(QuantumError::InvalidQuantumState {
                reason: "振幅向量不能为空".to_string(),
            });
        }

        // 检查是否为2的幂
        let len = amplitudes.len();
        if len & (len - 1) != 0 {
            return Err(QuantumError::InvalidQuantumState {
                reason: "振幅向量长度必须是2的幂".to_string(),
            });
        }

        let num_qubits = len.trailing_zeros() as usize;

        // 验证归一化
        let norm_squared: f64 = amplitudes.iter()
            .map(|amp| amp.magnitude() * amp.magnitude())
            .sum();

        if (norm_squared - 1.0).abs() > 1e-10 {
            return Err(QuantumError::InvalidQuantumState {
                reason: format!("量子态未归一化，模长平方为: {}", norm_squared),
            });
        }

        Ok(Self {
            amplitudes,
            num_qubits,
        })
    }

    /// 获取量子比特数量
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// 获取状态向量大小
    pub fn state_size(&self) -> usize {
        self.amplitudes.len()
    }

    /// 获取振幅
    pub fn amplitudes(&self) -> &[Complex] {
        &self.amplitudes
    }

    /// 获取可变振幅引用
    pub fn amplitudes_mut(&mut self) -> &mut [Complex] {
        &mut self.amplitudes
    }

    /// 获取指定基态的振幅
    pub fn get_amplitude(&self, state: usize) -> QuantumResult<&Complex> {
        if state >= self.amplitudes.len() {
            return Err(QuantumError::QubitIndexOutOfRange {
                index: state,
                max: self.amplitudes.len(),
            });
        }
        Ok(&self.amplitudes[state])
    }

    /// 设置指定基态的振幅
    pub fn set_amplitude(&mut self, state: usize, amplitude: Complex) -> QuantumResult<()> {
        if state >= self.amplitudes.len() {
            return Err(QuantumError::QubitIndexOutOfRange {
                index: state,
                max: self.amplitudes.len(),
            });
        }
        self.amplitudes[state] = amplitude;
        Ok(())
    }

    /// 归一化量子态
    pub fn normalize(&mut self) -> QuantumResult<()> {
        let norm_squared: f64 = self.amplitudes.iter()
            .map(|amp| amp.magnitude() * amp.magnitude())
            .sum();

        if norm_squared == 0.0 {
            return Err(QuantumError::InvalidQuantumState {
                reason: "无法归一化零态".to_string(),
            });
        }

        let norm = norm_squared.sqrt();
        for amplitude in &mut self.amplitudes {
            amplitude.real /= norm;
            amplitude.imag /= norm;
        }

        Ok(())
    }

    /// 计算量子态的保真度
    pub fn fidelity(&self, other: &Self) -> QuantumResult<f64> {
        if self.num_qubits != other.num_qubits {
            return Err(QuantumError::DimensionMismatch {
                expected: vec![self.num_qubits],
                actual: vec![other.num_qubits],
            });
        }

        let inner_product: Complex = self.amplitudes.iter()
            .zip(other.amplitudes.iter())
            .map(|(a, b)| Complex::new(
                a.real * b.real + a.imag * b.imag,
                a.imag * b.real - a.real * b.imag,
            ))
            .fold(Complex::zero(), |acc, x| Complex::new(
                acc.real + x.real,
                acc.imag + x.imag,
            ));

        Ok(inner_product.magnitude())
    }

    /// 测量指定量子比特
    ///
    /// 返回测量结果(0或1)和对应的概率
    pub fn measure_qubit(&self, qubit: usize) -> QuantumResult<(u8, f64)> {
        if qubit >= self.num_qubits {
            return Err(QuantumError::QubitIndexOutOfRange {
                index: qubit,
                max: self.num_qubits,
            });
        }

        // 计算测量到|0⟩的概率
        let prob_0: f64 = self.amplitudes.iter()
            .enumerate()
            .filter(|(state, _)| (state >> qubit) & 1 == 0)
            .map(|(_, amp)| amp.magnitude() * amp.magnitude())
            .sum();

        // 简化的测量实现（实际应该使用随机数）
        // 这里返回概率较大的结果
        if prob_0 > 0.5 {
            Ok((0, prob_0))
        } else {
            Ok((1, 1.0 - prob_0))
        }
    }

    /// 测量整个量子系统
    ///
    /// 返回测量结果的基态索引和概率
    pub fn measure_all(&self) -> QuantumResult<(usize, f64)> {
        // 找到概率最大的基态
        let (max_state, max_prob) = self.amplitudes.iter()
            .enumerate()
            .map(|(state, amp)| (state, amp.magnitude() * amp.magnitude()))
            .max_by(|(_, prob1), (_, prob2)| prob1.partial_cmp(prob2).unwrap())
            .unwrap();

        Ok((max_state, max_prob))
    }

    /// 获取量子态的字符串表示
    pub fn to_string_representation(&self) -> String {
        let mut result = String::new();
        let mut first = true;

        for (state, amplitude) in self.amplitudes.iter().enumerate() {
            if amplitude.magnitude() > 1e-10 {
                if !first {
                    result.push_str(" + ");
                }
                first = false;

                // 格式化振幅
                if amplitude.imag.abs() < 1e-10 {
                    result.push_str(&format!("{:.3}", amplitude.real));
                } else if amplitude.real.abs() < 1e-10 {
                    result.push_str(&format!("{:.3}i", amplitude.imag));
                } else {
                    result.push_str(&format!("({:.3}+{:.3}i)", amplitude.real, amplitude.imag));
                }

                // 格式化基态
                result.push_str(&format!("|{:0width$b}⟩", state, width = self.num_qubits));
            }
        }

        if result.is_empty() {
            result = "0".to_string();
        }

        result
    }
}

impl fmt::Display for QuantumArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "QuantumArray({} qubits): {}", 
               self.num_qubits, 
               self.to_string_representation())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_array_creation() {
        let qarray = QuantumArray::new(2).unwrap();
        assert_eq!(qarray.num_qubits(), 2);
        assert_eq!(qarray.state_size(), 4);
        
        // 检查初始态为|00⟩
        assert_eq!(qarray.get_amplitude(0).unwrap().real, 1.0);
        assert_eq!(qarray.get_amplitude(1).unwrap().real, 0.0);
    }

    #[test]
    fn test_quantum_array_from_amplitudes() {
        // 创建叠加态 (|00⟩ + |11⟩)/√2
        let amplitudes = vec![
            Complex::new(1.0/2.0_f64.sqrt(), 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0/2.0_f64.sqrt(), 0.0),
        ];
        
        let qarray = QuantumArray::from_amplitudes(amplitudes).unwrap();
        assert_eq!(qarray.num_qubits(), 2);
    }

    #[test]
    fn test_quantum_measurement() {
        let qarray = QuantumArray::new(1).unwrap();
        let (result, prob) = qarray.measure_qubit(0).unwrap();
        
        assert_eq!(result, 0); // 应该测量到|0⟩
        assert!((prob - 1.0).abs() < 1e-10); // 概率应该为1
    }

    #[test]
    fn test_quantum_fidelity() {
        let qarray1 = QuantumArray::new(2).unwrap();
        let qarray2 = QuantumArray::new(2).unwrap();
        
        let fidelity = qarray1.fidelity(&qarray2).unwrap();
        assert!((fidelity - 1.0).abs() < 1e-10); // 相同态的保真度为1
    }

    #[test]
    fn test_quantum_normalization() {
        let mut amplitudes = vec![
            Complex::new(2.0, 0.0),
            Complex::new(0.0, 0.0),
        ];
        
        let mut qarray = QuantumArray::from_amplitudes(amplitudes).unwrap_or_else(|_| {
            // 如果归一化失败，手动创建并归一化
            let mut qa = QuantumArray::new(1).unwrap();
            qa.set_amplitude(0, Complex::new(2.0, 0.0)).unwrap();
            qa.normalize().unwrap();
            qa
        });
        
        let norm_squared: f64 = qarray.amplitudes().iter()
            .map(|amp| amp.magnitude() * amp.magnitude())
            .sum();
        assert!((norm_squared - 1.0).abs() < 1e-10);
    }
}
