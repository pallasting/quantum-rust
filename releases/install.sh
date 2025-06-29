#!/bin/bash
# Quantum Rust v1.0.0 安装脚本

set -e

QUANTUM_RUST_VERSION="1.0.0"
INSTALL_DIR="$HOME/.quantum-rust"
BACKUP_DIR="$HOME/.quantum-rust-backup"

echo "🚀 安装 Quantum Rust v$QUANTUM_RUST_VERSION"
echo "=================================="

# 检查系统要求
echo "🔍 检查系统要求..."
if ! command -v rustc &> /dev/null; then
    echo "⚠️  警告: 未检测到 Rust 安装"
    echo "建议先安装标准 Rust: https://rustup.rs/"
fi

# 创建安装目录
echo "📁 创建安装目录..."
mkdir -p "$INSTALL_DIR"
mkdir -p "$BACKUP_DIR"

# 备份现有安装
if [ -d "$INSTALL_DIR" ]; then
    echo "💾 备份现有安装..."
    cp -r "$INSTALL_DIR" "$BACKUP_DIR/quantum-rust-$(date +%Y%m%d-%H%M%S)" 2>/dev/null || true
fi

# 复制文件
echo "📦 安装 Quantum Rust..."
cp -r bin/* "$INSTALL_DIR/bin/" 2>/dev/null || mkdir -p "$INSTALL_DIR/bin"
cp -r lib/* "$INSTALL_DIR/lib/" 2>/dev/null || mkdir -p "$INSTALL_DIR/lib"
cp -r share/* "$INSTALL_DIR/share/" 2>/dev/null || mkdir -p "$INSTALL_DIR/share"

# 设置权限
chmod +x "$INSTALL_DIR/bin/"*

# 更新 PATH
SHELL_RC="$HOME/.bashrc"
if [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
fi

if ! grep -q "quantum-rust" "$SHELL_RC" 2>/dev/null; then
    echo "" >> "$SHELL_RC"
    echo "# Quantum Rust" >> "$SHELL_RC"
    echo 'export PATH="$HOME/.quantum-rust/bin:$PATH"' >> "$SHELL_RC"
    echo 'export QUANTUM_RUST_HOME="$HOME/.quantum-rust"' >> "$SHELL_RC"
    echo "" >> "$SHELL_RC"
fi

echo "✅ Quantum Rust v$QUANTUM_RUST_VERSION 安装完成!"
echo ""
echo "🔧 下一步:"
echo "1. 重新启动终端或运行: source $SHELL_RC"
echo "2. 验证安装: rustc --version"
echo "3. 查看文档: quantum-rustc --help"
echo ""
echo "🎉 欢迎来到量子编程的未来!"
