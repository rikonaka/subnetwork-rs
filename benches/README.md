# Test Environment (2025-5-23)

> Thanks to Reddit user `jaskij`, `Trader-One` and `CowRepresentative820` 😉.

CPU: Intel i5-11300H

OS: Debian Testing with 6.12.27-amd64 #1 SMP PREEMPT_DYNAMIC Debian 6.12.27-1 (2025-05-06) x86_64 GNU/Linux

Rust: rustc 1.87.0 (17067e9ac 2025-05-09)

Target: x86_64-unknown-linux-gnu

|         | subnetwork | ipnet  | ipnetwork | cidr  |
| :-----: | :--------: | :----: | :-------: | :---: |
| version |   0.5.4    | 2.11.0 |  0.21.0   | 0.3.1 |

## Benchmark from criterion

### First Shot

```bash
cidr                    time:   [9.7468 ms 9.8063 ms 9.8736 ms]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

ipnetwork               time:   [37.261 ms 37.362 ms 37.474 ms]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

ipnet                   time:   [18.282 ms 18.360 ms 18.446 ms]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

subnetwork              time:   [6.4903 ms 6.5277 ms 6.5705 ms]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
```

### Second Shot

```bash
cidr                    time:   [9.8570 ms 9.9841 ms 10.140 ms]
                        change: [+0.3137% +1.8128% +3.6125%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

ipnetwork               time:   [35.985 ms 36.356 ms 36.849 ms]
                        change: [−3.7572% −2.6925% −1.3371%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

ipnet                   time:   [18.385 ms 18.489 ms 18.589 ms]
                        change: [+0.0012% +0.7024% +1.3940%] (p = 0.06 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe

subnetwork              time:   [6.5836 ms 6.6687 ms 6.7748 ms]
                        change: [+0.5541% +2.1607% +3.9783%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
```

### Third Shot

```bash
cidr                    time:   [9.8599 ms 9.9237 ms 9.9912 ms]
                        change: [−2.2603% −0.6055% +0.8157%] (p = 0.47 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

ipnetwork               time:   [35.983 ms 36.171 ms 36.386 ms]
                        change: [−1.9407% −0.5087% +0.6992%] (p = 0.48 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

ipnet                   time:   [18.252 ms 18.348 ms 18.449 ms]
                        change: [−1.4992% −0.7611% +0.0472%] (p = 0.05 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe

subnetwork              time:   [6.6370 ms 6.7392 ms 6.8773 ms]
                        change: [−1.1404% +1.0563% +3.5920%] (p = 0.40 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
```

# Test Environment (old)

CPU: AMD 5950x

OS: Debian 11 with 5.10.0-23-amd64 #1 SMP Debian 5.10.179-1 (2023-05-12) x86_64 GNU/Linux

Rust: rustc 1.70.0 (90c541806 2023-05-31)

Target: x86_64-unknown-linux-gnu

|         | subnetwork | ipnetwork | cidr  |
| :-----: | :--------: | :-------: | :---: |
| version |   0.2.7    |  0.20.0   | 0.2.1 |

## Test Content

Run the compiled program 10,000 times and get the final running time. The program returns all subnet IPs within 192.168.0.0/16.

## Benchmark

|  id   | subnetwork |       |        | ipnetwork |       |        |   cidr    |       |        |
| :---: | :--------: | :---: | :----: | :-------: | :---: | :----: | :-------: | :---: | :----: |
|       | **total**  | user  | system | **total** | user  | system | **total** | user  | system |
|  #1   |   7.483    | 5.67  |  2.03  |   9.513   | 7.47  |  2.26  |  11.198   | 8.84  |  2.58  |
|  #2   |   7.898    | 6.04  |  2.10  |   8.989   | 7.06  |  2.15  |  11.217   | 8.83  |  2.60  |
|  #3   |   7.841    | 5.94  |  2.14  |   9.647   | 7.58  |  2.29  |  11.163   | 8.78  |  2.60  |
