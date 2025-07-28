# 🦀 CargoPal

The Vite of Rust. Scaffold and run Rust apps with ease.

`cargopal` is a developer tool for Rust inspired by Vite. It aims to provide a fast and lean development experience for Rust web projects. With `cargopal`, you can quickly scaffold new projects from templates and run a development server with hot-reloading capabilities.

<div align="center">
    <img src=".github/assets/cargopal-demo.gif" alt="CargoPal Demo GIF" width="500"/>
</div>

> ⚡️ Created during the Boot.dev Hackathon, July 2025

## ✨ Features

- 🛠️ `cargopal new <template> <name>` — Generate a new project in seconds
- 🌐 Web server templates using `axum`
- 💻 CLI app templates using `clap`
- 🔁 `cargopal dev` — Hot-reloading dev server (for web apps)
- 📦 Easy install via `cargo install`

---

## 📦 Installation

### Prerequisites

Before you begin, ensure you have the Rust toolchain installed. You can install it using `rustup`.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This will also install `cargo`, Rust's package manager, which is required to install `cargopal`.

### Installing CargoPal

```sh
# Install directly from GitHub
cargo install --git https://github.com/kei-the-gae/cargopal
```

> ⏱️ Takes less than 5 minutes to install and scaffold your first app!

---

## 🚀 Quick Start

```sh
# Create a new web project
cargopal new web my-web-app
cd my-web-app

# Start development server with hot reload
cargopal dev
```

Or:

```sh
# Create a new CLI project
cargopal new cli my-cli-tool
cd my-cli-tool

# Run the tool with Cargo as normal
cargo run
```

---

## ⚡️ Usage

### Scaffolding a New Project

To create a new project from a template, use the `new` command:

```sh
cargopal new <template-name> <project-name>
```

For example, to create a new project using an Axum template:

```sh
cargopal new web my-awesome-app
```

### Running the Dev Server

Navigate into your newly created project directory and run the `dev` command:

```sh
cd my-awesome-app
cargopal dev
```

This will start a development server that watches for file changes in your project. When a change is detected, `cargopal` will automatically recompile and restart your application, giving you a smooth, hot-reloading experience.

---

## 📁 Available Templates

| Template | Description                 | Stack                                            |
| -------- | --------------------------- | ------------------------------------------------ |
| `web`    | REST-ready async web server | [`axum`](https://crates.io/crates/axum), `tokio` |
| `cli`    | Command-line app scaffold   | [`clap`](https://crates.io/crates/clap)          |

More templates coming soon!

---

## 🚀 Stretch Goals

Here are some ideas for the future of `cargopal`:

- **User-Defined Templates**: Allow users to create and use their own project templates from a local directory or a Git repository.
- **Automatic Browser Reloading**: Integrate with the browser via WebSockets to trigger a full page reload or HMR (Hot Module Replacement) when the backend restarts.
- **Clear stdout on Reload**: Add an option to clear the terminal screen before each rebuild to keep the development output clean and readable.
- **Configuration File**: Introduce a `cargopal.toml` file for project-specific configurations, such as custom build commands or watch paths.
- **More Official Templates**: Create a wider variety of official templates for popular frameworks like Actix Web, Rocket, and full-stack setups.

---

## 🛠️ Powered By

- [`clap`](https://crates.io/crates/clap)
- [`handlebars`](https://crates.io/crates/handlebars)
- [`axum`](https://crates.io/crates/axum)
- ❤️ Rustacean energy

---

## 🙌 Contributing

Contributions are welcome! If you have ideas for new features, templates, or improvements, please open an issue to discuss it first. Pull requests are also appreciated.
