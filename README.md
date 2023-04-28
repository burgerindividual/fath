# fath
### Fa(st ma)th library written in Rust, built for speed.
--------------------

Most of the functions in here are faster than equivalent functions in sleef, at the expense of safety. This library heavily relies on unsafe and nightly features. Use at your own risk.
When using SIMD functions in this package, compile with LTO and `opt-level=3` to ensure that auto-vectorization takes place. All SIMD functions have a feature cap at AVX2, and nothing in this library utilizes anything from AVX512. If certain functions vectorize on lower requirements, that's a bonus.

(TODO: add comparison to sleef_rs)

## Currently Implemented Functions
**Approximate `f32` Functions:**

Allows setting a variable precision level as a `const` generic.
* `sin` and `cos`
  * Does include wrapping with a range reduction, but will become less accurate as the input gets larger.
* `log` (`const` base and variable base)
  * This is based on a log base 2 approximation, and is scaled for other bases. The fastest version of this is `const` base 2.0.

**Exact Unsigned Integer Functions:**
* `ilog` with `const` base
  * Has multiple implementations depending on the base to achieve maximum performance. The fastest impl is for log base 2.

## Contributing
Any help on the library is greatly appreciated. If you'd like to contribute, just submit a PR and I'll respond to it as soon as I can.
For development, I'd recommend looking at the genertated assembly often. For development of individual functions, I would recommend using a tool like [Compiler Explorer](https://rust.godbolt.org/), and using something like llvm-mca (available in CE under "tools") often to get an idea of performance on different platforms.
