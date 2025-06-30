#!/bin/bash
# Quantum Rust Installation Script

echo "🚀 Installing Quantum Rust Distribution"
echo "========================================"

# Check system requirements
echo "🔍 Checking system requirements..."

# Install binaries
echo "📦 Installing quantum tools..."
sudo cp bin/* /usr/local/bin/

# Install libraries
echo "📚 Installing quantum libraries..."
sudo cp -r lib/* /usr/local/lib/

# Install documentation
echo "📖 Installing documentation..."
sudo cp -r doc/* /usr/local/share/doc/quantum-rust/

echo "✅ Quantum Rust installed successfully!"
echo "🔮 Run 'quantum-rustc --version' to verify installation"
