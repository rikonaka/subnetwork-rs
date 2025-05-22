#!/bin/bash

# subnetwork
start_time=$(date +%s.%N)
for i in {1..10000}; do ./test_subnetwork/target/x86_64-unknown-linux-gnu/release/test_subnetwork; done
end_time=$(date +%s.%N)

elapsed=$(echo "$end_time - $start_time" | bc)
echo "subnetwork: $elapsed s"

# ipnetwork
start_time=$(date +%s.%N)
for i in {1..10000}; do ./test_ipnetwork/target/x86_64-unknown-linux-gnu/release/test_ipnetwork; done
end_time=$(date +%s.%N)

elapsed=$(echo "$end_time - $start_time" | bc)
echo "ipnetwork: $elapsed s"

# cidr
start_time=$(date +%s.%N)
for i in {1..10000}; do ./test_cidr/target/x86_64-unknown-linux-gnu/release/test_cidr; done
end_time=$(date +%s.%N)

elapsed=$(echo "$end_time - $start_time" | bc)
echo "cidr: $elapsed s"