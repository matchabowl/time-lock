# Brute-force computation of RSW-style time-lock puzzles
The RSW-style time-lock puzzles are the task to compute $y = x^{2^T} \pmod M$ given integers $x, T, M$. This code brute-forces the solution through repeated squaring. Throughout, it stores intermediate results, so it is possible to resume from interrupted runs.

## Word of Warning
If you intend to run this code, ensure you have checked various more efficient methods of computing $y$, as for reasonably large values of $T$ and $M$ it will at least run for hours and is likely to take years.

In particular, if $x \in \{0, \pm 1\}$ or if a factorization of $M$ can be found, this code should not be used. Treat this code as a last resort, if no better approach works.

## Installation

## Usage
