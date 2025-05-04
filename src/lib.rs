use hex;
use pyo3::prelude::*;
use sha2::{Digest, Sha256};

/// Hash SHA-256 de un entero sin liberar el GIL
#[pyfunction]
fn hash_id_simple(x: u32) -> String {
    let mut hasher = Sha256::new();
    hasher.update(x.to_be_bytes());
    hex::encode(hasher.finalize())
}

/// Hash SHA-256 de un entero, liberando el GIL durante el cálculo
#[pyfunction]
fn hash_id(py: Python<'_>, x: u32) -> PyResult<String> {
    // convertimos a bytes antes de soltar el GIL
    let bytes = x.to_be_bytes();
    py.allow_threads(move || {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        Ok(hex::encode(hasher.finalize()))
    })
}

/// Hash SHA-256 de una cadena UTF-8, liberando el GIL
#[pyfunction]
fn hash_string(py: Python<'_>, s: &str) -> PyResult<String> {
    let data = s.as_bytes().to_vec();
    py.allow_threads(move || {
        let mut hasher = Sha256::new();
        hasher.update(&data);
        Ok(hex::encode(hasher.finalize()))
    })
}

/// Hash SHA-256 de cualquier buffer de bytes (`bytes` de Python), liberando el GIL
#[pyfunction]
fn hash_bytes(py: Python<'_>, data: &[u8]) -> PyResult<String> {
    // data ya es &[u8], podemos soltar el GIL directamente
    let data_vec = data.to_vec(); // Clone data for thread safety
    py.allow_threads(move || {
        let mut hasher = Sha256::new();
        hasher.update(&data_vec);
        Ok(hex::encode(hasher.finalize()))
    })
}

/// Inicialización del módulo Python
#[pymodule]
fn devlacruz_hashlib(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hash_id_simple, m)?)?;
    m.add_function(wrap_pyfunction!(hash_id, m)?)?;
    m.add_function(wrap_pyfunction!(hash_string, m)?)?;
    m.add_function(wrap_pyfunction!(hash_bytes, m)?)?;
    Ok(())
}
