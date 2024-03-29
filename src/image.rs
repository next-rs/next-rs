use crate::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::js_sys::Function;
use web_sys::wasm_bindgen::JsValue;
use web_sys::{IntersectionObserver, IntersectionObserverInit, RequestCache};

/// Properties for the Image component.
#[derive(Properties, Clone, PartialEq)]
pub struct ImageProps {
    /// The source URL for the image.
    #[prop_or_default]
    pub src: &'static str,

    /// The alternative text for the image.
    #[prop_or_default]
    pub alt: &'static str,

    /// The width of the image.
    #[prop_or_default]
    pub width: &'static str,

    /// The height of the image.
    #[prop_or_default]
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

    #[prop_or_default]
    /// Image layout.
    pub layout: &'static str,

    #[prop_or_default]
    /// Reference to the DOM node.
    pub node_ref: NodeRef,

    #[prop_or_default]
    /// Indicates the current state of the image in a navigation menu. Valid values: "page", "step", "location", "date", "time", "true", "false".
    pub aria_current: &'static str,

    #[prop_or_default]
    /// Describes the image using the ID of the element that provides a description.
    pub aria_describedby: &'static str,

    #[prop_or_default]
    /// Indicates whether the content associated with the image is currently expanded or collapsed. Valid values: "true", "false".
    pub aria_expanded: &'static str,

    #[prop_or_default]
    /// Indicates whether the image is currently hidden from the user. Valid values: "true", "false".
    pub aria_hidden: &'static str,

    #[prop_or_default]
    /// Indicates whether the content associated with the image is live and dynamic. Valid values: "off", "assertive", "polite".
    pub aria_live: &'static str,

    #[prop_or_default]
    /// Indicates whether the image is currently pressed or selected. Valid values: "true", "false", "mixed", "undefined".
    pub aria_pressed: &'static str,

    #[prop_or_default]
    /// ID of the element that the image controls or owns.
    pub aria_controls: &'static str,

    #[prop_or_default]
    /// ID of the element that labels the image.
    pub aria_labelledby: &'static str,
}

impl Default for ImageProps {
    fn default() -> Self {
        ImageProps {
            src: "",
            alt: "Image",
            width: "300",
            height: "200",
            style: "",
            class: "",
            sizes: "",
            quality: "",
            priority: false,
            placeholder: "empty",
            on_loading_complete: Callback::noop(),
            object_fit: "cover",
            object_position: "center",
            on_error: Callback::noop(),
            decoding: "",
            blur_data_url: "",
            lazy_boundary: "100px",
            unoptimized: false,
            layout: "responsive",
            node_ref: NodeRef::default(),
            aria_current: "",
            aria_describedby: "",
            aria_expanded: "",
            aria_hidden: "",
            aria_live: "",
            aria_pressed: "",
            aria_controls: "",
            aria_labelledby: "",
        }
    }
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
/// use next_rs::prelude::*;
/// use next_rs::{Image, ImageProps, log};
///
/// #[func]
/// pub fn MyComponent() -> Html {
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
///             log(&format!("Image loading is complete!").into());
///         }),
///         object_fit: "cover",
///         object_position: "center",
///         on_error: Callback::from(|err| {
///             log(&format!("Error loading image 1: {:#?}", err).into());
///         }),
///         decoding: "async",
///         blur_data_url: "data:image/png;base64,....",
///         lazy_boundary: "200px",
///         unoptimized: false,
///         node_ref: NodeRef::default(),
///         ..ImageProps::default()
///     };
///
///     rsx! {
///         <Image ..image_props />
///     }
/// }
/// ```
#[func]
pub fn Image(props: &ImageProps) -> Html {
    let props = props.clone();
    let img_ref = props.node_ref.clone();

    use_effect_with(JsValue::from(props.src), move |deps| {
        // Define the callback function for the IntersectionObserver
        let callback = Function::new_no_args(
            r###"
            {
                let img_ref = img_ref.clone();
                let on_loading_complete = props.on_loading_complete.clone();
                let on_error = props.on_error.clone();
                
                move || {
                    let entries: Vec<web_sys::IntersectionObserverEntry> = js_sys::try_iter(deps)
                        .unwrap()
                        .unwrap()
                        .map(|v| v.unwrap().unchecked_into())
                        .collect();

                    // Check if the image is intersecting with the viewport
                    if let Some(entry) = entries.get(0) {
                        if entry.is_intersecting() {
                            // Load the image when it becomes visible
                            let img: HtmlImageElement = img_ref.cast().unwrap();
                            img.set_src(&props.src);

                            // Call the loading complete callback
                            on_loading_complete.emit(());
                        }
                    }
                }
            }
            "###,
        );

        // Create IntersectionObserver configuration
        let mut options = IntersectionObserverInit::new();
        options.threshold(deps);
        options.root(Some(
            &web_sys::window()
                .and_then(|win| win.document())
                .unwrap()
                .body()
                .unwrap(),
        ));

        // Create IntersectionObserver instance
        let observer = IntersectionObserver::new_with_options(&callback, &options)
            .expect("Failed to create IntersectionObserver");

        // Observe the image element
        if let Some(img) = img_ref.cast::<web_sys::HtmlElement>() {
            observer.observe(&img);
        }

        // Cleanup: Disconnect the IntersectionObserver when the component unmounts
        return move || {
            observer.disconnect();
        };
    });

    let fetch_data = {
        Callback::from(move |_| {
            let loading_complete_callback = props.on_loading_complete.clone();
            let on_error_callback = props.on_error.clone();
            spawn_local(async move {
                match Request::get(props.src)
                    .cache(RequestCache::Reload)
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status() == 200 {
                            let json_result = response.json::<serde_json::Value>();
                            match json_result.await {
                                Ok(_data) => {
                                    loading_complete_callback.emit(());
                                }
                                Err(_err) => {
                                    on_error_callback.emit(format!("Image Not Found!"));
                                }
                            }
                        } else {
                            let status = response.status();
                            let body = response.text().await.unwrap_or_else(|_| {
                                String::from("Failed to retrieve response body")
                            });
                            on_error_callback.emit(format!(
                                "Failed to load image. Status: {}, Body: {:?}",
                                status, body
                            ));
                        }
                    }

                    Err(err) => {
                        // Handle network errors
                        on_error_callback.emit(format!("Network error: {}", err.to_string()));
                    }
                }
            });
        })
    };

    let img_style = {
        let mut style = String::new();
        if !props.object_fit.is_empty() {
            style.push_str(&format!("object-fit: {};", props.object_fit));
        }
        if !props.object_position.is_empty() {
            style.push_str(&format!("object-position: {};", props.object_position));
        }
        if !props.style.is_empty() {
            style.push_str(props.style);
        }
        style
    };

    let blur_style = if props.placeholder == "blur" {
        format!(
            "background-size: {}; background-position: {}; filter: blur(20px); background-image: url(\"{}\")",
            props.sizes,
            props.object_position,
            props.blur_data_url
        )
    } else {
        String::new()
    };

    let layout = if props.layout == "fill" {
        rsx! {
            <span style={String::from("display: block; position: absolute; top: 0; left: 0; bottom: 0; right: 0;")}>
                <img
                    src={props.src}
                    alt={props.alt}
                    width={props.width}
                    height={props.height}
                    style={img_style}
                    class={props.class}
                    loading={if props.priority { "eager" } else { "lazy" }}
                    sizes={props.sizes}
                    quality={props.quality}
                    placeholder={props.placeholder}
                    decoding={props.decoding}
                    ref={props.node_ref}
                    role="img"
                    aria-label={props.alt}
                    aria-labelledby={props.aria_labelledby}
                    aria-describedby={props.aria_describedby}
                    aria-hidden={props.aria_hidden}
                    aria-current={props.aria_current}
                    aria-expanded={props.aria_expanded}
                    aria-live={props.aria_live}
                    aria-pressed={props.aria_pressed}
                    aria-controls={props.aria_controls}
                    onerror={fetch_data}
                    style={blur_style}
                />
            </span>
        }
    } else if !props.width.is_empty() && !props.height.is_empty() {
        let quotient: f64 =
            props.height.parse::<f64>().unwrap() / props.width.parse::<f64>().unwrap();
        let padding_top: String = if quotient.is_nan() {
            "100%".to_string()
        } else {
            format!("{}%", quotient * 100.0)
        };

        if props.layout == "responsive" {
            rsx! {
                <span style={String::from("display: block; position: relative;")}>
                    <span style={String::from("padding-top: ") + &padding_top}>
                        <img
                            src={props.src}
                            alt={props.alt}
                            width={props.width}
                            height={props.height}
                            style={img_style}
                            class={props.class}
                            loading={if props.priority { "eager" } else { "lazy" }}
                            sizes={props.sizes}
                            quality={props.quality}
                            placeholder={props.placeholder}
                            decoding={props.decoding}
                            ref={props.node_ref}
                            role="img"
                            aria-label={props.alt}
                            aria-labelledby={props.aria_labelledby}
                            aria-describedby={props.aria_describedby}
                            aria-hidden={props.aria_hidden}
                            aria-current={props.aria_current}
                            aria-expanded={props.aria_expanded}
                            aria-live={props.aria_live}
                            aria-pressed={props.aria_pressed}
                            aria-controls={props.aria_controls}
                            onerror={fetch_data}
                            style={blur_style}
                        />
                    </span>
                </span>
            }
        } else if props.layout == "intrinsic" {
            rsx! {
                <span style={String::from("display: inline-block; position: relative; max-width: 100%;")}>
                    <span style={String::from("max-width: 100%;")}>
                        <img
                            src={props.src}
                            alt={props.alt}
                            width={props.width}
                            height={props.height}
                            style={img_style}
                            class={props.class}
                            loading={if props.priority { "eager" } else { "lazy" }}
                            sizes={props.sizes}
                            quality={props.quality}
                            placeholder={props.placeholder}
                            decoding={props.decoding}
                            ref={props.node_ref}
                            role="img"
                            aria-label={props.alt}
                            aria-labelledby={props.aria_labelledby}
                            aria-describedby={props.aria_describedby}
                            aria-hidden={props.aria_hidden}
                            aria-current={props.aria_current}
                            aria-expanded={props.aria_expanded}
                            aria-live={props.aria_live}
                            aria-pressed={props.aria_pressed}
                            aria-controls={props.aria_controls}
                            onerror={fetch_data}
                            style={blur_style}
                        />
                    </span>
                    <img
                        src={props.blur_data_url}
                        style={String::from("display: none;")}
                        alt={props.alt}
                        aria-hidden="true"
                    />
                </span>
            }
        } else if props.layout == "fixed" {
            rsx! {
                <span style={String::from("display: inline-block; position: relative;")}>
                    <img
                        src={props.src}
                        alt={props.alt}
                        width={props.width}
                        height={props.height}
                        style={img_style}
                        class={props.class}
                        loading={if props.priority { "eager" } else { "lazy" }}
                        sizes={props.sizes}
                        quality={props.quality}
                        placeholder={props.placeholder}
                        decoding={props.decoding}
                        ref={props.node_ref}
                        role="img"
                        aria-label={props.alt}
                        aria-labelledby={props.aria_labelledby}
                        aria-describedby={props.aria_describedby}
                        aria-hidden={props.aria_hidden}
                        aria-current={props.aria_current}
                        aria-expanded={props.aria_expanded}
                        aria-live={props.aria_live}
                        aria-pressed={props.aria_pressed}
                        aria-controls={props.aria_controls}
                        onerror={fetch_data}
                        style={blur_style}
                    />
                </span>
            }
        } else {
            rsx! {}
        }
    } else {
        rsx! {
            <span style={String::from("display: block;")}>
                <img
                    src={props.src}
                    alt={props.alt}
                    style={img_style}
                    class={props.class}
                    loading={if props.priority { "eager" } else { "lazy" }}
                    sizes={props.sizes}
                    quality={props.quality}
                    placeholder={props.placeholder}
                    decoding={props.decoding}
                    ref={props.node_ref}
                    role="img"
                    aria-label={props.alt}
                    aria-labelledby={props.aria_labelledby}
                    aria-describedby={props.aria_describedby}
                    aria-hidden={props.aria_hidden}
                    aria-current={props.aria_current}
                    aria-expanded={props.aria_expanded}
                    aria-live={props.aria_live}
                    aria-pressed={props.aria_pressed}
                    aria-controls={props.aria_controls}
                    onerror={fetch_data}
                    style={blur_style}
                />
            </span>
        }
    };
    rsx! {
            {layout}
    }
}
