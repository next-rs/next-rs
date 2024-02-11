use crate::pages::landing::LandingPage;
use next_rs::prelude::*;
use next_rs::router::*;

pub fn switch(route: String) -> Html {
    match route.as_str() {
        "/" => rsx! {<LandingPage />},
        "/landing" => rsx! {<LandingPage />},
        _ => rsx! {<></>},
    }
}

#[func]
pub fn App() -> Html {
    rsx! {
      <NextRouter>
           <Switch render={switch} />
      </NextRouter>
    }
}
