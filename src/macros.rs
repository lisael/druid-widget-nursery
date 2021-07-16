/// Generates `Selector`s based on module, line and column
/// ```
/// # use druid_widget_nursery::selectors;
/// selectors! {
///     #[doc = "Fooes the baz"]
///     FOO,
///     #[doc = "Bars the foobar by the given value"]
///     BAR: usize,
/// }
/// ```
/// expands to
/// ```
/// # use druid::Selector;
/// /// Fooes the bar
/// pub const FOO: Selector = Selector::new("path::to::module::FOO@0:0");
/// /// Bars the foobar by the given value
/// pub const BAR: Selector<usize> = Selector::new("path::to::module::BAR@0:0");
/// ```
#[macro_export]
macro_rules! selectors {
    (
        $(
            $(#[$inner:ident $($args:tt)*])*
            $name:ident $( : $ty:ty)?
        ),* $(,)?
    ) => {
        $(
            $(#[$inner $($args)*])*
            pub const $name: ::druid::Selector<$($ty)?> = ::druid::Selector::new(concat!(
                module_path!(),
                "::",
                stringify!($name),
                "@",
                line!(),
                ":",
                column!()
            ));
        )*
    };
}

/// Generates `Key`s based on module, line and column
/// ```
/// # use druid_widget_nursery::keys;
/// keys! {
///     BAR: usize,
/// }
/// ```
/// expands to
/// ```
/// # use druid::Key;
/// pub const BAR: Key<usize> = Key::new("path::to::module::BAR@0:0");
/// ```
#[macro_export]
macro_rules! keys {
    (
        $(
            $(#[$inner:ident $($args:tt)*])*
            $name:ident : $ty:ty
        ),* $(,)?
    ) => {
        $(
            $(#[$inner $($args)*])*
            pub const $name: ::druid::Key<$ty> = ::druid::Key::new(concat!(
                module_path!(),
                "::",
                stringify!($name),
                "@",
                line!(),
                ":",
                column!()
            ));
        )*
    };
}
