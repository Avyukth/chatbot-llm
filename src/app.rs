use model::conversions::*;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (conversions,set_conversions) = create_signal(cx, Conversion::new());
    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        
        // sets the document title
        <Title text="Welcome to Testy Chat"/>
        <ChatArea/>
        <TypeArea/>

    }
}
