# Microlisp

A tiny mathematical lisp, made for fun. Inspired by [lisp-rs](https://vishpat.github.io/lisp-rs/)

## Installation
**You need a working rust toolchain to install this.**
Clone this repository, and run `cargo build` and run the created executable or `cargo run`.

## Usage

`<microlisp executable> <filename>`

## Example Program
```
(
  (define factorial (lambda (n) (if (< n 1) 1 (* n (factorial (- n 1))))))
  (factorial 5)
)
```
