# injektor

A simple CLI to inject environment variables from a `.env` file when running any command — no need for language-specific libs (like `python-dotenv`, Node's `dotenv`, etc).

## How it works

`injektor` looks for a `.env` file starting from the current directory, walking up one level at a time until it finds one (or reaches the filesystem root). Once found, it injects the variables into the environment of the given command and executes it.

## Usage

```bash
injektor <command> [args...]
```

### Examples

```bash
injektor python script.py
injektor npm run build
injektor cargo run
```

## `.env` format

```env
# comments are ignored
KEY=value
KEY_WITH_QUOTES="value with spaces"
KEY_SINGLE='another value'
```

- Empty lines and comments (`#`) are ignored
- Single or double quotes wrapping a value are automatically stripped

## Build

```bash
cargo build --release
```

The binary will be at `target/release/injektor` (or `injektor.exe` on Windows).

## Install via cargo

```bash
cargo install injektor
```

or

```bash
cargo install --git https://github.com/neozmmv/injektor
```

## Update already installed binary
```bash
cargo install injektor --force
```

## License

MIT