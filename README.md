# rugrep
A grep-like program for windows written in rust 

## Features

| Feature | Status |
|---------|--------|
| Basic text search | ✓ |
| Single file search | ✓ |
| Directory file search | ✓ |
| Line numbers | ✓ |
| Regular expressions | ✗ |
| Recursive directory search | ✗ |
| Case-insensitive search | ✗ |
| Colored output | ✗ |
| Multiple file search | ✗ |
| Inverted match | ✗ |
| Count matches | ✗ |
| Context lines (before/after) | ✗ |
| Binary file detection | ✗ |
## Usage

```bash
# Basic search
rugrep "pattern" file.txt
```

## Options
| Option | Description |
|---------|--------|
| -n | Shows file and line of pattern |
| -nf | Disables formating, useful for older windows consoles |

Currently no other options are implemented

## License

MIT
