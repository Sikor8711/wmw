use leptos::html;
use leptos::prelude::*;
use leptos_use::{use_intersection_observer_with_options, UseIntersectionObserverOptions};
#[component]
pub fn AnimateIn(
    #[prop(default = "animate-soulful")] animation_in: &'static str,
    #[prop(default = "animate-soulful-out")] animation_out: &'static str,
    children: Children,
) -> impl IntoView {
    let sensor_ref = NodeRef::<html::Div>::new(); // The "Anchor"
    let (is_visible, set_is_visible) = signal(false);

    Effect::new(move |_| {
        let _ = use_intersection_observer_with_options(
            sensor_ref, // We watch the anchor
            move |entries, _| {
                set_is_visible.set(entries[0].is_intersecting());
            },
            UseIntersectionObserverOptions::default()
                .thresholds(vec![0.3])
                .root_margin("-50px"),
        );
    });

    view! {
        // This DIV stays perfectly still in the layout
        <div node_ref=sensor_ref class="relative">
            // This DIV does the actual animating
            <div class=move || if is_visible.get() {
                animation_in.to_string()
            } else {
                animation_out.to_string()
            }>
                {children()}
            </div>
        </div>
    }
}
