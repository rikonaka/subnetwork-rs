# Test Environment (2025-5-22)

CPU: Intel i5-11300H

OS: Debian Testing with 6.12.27-amd64 #1 SMP PREEMPT_DYNAMIC Debian 6.12.27-1 (2025-05-06) x86_64 GNU/Linux

Rust: rustc 1.87.0 (17067e9ac 2025-05-09)

Target: x86_64-unknown-linux-gnu

|         | subnetwork | ipnetwork | cidr  |
| :-----: | :--------: | :-------: | :---: |
| version |   0.5.4    |  0.21.0   | 0.3.1 |

# Executable File Size

```bash
➜  benches git:(main) ✗ du test_cidr/target/x86_64-unknown-linux-gnu/release/test_cidr -h
424K    test_cidr/target/x86_64-unknown-linux-gnu/release/test_cidr
➜  benches git:(main) ✗ du test_ipnetwork/target/x86_64-unknown-linux-gnu/release/test_ipnetwork -h
436K    test_ipnetwork/target/x86_64-unknown-linux-gnu/release/test_ipnetwork
➜  benches git:(main) ✗ du test_subnetwork/target/x86_64-unknown-linux-gnu/release/test_subnetwork -h
436K    test_subnetwork/target/x86_64-unknown-linux-gnu/release/test_subnetwork
```

# Benchmark from script

|  id   |   subnetwork    |            |    ipnetwork    |            |      cidr       |            |
| :---: | :-------------: | :--------: | :-------------: | :--------: | :-------------: | :--------: |
|       | **total (sec)** | user (sec) | **total (sec)** | user (sec) | **total (sec)** | user (sec) |
|  #1   |      27.81      |    9.16    |      29.04      |    9.25    |      28.79      |   10.12    |
|  #2   |      27.94      |    9.23    |      29.55      |   10.89    |      29.85      |   12.03    |
|  #3   |      28.32      |    9.85    |      28.65      |    9.57    |      29.02      |   10.68    |
|  #4   |      28.23      |    8.90    |      28.91      |    9.81    |      29.17      |   10.11    |
|  avg  |     28.075      |            |     29.0375     |            |     29.2075     |            |

# Benchmark from criterion  

```bash
cidr                    time:   [7.3136 ms 7.3865 ms 7.4748 ms]
                        change: [+9630.4% +9824.5% +10007%] (p = 0.00 < 0.05)
                        Performance has regressed.

ipnetwork               time:   [9.0608 ms 9.1397 ms 9.2195 ms]
                        change: [+9759.5% +9908.8% +10056%] (p = 0.00 < 0.05)
                        Performance has regressed.

subnetwork              time:   [10.589 ms 10.668 ms 10.752 ms]
                        change: [+9911.0% +10025% +10147%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

# Test Environment (old)

CPU: AMD 5950x

OS: Debian 11 with 5.10.0-23-amd64 #1 SMP Debian 5.10.179-1 (2023-05-12) x86_64 GNU/Linux

Rust: rustc 1.70.0 (90c541806 2023-05-31)

Target: x86_64-unknown-linux-gnu

|         | subnetwork | ipnetwork | cidr  |
| :-----: | :--------: | :-------: | :---: |
| version |   0.2.7    |  0.20.0   | 0.2.1 |

# Test Content

Run the compiled program 10,000 times and get the final running time. The program returns all subnet IPs within 192.168.0.0/16.

# Benchmark

|  id   | subnetwork |       |        | ipnetwork |       |        |   cidr    |       |        |
| :---: | :--------: | :---: | :----: | :-------: | :---: | :----: | :-------: | :---: | :----: |
|       | **total**  | user  | system | **total** | user  | system | **total** | user  | system |
|  #1   |   7.483    | 5.67  |  2.03  |   9.513   | 7.47  |  2.26  |  11.198   | 8.84  |  2.58  |
|  #2   |   7.898    | 6.04  |  2.10  |   8.989   | 7.06  |  2.15  |  11.217   | 8.83  |  2.60  |
|  #3   |   7.841    | 5.94  |  2.14  |   9.647   | 7.58  |  2.29  |  11.163   | 8.78  |  2.60  |
