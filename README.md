# Alt BN 128 Benchmark

<a name="top"></a>
<!-- RESULTS -->

## About

We need to know, how much BPF instructions will be used for every operation on elliptic curves.
We can't to know it directly, but we can compare the time needed for these operations and time, needed for Keccak hash for the same data.
These benchmarks allow us to approximately define the amount of data in bytes for ellicptic curves operations and Keccak hash and compare result times.
<p align="right">(<a href="#top">back to top</a>)</p>

## Results

| Operation      | like Keccak Hash of |
|----------------|---------------------|
| Addition       | ~400 bytes          |
| Multiplication | ~12'000 bytes       |
| Pairing        | ~140'000 bytes      |

<p align="right">(<a href="#top">back to top</a>)</p>

## Detailed results

### Addition

| Operation | Rounds | Size  | Pure K-ratio | K-ratio |
|-----------|--------|-------|--------------|---------|


### Multiplication

| Operation | Rounds | Size  | Pure K-ratio | K-ratio |
|-----------|--------|-------|--------------|---------|

### Pairing

| Operation | Rounds | Size  | Pure K-ratio | K-ratio |
|-----------|--------|-------|--------------|---------|

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
See the [open issues](https://github.com/neonlabsorg/alt-bn128-bench/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#top">back to top</a>)</p>
