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

> cargo bench *pattern*

Runs all of the benchmarks with names that contain the pattern.

## Element access

For element access, the goal is to compare the speed of using various
access methods for array and vectors. All of the functions use the
&[i32] type that accepts both arrays and vectors.

**TL/DR:** The big story is that nothing stands out and that Rust
programmers can use the high level functions without concern. The
total variation is less than 5%. I often see a minor slow down using
vec compared array.

* element index array     time:   [334.14 ns 334.62 ns 335.17 ns]
* element for array       time:   [334.34 ns 334.78 ns 335.27 ns]
* element sum array       time:   [334.11 ns 334.60 ns 335.21 ns]
* element fold array      time:   [336.04 ns 336.84 ns 337.62 ns]
* element index vec       time:   [340.57 ns 341.15 ns 341.74 ns]
* element for vec         time:   [342.58 ns 343.05 ns 343.57 ns]
* element sum vec         time:   [346.27 ns 346.65 ns 347.03 ns]
* element fold vec        time:   [345.55 ns 345.99 ns 346.40 ns]

## Sliding windows

This category tests the performance of window relative to a loop.

**TL/DR:** There is basically no difference here.

* sliding window          time:   [692.19 ns 693.76 ns 695.81 ns]
* sliding for             time:   [698.90 ns 699.69 ns 700.50 ns]

## Pair access

This category tests the performance of zip relative to a loop.

**TL/DR:** There is basically no difference here.

* dual zip                time:   [537.16 ns 537.80 ns 538.49 ns]
* dual for                time:   [539.97 ns 540.98 ns 542.07 ns]

## Tail recursion

This category tests the performance of tail recursion. You can
use the tailcall attribute to guarantee that your method is
tail recursive.

**TL/DR:** Minor changes to the code have big impacts and the best
case is that it equals the performance of the functional programming,
so I'd avoid using tail recursion.

The results fall into 3 groups:

* **painful** (~50 microseconds) In sum_via_match, the compiler doesn't see the tail recursion.
* **slow** (~3 microseconds) We get tail recursion, but with 3 way matches.
* **fast** (~0.3 microseconds) The rest of the tests perform like the iterative versions.

The most interesting is the difference between sum_via_match_accum and
sum_via_match2_accum where deleteing one of the match branches makes
the code 8x faster.

I don't see any performance change by adding tailcall. There is one
case that tailcall doesn't accept (eg. sum_via_if), which is optimized
by the compiler.

* painful
  * tail match              time:   [52.022 µs 52.132 µs 52.248 µs]
* slow
  * tail match accum        time:   [3.0251 µs 3.0368 µs 3.0476 µs]
  * tail len match accum    time:   [3.0406 µs 3.0504 µs 3.0597 µs]
* fast
  * tail match2 accum       time:   [344.87 ns 345.50 ns 346.18 ns]
  * tail if idx accum       time:   [346.24 ns 346.83 ns 347.45 ns]
  * tail if accum           time:   [350.67 ns 351.20 ns 351.69 ns]
  * tail if                 time:   [345.83 ns 346.36 ns 346.90 ns]

## Option processing

This category tests processing an array of Option<String> by computing
the sum of the string lengths.

**TL/DR:** The iterators with a sum are faster than loops. I don't yet
understand why this happens. Surpisingly, "if let" is slower than the
other approaches.

* slow
  * option for if           time:   [5.7662 µs 5.7959 µs 5.8447 µs]
  * option for if_let       time:   [6.1111 µs 6.3317 µs 6.6122 µs]
  * option for match        time:   [5.6676 µs 5.8533 µs 6.1018 µs]
* fast
  * option filer_map        time:   [2.6265 µs 2.6312 µs 2.6362 µs]
  * option match sum        time:   [2.6810 µs 2.6863 µs 2.6916 µs]
  * option map_or           time:   [2.6883 µs 2.6928 µs 2.6971 µs]

## Branching

This category tests differnt forms of branching. It also
includes some data look up for comparison.

**TL/DR:** Handling programming errors using panic instead of
Result<i32,String> runs 31% faster. For 10 items, looking them
up in an array is 8% faster than using match.

* lookup
  * lookup array            time:   [2.8229 µs 2.8286 µs 2.8346 µs]
  * lookup hashmap          time:   [60.310 µs 60.432 µs 60.562 µs]
* branching
  * branching iter match    time:   [3.0762 µs 3.0813 µs 3.0868 µs]
  * branching iter match result     time:   [4.4563 µs 4.4636 µs 4.4710 µs]
  * branching for match     time:   [3.1471 µs 3.1522 µs 3.1570 µs]
  * branching iter if       time:   [3.1802 µs 3.1867 µs 3.1928 µs]
  * branching for if        time:   [3.1749 µs 3.1836 µs 3.1924 µs]

## Dispatch

This category test differnt forms of dispatching.

**TL/DR:** Using a template for a passed in function is roughly 2.5x
faster. Dynamic dispatch through traits is 7x faster than lambdas.
Using a member function on an enum is 3x faster than dynamic dispatch.

* single
  * dispatch func           time:   [8.2092 µs 8.2260 µs 8.2435 µs]
  * dispatch func template  time:   [3.0777 µs 3.0827 µs 3.0882 µs]
* multiple
  * dispatch lambdas        time:   [57.953 µs 58.084 µs 58.227 µs]
  * dispatch iter objs      time:   [8.2814 µs 8.3071 µs 8.3319 µs]
  * dispatch iter enums     time:   [2.7508 µs 2.7563 µs 2.7618 µs]


