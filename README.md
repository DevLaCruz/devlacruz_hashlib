# devlacruz_hashlib

High-performance library for SHA-256 hash functions implemented in Rust with PyO3.

[![PyPI version](https://img.shields.io/pypi/v/devlacruz_hashlib.svg)](https://pypi.org/project/devlacruz_hashlib/)
[![Python versions](https://img.shields.io/pypi/pyversions/devlacruz_hashlib.svg)](https://pypi.org/project/devlacruz_hashlib/)

## Features

- Ultra-fast computation of SHA-256 hashes
- GIL release during intensive operations
- Compatible with Python 3.7+
- Support for hashing integers, strings, and bytes
- Performance superior to pure Python implementations

## Installation

```bash
pip install devlacruz_hashlib
```text

## Basic Usage

```python
import devlacruz_hashlib

# Hash of an integer (basic version)
hash1 = devlacruz_hashlib.hash_id_simple(12345)
print(f"Simple hash of integer: {hash1}")

# Hash of an integer (GIL-releasing version)
hash2 = devlacruz_hashlib.hash_id(12345)
print(f"Hash of integer: {hash2}")

# Hash of a string
hash3 = devlacruz_hashlib.hash_string("Hello world")
print(f"Hash of string: {hash3}")

# Hash of bytes
hash4 = devlacruz_hashlib.hash_bytes(b"binary data")
print(f"Hash of bytes: {hash4}")
```text

## Performance

This library provides a SHA-256 hash implementation significantly faster than pure Python solutions. Particularly useful for:

- Processing large volumes of data
- Computing hashes in intensive loops
- Applications where performance is critical

### Performance Comparison

```python
import time
import hashlib
import devlacruz_hashlib

# Test data
data = b"x" * 1000000  # ~1MB of data

# Time with Python's hashlib
start = time.time()
for _ in range(100):
    hashlib.sha256(data).hexdigest()
py_time = time.time() - start
print(f"Python hashlib: {py_time:.3f} seconds")

# Time with devlacruz_hashlib
start = time.time()
for _ in range(100):
    devlacruz_hashlib.hash_bytes(data)
rust_time = time.time() - start
print(f"devlacruz_hashlib: {rust_time:.3f} seconds")

print(f"Performance improvement: {py_time/rust_time:.1f}x faster")
```text

## Full API

### `hash_id_simple(x: int) → str`

Calculates the SHA-256 hash of an integer. This version does not release Python's GIL.

### `hash_id(x: int) → str`

Calculates the SHA-256 hash of an integer while releasing the GIL. Recommended for parallel processing.

### `hash_string(s: str) → str`

Calculates the SHA-256 hash of a text string. Releases the GIL during processing.

### `hash_bytes(data: bytes) → str`

Calculates the SHA-256 hash of a bytes object. Releases the GIL during processing.

## Technical Considerations

- The module releases the GIL (Global Interpreter Lock) during intensive hash operations, enabling true parallelism in multi-threaded applications.
- The Rust implementation provides memory and type safety without performance overhead.
- Compatible with all major platforms: Linux, Windows, and macOS.

## When to Use devlacruz_hashlib vs Standard hashlib

Use devlacruz_hashlib when:
- Maximum performance is needed for SHA-256 hash operations
- Working with large volumes of data
- Performing hash operations in critical loops
- Utilizing multi-threaded processing where releasing the GIL is advantageous

Use standard `hashlib` when:
- Other algorithms besides SHA-256 are needed
- Simplicity and maintainability are more important than performance
- Avoiding additional dependencies is preferred

## Contributions

Contributions are welcome. To contribute:

1. Fork the repository
2. Create a branch for your feature (`git checkout -b new-feature`)
3. Make your changes and commit (`git commit -am 'Add new feature'`)
4. Push your changes (`git push origin new-feature`)
5. Create a Pull Request

## License

MIT License - See the LICENSE file for more details.

## Author

Alejandro De La Cruz (devlacruz@axtosys.com)
