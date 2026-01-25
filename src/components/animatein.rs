use leptos::html;
use leptos::prelude::*;
#[component]
pub fn AnimateIn(
    #[prop(default = "animate-soulful")] animation: &'static str,
    children: Children,
) -> impl IntoView {
    let el = NodeRef::<html::Div>::new();
    let (is_visible, set_is_visible) = signal(false);

    Effect::new(move |_| {
        use leptos_use::{use_intersection_observer_with_options, UseIntersectionObserverOptions};

        let _ = use_intersection_observer_with_options(
            el,
            move |entries, _| {
                if entries[0].is_intersecting() {
                    set_is_visible.set(true);
                }
            },
            // The builder chain ends here, THEN you close the function
            UseIntersectionObserverOptions::default()
                .thresholds(vec![0.3])
                .root_margin("-50px"), // No comma here
        ); // The closing parenthesis for the whole function goes here
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
