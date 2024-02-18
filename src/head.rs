use crate::prelude::*;
use std::collections::{HashMap, HashSet};
use web_sys::window;
use yew::virtual_dom::VTag;

// Define METATYPES as a static array
static METATYPES: [&'static str; 5] = ["name", "httpEquiv", "charSet", "itemProp", "property"];

// MetaCategories type to store meta category information
type MetaCategories = HashMap<&'static str, HashSet<String>>;

/// Generates the default `<head>` element with a charset meta tag.
///
/// # Example
/// ```rust
/// use next_rs::prelude::*;
/// use next_rs::head::default_head;
///
/// #[func]
/// pub fn MyComponent() -> Html {
///
///     rsx! {
///         <>{default_head()}</>
///     }
/// }
/// ```
pub fn default_head() -> Html {
    rsx! { <meta charset="utf-8" /> }
}

/// Reduces a vector of HTML components by flattening and filtering out duplicates.
///
/// # Example
/// ```rust
/// use next_rs::head::{map_components, default_head};
///
/// let components = vec![default_head()];
/// let new_components = map_components(components);
/// ```
pub fn map_components(components: Vec<Html>) -> Vec<Html> {
    let flattened: Vec<Html> = components
        .into_iter()
        .flat_map(|c| match c {
            Html::VTag(tag) => tag.children().into_iter().cloned().collect::<Vec<_>>(),
            _ => vec![],
        })
        .collect();

    let filtered: Vec<Html> = flattened.into_iter().filter(unique).collect();

    let mut head = vec![default_head()];

    for child in filtered.clone() {
        match child {
            Html::VTag(_tag) => {
                // TODO
            }
            Html::VText(text) => {
                // Hack: Handle VText case, like title tag
                let text_str = text.text;
                let mut tag = VTag::new("title");
                tag.add_child(text_str.into());
                head.push(tag.into());
            }
            Html::VComp(_component) => {
                // TODO
            }
            _ => {}
        }
    }

    // add next-rs trademark, rn
    let final_result: Vec<Html> = head
        .into_iter()
        .map(|c| match c {
            Html::VTag(mut tag) => {
                let class_name = format!(
                    "{} {}",
                    "next-rs-tag",
                    tag.attributes
                        .iter()
                        .find(|(key, _)| *key == "class")
                        .map(|(_, value)| value)
                        .unwrap_or_default()
                );
                tag.add_attribute("class", class_name);
                Html::VTag(tag)
            }
            _ => c,
        })
        .collect();

    final_result
}

/// Returns a function for filtering head child elements which shouldn't be duplicated, like <title/>.
pub fn unique(head: &Html) -> bool {
    match head {
        Html::VTag(tag) => match tag.tag() {
            "title" | "base" => tag.key.is_some(),
            "meta" => {
                for metatype in METATYPES.iter() {
                    if !tag
                        .attributes
                        .iter()
                        .find(|(key, _)| *key == *metatype)
                        .map(|(_, value)| value)
                        .unwrap_or_default()
                        .is_empty()
                    {
                        match *metatype {
                            "charSet" => {
                                if !tag
                                    .attributes
                                    .iter()
                                    .find(|(key, _)| *key == "charSet")
                                    .map(|(_, value)| value)
                                    .unwrap_or_default()
                                    .is_empty()
                                {
                                    return false;
                                }
                            }
                            _ => {
                                let category = tag
                                    .attributes
                                    .iter()
                                    .find(|(key, _)| *key == *metatype)
                                    .map(|(_, value)| value)
                                    .unwrap_or_default();
                                let mut meta_categories = MetaCategories::new();
                                let categories = meta_categories
                                    .entry(metatype)
                                    .or_insert_with(|| HashSet::new());
                                if categories.contains(&category.to_string()) {
                                    return false;
                                }

                                categories.insert(category.to_string());
                            }
                        }
                    }
                }
                true
            }
            _ => true,
        },
        _ => true,
    }
}

// Define the HeadProps struct
#[derive(Properties, Clone, PartialEq)]
pub struct HeadProps {
    pub children: Html,
}

/// A component representing the `<head>` element.
///
/// # Example
/// ```rust
/// use next_rs::head::Head;
/// use next_rs::prelude::*;
///
/// #[func]
/// pub fn MyComponent() -> Html {
///
///     rsx! {
///         <Head>
///             <title>{"Next RS Title"}</title>
///         </Head>
///     }
/// }
/// ```
#[func]
pub fn Head(props: &HeadProps) -> Html {
    let state: Vec<Html> = map_components(vec![props.children.clone()]);

    let document = window().and_then(|win| win.document()).unwrap();
    let head = document.head().expect("Failed to get head element");

    create_portal(rsx! {<>{ for state.into_iter() }</> }, head.clone().into())
}
