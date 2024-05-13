use pyo3::pyfunction;
use pyo3::pymodule;
use pyo3::types::PyModule;
use pyo3::wrap_pyfunction;
use pyo3::PyResult;
use pyo3::Python;

#[pymodule]
fn rust_methods(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sieve_rust, m)?)?;
    Ok(())
}

#[pyfunction]
#[pyo3(text_signature = "(up_to,/)")]
fn sieve_rust(up_to: i64) -> PyResult<Vec<i64>> {
    Ok(sieve(up_to))
}

pub fn sieve(up_to: i64) -> Vec<i64> {
    let mut is_prime = vec![true; (up_to + 1) as usize];
    let mut primes = Vec::new();

    for num in 2..=up_to {
        if is_prime[num as usize] {
            primes.push(num);
            let mut multiple = num * 2;
            while multiple <= up_to {
                is_prime[multiple as usize] = false;
                multiple += num;
            }
        }
    }

    primes
}
