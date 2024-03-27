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
    Ok(sieve(&up_to))
}

pub fn sieve(&up_to: &i64) -> Vec<i64> {
    //Starting lowest prime = 2
    let mut prime_index: usize = 0;
    // Put all values up to the target value in the initial vector
    let mut primes = Vec::from_iter(2..up_to);

    while prime_index < primes.len() {
        let mut composite = primes[prime_index];
        while composite < up_to {
            composite += primes[prime_index];
            if primes.contains(&composite) {
                let index = primes.iter().position(|x| *x == composite).unwrap();
                primes.remove(index);
            }
        }
        prime_index += 1;
    }
    {
        primes
    }
}
