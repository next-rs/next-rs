use next_rs::prelude::*;
use next_rs::router::*;

use crate::router::{switch, Route};

#[func]
pub fn App() -> Html {
    rsx! {
      <BrowserRouter>
           <Switch<Route> render={switch} />
      </BrowserRouter>
    }
}
