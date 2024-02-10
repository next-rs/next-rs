use crate::log;
use std::borrow::Cow;

use crate::history::{AnyHistory, BrowserHistory, History, HistoryError, HistoryResult};
use crate::prelude::*;
use crate::use_context;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use yew_router::prelude::Location;

use gloo_net::http::Request;
use web_sys::{EventListener, RequestCache};

use wasm_bindgen_futures::spawn_local;
use web_sys::js_sys::Function;

/// Represents errors related to navigation.
pub type NavigationError = HistoryError;

/// Represents results of navigation operations.
pub type NavigationResult<T> = HistoryResult<T>;

/// Represents the context of the current location.
#[derive(Clone)]
pub struct LocationContext {
    location: Location,
    // Counter to force update.
    ctr: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentInfo {
    pub component: Html,
    pub err: &'static str,
}

impl LocationContext {
    /// Returns the current location.
    pub fn location(&self) -> Location {
        self.location.clone()
    }
}

impl PartialEq for LocationContext {
    fn eq(&self, rhs: &Self) -> bool {
        self.ctr == rhs.ctr
    }
}

impl Reducible for LocationContext {
    type Action = Location;

    /// Reduces the state by applying the provided action.
    ///
    /// # Arguments
    ///
    /// * `action` - The action to apply to the state.
    ///
    /// # Returns
    ///
    /// (Rc<Self>): A new reference-counted state after applying the action.
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Self {
            location: action,
            ctr: self.ctr + 1,
        }
        .into()
    }
}

/// Props for [`NextRouter`].
#[derive(Properties, PartialEq, Clone)]
pub struct RouterProps {
    /// Children components to be rendered.
    #[prop_or_default]
    pub children: Html,
    /// The history instance for navigation.
    #[prop_or(AnyHistory::Browser(BrowserHistory::new()))]
    pub history: AnyHistory,
    /// The base URL for the router.
    #[prop_or_default]
    pub basename: &'static str,
}

/// The kind of Router Provider.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RouterKind {
    /// Browser History.
    Browser,
    /// Hash History.
    Hash,
    /// Memory History.
    Memory,
}

/// Represents the context of the current router.
#[derive(Clone, PartialEq)]
pub struct RouterContext {
    router: Router,
}

impl RouterContext {
    /// Returns the current router instance.
    pub fn router(&self) -> Router {
        self.router.clone()
    }
}
/// A struct representing the router for navigation.
#[derive(Debug, Clone)]
pub struct Router {
    /// The history instance for navigation.
    history: AnyHistory,

    /// The base URL for the router.
    basename: &'static str,

    /// The current route of the router.
    route: &'static str,

    /// A mapping of route names to corresponding component information.
    components: HashMap<&'static str, ComponentInfo>,

    /// Set of routes currently being fetched or loaded.
    fetching_routes: HashSet<String>,

    /// Event listener for router events.
    events: EventListener,

    /// The error component to be rendered in case of errors.
    error_component: Html,

    /// The current pathname of the router.
    pathname: &'static str,

    /// The query parameters associated with the current route.
    query: Value,

    /// The path for the current route.
    as_path: &'static str,

    /// Subscriptions to router events with corresponding callbacks.
    subscriptions: Vec<Callback<ComponentInfo>>,

    /// Callback to cancel the loading of a component.
    component_load_cancel: Callback<()>,
}

// Implement PartialEq manually for Router
impl PartialEq for Router {
    fn eq(&self, other: &Self) -> bool {
        self.history == other.history
            && self.basename == other.basename
            && self.route == other.route
            && self.components == other.components
            && self.fetching_routes.len() == other.fetching_routes.len()
            && self.events == other.events
            && self.error_component == other.error_component
            && self.pathname == other.pathname
            && self.query == other.query
            && self.as_path == other.as_path
            && self.subscriptions.len() == other.subscriptions.len()
            && self.component_load_cancel == other.component_load_cancel
    }
}

impl Router {
    /// Creates a new router instance.
    ///
    /// # Arguments
    ///
    /// * `history` - The history instance for navigation.
    /// * `basename` - The base URL for the router.
    /// * `route` - The default route for the router.
    /// * `components` - A mapping of route names to component information.
    /// * `fetching_routes` - Set of routes currently being fetched.
    /// * `events` - Event listener for handling router events.
    /// * `error_component` - The component to display in case of navigation errors.
    /// * `pathname` - The current pathname of the router.
    /// * `query` - The current query parameters of the router.
    /// * `as_path` - The current path as a string.
    /// * `subscriptions` - List of callbacks for component information updates.
    /// * `component_load_cancel` - Callback for cancelling component loading.
    ///
    /// # Returns
    ///
    /// A new `Router` instance.
    pub fn new(
        history: AnyHistory,
        basename: &'static str,
        route: &'static str,
        components: HashMap<&'static str, ComponentInfo>,
        fetching_routes: HashSet<String>,
        events: EventListener,
        error_component: Html,
        pathname: &'static str,
        query: Value,
        as_path: &'static str,
        subscriptions: Vec<Callback<ComponentInfo>>,
        component_load_cancel: Callback<()>,
    ) -> Self {
        Self {
            history,
            basename,
            route,
            components,
            fetching_routes,
            events,
            error_component,
            pathname,
            query,
            as_path,
            subscriptions,
            component_load_cancel,
        }
    }

    /// Returns the basename of the current router.
    pub fn basename(&self) -> &'static str {
        self.basename
    }

    /// Navigates back by one page.
    pub fn back(&self) {
        self.go(-1);
    }

    /// Navigates forward by one page.
    pub fn forward(&self) {
        self.go(1);
    }

    /// Navigates to a specific page with a `delta` relative to the current page.
    ///
    /// # Arguments
    ///
    /// * `delta` - The number of pages to navigate (positive for forward, negative for backward).
    ///
    /// See: <https://developer.mozilla.org/en-US/docs/Web/API/History/go>
    pub fn go(&self, delta: isize) {
        self.history.go(delta);
    }

    /// Pushes a route onto the history stack.
    ///
    /// # Arguments
    ///
    /// * `route` - The route to be pushed.
    pub fn push(&self, route: &'static str) {
        self.history.push(self.prefix_basename(route));
    }

    /// Replaces the current history entry with the provided route.
    ///
    /// # Arguments
    ///
    /// * `route` - The route to replace the current history entry.
    pub fn replace(&self, route: &'static str) {
        self.history.replace(self.prefix_basename(route));
    }

    /// Pushes a route onto the history stack with state.
    ///
    /// # Arguments
    ///
    /// * `route` - The route to be pushed.
    /// * `state` - The state to be associated with the route.
    pub fn push_with_state(&self, route: &'static str, state: &'static str) {
        self.history
            .push_with_state(self.prefix_basename(route), state);
    }

    /// Replaces the current history entry with the provided route and state.
    ///
    /// # Arguments
    ///
    /// * `route` - The route to replace the current history entry.
    /// * `state` - The state to be associated with the route.
    pub fn replace_with_state(&self, route: &'static str, state: &'static str) {
        self.history
            .replace_with_state(self.prefix_basename(route), state);
    }

    /// Pushes a route onto the history stack with query parameters.
    ///
    /// # Arguments
    ///
    /// * `route` - The route to be pushed.
    /// * `query` - The query parameters to be associated with the route.
    ///
    /// # Returns
    ///
    /// A `NavigationResult` indicating the success of the operation.
    pub fn push_with_query(&self, route: &'static str, query: &Value) -> NavigationResult<()> {
        self.history
            .push_with_query(self.prefix_basename(route), query)
    }

    /// Pushes a route onto the history stack with query parameters and state.
    ///
    /// # Arguments
    ///
    /// * `route` - The route to be pushed.
    /// * `query` - The query parameters to be associated with the route.
    /// * `state` - The state to be associated with the route.
    ///
    /// # Returns
    ///
    /// A `NavigationResult` indicating the success of the operation.
    pub fn push_with_query_and_state(
        &self,
        route: &'static str,
        query: &Value,
        state: &'static str,
    ) -> NavigationResult<()> {
        self.history
            .push_with_query_and_state(self.prefix_basename(route), query, state)
    }

    /// Replaces the current history entry with the provided route, query parameters, and state.
    ///
    /// # Arguments
    ///
    /// * `route` - The route to replace the current history entry.
    /// * `query` - The query parameters to be associated with the route.
    /// * `state` - The state to be associated with the route.
    ///
    /// # Returns
    ///
    /// A `NavigationResult` indicating the success of the operation.
    pub fn replace_with_query_and_state(
        &self,
        route: &'static str,
        query: &Value,
        state: Value,
    ) -> NavigationResult<()> {
        self.history
            .replace_with_query_and_state(self.prefix_basename(route), query, state)
    }

    /// Returns the kind of the router.
    ///
    /// # Returns
    ///
    /// A `RouterKind` enum representing the type of the router.
    pub fn kind(&self) -> RouterKind {
        match &self.history {
            AnyHistory::Browser(_) => RouterKind::Browser,
            AnyHistory::Hash(_) => RouterKind::Hash,
            AnyHistory::Memory(_) => RouterKind::Memory,
        }
    }

    /// Prefixes the basename to the route.
    ///
    /// # Arguments
    ///
    /// * `route_s` - The route to prefix with the basename.
    ///
    /// # Returns
    ///
    /// A `Cow<'a, str>` containing the combined route with the basename.
    pub fn prefix_basename<'a>(&self, route_s: &'a str) -> Cow<'a, str> {
        let base = self.basename();
        if !base.is_empty() {
            if route_s.is_empty() && route_s.is_empty() {
                Cow::from("/")
            } else {
                Cow::from(format!("{base}{route_s}"))
            }
        } else {
            route_s.into()
        }
    }

    /// Strips the basename from the path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to strip the basename from.
    ///
    /// # Returns
    ///
    /// A `Cow<'a, str>` containing the path with the basename stripped.
    pub fn strip_basename<'a>(&self, path: Cow<'a, str>) -> Cow<'a, str> {
        let m = self.basename();
        if !m.is_empty() {
            let mut path = path
                .strip_prefix(m)
                .map(|m| Cow::from(m.to_owned()))
                .unwrap_or(path);

            if !path.starts_with('/') {
                path = format!("/{m}").into();
            }

            path
        } else {
            path
        }
    }

    /// Prefetches the specified URL by fetching its route information.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to prefetch.
    pub fn prefetch(&mut self, url: &'static str) {
        self.fetch_route(url.to_string());
    }

    /// Asynchronously fetches route information for the given URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL for which to fetch route information.
    ///
    /// # Returns
    ///
    /// A `Result` containing `ComponentInfo` on success and an error `Value` on failure.
    async fn fetch_gloo_net(url: &str) -> Result<ComponentInfo, Value> {
        let response = Request::get(url)
            .cache(RequestCache::Reload)
            .send()
            .await
            .unwrap();

        let json_result = response.json::<serde_json::Value>().await;

        match json_result {
            Ok(_data) => Ok(ComponentInfo {
                component: rsx! {},
                err: "",
            }),
            Err(err) => Err(err.to_string().into()),
        }
    }

    /// Initiates the fetching of route information for the specified route.
    ///
    /// # Arguments
    ///
    /// * `route` - The route to fetch.
    fn fetch_route(&mut self, route: String) {
        // let url = format!("/{}/index.json", route);
        // TODO: for demonstration purposes using an external api. Replace it with a local component json file
        let url = "https://dog.ceo/api/breeds/image/random";
        let events = EventListener::new();
        let subscriptions = self.subscriptions.clone();
        let as_path = self.as_path;
        let route = route.clone();
        let self_route = self.route;
        let fetching_routes = Callback::from(move |_: String| {
            // let url = url.clone();
            let mut fetching_routes = HashSet::new();
            let mut events = events.clone();
            let subscriptions = subscriptions.clone();
            let as_path = as_path;
            let route = route.clone();
            let self_route = self_route;
            spawn_local(async move {
                match Self::fetch_gloo_net(&url).await {
                    Ok(component_info) => {
                        fetching_routes.insert(route.clone());
                        if self_route == route {
                            let ComponentInfo { component: _, err } = component_info.clone();
                            if !err.is_empty() {
                                events.handle_event(&Function::new_with_args(
                                    "route_change_error",
                                    as_path,
                                ));
                            }
                            Self::notify(subscriptions, component_info);
                            events.handle_event(&Function::new_with_args(
                                "route_change_complete",
                                as_path,
                            ));
                        }
                    }
                    Err(_err) => {
                        fetching_routes.insert(route.clone());
                        if self_route == route {
                            let component_info = ComponentInfo {
                                component: rsx! {},
                                err: "Error fetching route",
                            };
                            Self::notify(subscriptions, component_info);
                            events.handle_event(&Function::new_with_args(
                                "route_change_complete",
                                as_path,
                            ));
                        }
                    }
                }
                fetching_routes.insert(route.clone());
            });
            // fetching_routes.clone()
        });
        // self.fetching_routes = fetching_routes.emit("".to_string());
        fetching_routes.emit("".to_string())
    }

    /// Notifies all subscribed callbacks with the provided route information.
    ///
    /// # Arguments
    ///
    /// * `subscriptions` - A vector of callbacks to notify.
    /// * `data` - The route information to emit to the callbacks.
    fn notify(subscriptions: Vec<Callback<ComponentInfo>>, data: ComponentInfo) {
        subscriptions.iter().for_each(|callback| {
            callback.emit(data.clone());
        });
    }

    /// Subscribes to route change events and returns an unsubscribe callback.
    ///
    /// # Arguments
    ///
    /// * `callback` - The callback to be notified on route changes.
    ///
    /// # Returns
    ///
    /// A `Callback<()>` that can be used to unsubscribe from route change events.
    fn _subscribe(&mut self, callback: Callback<ComponentInfo>) -> Callback<()> {
        // Creates a Listener that will be notified when current state changes.
        // self.history.listen(callback);
        self.subscriptions.push(callback.clone());
        Callback::from(move |_| {
            // TODO: Implement unsubscribe, rm from subscriptions vec
        })
    }
}

/// The base router component.
///
/// This component ensures that `<Router />` has the same virtual DOM layout as `<BrowserRouter />`
/// and `<HashRouter />`.
///
/// # Arguments
///
/// * `props` - The properties of the router.
///
/// # Returns
///
/// (Html): An HTML representation of the router component.
///
/// # Example
/// ```
/// use next_rs::prelude::*;
/// use next_rs::router::*;
/// use next_rs::history::{BrowserHistory, AnyHistory};
///
/// #[func]
/// fn MyComponent() -> Html {
///     rsx! {
///         <BaseRouter basename="" history={AnyHistory::Browser(BrowserHistory::new())}>
///             <div />
///         </BaseRouter>
///     }
/// }
/// ```
#[func]
pub fn BaseRouter(props: &RouterProps) -> Html {
    let RouterProps {
        history,
        children,
        basename,
    } = props.clone();

    let loc_ctx = use_reducer(|| LocationContext {
        location: history.location(),
        ctr: 0,
    });

    let trigger = use_force_update();
    let prefetched_component = use_state(|| rsx! {<></>});
    let component_value = (*prefetched_component).clone();

    let basename = basename.strip_suffix('/').unwrap_or(basename);

    let route = "/";
    let components = HashMap::new();
    let fetching_routes = HashSet::new();
    let events = EventListener::new();
    let error_component = Html::default();
    let pathname = "";
    let query = Value::default();
    let as_path = "";
    let mut subscriptions = Vec::new();
    subscriptions.push(Callback::from(move |component: ComponentInfo| {
        prefetched_component.set(component.component);
        trigger.force_update();
        log(&format!("prefetch callback...").into());
    }));
    let component_load_cancel = Callback::default();

    let router = Router::new(
        history.clone(),
        basename,
        route,
        components,
        fetching_routes,
        events,
        error_component,
        pathname,
        query,
        as_path,
        subscriptions,
        component_load_cancel,
    );
    let navi_ctx = RouterContext {
        router: router.clone(),
    };

    {
        let loc_ctx_dispatcher = loc_ctx.dispatcher();

        use_effect_with(history, move |history| {
            let history = history.clone();
            // Force location update when history changes.
            loc_ctx_dispatcher.dispatch(history.location());

            let history_cb = {
                let history = history.clone();
                move || loc_ctx_dispatcher.dispatch(history.location())
            };

            let listener = history.listen(history_cb);

            // We hold the listener in the destructor.
            move || {
                std::mem::drop(listener);
            }
        });
    }

    rsx! {
        <ContextProvider<RouterContext> context={navi_ctx}>
            <ContextProvider<LocationContext> context={(*loc_ctx).clone()}>
                {children}
                {component_value}
            </ContextProvider<LocationContext>>
        </ContextProvider<RouterContext>>
    }
}

/// Props for [`Switch`]
#[derive(Properties, PartialEq, Clone)]
pub struct SwitchProps {
    /// Callback which returns [`Html`] to be rendered for the current route.
    pub render: Callback<String, Html>,
    #[prop_or_default]
    pub pathname: &'static str,
}

/// A Switch that dispatches routes among variants of a [`Routable`].
///
/// When a route can't be matched, including when the path is matched but the deserialization fails,
/// it looks for the route with the `not_found` attribute.
/// If such a route is provided, it redirects to the specified route.
/// Otherwise, an empty HTML element is rendered, and a message is logged to the console
/// stating that no route can be matched.
/// See the [crate level document][crate] for more information.
///
/// # Arguments
///
/// * `props` - The properties of the switch.
///
/// # Returns
///
/// (Html): An HTML representation of the switch component.
///
/// # Example
/// ```
/// use next_rs::prelude::*;
/// use next_rs::router::*;
///
/// pub fn switch(route: String) -> Html {
///     match route.as_str() {
///         "/" => rsx! {<div />},
///         _ => rsx! {<></>},
///     }
/// }
///
/// #[func]
/// fn MySwitch() -> Html {
///     rsx! {
///         <Switch render={switch} />
///     }
/// }
/// ```
#[func]
pub fn Switch(props: &SwitchProps) -> Html {
    let mut route = use_route();

    if route.is_empty() {
        route = std::borrow::Cow::Borrowed(props.pathname);
    }

    if !route.is_empty() {
        props.render.emit(route.to_string())
    } else {
        Html::default()
    }
}

/// The NextRouter component.
///
/// This component provides location and navigator context to its children and switches.
///
/// If you are building a web application, you may want to consider using [`BrowserRouter`] instead.
///
/// You only need one `<Router />` for each application.
///
/// # Arguments
///
/// * `props` - The properties of the router.
///
/// # Returns
///
/// (Html): An HTML representation of the router component.
///
/// # Example
/// ```
/// use next_rs::prelude::*;
/// use next_rs::router::*;
/// use next_rs::history::{BrowserHistory, AnyHistory};
///
/// #[func]
/// fn MyComponent() -> Html {
///     rsx! {
///         <NextRouter basename="" history={AnyHistory::Browser(BrowserHistory::new())}>
///             <div />
///         </NextRouter>
///     }
/// }
/// ```
#[func]
pub fn NextRouter(props: &RouterProps) -> Html {
    rsx! {
        <BaseRouter ..props.clone() />
    }
}

/// A hook to access the [`Router`] instance.
///
/// This hook allows components to access the router, which manages the application's navigation and routes.
/// It retrieves the router from the current context and returns it.
#[hook]
pub fn use_router() -> Router {
    use_context::<RouterContext>()
        .map(|m| m.router())
        .expect("router")
}

/// A hook to access the current [`Location`] information.
///
/// This hook provides components with access to the current location, including details such as the path and query parameters.
/// It retrieves the location from the current context and returns it as an `Option`.
#[hook]
pub fn use_location() -> Option<Location> {
    Some(use_context::<LocationContext>()?.location())
}

/// A hook to access the current route path with the basename stripped.
///
/// This hook is useful for components that need the current route path with the basename removed.
/// It uses the `use_router` and `use_location` hooks to get the router and location information,
/// then strips the basename from the location path, returning the stripped path as a `Cow<'static, str>`.
#[hook]
pub fn use_route() -> Cow<'static, str> {
    let router = use_router();
    let location = use_location().expect("location");

    let stripped_path: Cow<'static, str> = router
        .strip_basename(Cow::Borrowed(location.path()))
        .into_owned()
        .into();

    stripped_path
}
