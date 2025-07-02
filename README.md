# Directory Content Comparator

![Rust](https://img.shields.io/badge/Rust-1.60+-blue.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)

A high-performance Rust utility for comparing fixed-size (32-byte) files between directory structures.

## Features
- Recursively compares directory structures
- Detects:
-- New files in the second directory
-- Changed files (with identical names but different content)
- Handles files with identical names in different subdirectories
- Optimized for fixed-size 32-byte files
- Cross-platform path handling

## Requirements
- Rust 1.60+ (install via rustup)
- Cargo (comes with Rust)

## Installation
- Clone the repository:

```bash
git clone https://github.com/berezhko/compare_md5dir.git
cd compare_md5dir
```

- Build the release version:

```bash
cargo build --release
```

The binary will be located at ./target/release/compare_md5dir

## Usage
```bash
compare_md5dir <md5 directory1> <md5 directory2>
```

### Example:
```bash
compare_md5dir ./backup ./current
```

### Output Format:
- New: <path> - File exists in second directory but not in first
- Changed: <path> - File exists in both but content differs

## Performance Notes
- Optimized for comparing 32-byte files
- Uses memory-mapped files for efficient comparison
- Processes directories in parallel where possible

## Limitations
- Only works with files of exactly 32 bytes in size
- Follows symbolic links (can be disabled in code)
- Maximum path length depends on OS limitations

## License
MIT License - see LICENSE file

## Contributing
- Fork the project
- Create your feature branch (git checkout -b feature/AmazingFeature)
- Commit your changes (git commit -m 'Add some amazing feature')
- Push to the branch (git push origin feature/AmazingFeature)
- Open a Pull Request

## Support
For issues or questions, please open an issue
