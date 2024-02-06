use next_rs::prelude::*;
use next_rs::router::*;

use crate::pages::landing::LandingPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    LandingPage,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::LandingPage => rsx! {<LandingPage />},
    }
}
