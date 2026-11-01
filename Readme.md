# Brute-force computation of RSW-style time-lock puzzles
The RSW-style time-lock puzzles are the task to compute $y = x^{2^T} \pmod M$ given integers $x, T, M$. This code brute-forces the solution through repeated squaring. Throughout, it stores intermediate results, so it is possible to resume from interrupted runs.

## Word of Warning
If you intend to run this code, ensure you have checked various more efficient methods of computing $y$, as for reasonably large values of $T$ and $M$ it will at least run for hours and is likely to take years.

In particular, if $x \in \{0, \pm 1\}$ or if a factorization of $M$ can be found, this code should not be used. Treat this code as a last resort, if no better approach works.

## Installation
Building requires a [Rust toolchain](https://rustup.rs/), version 1.85 or newer. From the root of this repository, run:

```sh
cd rsw
cargo build --release
```

This creates the executable `rsw/target/release/rsw`. Make sure to use the `--release` build, since an unoptimized build is much slower.

## Usage
The commands below are run from the `rsw` directory.

### Starting a computation
Create an input file containing $x$, $T$ and $M$ on separate lines, in the following format:

```
x = 3
t = 10
m = 1000003
```

Then start the computation, providing a file to store the progress in:

```sh
./target/release/rsw --mode start --input puzzle.txt --output progress.txt
```

The input and output must be different files. **If the output file already exists, it is overwritten**, so take care not to reuse the progress file of an earlier run.

When the computation is done, the result is printed and appended to the output file:

```
x = 3; t = 10; m = 1000003
Finished; x = 96172
```

The value of $y$ is the `x` on the `Finished` line.

### Resuming a computation
Every 100,000,000 squarings, the current value is appended to the output file, along with the number of squarings still to do:

```
x = 3; t = 100000005; m = 1000003
x = 470184; remaining_t = 5
```

To continue an interrupted run from its last saved line, pass the progress file as the input:

```sh
./target/release/rsw --mode resume --input progress.txt
```

Further progress is appended to this same file, so `--output` is not allowed in this mode. Any squarings done since the last saved line are lost and computed again.

### Options

| Option | Short | Description |
| --- | --- | --- |
| `--mode <start\|resume>` | `-m` | Start a new computation, or resume an interrupted one. |
| `--input <file>` | `-i` | The puzzle file (`start`) or the progress file (`resume`). |
| `--output <file>` | `-o` | The progress file to create. Required for `start`, not allowed for `resume`. |
