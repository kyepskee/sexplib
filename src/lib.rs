pub use main::*;
pub use _expr;

#[macro_export]
macro_rules! expr {( $($input:tt)* ) => (
    $crate::_expr::expr! {
        [$crate]
        $($input)*
    }
)}
