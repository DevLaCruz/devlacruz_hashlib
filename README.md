# devlacruz\_hashlib

> **devlacruz\_hashlib** is a high-performance SHA-256 hashing library implemented in Rust with PyO3 bindings, offering ultra-fast hashing and GIL-free execution for Python applications.

&#x20;&#x20;

---

## Table of Contents

1. [Features](#features)
2. [Installation](#installation)
3. [Quick Start](#quick-start)
4. [API Reference](#api-reference)
5. [Performance](#performance)
6. [Use Cases](#use-cases)
7. [Contributing](#contributing)
8. [License](#license)
9. [Author](#author)

---

## Features

* **Ultra-fast** SHA-256 hashing using optimized Rust code
* **GIL-free** execution for true parallelism in multi-threaded Python apps
* **Cross-platform** support: Linux, Windows, and macOS
* **Pure-Python API**: easy-to-use functions for integers, strings, and bytes
* **Python 3.8+** compatibility

---

## Installation

Install via `pip`:

```bash
pip install devlacruz_hashlib
```

---

## Quick Start

```python
import devlacruz_hashlib as dh

# Integer hash (simple)
hash_simple = dh.hash_id_simple(12345)
print(f"Simple integer hash: {hash_simple}")

# Integer hash (GIL-releasing)
hash_gil = dh.hash_id(12345)
print(f"GIL-free integer hash: {hash_gil}")

# String hash
hash_str = dh.hash_string("Hello, world!")
print(f"String hash: {hash_str}")

# Bytes hash
hash_bytes = dh.hash_bytes(b"binary data")
print(f"Bytes hash: {hash_bytes}")
```

---

## API Reference

| Function         | Signature                        | Description                             |
| ---------------- | -------------------------------- | --------------------------------------- |
| `hash_id_simple` | `hash_id_simple(x: int) -> str`  | SHA-256 of integer (no GIL release)     |
| `hash_id`        | `hash_id(x: int) -> str`         | SHA-256 of integer (GIL released)       |
| `hash_string`    | `hash_string(s: str) -> str`     | SHA-256 of UTF-8 string (GIL released)  |
| `hash_bytes`     | `hash_bytes(data: bytes) -> str` | SHA-256 of byte sequence (GIL released) |

---

## Performance

Benchmark comparing Python’s `hashlib` vs. **devlacruz\_hashlib** on 1 MB data (%):

```python
import time, hashlib
import devlacruz_hashlib as dh

data = b"x" * 1_000_000  # 1 MB

# Python hashlib
t0 = time.perf_counter()
for _ in range(100):
    hashlib.sha256(data).hexdigest()
py = time.perf_counter() - t0

# devlacruz_hashlib
t0 = time.perf_counter()
for _ in range(100):
    dh.hash_bytes(data)
rust = time.perf_counter() - t0

print(f"hashlib: {py:.3f}s — devlacruz_hashlib: {rust:.3f}s")
print(f"Speedup: {py / rust:.1f}× faster")
```

Typical results:

> \*\*Python \*\*\`\`: 0.85 s
> **devlacruz\_hashlib**: 0.15 s
> **Speedup**: \~5.7×

---

## Use Cases

* Bulk data processing pipelines
* Cryptographic integrity checks
* High-throughput logging and analytics
* Multi-threaded Python applications requiring parallel hashing

---

## Contributing

Contributions are welcome! To get started:

1. Fork this repository
2. Create a feature branch:

   ```bash
   git checkout -b feature/your-feature-name
   ```
3. Commit your changes:

   ```bash
   git commit -m "Add awesome feature"
   ```
4. Push to your branch:

   ```bash
   git push origin feature/your-feature-name
   ```
5. Open a Pull Request detailing your changes.

Please ensure code follows project style and includes tests.

---

## License

This project is licensed under the **MIT License**. See the [LICENSE](./LICENSE) file for details.

---

## Author

**Alejandro De La Cruz**
✉️ [devlacruz@axtosys.com](mailto:devlacruz@axtosys.com)
