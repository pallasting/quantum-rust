//! 量子算法模块
//!
//! 提供各种量子算法的实现，包括：
//! - 量子门操作
//! - 量子傅里叶变换 (QFT)
//! - 量子测量
//! - 量子搜索算法

use crate::quantum::{QuantumError, QuantumResult};
use crate::quantum::array::{QuantumArray, Complex};
use crate::vec::Vec;

/// 应用Hadamard门到指定量子比特
///
/// Hadamard门将|0⟩变为(|0⟩+|1⟩)/√2，将|1⟩变为(|0⟩-|1⟩)/√2
///
/// # 参数
/// - `qarray`: 量子态数组
/// - `qubit`: 目标量子比特索引
///
/// # 示例
/// ```rust
/// use std::quantum::algorithms::apply_hadamard_gate;
/// use std::quantum::array::QuantumArray;
///
/// let mut qarray = QuantumArray::new(1)?;
/// apply_hadamard_gate(&mut qarray, 0)?;
/// ```
pub fn apply_hadamard_gate(qarray: &mut QuantumArray, qubit: usize) -> QuantumResult<()> {
    if qubit >= qarray.num_qubits() {
        return Err(QuantumError::QubitIndexOutOfRange {
            index: qubit,
            max: qarray.num_qubits(),
        });
    }

    let amplitudes = qarray.amplitudes_mut();
    let sqrt_2_inv = 1.0 / 2.0_f64.sqrt();

    // 对每一对相关的基态应用Hadamard变换
    for i in 0..amplitudes.len() {
        if (i >> qubit) & 1 == 0 {
            let j = i | (1 << qubit); // 翻转第qubit位
            if j < amplitudes.len() {
                let amp_0 = amplitudes[i].clone();
                let amp_1 = amplitudes[j].clone();

                // Hadamard变换矩阵: [1, 1; 1, -1] / √2
                amplitudes[i] = Complex::new(
                    (amp_0.real + amp_1.real) * sqrt_2_inv,
                    (amp_0.imag + amp_1.imag) * sqrt_2_inv,
                );
                amplitudes[j] = Complex::new(
                    (amp_0.real - amp_1.real) * sqrt_2_inv,
                    (amp_0.imag - amp_1.imag) * sqrt_2_inv,
                );
            }
        }
    }

    Ok(())
}

/// 应用相位门到指定量子比特
///
/// 相位门将|1⟩乘以e^(iφ)，|0⟩保持不变
///
/// # 参数
/// - `qarray`: 量子态数组
/// - `qubit`: 目标量子比特索引
/// - `phase`: 相位角度（弧度）
pub fn apply_phase_gate(qarray: &mut QuantumArray, qubit: usize, phase: f64) -> QuantumResult<()> {
    if qubit >= qarray.num_qubits() {
        return Err(QuantumError::QubitIndexOutOfRange {
            index: qubit,
            max: qarray.num_qubits(),
        });
    }

    let amplitudes = qarray.amplitudes_mut();
    let cos_phase = phase.cos();
    let sin_phase = phase.sin();

    // 对|1⟩态应用相位
    for i in 0..amplitudes.len() {
        if (i >> qubit) & 1 == 1 {
            let amp = &amplitudes[i];
            amplitudes[i] = Complex::new(
                amp.real * cos_phase - amp.imag * sin_phase,
                amp.real * sin_phase + amp.imag * cos_phase,
            );
        }
    }

    Ok(())
}

/// 应用Pauli-X门（NOT门）到指定量子比特
pub fn apply_pauli_x_gate(qarray: &mut QuantumArray, qubit: usize) -> QuantumResult<()> {
    if qubit >= qarray.num_qubits() {
        return Err(QuantumError::QubitIndexOutOfRange {
            index: qubit,
            max: qarray.num_qubits(),
        });
    }

    let amplitudes = qarray.amplitudes_mut();

    // 交换|0⟩和|1⟩态的振幅
    for i in 0..amplitudes.len() {
        if (i >> qubit) & 1 == 0 {
            let j = i | (1 << qubit);
            if j < amplitudes.len() {
                amplitudes.swap(i, j);
            }
        }
    }

    Ok(())
}

/// 应用受控NOT门（CNOT门）
///
/// # 参数
/// - `qarray`: 量子态数组
/// - `control`: 控制量子比特索引
/// - `target`: 目标量子比特索引
pub fn apply_cnot_gate(qarray: &mut QuantumArray, control: usize, target: usize) -> QuantumResult<()> {
    if control >= qarray.num_qubits() || target >= qarray.num_qubits() {
        return Err(QuantumError::QubitIndexOutOfRange {
            index: control.max(target),
            max: qarray.num_qubits(),
        });
    }

    if control == target {
        return Err(QuantumError::InvalidQuantumState {
            reason: "控制比特和目标比特不能相同".to_string(),
        });
    }

    let amplitudes = qarray.amplitudes_mut();

    // 当控制比特为|1⟩时，翻转目标比特
    for i in 0..amplitudes.len() {
        if (i >> control) & 1 == 1 && (i >> target) & 1 == 0 {
            let j = i | (1 << target);
            if j < amplitudes.len() {
                amplitudes.swap(i, j);
            }
        }
    }

    Ok(())
}

/// 量子测量
///
/// 对整个量子系统进行测量，返回测量结果和概率
///
/// # 参数
/// - `qarray`: 量子态数组
///
/// # 返回
/// - `(state, probability)`: 测量到的基态和对应概率
pub fn quantum_measurement(qarray: &QuantumArray) -> QuantumResult<(usize, f64)> {
    qarray.measure_all()
}

/// 量子傅里叶变换 (QFT)
///
/// 对量子态应用量子傅里叶变换
///
/// # 参数
/// - `qarray`: 量子态数组
pub fn quantum_fft(qarray: &mut QuantumArray) -> QuantumResult<()> {
    let n = qarray.num_qubits();
    
    // QFT算法实现
    for i in 0..n {
        // 应用Hadamard门
        apply_hadamard_gate(qarray, i)?;
        
        // 应用受控相位门
        for j in (i + 1)..n {
            let phase = 2.0 * std::f64::consts::PI / (1 << (j - i + 1)) as f64;
            apply_controlled_phase_gate(qarray, j, i, phase)?;
        }
    }
    
    // 反转量子比特顺序
    for i in 0..(n / 2) {
        swap_qubits(qarray, i, n - 1 - i)?;
    }
    
    Ok(())
}

/// 应用受控相位门
fn apply_controlled_phase_gate(qarray: &mut QuantumArray, control: usize, target: usize, phase: f64) -> QuantumResult<()> {
    if control >= qarray.num_qubits() || target >= qarray.num_qubits() {
        return Err(QuantumError::QubitIndexOutOfRange {
            index: control.max(target),
            max: qarray.num_qubits(),
        });
    }

    let amplitudes = qarray.amplitudes_mut();
    let cos_phase = phase.cos();
    let sin_phase = phase.sin();

    // 当控制比特和目标比特都为|1⟩时应用相位
    for i in 0..amplitudes.len() {
        if (i >> control) & 1 == 1 && (i >> target) & 1 == 1 {
            let amp = &amplitudes[i];
            amplitudes[i] = Complex::new(
                amp.real * cos_phase - amp.imag * sin_phase,
                amp.real * sin_phase + amp.imag * cos_phase,
            );
        }
    }

    Ok(())
}

/// 交换两个量子比特
fn swap_qubits(qarray: &mut QuantumArray, qubit1: usize, qubit2: usize) -> QuantumResult<()> {
    if qubit1 >= qarray.num_qubits() || qubit2 >= qarray.num_qubits() {
        return Err(QuantumError::QubitIndexOutOfRange {
            index: qubit1.max(qubit2),
            max: qarray.num_qubits(),
        });
    }

    if qubit1 == qubit2 {
        return Ok(()); // 无需交换
    }

    // 使用三个CNOT门实现SWAP
    apply_cnot_gate(qarray, qubit1, qubit2)?;
    apply_cnot_gate(qarray, qubit2, qubit1)?;
    apply_cnot_gate(qarray, qubit1, qubit2)?;

    Ok(())
}

/// 经典FFT（用于性能对比）
///
/// # 参数
/// - `data`: 输入数据
///
/// # 返回
/// - FFT变换结果
pub fn classical_fft(data: &[f64]) -> QuantumResult<Vec<Complex>> {
    let n = data.len();
    
    if n == 0 {
        return Ok(Vec::new());
    }
    
    if n == 1 {
        return Ok(vec![Complex::new(data[0], 0.0)]);
    }
    
    // 简化的FFT实现
    let mut result = Vec::with_capacity(n);
    
    for k in 0..n {
        let mut sum = Complex::zero();
        for j in 0..n {
            let angle = -2.0 * std::f64::consts::PI * (k * j) as f64 / n as f64;
            let cos_val = angle.cos();
            let sin_val = angle.sin();
            
            sum.real += data[j] * cos_val;
            sum.imag += data[j] * sin_val;
        }
        result.push(sum);
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hadamard_gate() {
        let mut qarray = QuantumArray::new(1).unwrap();
        apply_hadamard_gate(&mut qarray, 0).unwrap();
        
        // 检查叠加态
        let amp_0 = qarray.get_amplitude(0).unwrap();
        let amp_1 = qarray.get_amplitude(1).unwrap();
        
        let expected = 1.0 / 2.0_f64.sqrt();
        assert!((amp_0.real - expected).abs() < 1e-10);
        assert!((amp_1.real - expected).abs() < 1e-10);
    }

    #[test]
    fn test_pauli_x_gate() {
        let mut qarray = QuantumArray::new(1).unwrap();
        apply_pauli_x_gate(&mut qarray, 0).unwrap();
        
        // 应该从|0⟩变为|1⟩
        let amp_0 = qarray.get_amplitude(0).unwrap();
        let amp_1 = qarray.get_amplitude(1).unwrap();
        
        assert!((amp_0.real - 0.0).abs() < 1e-10);
        assert!((amp_1.real - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_cnot_gate() {
        let mut qarray = QuantumArray::new(2).unwrap();
        
        // 先将控制比特设为|1⟩
        apply_pauli_x_gate(&mut qarray, 0).unwrap();
        
        // 应用CNOT门
        apply_cnot_gate(&mut qarray, 0, 1).unwrap();
        
        // 应该得到|11⟩态
        let amp_3 = qarray.get_amplitude(3).unwrap(); // |11⟩ = 3
        assert!((amp_3.real - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_quantum_measurement() {
        let qarray = QuantumArray::new(2).unwrap();
        let (state, prob) = quantum_measurement(&qarray).unwrap();
        
        assert_eq!(state, 0); // 应该测量到|00⟩
        assert!((prob - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_classical_fft() {
        let data = vec![1.0, 0.0, 0.0, 0.0];
        let result = classical_fft(&data).unwrap();
        
        assert_eq!(result.len(), 4);
        // 验证FFT结果的基本性质
        assert!((result[0].real - 1.0).abs() < 1e-10);
    }
}
