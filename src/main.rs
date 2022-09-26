pub(crate) mod logic;

use app::App;
mod app;


fn main() {
    yew::start_app_with_props::<App>(
        (  ),
    );
}
