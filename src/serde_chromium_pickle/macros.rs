#[macro_export(local_inner_macros)]
macro_rules! forward_to_serialize_any {
    ($($func:ident)*) => {
        $(forward_to_serialize_any_helper!{$func})*
    };
}

#[macro_export]
macro_rules! forward_to_serialize_any_method {
    ($func:ident($($arg:ident : $ty:ty),*)) => {
        #[inline]
        fn $func(self, $($arg: $ty,)*) -> core::result::Result<Self::Ok, Self::Error> {
            self.serialize_any_unit()
        }
    };
    ($func:ident<$l:ty>($($arg:ident : $ty:ty),*)) => {
        #[inline]
        fn $func(self, $($arg: $ty,)* v: $l) -> core::result::Result<Self::Ok, Self::Error> {
            self.serialize_any(v)
        }
    };
    ($func:ident<T: ?Sized, $l:ty>($($arg:ident : $ty:ty),*)) => {
        #[inline]
        fn $func<T: ?Sized>(self, $($arg: $ty,)* v: $l) -> core::result::Result<Self::Ok, Self::Error> {
            self.serialize_any_ref(v)
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! forward_to_serialize_any_helper {
    (bool) => {
        forward_to_serialize_any_method!{serialize_bool<bool>()}
    };
    (i8) => {
        forward_to_serialize_any_method!{serialize_i8<i8>()}
    };
    (i16) => {
        forward_to_serialize_any_method!{serialize_i16<i16>()}
    };
    (i32) => {
        forward_to_serialize_any_method!{serialize_i32<i32>()}
    };
    (i64) => {
        forward_to_serialize_any_method!{serialize_i64<i64>()}
    };
    (i128) => {
        serde::serde_if_integer128! {
            forward_to_serialize_any_method!{serialize_i128<i128>()}
        }
    };
    (u8) => {
        forward_to_serialize_any_method!{serialize_u8<u8>()}
    };
    (u16) => {
        forward_to_serialize_any_method!{serialize_u16<u16>()}
    };
    (u32) => {
        forward_to_serialize_any_method!{serialize_u32<u32>()}
    };
    (u64) => {
        forward_to_serialize_any_method!{serialize_u64<u64>()}
    };
    (u128) => {
        serde::serde_if_integer128! {
            forward_to_serialize_any_method!{serialize_u128<u128>()}
        }
    };
    (f32) => {
        forward_to_serialize_any_method!{serialize_f32<f32>()}
    };
    (f64) => {
        forward_to_serialize_any_method!{serialize_f64<f64>()}
    };
    (char) => {
        forward_to_serialize_any_method!{serialize_char<char>()}
    };
    (str) => {
        forward_to_serialize_any_method!{serialize_str<&str>()}
    };
    (bytes) => {
        forward_to_serialize_any_method!{serialize_bytes<&[u8]>()}
    };
    (none) => {
        forward_to_serialize_any_method!{serialize_none()}
    };
    (some) => {
        forward_to_serialize_any_method!{serialize_some<T: ?Sized, &T>()}
    };
    (unit) => {
        forward_to_serialize_any_method!{serialize_unit()}
    };
    (unit_struct) => {
        forward_to_serialize_any_method!{serialize_unit_struct(name: &'static str)}
    };
    (unit_variant) => {
        forward_to_serialize_any_method!{serialize_unit_variant(name: &'static str, variant_index: u32, variant: &'static str)}
    };
    (newtype_struct) => {
        forward_to_serialize_any_method!{serialize_newtype_struct<T: ?Sized, &T>(name: &'static str)}
    };
    (newtype_variant) => {
        forward_to_serialize_any_method!{serialize_newtype_variant<T: ?Sized, &T>(name: &'static str, variant_index: u32, variant: &'static str)}
    };
}
