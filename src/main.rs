pub(crate) mod rendering;
pub(crate) mod logic;
pub(crate) mod app;


fn main() {
    yew::start_app_with_props::<app::App>(());
}
