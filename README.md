# Myrias

Arbitrary code execution server using Docker //in Rust//.

## Setup
You have to [install](https://gvisor.dev/docs/user_guide/docker/) [gVisor](https://github.com/google/gvisor) as a runtime for docker to provide an additional isolation boundary between the containers and the host kernel.

```sh
(
    set -e 
    wget https://storage.googleapis.com/gvisor/releases/nightly/latest/runsc
    wget https://storage.googleapis.com/gvisor/releases/nightly/latest/runsc.sha512
    sha512sum -c runsc.sha512
    sudo mv runsc /usr/local/bin
    sudo chown root:root /usr/local/bin/runsc
    sudo chmod 0755 /usr/local/bin/runsc
)
```

`/etc/docker/daemon.json`:
```json
{
    "runtimes": {
        "runsc": {
            "path": "/usr/local/bin/runsc",
            "runtimeArgs": [
                "--network=none",
                "--overlay"
            ]
        },
        "runsc-kvm": {
            "path": "/usr/local/bin/runsc",
            "runtimeArgs": [
                "--platform=kvm",
                "--network=none",
                "--overlay"
            ]
        }
    }
}
```
You may have to create this file if it does not exist.

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
