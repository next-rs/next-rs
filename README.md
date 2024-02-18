# 📦 Next RS

[![Crates.io](https://img.shields.io/crates/v/next-rs)](https://crates.io/crates/next-rs)
[![Crates.io Downloads](https://img.shields.io/crates/d/next-rs)](https://crates.io/crates/next-rs)
![Crates.io License](https://img.shields.io/crates/l/next-rs)
![Rust](https://img.shields.io/badge/rust-stable-orange)

## 📜 Introduction

Next RS is a UI framework, written 100% in Rust btw, that simplifies building user interfaces. It provides a collection of optional features, each designed to enhance different aspects of UI development. Explore the documentation to learn about the features and how to integrate them into your Next RS projects.

## 🚀 Features

Next RS offers the following features:

| Feature  Flag     | Crate Dependency         | GitHub Repository     | Description |
|----------------|-------------------|----------|---------------|
| `navbar`       |   `yew-navbar`           | [![GitHub](https://img.shields.io/github/stars/next-rs/yew-navbar)](https://github.com/next-rs/yew-navbar)           | Create a responsive top-level navigation bar.             |
| `sidebar`      |   `yew-sidebar`          | [![GitHub](https://img.shields.io/github/stars/next-rs/yew-sidebar)](https://github.com/next-rs/yew-sidebar)        | Build a customizable sidebar navigation component.     |
| `accordion`    | `yew-accordion`          | [![GitHub](https://img.shields.io/github/stars/next-rs/yew-accordion)](https://github.com/next-rs/yew-accordion)     | Build interactive accordion-style components.              |
| `alert`        | `yew-alert`              | [![GitHub](https://img.shields.io/github/stars/next-rs/yew-alert)](https://github.com/next-rs/yew-alert)           | Display alerts with customizable styling and properties.   |
| `i18n`         | `yew-i18n`               | [![GitHub](https://img.shields.io/github/stars/next-rs/yew-i18n)](https://github.com/next-rs/yew-i18n)             | Implement internationalization for multi-language support.  |
| `input`        | `input_yew`              | [![GitHub](https://img.shields.io/github/stars/next-rs/input-yew)](https://github.com/next-rs/input-yew)        | Utilize custom input components for enhanced form handling. |
| `css`          | `stylist`                | [![GitHub](https://img.shields.io/github/stars/futursolo/stylist-rs)](https://github.com/futursolo/stylist-rs)           | Apply styling to your components using the Stylist crate integration.|

To use a specific feature, add the corresponding feature flag in your `Cargo.toml` file.

## ⚙️ Installation

Integrating Next RS into your project is straightforward. Add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
next-rs = "0.0.11"
```

Additionally, Next RS provides a flexible configuration through feature flags. Modify the `Cargo.toml` file to enable or disable specific features.

```toml
[dependencies]
next-rs = { version = "0.0.11", features = ["navbar", "sidebar", "accordion", "alert", "css"] }
```

## 📙 Examples

If you want to explore different Next RS components and features, you can check out the [examples folder](examples) for more information.

| Example | URL |
| --- | --- |
| Link And Head Components | [![Netlify Status](https://api.netlify.com/api/v1/badges/0f5cbba1-4179-45c7-91e2-5540f3539a12/deploy-status)](https://next-rs-link.netlify.app) |
| Image Component | [![Netlify Status](https://api.netlify.com/api/v1/badges/0f5cbba1-4179-45c7-91e2-5540f3539a12/deploy-status)](https://next-rs-image.netlify.app/) |


## 🤝 Contribution

We welcome contributions from the community to enhance Next RS. Feel free to open issues, submit pull requests, or provide feedback. Let's collaborate to make UI development in Rust even more efficient and enjoyable!

## 📜 License

Next RS is licensed under the `MIT` License, allowing you to use, modify, and distribute it freely. Refer to the [`LICENSE`](LICENSE) file for more details.
