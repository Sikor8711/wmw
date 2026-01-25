use leptos::html;
use leptos::prelude::*;
use leptos_use::{use_intersection_observer, UseIntersectionObserverReturn};

#[component]
pub fn AnimateIn(
    #[prop(default = "animate-soulful")] animation: &'static str,
    children: Children,
) -> impl IntoView {
    let el = NodeRef::<html::Div>::new();
    let (is_visible, set_is_visible) = signal(false);

    Effect::new(move |_| {
        let _ = use_intersection_observer(el, move |entries, _| {
            if entries[0].is_intersecting() {
                set_is_visible.set(true);
            }
        });
    });
    view! {
        <div
            node_ref=el
            class=move || if is_visible.get() {
                animation.to_string()
            } else {
                "opacity-0".to_string()
            }
            >
            {children()}
            </div>
    }
}
