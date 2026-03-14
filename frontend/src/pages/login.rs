use crate::components::ui::button::Button;
use leptos::prelude::*;
use leptos::IntoView;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <h1>Hello Login</h1>
        <Button>Click me</Button>
    }
}
