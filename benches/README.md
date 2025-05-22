# Test Environment (2025-5-22)

CPU: AMD 5950x

OS: Debian 12 with 6.1.0-35-amd64 #1 SMP PREEMPT_DYNAMIC Debian 6.1.137-1 (2025-05-07) x86_64 GNU/Linux

Rust: rustc 1.87.0 (17067e9ac 2025-05-09)

Target: x86_64-unknown-linux-gnu

|         | subnetwork | ipnetwork | cidr  |
| :-----: | :--------: | :-------: | :---: |
| version |   0.5.4    |  0.21.0   | 0.3.1 |

# Benchmark from script

|  id   |   subnetwork    |    ipnetwork    |      cidr       |
| :---: | :-------------: | :-------------: | :-------------: |
|       | **total (sec)** | **total (sec)** | **total (sec)** |
|  #1   |      11.11      |      11.44      |      11.32      |
|  #2   |      11.08      |      11.43      |      11.34      |
|  #3   |      11.08      |      11.41      |      11.46      |
|  #4   |      11.10      |      11.30      |      11.25      |
|  #5   |      11.12      |      11.42      |      11.31      |
|  #6   |      11.13      |      11.39      |      11.34      |

# Benchmark from criterion

## First Shot

```bash
cidr                    time:   [8.5702 ms 8.5896 ms 8.6121 ms]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

ipnetwork               time:   [10.720 ms 10.758 ms 10.799 ms]
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high mild

subnetwork              time:   [7.1939 ms 7.2092 ms 7.2273 ms]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe
```

## Second Shot

```bash
cidr                    time:   [8.4223 ms 8.4636 ms 8.5133 ms]
                        change: [−1.9902% −1.4673% −0.8426%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

ipnetwork               time:   [8.5526 ms 8.5825 ms 8.6128 ms]
                        change: [−20.641% −20.223% −19.807%] (p = 0.00 < 0.05)
                        Performance has improved.

subnetwork              time:   [6.9187 ms 6.9336 ms 6.9495 ms]
                        change: [−4.1210% −3.8231% −3.5048%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  10 (10.00%) high mild
  1 (1.00%) high severe
```

## Third Shot

```bash
cidr                    time:   [8.4565 ms 8.4634 ms 8.4696 ms]
                        change: [−0.5825% −0.0018% +0.4944%] (p = 1.00 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) low severe
  3 (3.00%) high mild
  6 (6.00%) high severe

ipnetwork               time:   [8.5778 ms 8.5932 ms 8.6080 ms]
                        change: [−0.2714% +0.1248% +0.5196%] (p = 0.53 > 0.05)
                        No change in performance detected.
Found 27 outliers among 100 measurements (27.00%)
  15 (15.00%) low severe
  2 (2.00%) low mild
  3 (3.00%) high mild
  7 (7.00%) high severe

subnetwork              time:   [7.0644 ms 7.0712 ms 7.0769 ms]
                        change: [+1.7279% +1.9837% +2.2174%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low severe
  2 (2.00%) high mild
  6 (6.00%) high severe
```

I found that on my other AMD desktop, `subnetwork` outperformed the other two libraries, but on an Intel laptop the opposite was true.

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
