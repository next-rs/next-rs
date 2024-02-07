# 📚 Next RS Image Component Example

## 🛠️ Pre-requisites:

1. Install [`rustup`](https://www.rust-lang.org/tools/install):

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

1. Install [`railwind`](https://github.com/pintariching/railwind):

    ```bash
    cargo install --locked railwind
    ```

1. Install [`trunk`](https://trunkrs.dev/):

    ```bash
    cargo install --locked trunk
    ```

1. Add Wasm target:

    ```bash
    rustup target add wasm32-unknown-unknown
    ```

## 🚀 Building and Running

1. Fork/Clone the GitHub repository.

    ```bash
    git clone https://github.com/next-rs/next-rs
    ```

1. Navigate to the application directory.

    ```bash
    cd next-rs/examples/images
    ```

1. Run the client:

    ```sh
    trunk serve --port 3000
    ```

1. Uncomment this line:
    https://github.com/next-rs/next-rs/blob/b648172b5e296a4a4d204251080a741690c9c3fb/examples/images/index.html#L5

Navigate to http://localhost:3000 to explore the landing page.
