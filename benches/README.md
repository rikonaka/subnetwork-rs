# Test Environment (2025-5-23)

> Thanks to Reddit user `jaskij`, `Trader-One` and `CowRepresentative820` 😉.

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
|  #1   |      1.78       |      2.10       |      1.79       |
|  #2   |      1.78       |      2.13       |      1.83       |
|  #3   |      1.77       |      2.11       |      1.81       |
|  #4   |      1.75       |      2.08       |      1.82       |
|  #5   |      1.78       |      2.09       |      1.81       |
|  #6   |      1.77       |      2.10       |      1.81       |

# Benchmark from criterion

## First Shot

```bash
cidr                    time:   [10.047 ms 10.062 ms 10.075 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) low severe

ipnetwork               time:   [39.893 ms 39.906 ms 39.923 ms]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

subnetwork              time:   [8.2594 ms 8.2833 ms 8.3078 ms]
```

## Second Shot

```bash
cidr                    time:   [10.088 ms 10.101 ms 10.115 ms]
                        change: [+0.1997% +0.3941% +0.5860%] (p = 0.00 < 0.05)
                        Change within noise threshold.

ipnetwork               time:   [40.746 ms 40.759 ms 40.774 ms]
                        change: [+2.0819% +2.1379% +2.1897%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

subnetwork              time:   [8.2852 ms 8.3096 ms 8.3335 ms]
                        change: [−0.1052% +0.3175% +0.7279%] (p = 0.14 > 0.05)
                        No change in performance detected.
```

## Third Shot

```bash
cidr                    time:   [10.744 ms 10.808 ms 10.883 ms]
                        change: [+6.2596% +7.0023% +7.6855%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

ipnetwork               time:   [39.897 ms 39.915 ms 39.937 ms]
                        change: [−2.1285% −2.0728% −2.0100%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

subnetwork              time:   [8.1547 ms 8.1616 ms 8.1706 ms]
                        change: [−2.0800% −1.7807% −1.4760%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) low mild
  6 (6.00%) high mild
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

# Test Content

Run the compiled program 10,000 times and get the final running time. The program returns all subnet IPs within 192.168.0.0/16.

# Benchmark

|  id   | subnetwork |       |        | ipnetwork |       |        |   cidr    |       |        |
| :---: | :--------: | :---: | :----: | :-------: | :---: | :----: | :-------: | :---: | :----: |
|       | **total**  | user  | system | **total** | user  | system | **total** | user  | system |
|  #1   |   7.483    | 5.67  |  2.03  |   9.513   | 7.47  |  2.26  |  11.198   | 8.84  |  2.58  |
|  #2   |   7.898    | 6.04  |  2.10  |   8.989   | 7.06  |  2.15  |  11.217   | 8.83  |  2.60  |
|  #3   |   7.841    | 5.94  |  2.14  |   9.647   | 7.58  |  2.29  |  11.163   | 8.78  |  2.60  |
