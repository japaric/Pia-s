#[macro_export]
macro_rules! inheritance {
    ($child:ident : $parent:path) => {
        #[derive(Clone)]
        #[repr(transparent)]
        pub struct $child {
            inner: $parent,
        }

        impl core::ops::Deref for $child {
            type Target = $parent;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        unsafe impl $crate::IsValue for $child {}

        impl $crate::Downcast<$child> for $crate::Value {
            fn downcast(self) -> $child {
                unsafe { core::mem::transmute(self) }
            }
        }

        impl $crate::Upcast for $child {
            type Supertype = $parent;

            fn upcast(self) -> $parent {
                self.inner
            }
        }
    };
}

#[macro_export]
macro_rules! call {
    ($object:expr, $method:expr $(, $arg:expr)*) => {{
        let args = $crate::Array::new();
        $( args.push($arg); )*
        $object.call(&stringify!($method).into(), &args)
    }}
}
