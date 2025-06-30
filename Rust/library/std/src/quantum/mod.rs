//! 量子计算标准库模块
//!
//! 这个模块提供了Rust语言的量子计算能力，包括：
//! - 量子数据结构和数组操作
//! - 量子算法和量子门操作
//! - 量子编译器优化接口
//!
//! # 特性标志
//!
//! 这个模块需要启用 `quantum` 特性标志：
//!
//! ```toml
//! [dependencies]
//! std = { features = ["quantum"] }
//! ```
//!
//! # 快速开始
//!
//! ```rust
//! use std::quantum::prelude::*;
//!
//! // 创建量子态
//! let mut qstate = QuantumState::new(2); // 2量子比特
//!
//! // 应用Hadamard门
//! qstate.apply_hadamard(0)?;
//!
//! // 创建多维数组
//! let data = vec![1.0, 2.0, 3.0, 4.0];
//! let array = MultiDimArray::new(vec![2, 2], data)?;
//!
//! // 量子FFT
//! let fft_result = quantum_fft(&array)?;
//! ```

#![cfg(feature = "quantum")]
#![unstable(feature = "quantum", issue = "none")]

pub mod array;
pub mod algorithms;
pub mod compiler;

pub mod prelude {
    //! 量子计算预导入模块
    //!
    //! 包含最常用的量子计算类型和函数

    pub use super::array::{MultiDimArray, QuantumArray};
    pub use super::algorithms::{
        quantum_fft, apply_hadamard_gate, apply_phase_gate, quantum_measurement
    };
    pub use super::compiler::{QuantumAnalyzer, OptimizationHint};
}

/// 量子计算错误类型
#[derive(Debug, Clone)]
pub enum QuantumError {
    /// 维度不匹配错误
    DimensionMismatch {
        expected: Vec<usize>,
        actual: Vec<usize>,
    },
    /// 量子比特索引超出范围
    QubitIndexOutOfRange {
        index: usize,
        max: usize,
    },
    /// 无效的量子态
    InvalidQuantumState {
        reason: String,
    },
    /// 量子算法执行错误
    AlgorithmError {
        algorithm: String,
        reason: String,
    },
    /// 编译器分析错误
    CompilerError {
        phase: String,
        message: String,
    },
    /// 量子振幅超出有效范围
    InvalidAmplitude {
        value: f64,
        valid_range: (f64, f64),
        recovery_hint: String,
    },
    /// 量子相位无效
    InvalidPhase {
        value: f64,
        reason: String,
    },
    /// 量子态退化错误
    QuantumStateDegeneration {
        state_id: usize,
        recovery_hint: String,
        auto_recovery_attempted: bool,
    },
    /// 纠缠网络故障
    EntanglementNetworkFailure {
        network_id: usize,
        affected_nodes: Vec<usize>,
        recovery_strategy: String,
    },
    /// 量子相干性丢失
    CoherenceLoss {
        original_coherence: f64,
        current_coherence: f64,
        threshold: f64,
    },
    /// VQE算法收敛失败
    VqeConvergenceFailure {
        iterations: usize,
        final_energy: f64,
        convergence_threshold: f64,
    },
}

impl core::fmt::Display for QuantumError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            QuantumError::DimensionMismatch { expected, actual } => {
                write!(f, "维度不匹配: 期望 {:?}, 实际 {:?}", expected, actual)
            }
            QuantumError::QubitIndexOutOfRange { index, max } => {
                write!(f, "量子比特索引 {} 超出范围 [0, {})", index, max)
            }
            QuantumError::InvalidQuantumState { reason } => {
                write!(f, "无效的量子态: {}", reason)
            }
            QuantumError::AlgorithmError { algorithm, reason } => {
                write!(f, "量子算法 '{}' 执行错误: {}", algorithm, reason)
            }
            QuantumError::CompilerError { phase, message } => {
                write!(f, "编译器错误 ({}): {}", phase, message)
            }
            QuantumError::InvalidAmplitude { value, valid_range, recovery_hint } => {
                write!(f, "量子振幅 {} 超出有效范围 [{}, {}]. 建议: {}",
                       value, valid_range.0, valid_range.1, recovery_hint)
            }
            QuantumError::InvalidPhase { value, reason } => {
                write!(f, "量子相位 {} 无效: {}", value, reason)
            }
            QuantumError::QuantumStateDegeneration { state_id, recovery_hint, auto_recovery_attempted } => {
                write!(f, "量子态 {} 退化{}. 建议: {}",
                       state_id,
                       if *auto_recovery_attempted { " (已尝试自动恢复)" } else { "" },
                       recovery_hint)
            }
            QuantumError::EntanglementNetworkFailure { network_id, affected_nodes, recovery_strategy } => {
                write!(f, "纠缠网络 {} 故障，影响节点: {:?}. 恢复策略: {}",
                       network_id, affected_nodes, recovery_strategy)
            }
            QuantumError::CoherenceLoss { original_coherence, current_coherence, threshold } => {
                write!(f, "量子相干性丢失: {} -> {} (阈值: {})",
                       original_coherence, current_coherence, threshold)
            }
            QuantumError::VqeConvergenceFailure { iterations, final_energy, convergence_threshold } => {
                write!(f, "VQE算法在 {} 次迭代后未收敛，最终能量: {}，收敛阈值: {}",
                       iterations, final_energy, convergence_threshold)
            }
        }
    }
}

impl core::error::Error for QuantumError {}

/// 量子计算结果类型
pub type QuantumResult<T> = Result<T, QuantumError>;

/// 错误恢复统计
#[derive(Debug, Clone, Default)]
pub struct ErrorRecoveryStats {
    pub total_errors: usize,
    pub successful_recoveries: usize,
    pub failed_recoveries: usize,
    pub auto_recovery_attempts: usize,
}

/// 量子错误恢复工具
pub struct QuantumErrorRecovery {
    stats: ErrorRecoveryStats,
    max_recovery_attempts: usize,
    enable_auto_recovery: bool,
}

impl QuantumErrorRecovery {
    pub fn new(max_attempts: usize, auto_recovery: bool) -> Self {
        Self {
            stats: ErrorRecoveryStats::default(),
            max_recovery_attempts: max_attempts,
            enable_auto_recovery: auto_recovery,
        }
    }

    /// 尝试从量子错误中恢复
    pub fn attempt_recovery<T, F>(&mut self, error: QuantumError, recovery_fn: F) -> QuantumResult<T>
    where
        F: Fn(&QuantumError) -> QuantumResult<T>,
    {
        self.stats.total_errors += 1;

        if !self.enable_auto_recovery {
            return Err(error);
        }

        self.stats.auto_recovery_attempts += 1;

        match recovery_fn(&error) {
            Ok(result) => {
                self.stats.successful_recoveries += 1;
                Ok(result)
            }
            Err(recovery_error) => {
                self.stats.failed_recoveries += 1;
                // 返回原始错误，但标记已尝试恢复
                match error {
                    QuantumError::QuantumStateDegeneration { state_id, recovery_hint, .. } => {
                        Err(QuantumError::QuantumStateDegeneration {
                            state_id,
                            recovery_hint,
                            auto_recovery_attempted: true,
                        })
                    }
                    _ => Err(error),
                }
            }
        }
    }

    /// 获取恢复统计
    pub fn get_stats(&self) -> &ErrorRecoveryStats {
        &self.stats
    }

    /// 重置统计
    pub fn reset_stats(&mut self) {
        self.stats = ErrorRecoveryStats::default();
    }
}

/// 量子计算版本信息
pub const QUANTUM_VERSION: &str = "1.0.0-alpha";

/// 检查量子特性是否可用
pub fn is_quantum_available() -> bool {
    cfg!(feature = "quantum")
}

/// 获取量子计算能力信息
pub fn quantum_capabilities() -> QuantumCapabilities {
    QuantumCapabilities {
        max_qubits: 64,
        supports_gpu: cfg!(feature = "quantum-gpu"),
        supports_simd: cfg!(feature = "quantum-simd"),
        version: QUANTUM_VERSION,
    }
}

/// 量子计算能力描述
#[derive(Debug, Clone)]
pub struct QuantumCapabilities {
    /// 支持的最大量子比特数
    pub max_qubits: usize,
    /// 是否支持GPU加速
    pub supports_gpu: bool,
    /// 是否支持SIMD优化
    pub supports_simd: bool,
    /// 量子模块版本
    pub version: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_available() {
        assert!(is_quantum_available());
    }

    #[test]
    fn test_quantum_capabilities() {
        let caps = quantum_capabilities();
        assert_eq!(caps.version, QUANTUM_VERSION);
        assert!(caps.max_qubits > 0);
    }

    #[test]
    fn test_quantum_error_display() {
        let error = QuantumError::DimensionMismatch {
            expected: vec![2, 3],
            actual: vec![3, 2],
        };
        let display = format!("{}", error);
        assert!(display.contains("维度不匹配"));
    }
}
