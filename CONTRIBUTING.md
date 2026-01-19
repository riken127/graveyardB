# Contributing to graveyar_db

Thank you for your interest in contributing to `graveyar_db`!

## Getting Started

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/riken127/graveyar_db.git
    cd graveyar_db
    ```

2.  **Install Dependencies**:
    - Rust (latest stable)
    - `protoc` (Protocol Buffers compiler)
    - `make` (optional, for convenience commands)

3.  **Run Tests**:
    ```bash
    cargo test
    # or
    make test
    ```

## Development Guidelines

- **Code Style**: We use `rustfmt` for formatting. Please run `make fmt` before committing.
- **Linting**: We use `clippy` to catch common mistakes. Ensure `make lint` passes.
- **Commits**: Write clear, concise commit messages.

## Submitting a Pull Request

1.  Fork the repository and branch off `main`.
2.  Make your changes.
3.  Add tests for any new functionality.
4.  Run `make all` to ensure everything is correct.
5.  Push your branch and open a PR.

## Concurrent Development Note

Please coordinate with the team if you are working on core shared components to avoid conflicts.
