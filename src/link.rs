use crate::prelude::*;
use crate::router::*;
use web_sys::window;
use web_sys::{ScrollBehavior, ScrollToOptions};

/// Properties for the Link component.
#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps<R: Routable + PartialEq> {
    /// The target URL for the link.
    pub href: R,

    /// The content to be displayed within the link.
    #[prop_or_default]
    pub children: Html,

    /// Enable scrolling behavior when clicking the link.
    #[prop_or_default]
    pub scroll: bool,

    /// Offset for the scrolling behavior, specifying how far from the top the scroll should stop.
    #[prop_or_default]
    pub scroll_offset: f64,

    /// Scroll behavior when clicking the link. Valid values: "auto", "instant", "smooth".
    #[prop_or("auto")]
    pub scroll_behavior: &'static str,

    /// Callback function for handling errors, typically invoked when navigating to the link fails.
    #[prop_or_default]
    pub on_error: Callback<String>,

    /// Indicates the current state of the link in a navigation menu. Valid values: "page", "step", "location", "date", "time", "true", "false".
    #[prop_or_default]
    pub aria_current: &'static str,

    /// Describes the link using the ID of the element that provides a description.
    #[prop_or_default]
    pub aria_describedby: &'static str,

    /// Indicates whether the content associated with the link is currently expanded or collapsed. Valid values: "true", "false".
    #[prop_or_default]
    pub aria_expanded: &'static str,

    /// Indicates whether the link is currently hidden from the user. Valid values: "true", "false".
    #[prop_or_default]
    pub aria_hidden: &'static str,

    /// Indicates whether the content associated with the link is live and dynamic. Valid values: "off", "assertive", "polite".
    #[prop_or_default]
    pub aria_live: &'static str,

    /// Indicates whether the link is currently pressed or selected. Valid values: "true", "false", "mixed", "undefined".
    #[prop_or_default]
    pub aria_pressed: &'static str,

    /// ID of the element that the link controls or owns.
    #[prop_or_default]
    pub aria_controls: &'static str,

    /// ID of the element that labels the link.
    #[prop_or_default]
    pub aria_labelledby: &'static str,
}

/// The Link component is used for creating accessible links with additional features.
///
/// # Arguments
/// * `props` - The properties of the component.
///
/// # Returns
/// (Html): An HTML representation of the link component.
///
/// # Examples
/// ```
/// // Example of using the Link component
/// use crate::router::Route; // Your `Route` enum
/// use next_rs::{Link, LinkProps};
/// use next_rs::prelude::*;
///
/// #[function_component(MyComponent)]
/// pub fn my_component() -> Html {
///
///     html! {
///         <Link<Route>
///             scroll_offset=300.0
///             scroll_behavior="smooth"
///             href={Route::LandingPage}
///             scroll=true
///             on_error={Callback::from(|err| {
///                 println!("Navigation error: {:?}", err);
///             })}
///         >{ "Go Home" }</Link<Route>>
///     }
/// }
/// ```
#[function_component(Link)]
pub fn link<R: PartialEq + Routable + 'static>(props: &LinkProps<R>) -> Html {
    let navigator = use_navigator().unwrap();

    let on_error_callback = props.on_error.clone();
    let props = props.clone();

    let onclick_callback = {
        let props = props.clone();
        Callback::from(move |_| {
            match navigator.push(&props.href) {
                () => {
                    on_error_callback.emit("error occurred!".to_string());
                }
            };

            if props.scroll {
                let scroll_behavior = match props.scroll_behavior {
                    "auto" => ScrollBehavior::Auto,
                    "instant" => ScrollBehavior::Instant,
                    "smooth" => ScrollBehavior::Smooth,
                    &_ => ScrollBehavior::Auto,
                };

                window()
                    .and_then(|win| {
                        Some(
                            win.scroll_to_with_scroll_to_options(
                                &ScrollToOptions::new()
                                    .top(props.scroll_offset)
                                    .behavior(scroll_behavior),
                            ),
                        )
                    })
                    .expect("Failed to scroll");
            }
        })
    };

    let aria_label = "Link to ".to_string() + &props.href.to_path();

    let tabindex = if props.scroll { "0" } else { "-1" };

    html! {
        <a
            href={props.href.clone().to_path()}
            onclick={onclick_callback}
            role="link"
            tabindex={tabindex}
            aria-label={aria_label.clone()}
            title={aria_label.clone()}
            aria-haspopup="true"
            aria-current={props.aria_current}
            aria-describedby={props.aria_describedby}
            aria-expanded={props.aria_expanded}
            aria-hidden={props.aria_hidden}
            aria-live={props.aria_live}
            aria-pressed={props.aria_pressed}
            aria-controls={props.aria_controls}
            aria-labelledby={props.aria_labelledby}
        >{ props.children.clone() }</a>
    }
}
