# Owen's Rust Benchmarks

This is a set of benchmarks that I wrote to test the performance of
various Rust features and their relative performance. Many of the
benchmarks are based on similar ones that Pavan wrote for Scala.

My environment currently is:

* Mac OS 14.4.1
* Apple M2 Max, 12 cores, 38 GPU (Mac14,6)
* Rust 1.77.1

As always, benchmarks run on a multi-process OS have substantial
jitter. (Criterion does do warm ups and many iterations, but the
exact numbers change.)

The benchmarks all use random data with a fixed seed to create
an array of 10,000 elements. I fixed the seed to remove that
source of noise.

## Usage

> cargo bench <pattern>

Runs all of the benchmarks with names that contain the pattern.

## Element access

For element access, the goal is to compare the speed of using various
access methods for array and vectors. All of the functions use the
&[i32] type that accepts both arrays and vectors.

**TL/DR:** The big story is that nothing stands out and that Rust
programmers can use the high level functions without concern. The
total variation is less than 5%. I often see a minor slow down using
vec compared array.

element index array     time:   [334.14 ns 334.62 ns 335.17 ns]
element for array       time:   [334.34 ns 334.78 ns 335.27 ns]
element sum array       time:   [334.11 ns 334.60 ns 335.21 ns]
element fold array      time:   [336.04 ns 336.84 ns 337.62 ns]
element index vec       time:   [340.57 ns 341.15 ns 341.74 ns]
element for vec         time:   [342.58 ns 343.05 ns 343.57 ns]
element sum vec         time:   [346.27 ns 346.65 ns 347.03 ns]
element fold vec        time:   [345.55 ns 345.99 ns 346.40 ns]

## Sliding windows

This category tests the performance of window relative to a loop.

**TL/DR:** There is basically no difference here.

sliding window          time:   [692.19 ns 693.76 ns 695.81 ns]
sliding for             time:   [698.90 ns 699.69 ns 700.50 ns]

## Pair access

This category tests the performance of zip relative to a loop.

**TL/DR:** There is basically no difference here.

dual zip                time:   [537.16 ns 537.80 ns 538.49 ns]
dual for                time:   [539.97 ns 540.98 ns 542.07 ns]

## Tail recursion

This category tests the performance of tail recursion. You can
use the tailcall attribute to guarantee that your method is
tail recursive.

**TL/DR:** Minor changes to the code have big impacts and the best
case is that it equals the performance of the functional programming,
so I'd avoid using tail recursion.

The results fall into 3 groups:

* **painful** (~50 microseconds) In sum_via_match, the compiler doesn't see the tail recursion.
* **slow** (~3 microseconds) In sum_via_match_accum and sum_via_len_match, we get tail recursion, but 3 way branches.
* **fast** (~0.3 microseconds) The rest of them perform like the iterative versions.

The most interesting is the difference between sum_via_match_accum and
sum_via_match2_accum where deleteing one of the match branches makes
the code 8x faster.

I don't see any performance change by adding tailcall and some cases
it doesn't accept (eg. tail::sum_via_if) are recognized by the
compiler.

tail match              time:   [52.022 µs 52.132 µs 52.248 µs]
tail match accum        time:   [3.0251 µs 3.0368 µs 3.0476 µs]
tail match2 accum       time:   [344.87 ns 345.50 ns 346.18 ns]
tail len match accum    time:   [3.0406 µs 3.0504 µs 3.0597 µs]
tail if idx accum       time:   [346.24 ns 346.83 ns 347.45 ns]
tail if accum           time:   [350.67 ns 351.20 ns 351.69 ns]
tail if accum           time:   [350.67 ns 351.20 ns 351.69 ns]


