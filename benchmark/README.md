# Test Environment

CPU: AMD 5950x

OS: Debian 11 with 5.10.0-23-amd64 #1 SMP Debian 5.10.179-1 (2023-05-12) x86_64 GNU/Linux

Rust: rustc 1.70.0 (90c541806 2023-05-31)

Target: x86_64-unknown-linux-gnu

|         | subnetwork | ipnetwork | cidr  |
| :-----: | :--------: | :-------: | :---: |
| version |   0.2.7    |  0.20.0   | 0.2.1 |

# Test Content

Run the 10,000 times calculations separately which return all ip's for subnet `192.168.0.0/16` (not printed), final statistics on the time required to run.

# Benchmark

| time(s) | subnetwork |       |        | ipnetwork |       |        |   cidr    |       |        |
| :-----: | :--------: | :---: | :----: | :-------: | :---: | :----: | :-------: | :---: | :----: |
|         | **total**  | user  | system | **total** | user  | system | **total** | user  | system |
|   #1    |   7.483    | 5.67  |  2.03  |   9.513   | 7.47  |  2.26  |  11.198   | 8.84  |  2.58  |
|   #2    |   7.898    | 6.04  |  2.10  |   8.989   | 7.06  |  2.15  |  11.217   | 8.83  |  2.60  |
|   #3    |   7.841    | 5.94  |  2.14  |   9.647   | 7.58  |  2.29  |  11.163   | 8.78  |  2.60  |