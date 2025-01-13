# GLUE 🌟

GLUE is a command-line utility that combines multiple files into a single, well-structured output file. It's specifically designed to prepare project contexts for Large Language Models (LLMs) by concatenating source files while preserving their structure and metadata.

## Features

- **Smart File Selection**: Uses glob patterns for inclusion and exclusion
- **Gitignore Integration**: Respects `.gitignore` rules by default
- **Binary File Handling**: Automatically detects and skips binary files
- **UTF-8 Validation**: Ensures all content is valid UTF-8
- **Flexible Output**: Write to file or stdout
- **Structured Format**: Clear file separators and metadata preservation
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Installation

### From Source

1. Ensure you have Rust installed (1.70.0 or later)
2. Clone the repository:
```bash
git clone https://github.com/tristanpoland/GLUE
cd GLUE
```
3. Build and install:
```bash
cargo install --path .
```
-- OR --
```bash
grip install glue
```

## Usage

Basic usage:
```bash
glue "src/**/*.rs" -o output.glue
```

### Command Line Options

```bash
USAGE:
    glue [OPTIONS] <PATTERNS>...

ARGS:
    <PATTERNS>...    Input patterns (supports glob syntax)

OPTIONS:
    -e, --exclude <PATTERNS>    Patterns to exclude (in addition to .gitignore)
    -o, --output <FILE>        Output file [default: output.glue]
        --no-ignore           Include files that would be ignored by .gitignore
        --include-binary      Include binary files (they will be skipped by default)
    -h, --help                Print help information
    -V, --version             Print version information
```

### Examples

Include all Rust files, exclude test files:
```bash
glue "**/*.rs" --exclude "**/tests/*" -o project.glue
```

Process multiple file types:
```bash
glue "**/*.rs" "**/*.toml" "*.md" -o full-project.glue
```

Include ignored files:
```bash
glue "src/**/*.rs" --no-ignore -o all-source.glue
```

Output to stdout:
```bash
glue "src/**/*.rs" -o -
```

## File Format

GLUE files use a simple, readable format:

```
$$--GLUE--$$
# This is a GLUE file; an amalgamation of files across one or more paths
$$--GLUE--$$
./path/to/file1.rs
$$--GLUE--$$
[content of file1.rs]
$$--GLUE--$$
./path/to/file2.rs
$$--GLUE--$$
[content of file2.rs]
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Known Limitations

- Binaries are ignored entirely
- No file metadata preservation (timestamps, permissions)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Todo

- [ ] Include file metadata options
- [ ] Add progress indicators for large file sets
- [ ] Add maximum file size limit options
- [ ] Improve error reporting structure
