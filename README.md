## All Rust libraries and binaries created by Ben Santora. All were coded using AI: ChatGPT - Widenex Rust GPT - Microsoft CoPilot

## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

# riscv_sysmon (binary)

**riscv_sysmon** is a system monitoring tool for Linux-based RISC-V single-board computers (SBCs). Written in Rust, it provides real-time metrics on CPU usage, memory statistics, and system health, tailored for RISC-V Linux environments.

## Features
- **CPU Usage**: Track system-wide and per-core CPU usage.
- **Memory Stats**: Monitor available and used memory.
- **System Health**: Display overall system health with lightweight, real-time metrics.

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/bensatlantik/riscv_sysmon.git
   cd riscv_sysmon
```
## Build the project:

```bash
cargo build --release --target=riscv64gc-unknown-linux-gnu
```

## Run the binary:

```bash
./target/riscv64gc-unknown-linux-gnu/release/riscv_sysmon
```

## Usage
Run riscv_sysmon with the following command:

```bash
riscv_sysmon --interval 2
--interval <seconds>: Set the update interval for monitoring output (default: 1 second).
```

## License
This project is licensed under the MIT License

## Author
Ben Santora 
