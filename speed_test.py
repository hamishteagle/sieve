from rust_methods import sieve_rust
from src.sieve import sieve
import time


if __name__ == "__main__":
    start = time.time()
    primes_rust = sieve_rust(100000)
    end = time.time()
    print(f"Time taken rust: {end-start}")

    start = time.time()
    primes_python = sieve(100000)
    end = time.time()
    print(f"Time taken python: {end-start}")

    assert primes_rust == primes_python
