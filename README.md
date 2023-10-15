# My Personal Page

Built with Rust, Yew and Tailwind CSS.


# [VISIT HERE!](https://yilunallenchen.github.io)


# Developer setup
1.  Install Rust
    ```bash
    curl https://sh.rustup.rs -sSf | sh
    ```
2. Install trunk & add wasm target
    ```bash
    cargo install trunk wasm-bindgen-cli
    rustup target add wasm32-unknown-unknown
    ```
3. Run under root
    ```bash
    trunk serve
    ```

4. If run into issues, try unintall yew and recomile everything
    ```bash
    cargo remove yew
    cargo clean
    cargo build
    ```
