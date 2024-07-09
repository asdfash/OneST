# OneST

Collection of services for OneST
click on the specific folder for the service

## Building from source

### 1a. Installation (Windows)

- Download Git - <https://git-scm.com/downloads>
- Download Rust - <https://www.rust-lang.org/learn/get-started>
- Download Windows Build Tools - <https://visualstudio.microsoft.com/downloads/>  
  - Select C++ Tools in the window  
- Download CMake - <https://cmake.org/download/>  
- Clone the repository

```bash
git clone https://github.com/asdfash/OneST
```

### 1b. Installation (Mac/Linux)

Install Rust via rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository

```bash
git clone https://github.com/asdfash/OneST
```

### 2. Building binaries

Build all services
navigate to service folder

```bash
cargo build --release
```
