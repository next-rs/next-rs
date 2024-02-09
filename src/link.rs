use crate::prelude::*;
use crate::router::*;
use serde_json::Value;
use web_sys::window;
use web_sys::{ScrollBehavior, ScrollToOptions};

/// Properties for the Link component.
#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    /// The target URL for the link.
    #[prop_or_default]
    pub to: &'static str,

    /// The CSS class for styling the link.
    #[prop_or_default]
    pub class: &'static str,

    /// The target attribute for the link.
    #[prop_or("_blank")]
    pub target: &'static str,

    /// The "rel" attribute for the link.
    #[prop_or("noreferrer")]
    pub rel: &'static str,

    /// Route query data
    #[prop_or_default]
    pub query: Value,

    /// Route state data
    #[prop_or_default]
    pub state: &'static str,

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
/// use next_rs::prelude::*;
/// use next_rs::Link;
///
/// #[func]
/// pub fn MyComponent() -> Html {
///
///     rsx! {
///         <Link
///             scroll_offset=300.0
///             scroll_behavior="smooth"
///             to={"#about"}
///             scroll=true
///         >{ "Go Home" }</Link>
///     }
/// }
/// ```
#[func]
pub fn Link(props: &LinkProps) -> Html {
    let props = props.clone();
    let to = props.to;
    #[allow(unused_variables)]
    let state = props.state;
    #[allow(unused_variables)]
    let query = props.query;
    let router = use_router();

    let (target, href) = if props.to.starts_with("/#") {
        // local anchor
        ("_self", &props.to[1..])
    } else if props.to.starts_with('#') {
        // also local anchor
        ("_self", props.to)
    } else {
        // external
        (props.target, props.to)
    };
    let onclick = Callback::from(move |event: MouseEvent| {
        let query = query.clone();
        // adjusted from https://docs.rs/yew-router/latest/src/yew_router/components/link.rs.html#69-86
        match (props.state, query) {
            ("", Value::Null) => {
                // Don't push the url twice onto the stack
                if target != "_blank" {
                    router.push(to);
                }
            }
            (state, Value::Null) => {
                event.prevent_default();
                router.push_with_state(to, state);
            }
            ("", query) => {
                event.prevent_default();
                router
                    .push_with_query(to, &query)
                    .expect("failed push history with query");
            }
            (state, query) => {
                event.prevent_default();
                router
                    .push_with_query_and_state(to, &query, state)
                    .expect("failed push history with query and state");
            }
        }
        if props.scroll {
            let scroll_behavior = match props.scroll_behavior {
                "auto" => ScrollBehavior::Auto,
                "instant" => ScrollBehavior::Instant,
                "smooth" => ScrollBehavior::Smooth,
                _ => ScrollBehavior::Auto,
            };

            if props.to.starts_with('#') || props.to.starts_with("/#") {
                // Prevent default navigation behavior("instant")
                event.prevent_default();
                // Local anchor link
                if let Some(element) = window()
                    .and_then(|win| win.document())
                    .and_then(|doc| doc.get_element_by_id(&href[1..]))
                {
                    let offset_top = element.get_bounding_client_rect().y();
                    window()
                        .map(|win| {
                            win.scroll_to_with_scroll_to_options(
                                ScrollToOptions::new()
                                    .top(offset_top)
                                    .behavior(scroll_behavior),
                            )
                        })
                        .expect("Failed to scroll to local anchor link");
                } else {
                    // Fallback to prop offset if element is not found
                    window()
                        .map(|win| {
                            win.scroll_to_with_scroll_to_options(
                                ScrollToOptions::new()
                                    .top(props.scroll_offset)
                                    .behavior(scroll_behavior),
                            )
                        })
                        .expect("Failed to scroll to fallback offset");
                }
            } else {
                // External link
                window()
                    .map(|win| {
                        win.scroll_to_with_scroll_to_options(
                            ScrollToOptions::new()
                                .top(props.scroll_offset)
                                .behavior(scroll_behavior),
                        )
                    })
                    .expect("Failed to scroll to external link");
            }
        }
    });
    let aria_label = "Link to ".to_string() + href;

    let tabindex = if props.scroll { "0" } else { "-1" };

    rsx! {
        <a
            href={href}
            target={target}
            rel={props.rel}
            class={props.class}
            onclick={onclick}
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
