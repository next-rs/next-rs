use std::borrow::Cow;

use crate::history::{AnyHistory, BrowserHistory, History, HistoryError, HistoryResult};
use crate::prelude::*;
use crate::use_context;
use serde_json::Value;
use std::rc::Rc;
use yew_router::prelude::Location;

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

/// A struct representing the router for navigation.
#[derive(Debug, PartialEq, Clone)]
pub struct Router {
    history: AnyHistory,
    basename: &'static str,
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

impl Router {
    /// Creates a new router instance.
    ///
    /// # Arguments
    ///
    /// * `history` - The history instance for navigation.
    /// * `basename` - The base URL for the router.
    pub fn new(history: AnyHistory, basename: &'static str) -> Self {
        Self { history, basename }
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

    let basename = basename.strip_suffix('/').unwrap_or(basename);
    let navi_ctx = RouterContext {
        router: Router::new(history.clone(), basename),
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
/// #[function_component]
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

/// A hook to access the [`Router`].
#[hook]
pub fn use_router() -> Router {
    use_context::<RouterContext>()
        .map(|m| m.router())
        .expect("router")
}

/// A hook to access the current [`Location`].
#[hook]
pub fn use_location() -> Option<Location> {
    Some(use_context::<LocationContext>()?.location())
}

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
