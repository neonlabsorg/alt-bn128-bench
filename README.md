# Alt BN 128 Benchmark

<a name="top"></a>
<!-- RESULTS -->

## About

We need to know, how much BPF instructions will be used for every operation on elliptic curves.
We can't to know it directly, but we can compare the time needed for these operations and time, needed for Keccak hash for the same data.
These benchmarks allow us to approximately define the amount of data in bytes for ellicptic curves operations and Keccak hash and compare result times.
<p align="right">(<a href="#top">back to top</a>)</p>

## Results

| Operation      | like Keccak Hash of        |
|----------------|----------------------------|
| Addition       | ~120 bytes                 |
| Multiplication | ~200 bytes                 |
| Pairing        | ~110 bytes                 |

<p align="right">(<a href="#top">back to top</a>)</p>

## Detailed results

### Addition

| Operation | Rounds | Size  | Pure K-ratio | K-ratio |
|-----------|--------|-------|--------------|---------|
| Addition  | 1000   | 120   | 0.8864 K     | 0.8864  |
| Addition  | 10000  | 120   | 0.7878 K     | 0.7878  |
| Addition  | 100000 | 120   | 0.8681 K     | 0.8681  |
| Addition  | 1000   | 1200  | 0.09466 K    | 0.9466  |
| Addition  | 10000  | 1200  | 0.09977 K    | 0.9977  |
| Addition  | 100000 | 1200  | 0.1001 K     | 1.001   |
| Addition  | 1000   | 12000 | 0.01135 K    | 1.135   |
| Addition  | 10000  | 12000 | 0.01013 K    | 1.013   |
| Addition  | 100000 | 12000 | 0.01017 K    | 1.017   |

### Multiplication

| Operation | Rounds | Size  | Pure K-ratio | K-ratio |
|-----------|--------|-------|--------------|---------|
| Addition  | 1000   | 200   | 0.7819 K     | 0.7819  |
| Addition  | 10000  | 200   | 0.6341 K     | 0.6341  |
| Addition  | 100000 | 200   | 0.7509 K     | 0.7509  |
| Addition  | 1000   | 2000  | 0.1053 K     | 1.053   |
| Addition  | 10000  | 2000  | 0.1000 K     | 1.000   |
| Addition  | 100000 | 2000  | 0.1033 K     | 1.033   |
| Addition  | 1000   | 20000 | 0.01075 K    | 1.075   |
| Addition  | 10000  | 20000 | 0.01062 K    | 1.062   |
| Addition  | 100000 | 20000 | 0.009835 K   | 0.9835  |

### Pairing

| Operation | Rounds | Size  | Pure K-ratio | K-ratio |
|-----------|--------|-------|--------------|---------|
| Addition  | 1000   | 110   | 0.3633 K     | 0.7819  |
| Addition  | 10000  | 110   | 0.5905 K     | 0.6341  |
| Addition  | 100000 | 110   | 1.121 K      | 0.7509  |
| Addition  | 1000   | 1100  | 0.04500 K    | 1.053   |
| Addition  | 10000  | 1100  | 0.09270 K    | 1.000   |
| Addition  | 100000 | 1100  | 0.1273 K     | 1.033   |
| Addition  | 1000   | 11000 | 0.007706 K   | 1.075   |
| Addition  | 10000  | 11000 | 0.01075 K    | 1.062   |
| Addition  | 100000 | 11000 | 0.01727 K    | 0.9835  |

<p align="right">(<a href="#top">back to top</a>)</p>

## Example of Usage

Starting 1000 rounds of ALL benchmarks on the random data of 256 bytes:
```bash
alt-bn128-bench --count 1000 --size 256
```

Same as above, but in short notaion:
```bash
alt-bn128-bench -c 1000 -s 256
```

Start only specific benchmark with default values:
```bash
# Start Addition benchmark
alt-bn128-bench add

# Start Multiplication benchmark
alt-bn128-bench mul

# Start Pairing benchmark
alt-bn128-bench pair
```

Starting 1000 rounds of Addition benchmark on the random data of 256 bytes:
```bash
alt-bn128-bench --count 1000 --size 256 add
```
<p align="right">(<a href="#top">back to top</a>)</p>

<!-- ARGUMENTS -->

## Arguments List

| CLI arg       | Short | Req | Type    | Default         | Example   | Description                                                                                       |
|--------------------|-------------|----------|---------|-----------------|-----------|---------------------------------------------------------------------------------------------------|
| `--count`          | `-c`        | No       | `usize` | 10000           | `-c 500` | Number of rounds                                                                                  |
| `--size`           | `-s`        | No       | `usize` | 10000           | `-s 256`  | Size of random input buffer in bytes                                                              |
| `add / mul / pair` |             | No       | `Bench` | -  | `add`     | We can specify the benchmarks to execute. If field is not set, all benchmarks will be processed.  |

<p align="right">(<a href="#top">back to top</a>)</p>

## Roadmap
<!--
- [] Feature 1
- [] Feature 2
- [] Feature 3
    - [] Nested Feature
-->
See the [open issues](https://github.com/github_username/repo_name/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#top">back to top</a>)</p>
