
#  Installation 
    rustup -V
    cargo install cargo-binutils
    cargo install cargo-generate
    cargo install cargo-flash
    rustup component add llvm-tools-preview
    npm install --global xpm@latest
    xpm install --global @xpack-dev-tools/openocd@latest

# Check if board responses
    openocd -f interface/stlink.cfg -f target/stm32f0x.cfg

# Build
    cargo build

# inspect compied data
    cargo readobj --bin embedded_rust -- -file-headers
    cargo size --bin embedded_rust --release -- -A

# memory.x 
* from https://www.st.com/resource/en/datasheet/stm32f103rb.pdf (P34)


# debug
    arm-none-eabi-gdb
    arm-none-eabi-gdb -q ".\target\thumbv6m-none-eabi\debug\embedded_rust"
    target remote :3333

Or as alternative 
    cargo run

# Flash
    cargo flash --release --chip STM32L073RZTx
