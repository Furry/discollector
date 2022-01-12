macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Clone, PartialEq)] // ewww
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

#[macro_export]
macro_rules! disc_result {
    ( $( $x:expr ),* ) => {
        
    };
}