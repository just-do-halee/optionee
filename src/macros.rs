// Licensed under either of Apache License, Version 2.0 or MIT license at your option.
// Copyright 2021 Hwakyeom Kim(=just-do-halee)

#[macro_export]
macro_rules! optionee {
    (@transform (>))
    => {
        core::cmp::Ordering::Greater
    };
    (@transform (<))
    => {
        core::cmp::Ordering::Less
    };
    (@transform (=))
    => {
        core::cmp::Ordering::Equal
    };
    (@transform ())
    => {
        None
    };
    (@transform ($e:expr))
    => {
        Some($e)
    };
    (
        $(
            $vis:vis $id:ident {
                $(
                    $name:ident {
                        $(
                            $var:ident: $t:ty [$cmp:tt] $val:expr $(, $err:expr)?
                        )+
                    }
                )+
            }
        )+
    ) => {
        $(
            pub struct $id;
            impl $id {
                $(
                    #[allow(non_snake_case, dead_code)]
                    pub fn $name(self) -> optionees::$id::$name {
                        optionees::$id::$name::new()
                    }
                )+
            }
        )+
        mod optionees {
            #![allow(unused_imports)]
            #![allow(dead_code)]
            use super::*;
            use core::{result::Result, cmp::{Ord, Ordering}, option::Option};
            pub struct Wrapping<T: Ord>(T, Ordering, Option<&'static str>);
            impl<T: Ord> Wrapping<T> {
                pub fn check(&self, value: T) -> Result<(), &'static str> {
                    let ordering = self.1;
                    let msg = self.2;
                    if value.cmp(&self.0) != ordering {
                        if let Some(v) = msg {
                            Err(v)
                        } else {
                            Err("not matched.")
                        }
                    } else {
                        Ok(())
                    }
                }
                pub fn get_value(&self) -> &T {
                    &self.0
                }
                pub fn get_error_message(&self) -> Option<&'static str> {
                    self.2
                }
                pub fn set_error_message(&mut self, msg: Option<&'static str>) {
                    self.2 = msg;
                }
            }
            $(
                #[allow(non_snake_case)]
                pub mod $id {
                    use super::*;
                    $(
                        pub struct $name {
                            $(
                                pub $var: Wrapping<$t>
                            ),+
                        }
                        impl $name {
                            pub fn new() -> Self {
                                Self {
                                    $(
                                        $var: Wrapping($val, optionee!(@transform ($cmp)), optionee!(@transform ($($err)?)))
                                    ),*
                                }
                            }
                            $(
                                pub fn $var(mut self, value: $t) -> Self {
                                    self.$var.0 = value;
                                    self
                                }
                            )*
                        }
                    )+
                }
            )+
        }
    };
}

#[macro_export]
macro_rules! orderable {
    (@transform [])
    => {
        false
    };
    (@transform [*])
    => {
        true
    };
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field:meta])*
                $var:ident: $t:ty$([$tt:tt])?$(,)?
            )+
        }
    ) => {
        $(#[$meta])*
        #[derive(Eq)]
        $vis struct $name {
            $(
                $(#[$field])*
                $var: $t
            ),+
        }
        impl $name {
            pub fn new($($var: $t),+) -> Self {
                Self {
                    $(
                        $var
                    ),+
                }
            }
        }
        impl core::cmp::Ord for $name {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                let mut all = alloc::vec![];
                $(
                    if orderable!(@transform [$($tt)?]) {
                        all.push(self.$var.cmp(&other.$var))
                    }
                )*
                let first = all[0];
                if !all.iter().all(|&item| item == first) {
                    panic!("not matched all items' ordering.");
                }
                first
            }
        }
        impl core::cmp::PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> core::option::Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl core::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                $(self.$var == other.$var)&&*
            }
        }
    };
}
