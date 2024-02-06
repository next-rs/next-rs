use next_rs::prelude::*;
use next_rs::{log, Image, ImageProps};

#[func]
pub fn LandingPage() -> Html {
    let images = vec![
        ImageProps {
            src: "images/image.jpg",
            alt: "Image 1 - Default",
            width: "400",
            height: "300",
            class: "rounded-md shadow-md",
            lazy_boundary: "150px",
            priority: true,
            placeholder: "blur",
            object_fit: "cover",
            object_position: "center",
            blur_data_url: "data:image/png;base64,iVBORwGgoAAAANSUhgAA...",
            unoptimized: false,
            node_ref: NodeRef::default(),
            aria_current: "page",
            aria_describedby: "description1",
            aria_expanded: "true",
            aria_hidden: "false",
            aria_live: "polite",
            aria_pressed: "mixed",
            aria_controls: "controls1",
            aria_labelledby: "label1",
            ..ImageProps::default()
        },
        ImageProps {
            src: "images/image.jpg",
            alt: "Image 2 - No Priority",
            width: "300",
            height: "200",
            class: "rounded-md shadow-md",
            lazy_boundary: "150px",
            priority: false,
            placeholder: "blur",
            object_fit: "cover",
            object_position: "center",
            on_error: Callback::noop(),
            blur_data_url: "data:image/png;base64,iVBORwGgoAAAANSUhgAA...",
            unoptimized: false,
            node_ref: NodeRef::default(),
            aria_current: "page",
            aria_describedby: "description1",
            aria_expanded: "true",
            aria_hidden: "false",
            aria_live: "polite",
            aria_pressed: "mixed",
            aria_controls: "controls1",
            aria_labelledby: "label1",
            ..ImageProps::default()
        },
        ImageProps {
            src: "images/image.jpg",
            alt: "Image 3 - High Quality",
            width: "350",
            height: "250",
            class: "rounded-lg shadow-lg",
            lazy_boundary: "200px",
            priority: true,
            placeholder: "blur",
            object_fit: "cover",
            object_position: "center",
            on_error: Callback::noop(),
            blur_data_url: "data:image/png;base64,iVBORwGgoAAAANSUhgAA...",
            quality: "high",
            unoptimized: false,
            node_ref: NodeRef::default(),
            aria_expanded: "false",
            aria_live: "off",
            aria_controls: "controls3",
            aria_labelledby: "label3",
            ..ImageProps::default()
        },
        ImageProps {
            src: "images/not-found.jpg",
            alt: "Image 4 - Non Existent Image, Press F12",
            on_loading_complete: Callback::from(|_| {
                log(&format!("Image loading is complete!").into());
            }),
            on_error: Callback::from(|err| {
                log(&format!("Error loading image 1: {:#?}", err).into());
            }),
            width: "300",
            height: "400",
            class: "rounded-md shadow-md",
            lazy_boundary: "180px",
            priority: true,
            placeholder: "blur",
            object_fit: "contain",
            object_position: "left",
            decoding: "async",
            blur_data_url: "data:image/png;base64,iVBORwGgoAAAANSUhgAA...",
            aria_current: "step",
            aria_expanded: "true",
            aria_pressed: "true",
            aria_labelledby: "label4",
            ..ImageProps::default()
        },
        ImageProps {
            src: "images/image.jpg",
            alt: "Image 5 - Custom Style",
            width: "350",
            height: "250",
            class: "rounded-md shadow-md",
            lazy_boundary: "180px",
            priority: true,
            placeholder: "empty",
            style: "max-width: 80%;",
            on_error: Callback::noop(),
            blur_data_url: "data:image/png;base64,iVBORwGgoAAAANSUhgAA...",
            aria_current: "step",
            aria_hidden: "true",
            aria_pressed: "false",
            ..ImageProps::default()
        },
        ImageProps {
            src: "images/image.jpg",
            alt: "Image 6 - Custom Size",
            width: "500",
            height: "350",
            class: "rounded-md shadow-md",
            lazy_boundary: "200px",
            priority: false,
            placeholder: "blur",
            object_fit: "cover",
            object_position: "center",
            on_error: Callback::noop(),
            blur_data_url: "data:image/png;base64,iVBORwGgoAAAANSUhgAA...",
            unoptimized: true,
            aria_current: "step",
            aria_hidden: "false",
            aria_pressed: "false",
            ..ImageProps::default()
        },
        ImageProps {
            src: "images/image.jpg",
            alt: "Image 7 - Custom Placeholder",
            width: "400",
            height: "300",
            class: "rounded-md shadow-md",
            lazy_boundary: "220px",
            priority: true,
            placeholder: "empty",
            on_error: Callback::noop(),
            blur_data_url: "data:image/png;base64,iVBORwGgoAAAANSUhgAA...",
            aria_labelledby: "desc7",
            ..ImageProps::default()
        },
        ImageProps {
            src: "images/image.jpg",
            alt: "Image 8 - Lazy Loading",
            width: "400",
            height: "300",
            class: "rounded-md shadow-md",
            lazy_boundary: "150px",
            priority: false,
            placeholder: "blur",
            object_fit: "cover",
            object_position: "center",
            on_error: Callback::noop(),
            blur_data_url: "data:image/png;base64,iVBORwGgoAAAANSUhgAA...",
            unoptimized: true,
            aria_current: "location",
            aria_hidden: "true",
            aria_pressed: "false",
            ..ImageProps::default()
        },
        ImageProps {
            src: "images/image.jpg",
            alt: "Image 9 - Custom ARIA",
            width: "400",
            height: "300",
            class: "rounded-md shadow-md",
            lazy_boundary: "180px",
            priority: true,
            placeholder: "blur",
            object_fit: "cover",
            object_position: "center",
            on_error: Callback::noop(),
            blur_data_url: "data:image/png;base64,iVBORwGgoAAAANSUhgAA...",
            aria_expanded: "true",
            aria_live: "assertive",
            aria_controls: "controls9",
            aria_labelledby: "label9",
            ..ImageProps::default()
        },
        ImageProps {
            src: "images/image.jpg",
            alt: "Image 10 - Rounded Corners",
            class: "rounded-lg shadow-md",
            lazy_boundary: "250px",
            priority: false,
            placeholder: "blur",
            object_fit: "cover",
            object_position: "center",
            on_error: Callback::noop(),
            blur_data_url: "data:image/png;base64,iVBORwGgoAAAANSUhgAA...",
            unoptimized: false,
            node_ref: NodeRef::default(),
            aria_expanded: "false",
            aria_live: "off",
            aria_controls: "controls13",
            aria_labelledby: "label13",
            ..ImageProps::default()
        },
        ImageProps {
            src: "images/image.jpg",
            alt: "Image 11 - No Object Position",
            width: "400",
            height: "300",
            class: "rounded-md shadow-md",
            lazy_boundary: "200px",
            priority: true,
            placeholder: "blur",
            object_fit: "cover",
            on_error: Callback::noop(),
            blur_data_url: "data:image/png;base64,iVBORwGgoAAAANSUhgAA...",
            unoptimized: false,
            node_ref: NodeRef::default(),
            aria_current: "step",
            aria_expanded: "true",
            aria_pressed: "true",
            aria_labelledby: "label14",
            ..ImageProps::default()
        },
        ImageProps {
            src: "images/image.jpg",
            alt: "Image 12 - Async Decoding",
            width: "400",
            height: "300",
            class: "rounded-md shadow-md",
            lazy_boundary: "180px",
            priority: true,
            placeholder: "blur",
            object_fit: "cover",
            object_position: "center",
            on_error: Callback::noop(),
            decoding: "async",
            blur_data_url: "data:image/png;base64,iVBORwGgoAAAANSUhgAA...",
            aria_current: "step",
            aria_expanded: "true",
            aria_pressed: "true",
            aria_labelledby: "label15",
            ..ImageProps::default()
        },
        ImageProps {
            src: "images/image.jpg",
            alt: "Image 13 - Centered Object",
            width: "350",
            height: "250",
            class: "rounded-md shadow-md",
            lazy_boundary: "180px",
            priority: true,
            placeholder: "blur",
            object_fit: "cover",
            object_position: "center",
            on_error: Callback::noop(),
            blur_data_url: "data:image/png;base64,iVBORwGgoAAAANSUhgAA...",
            aria_current: "step",
            aria_hidden: "true",
            aria_pressed: "false",
            ..ImageProps::default()
        },
        ImageProps {
            src: "images/image.jpg",
            alt: "Image 14 - Higher Quality",
            width: "400",
            height: "300",
            class: "rounded-md shadow-md",
            lazy_boundary: "200px",
            priority: true,
            placeholder: "blur",
            object_fit: "cover",
            object_position: "center",
            on_error: Callback::noop(),
            blur_data_url: "data:image/png;base64,iVBORwGgoAAAANSUhgAA...",
            quality: "high",
            unoptimized: false,
            aria_expanded: "false",
            aria_live: "off",
            aria_controls: "controls17",
            aria_labelledby: "label17",
            ..ImageProps::default()
        },
        ImageProps {
            src: "images/image.jpg",
            alt: "Image 15 - No Border",
            width: "400",
            height: "300",
            class: "rounded-md shadow-md",
            lazy_boundary: "180px",
            priority: true,
            placeholder: "blur",
            object_fit: "cover",
            object_position: "center",
            on_error: Callback::noop(),
            blur_data_url: "data:image/png;base64,iVBORwGgoAAAANSUhgAA...",
            aria_current: "step",
            aria_expanded: "true",
            aria_pressed: "true",
            aria_labelledby: "label18",
            ..ImageProps::default()
        },
    ];

    rsx! {
        <div class="mx-auto p-8 bg-gray-900 text-white justify-center items-center">
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 justify-center">
                {for images.iter().enumerate().map(|(i, props)| {
                    let description = format!(
                        "Width: {},\n\
                        Height: {},\n\
                        Object Fit: {},\n\
                        Object Position: {},\n\
                        Priority: {},\n\
                        Placeholder: {},\n\
                        Quality: {},\n\
                        Unoptimized: {},\n\
                        Style: {},\n\
                        Sizes: {},\n\
                        Decoding: {},\n\
                        Lazy Boundary: {},\n\
                        Node Ref: {},\n\
                        ARIA Current: {},\n\
                        ARIA Describedby: {},\n\
                        ARIA Expanded: {},\n\
                        ARIA Hidden: {},\n\
                        ARIA Live: {},\n\
                        ARIA Pressed: {},\n\
                        ARIA Controls: {},\n\
                        ARIA Labelledby: {}",
                        props.width, props.height, props.object_fit, props.object_position,
                        props.priority, props.placeholder, props.quality, props.unoptimized,
                        props.style, props.sizes,
                        props.decoding, props.lazy_boundary,
                        props.node_ref.get().map_or_else(|| "None".to_string(), |node| node.to_string().into()),
                        props.aria_current, props.aria_describedby, props.aria_expanded,
                        props.aria_hidden, props.aria_live, props.aria_pressed,
                        props.aria_controls, props.aria_labelledby
                    );
                    rsx! {
                        <div class={"w-full md:w-3/4"}>
                            <div class="mb-5 text-white bg-gray-800 p-4 rounded-md">
                                <h2 class="text-lg font-bold mb-2">{&props.alt}</h2>
                                <Image key={i} ..props.clone() />
                                <p class="text-sm text-gray-300 mt-4 whitespace-pre-line">{description}</p>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
