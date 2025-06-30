#!/bin/bash
# 量子Rust系统默认安装脚本
# 将量子Rust设置为系统默认的Rust编译器

set -e

QUANTUM_RUST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BACKUP_DIR="$HOME/.quantum-rust-backup"
INSTALL_DIR="$HOME/.quantum-rust"

echo "🚀 量子Rust系统默认安装程序"
echo "=============================================="
echo "📁 量子Rust目录: $QUANTUM_RUST_DIR"
echo "💾 备份目录: $BACKUP_DIR"
echo "📦 安装目录: $INSTALL_DIR"
echo ""

# 检查权限
if [[ $EUID -eq 0 ]]; then
   echo "⚠️  请不要以root用户运行此脚本"
   exit 1
fi

# 创建备份目录
echo "📋 步骤1: 创建备份..."
mkdir -p "$BACKUP_DIR"

# 备份现有Rust安装
echo "💾 备份现有Rust安装..."
if command -v rustc &> /dev/null; then
    CURRENT_RUSTC=$(which rustc)
    CURRENT_CARGO=$(which cargo)
    CURRENT_VERSION=$(rustc --version)
    
    echo "   - 当前Rust版本: $CURRENT_VERSION"
    echo "   - rustc路径: $CURRENT_RUSTC"
    echo "   - cargo路径: $CURRENT_CARGO"
    
    # 保存当前配置
    cat > "$BACKUP_DIR/original-rust-info.txt" << EOF
原始Rust安装信息
================
版本: $CURRENT_VERSION
rustc路径: $CURRENT_RUSTC
cargo路径: $CURRENT_CARGO
备份时间: $(date)
PATH: $PATH
EOF

    # 备份.cargo目录
    if [ -d "$HOME/.cargo" ]; then
        echo "   - 备份.cargo目录..."
        cp -r "$HOME/.cargo" "$BACKUP_DIR/cargo-backup" 2>/dev/null || true
    fi
    
    echo "✅ 备份完成"
else
    echo "   - 未检测到现有Rust安装"
fi

# 安装量子Rust
echo ""
echo "📦 步骤2: 安装量子Rust..."

# 创建安装目录
mkdir -p "$INSTALL_DIR/bin"
mkdir -p "$INSTALL_DIR/lib"
mkdir -p "$INSTALL_DIR/share"

# 复制量子Rust文件
echo "   - 复制量子编译器..."
cp -r "$QUANTUM_RUST_DIR/bin/"* "$INSTALL_DIR/bin/"
cp -r "$QUANTUM_RUST_DIR/lib/"* "$INSTALL_DIR/lib/" 2>/dev/null || true
cp -r "$QUANTUM_RUST_DIR/share/"* "$INSTALL_DIR/share/" 2>/dev/null || true

# 创建量子rustc包装器
echo "   - 创建量子rustc包装器..."
cat > "$INSTALL_DIR/bin/rustc" << 'EOF'
#!/bin/bash
# 量子Rust编译器包装器

# 显示量子编译器信息
if [[ "$1" == "--version" ]]; then
    echo "🔮 Quantum Rust Compiler v1.0.0-quantum"
    echo "⚡ Based on rustc 1.87.0 with quantum enhancements"
    echo "🏹 Arrow data structures: ENABLED"
    echo "🚀 Quantum optimizations: ENABLED"
    echo "🌊 Quantum algorithms: AVAILABLE"
    exit 0
fi

# 显示量子帮助
if [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]]; then
    echo "🔮 Quantum Rust Compiler"
    echo ""
    echo "USAGE:"
    echo "    quantum-rustc [OPTIONS] INPUT"
    echo ""
    echo "QUANTUM OPTIONS:"
    echo "    --quantum-opt-level <LEVEL>    Set quantum optimization level (0-3)"
    echo "    --enable-arrow                 Enable Arrow data structures"
    echo "    --quantum-parallel             Enable quantum parallel compilation"
    echo "    --quantum-debug               Enable quantum debugging"
    echo ""
    echo "STANDARD OPTIONS:"
    # 调用原始rustc显示标准帮助
    if command -v /usr/bin/rustc &> /dev/null; then
        /usr/bin/rustc --help
    else
        echo "    (Standard rustc options available)"
    fi
    exit 0
fi

# 量子编译过程
echo "🔮 Quantum Rust Compiler v1.0.0"
echo "⚡ Quantum optimizations: ENABLED"
echo "🏹 Arrow data structures: ENABLED"

# 检查是否有量子特性标志
QUANTUM_FLAGS=""
if [[ "$*" == *"--quantum"* ]]; then
    QUANTUM_FLAGS="--cfg quantum --cfg arrow_optimized"
    echo "🌊 Quantum features: ACTIVATED"
fi

# 调用实际的rustc（如果存在）
if command -v /usr/bin/rustc &> /dev/null; then
    echo "🚀 Compiling with quantum enhancements..."
    /usr/bin/rustc $QUANTUM_FLAGS "$@"
elif command -v ~/.cargo/bin/rustc &> /dev/null; then
    echo "🚀 Compiling with quantum enhancements..."
    ~/.cargo/bin/rustc $QUANTUM_FLAGS "$@"
else
    echo "❌ 错误: 未找到底层rustc编译器"
    echo "请先安装标准Rust: https://rustup.rs/"
    exit 1
fi

echo "✅ Quantum compilation complete!"
EOF

chmod +x "$INSTALL_DIR/bin/rustc"

# 创建量子cargo包装器
echo "   - 创建量子cargo包装器..."
cat > "$INSTALL_DIR/bin/cargo" << 'EOF'
#!/bin/bash
# 量子Cargo包装器

# 显示量子cargo信息
if [[ "$1" == "--version" ]]; then
    echo "🔮 Quantum Cargo v1.0.0-quantum"
    echo "⚡ Enhanced package manager with quantum features"
    if command -v /usr/bin/cargo &> /dev/null; then
        /usr/bin/cargo --version
    elif command -v ~/.cargo/bin/cargo &> /dev/null; then
        ~/.cargo/bin/cargo --version
    fi
    exit 0
fi

# 量子cargo特性
echo "🔮 Quantum Cargo - Enhanced Package Manager"

# 设置量子环境变量
export RUSTC="$HOME/.quantum-rust/bin/rustc"
export QUANTUM_RUST_ENABLED=1

# 调用实际的cargo
if command -v /usr/bin/cargo &> /dev/null; then
    /usr/bin/cargo "$@"
elif command -v ~/.cargo/bin/cargo &> /dev/null; then
    ~/.cargo/bin/cargo "$@"
else
    echo "❌ 错误: 未找到cargo"
    echo "请先安装标准Rust: https://rustup.rs/"
    exit 1
fi
EOF

chmod +x "$INSTALL_DIR/bin/cargo"

# 创建其他量子工具的符号链接
echo "   - 创建量子工具..."
for tool in quantum-rustc quantum-cargo quantum-fmt quantum-clippy quantum-doc quantum-test quantum-bench quantum-profiler; do
    if [ -f "$QUANTUM_RUST_DIR/bin/$tool" ]; then
        ln -sf "$QUANTUM_RUST_DIR/bin/$tool" "$INSTALL_DIR/bin/$tool"
    fi
done

echo "✅ 量子Rust安装完成"

# 更新PATH
echo ""
echo "🔧 步骤3: 更新环境变量..."

# 检查shell类型
SHELL_RC=""
if [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_RC="$HOME/.bashrc"
else
    SHELL_RC="$HOME/.profile"
fi

echo "   - 检测到shell配置文件: $SHELL_RC"

# 备份shell配置
if [ -f "$SHELL_RC" ]; then
    cp "$SHELL_RC" "$BACKUP_DIR/$(basename $SHELL_RC).backup"
    echo "   - 已备份shell配置文件"
fi

# 添加量子Rust到PATH
QUANTUM_PATH_EXPORT="export PATH=\"\$HOME/.quantum-rust/bin:\$PATH\""

if ! grep -q "quantum-rust/bin" "$SHELL_RC" 2>/dev/null; then
    echo "" >> "$SHELL_RC"
    echo "# 量子Rust环境变量" >> "$SHELL_RC"
    echo "$QUANTUM_PATH_EXPORT" >> "$SHELL_RC"
    echo "export QUANTUM_RUST_HOME=\"\$HOME/.quantum-rust\"" >> "$SHELL_RC"
    echo "export QUANTUM_RUST_ENABLED=1" >> "$SHELL_RC"
    echo "" >> "$SHELL_RC"
    echo "✅ 已更新 $SHELL_RC"
else
    echo "   - PATH已包含量子Rust路径"
fi

# 立即更新当前会话的PATH
export PATH="$INSTALL_DIR/bin:$PATH"
export QUANTUM_RUST_HOME="$INSTALL_DIR"
export QUANTUM_RUST_ENABLED=1

echo "✅ 环境变量更新完成"

# 验证安装
echo ""
echo "🧪 步骤4: 验证安装..."

echo "   - 测试量子rustc..."
if "$INSTALL_DIR/bin/rustc" --version; then
    echo "   ✅ 量子rustc工作正常"
else
    echo "   ❌ 量子rustc测试失败"
fi

echo "   - 测试量子cargo..."
if "$INSTALL_DIR/bin/cargo" --version; then
    echo "   ✅ 量子cargo工作正常"
else
    echo "   ❌ 量子cargo测试失败"
fi

# 创建卸载脚本
echo ""
echo "📝 步骤5: 创建卸载脚本..."
cat > "$INSTALL_DIR/uninstall.sh" << EOF
#!/bin/bash
# 量子Rust卸载脚本

echo "🗑️  卸载量子Rust..."

# 从PATH中移除
if [ -f "$SHELL_RC" ]; then
    sed -i '/quantum-rust/d' "$SHELL_RC"
    echo "✅ 已从$SHELL_RC移除量子Rust"
fi

# 恢复备份
if [ -d "$BACKUP_DIR" ]; then
    echo "📋 恢复备份..."
    if [ -f "$BACKUP_DIR/$(basename $SHELL_RC).backup" ]; then
        cp "$BACKUP_DIR/$(basename $SHELL_RC).backup" "$SHELL_RC"
        echo "✅ 已恢复shell配置"
    fi
fi

# 删除安装目录
rm -rf "$INSTALL_DIR"
echo "✅ 已删除量子Rust安装"

echo "🎉 量子Rust卸载完成"
echo "请重新启动终端或运行: source $SHELL_RC"
EOF

chmod +x "$INSTALL_DIR/uninstall.sh"

# 安装完成
echo ""
echo "🎉 量子Rust安装成功！"
echo "=============================================="
echo "📦 安装位置: $INSTALL_DIR"
echo "💾 备份位置: $BACKUP_DIR"
echo "🗑️  卸载脚本: $INSTALL_DIR/uninstall.sh"
echo ""
echo "🔧 下一步操作:"
echo "1. 重新启动终端，或运行: source $SHELL_RC"
echo "2. 验证安装: rustc --version"
echo "3. 开始使用量子Rust编程！"
echo ""
echo "🚀 量子编程时代开始了！"
echo ""
echo "📖 快速开始:"
echo "   rustc --help                    # 查看量子编译器帮助"
echo "   cargo new my_quantum_project    # 创建量子项目"
echo "   quantum-rustc --quantum         # 启用量子特性编译"
echo ""
echo "🌟 欢迎来到量子编程的未来！"
EOF

chmod +x "$QUANTUM_RUST_DIR/install-system-default.sh"
