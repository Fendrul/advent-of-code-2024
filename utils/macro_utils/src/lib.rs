#[macro_export]
macro_rules! destruct_options {
    ($($option_ident:ident),+ ? $expr:expr) => {
        match ($($option_ident),*) {
            ($(Some($option_ident)),*) => ($($option_ident),*),
            _ => $expr,
        }
    };
}
