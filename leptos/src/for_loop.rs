use leptos_dom::IntoView;
use leptos_macro::component;
use std::hash::Hash;

/// Iterates over children and displays them, keyed by the `key` function given.
///
/// This is much more efficient than naively iterating over nodes with `.iter().map(|n| view! { ... })...`,
/// as it avoids re-creating DOM nodes that are not being changed.
///
/// ```
/// # use leptos::*;
///
/// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// struct Counter {
///   id: usize,
///   count: RwSignal<i32>
/// }
///
/// #[component]
/// fn Counters() -> impl IntoView {
///   let (counters, set_counters) = create_signal::<Vec<Counter>>(vec![]);
///
///   view! {
///     <div>
///       <For
///         // a function that returns the items we're iterating over; a signal is fine
///         each=move || counters.get()
///         // a unique key for each item
///         key=|counter| counter.id
///         // renders each item to a view
///         view=move |counter: Counter| {
///           view! {
///             <button>"Value: " {move || counter.count.get()}</button>
///           }
///         }
///       />
///     </div>
///   }
/// }
/// ```
#[cfg_attr(
    any(debug_assertions, feature = "ssr"),
    tracing::instrument(level = "info", skip_all)
)]
#[component(transparent)]
pub fn For<IF, I, T, EF, N, KF, K>(
    /// Items over which the component should iterate.
    each: IF,
    /// A key function that will be applied to each item.
    key: KF,
    /// The view that will be displayed for each item.
    view: EF,
) -> impl IntoView
where
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = T>,
    EF: Fn(T) -> N + 'static,
    N: IntoView + 'static,
    KF: Fn(&T) -> K + 'static,
    K: Eq + Hash + 'static,
    T: 'static,
{
    leptos_dom::Each::new(each, key, view).into_view()
}
