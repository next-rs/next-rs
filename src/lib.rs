#![doc(
    html_logo_url = "https://github.com/next-rs/next-rs/assets/62179149/60e6d58f-6749-4308-86f0-fc0ff28c95f6",
    html_favicon_url = "https://github.com/next-rs/next-rs/assets/62179149/8ac122c9-e55c-4204-9b53-6981f17cefcc"
)]

//! # Next RS - Documentation
//!
//! Welcome to the official documentation for Next RS, a framework written in Rust that simplifies the process of building
//! user interfaces. This framework provides a collection of features, each designed to enhance different aspects
//! of UI development. Below are the features included in the Next RS ecosystem:
//!
//! ## Features
//!
//! | Feature        | Crate Dependency         | GitHub Repository                                         | Description                                                |
//! |----------------|--------------------------|------------------------------------------------------------|------------------------------------------------------------|
//! | `navbar`       |   `yew-navbar`           | [![GitHub](https://img.shields.io/github/stars/next-rs/yew-navbar)](https://github.com/next-rs/yew-navbar)           | Create a responsive top-level navigation bar.             |
//! | `sidebar`      |   `yew-sidebar`          | [![GitHub](https://img.shields.io/github/stars/next-rs/yew-sidebar)](https://github.com/next-rs/yew-sidebar)        | Build a customizable sidebar navigation component.     |
//! | `accordion`    | `yew-accordion`          | [![GitHub](https://img.shields.io/github/stars/next-rs/yew-accordion)](https://github.com/next-rs/yew-accordion)     | Build interactive accordion-style components.              |
//! | `alert`        | `yew-alert`              | [![GitHub](https://img.shields.io/github/stars/next-rs/yew-alert)](https://github.com/next-rs/yew-alert)           | Display alerts with customizable styling and properties.   |
//! | `i18n`         | `yew-i18n`               | [![GitHub](https://img.shields.io/github/stars/next-rs/yew-i18n)](https://github.com/next-rs/yew-i18n)             | Implement internationalization for multi-language support.  |
//! | `input`        | `input_yew`              | [![GitHub](https://img.shields.io/github/stars/next-rs/input-yew)](https://github.com/next-rs/input-yew)        | Utilize custom input components for enhanced form handling. |
//! | `css`          | `stylist`                | [![GitHub](https://img.shields.io/github/stars/futursolo/stylist-rs)](https://github.com/futursolo/stylist-rs)           | Apply styling to your components using the Stylist crate integration.|
//!
//! To use a specific feature, enable it using the `features` configuration in your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! next-rs = { version = "0.0.9", features = ["navbar", "sidebar", "css"] }
//! ```
//!
//! ## Usage
//!
//! To integrate Next RS into your application, follow these steps:
//!
//! 1. Add Next RS and the desired feature crates to your `Cargo.toml`.
//! 2. Import the necessary components from the Next RS crate into your Rust code.
//! 3. Use the imported components within your Next RS components.
//!
//! For example, to use the `Navbar` component, add the following dependency:
//!
//! ```toml
//! [dependencies]
//! next-rs = { version = "0.0.9", features = ["navbar"] }
//! ```
//!
//! Then, in your Rust code:
//!
//! ```rust,no_run
//! use next_rs::prelude::*;
//! use next_rs::Navbar;
//!
//! #[func]
//! pub fn MyNavbar() -> Html {
//!     let menus = vec![/* define your menu items here */];
//!
//!     rsx! {
//!         <Navbar menus={menus} />
//!         // Your component logic here...
//!     }
//! }
//! ```
//!
//! For more detailed information and examples, check the [examples] provided in the library.
//!
//! [examples]: https://github.com/next-rs/next-rs/tree/main/examples
//!
//! ## Contribution
//!
//! If you encounter any issues or have suggestions for improvements, feel free to contribute to the
//! [GitHub repository](https://github.com/next-rs/next-rs). We appreciate your feedback and involvement
//! in making Next RS better!
//!
//! ## Acknowledgments
//!
//! Special thanks to the Yew community and contributors for such an amazing framework.
//!

pub mod image;
pub mod link;

pub use image::{Image, ImageProps};
#[cfg(feature = "json")]
pub use serde_json::json;
#[cfg(feature = "input")]
pub use input_yew::CustomInput as Input;
pub use link::{Link, LinkProps};
#[cfg(feature = "css")]
pub use stylist::yew::styled_component;
pub use web_sys::console::log_1 as log;
pub use yew::Renderer;
#[cfg(feature = "accordion")]
pub use yew_accordion::{Accordion, AccordionButton, AccordionItem};
#[cfg(feature = "alert")]
pub use yew_alert::{Alert, AlertProps};
#[cfg(feature = "i18n")]
pub use yew_i18n::{use_translation, I18nProvider, YewI18n};
#[cfg(feature = "navbar")]
pub use yew_navbar::{Menu, Navbar, NavbarProps};
pub use yew_router::prelude as router;
#[cfg(feature = "sidebar")]
pub use yew_sidebar::{MenuItem, Sidebar, SidebarProps};

pub mod prelude {
    //! The Next RS Prelude

    #[cfg(feature = "csr")]
    pub use yew::app_handle::AppHandle;
    pub use yew::callback::Callback;
    pub use yew::context::{ContextHandle, ContextProvider};
    pub use yew::events::*;
    pub use yew::functional::function_component as func;
    pub use yew::functional::*;
    pub use yew::html::{
        create_portal, BaseComponent, Children, ChildrenWithProps, Classes, Component, Context,
        Html, HtmlResult, NodeRef, Properties, ToHtml,
    };
    pub use yew::macros::{classes, html as rsx, html_nested};
    #[cfg(feature = "csr")]
    pub use yew::renderer::{set_custom_panic_hook, Renderer};
    pub use yew::suspense::Suspense;
    pub use yew::virtual_dom::AttrValue;
}

pub use self::prelude::*;
