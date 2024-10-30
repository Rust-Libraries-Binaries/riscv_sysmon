## What riscv_sysmon Does
riscv_sysmon is a lightweight system monitoring tool designed for RISC-V single-board computers (SBCs) running Linux. It gives developers real-time visibility into CPU usage, memory statistics, and overall system health, all tailored to the RISC-V architecture.

## Key features:

CPU Monitoring: Tracks total CPU usage across cores, displaying a quick snapshot of CPU load on the system.
Memory Usage: Shows used vs. total memory, which helps gauge whether an application is memory-intensive and how it impacts system performance.
Real-Time Display: Refreshes at user-defined intervals, providing an updated view of resource usage without overloading the system.

## How RISC-V Developers Can Use It
For developers working on RISC-V Linux SBCs, riscv_sysmon is particularly useful because it provides insight into how their applications affect system resources in a RISC-V-specific environment. Here’s why that matters:

Performance Profiling: By seeing real-time resource usage, developers can spot if their applications are CPU or memory-intensive and optimize accordingly.
Resource Constraints: Many RISC-V SBCs have limited CPU and memory resources compared to ARM or x86, so monitoring these metrics is critical for applications needing to run efficiently.
Development Feedback: If a developer is building or porting software to RISC-V Linux, riscv_sysmon can help verify that the system remains stable and responsive under various loads.

## Usage
Once you have it installed, you can:

Run it alongside applications you’re testing on a RISC-V SBC to see their CPU and memory impact.
Experiment with performance tuning by watching how different settings or optimizations affect system metrics.
Identify resource bottlenecks when using the board in practical scenarios, such as for IoT, lightweight server tasks, or application prototyping.

## Getting Started
Choose a Board: The SiFive HiFive Unmatched and StarFive VisionFive are ideal choices for exploring RISC-V with Linux, but any RISC-V board capable of running Linux will work.

Install a Debian-Based OS: Look for an image provided by the manufacturer or community that supports your chosen board. A Debian-based system will make it easy to set up riscv_sysmon.

Run riscv_sysmon: After cloning and building the tool, you can start using it right away to monitor system performance at different load levels. You’ll get real-time feedback on how your applications or system changes impact the CPU and memory usage.

