# Myrias

Arbitrary code execution server using Docker //in Rust//.

## Installation
[Archives of precompiled binares for Myrias will be available for Windows, macOS and Linux.](https://github.com/iCrawl/myrias/releases)

Linux binaries are static executables. Windows binaries are available built with Microsoft Visual C++ (MSVC).

## Running

TBD

## Motivation
- [Myriad](https://github.com/1Computer1/myriad): I just really can't read/write Haskell.

## Endpoints

### **GET** `/languages`
List of enabled languages.
Example response:

```json
["rust", "typescript"]
```

### **POST** `/create_container`
Creates a language container (if not already present).
JSON payload with `language` key.
The `language` is as in the name of a subfolder in the `languages` directory.
Example payload:

```json
{ "language": "rust" }
```

### **POST** `/eval`
Evaluate code.
JSON payload with `language` and `code` keys.
The `language` is as in the name of a subfolder in the `languages` directory.
Example payload:

```json
{ "language": "rust", "code": "fn main() { println!(\"{}\", 1 + 1); }" }
```

Example response:
```json
{ "result": "2\n" }
```

Errors with 404 if `language` is not found, `504` if evaluation timed out, or `500` if evaluation failed for other reasons.

### **GET** `/containers`
List of containers being handled by Myrias.

### **POST** `/cleanup`
Kill all containers, giving back the names of the containers killed.
