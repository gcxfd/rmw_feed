#[macro_export]
macro_rules! count {
  ($name:ident) => { 1 };
  ($first:ident, $($rest:ident),*) => {
    1 + util::count!($($rest),*)
  }
}
