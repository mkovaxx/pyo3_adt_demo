# pyo3_adt_demo

Demo of full ADT support for PyO3

## How to Build

1. `poetry lock`
2. `poetry install`
3. `poetry run maturin develop`

## How to Run

`poetry run python python/main.py`

## How to Debug

```
cargo +nightly rustc --profile=check -- -Zunpretty=expanded > expanded.rs; rustfmt expanded.rs
```
