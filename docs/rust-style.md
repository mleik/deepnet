# Rust Style Guide

Style guide for the use of Rust for the DeepNet Project.

### What we want:

1. Safety safety safety. 
2. Developer productivity.

### What we are willing to trade.
* Runtime performance.

## 1. Listen to compiler error, warnings and clippy lints.
* PR must not have any error warnings or lint warnings.
* If you feel that an error or lint is totally wrong add a annotation disabling that rule on that code only.
  * Aka only ignore rules where the fix is uglier than an annotation. 

## 2. Unsafe
* DO NOT USE UNSAFE in DeepNet.
* Unsafe code in crates and stander library is acceptable. 
* Do not use unsafe to fix a performance issue.
  * Find a crate that implements what you need.
* If we need to call C code:
  * Create a dedicated crate for that.
* DO NOT USE UNSAFE!!!!!

## 3. Error handling.
* Log all errors.
* Try to recover from errors.
* Never use `unwrap()` as if there is an error it will just kill the program without a reason.
* If you are 110% sure that an error can never happen. Not even with future code changes outside the function you may use. `expect()` but:
  * Have it print a good explanation what assumption that would never happen.

## 4. Development productivity
* It's ok to use `clone()` to shut up the borrow checker.
* Rust have deep rabbits holes for fast and minimal memory usage. Just don't.
* It's more important to get more feature that fast code.
* Enums are better than Traits.
* Don't be afraid of Mutex and other locks.