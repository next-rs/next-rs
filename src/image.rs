use crate::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::RequestCache;
use yew::html;

/// Properties for the Image component.
#[derive(Properties, Clone, PartialEq)]
pub struct ImageProps {
    /// The source URL for the image.
    pub src: &'static str,

    /// The alternative text for the image.
    pub alt: &'static str,

    /// The width of the image.
    pub width: &'static str,

    /// The height of the image.
    pub height: &'static str,

    // Common props
    #[prop_or_default]
    /// The style attribute for the image.
    pub style: &'static str,

    #[prop_or_default]
    /// The CSS class for the image.
    pub class: &'static str,

    #[prop_or_default]
    /// The sizes attribute for the image.
    pub sizes: &'static str,

    #[prop_or_default]
    /// The quality attribute for the image.
    pub quality: &'static str,

    #[prop_or_default]
    /// Indicates if the image should have priority loading.
    pub priority: bool,

    #[prop_or_default]
    /// The placeholder attribute for the image.
    pub placeholder: &'static str,

    #[prop_or_default]
    /// Callback function for handling loading completion.
    pub on_loading_complete: Callback<()>,

    // Advanced Props
    #[prop_or_default]
    /// The object-fit attribute for the image.
    pub object_fit: &'static str,

    #[prop_or_default]
    /// The object-position attribute for the image.
    pub object_position: &'static str,

    #[prop_or_default]
    /// Callback function for handling errors during image loading.
    pub on_error: Callback<String>,

    #[prop_or_default]
    /// The decoding attribute for the image.
    pub decoding: &'static str,

    #[prop_or_default]
    /// The blur data URL for placeholder image.
    pub blur_data_url: &'static str,

    #[prop_or_default]
    /// The lazy boundary for lazy loading.
    pub lazy_boundary: &'static str,

    #[prop_or_default]
    /// Indicates if the image should be unoptimized.
    pub unoptimized: bool,

    // Other Props
    #[prop_or_default]
    /// Reference to the DOM node.
    pub node_ref: NodeRef,
}

/// The Image component for displaying images with various options.
///
/// # Arguments
/// * `props` - The properties of the component.
///
/// # Returns
/// (Html): An HTML representation of the image component.
///
/// # Examples
/// ```
/// // Example of using the Image component
/// use next_rs::{Image, ImageProps};
/// use next_rs::prelude::*;
///
/// #[function_component(MyComponent)]
/// pub fn my_component() -> Html {
///     let image_props = ImageProps {
///         src: "images/logo.png",
///         alt: "Example Image",
///         width: "200",
///         height: "300",
///         style: "border: 1px solid #ddd;",
///         class: "image-class",
///         sizes: "(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 33vw",
///         quality: "80",
///         priority: true,
///         placeholder: "blur",
///         on_loading_complete: Callback::from(|_| {
///             println!("Image loading is complete!");
///         }),
///         object_fit: "cover",
///         object_position: "center",
///         on_error: Callback::from(|err| {
///             println!("Error loading image: {:?}", err);
///         }),
///         decoding: "async",
///         blur_data_url: "data:image/png;base64,....",
///         lazy_boundary: "200px",
///         unoptimized: false,
///         node_ref: NodeRef::default(),
///     };
///
///     rsx! {
///         <Image ..image_props />
///     }
/// }
/// ```
#[function_component(Image)]
pub fn image(props: &ImageProps) -> Html {
    let props = props.clone();

    let fetch_data = {
        Callback::from(move |_| {
            let loader_url = props.src;
            let loading_complete_callback = props.on_loading_complete.clone();
            let on_error_callback = props.on_error.clone();
            spawn_local(async move {
                match Request::get(&loader_url)
                    .cache(RequestCache::Reload)
                    .send()
                    .await
                {
                    Ok(response) => {
                        let json_result = response.json::<serde_json::Value>();
                        match json_result.await {
                            Ok(_data) => {
                                loading_complete_callback.emit(());
                            }
                            Err(_err) => {
                                let event = Event::new("error").unwrap();
                                on_error_callback.emit(event.to_string().into());
                            }
                        }
                    }
                    Err(_err) => {
                        let event = Event::new("error").unwrap();
                        on_error_callback.emit(event.to_string().into());
                    }
                }
            });
        })
    };

    html! {
        <img
            src={props.src}
            alt={props.alt}
            width={props.width}
            height={props.height}
            style={props.style}
            class={props.class}
            loading={if props.priority { "eager" } else { "lazy" }}
            sizes={props.sizes}
            quality={props.quality}
            placeholder={props.placeholder}
            object-fit={props.object_fit}
            object-position={props.object_position}
            onerror={fetch_data.clone()}
            decoding={props.decoding}
            ref={props.node_ref}
        />
    }
}
