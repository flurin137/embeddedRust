
#  Installation 
    rustup -V
    cargo install cargo-binutils
    cargo install cargo-generate
    rustup component add llvm-tools-preview
    npm install --global xpm@latest
    xpm install --global @xpack-dev-tools/openocd@latest

# Check if board responses
    openocd -f interface/stlink.cfg -f target/stm32f0x.cfg

# Build
    cargo build

# inspect compied data
    cargo readobj --bin embeddedRust -- -file-headers
    cargo size --bin embeddedRust --release -- -A