pub mod image;
pub mod link;

pub use link::{Link, LinkProps};

pub use image::{Image, ImageProps};
// pub use yew::prelude;
pub use yew::Renderer;
pub use yew_router::prelude as router;
// pub yew::functional::function_component as function;
pub use input_yew::CustomInput as Input;
pub use yew_i18n::{use_translation, I18nProvider, YewI18n};
pub use yew_navbar::{Menu, Navbar, NavbarProps};
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
    pub use yew::macros::{classes, html, html_nested};
    #[cfg(feature = "csr")]
    pub use yew::renderer::{set_custom_panic_hook, Renderer};
    pub use yew::suspense::Suspense;
    pub use yew::virtual_dom::AttrValue;
}

pub use self::prelude::*;
