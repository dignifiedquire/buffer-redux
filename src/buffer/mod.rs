#![allow(missing_docs)]

mod std_buf;

use std::mem::MaybeUninit;

use self::std_buf::StdBuf;

pub enum BufImpl {
    Std(StdBuf),
}

macro_rules! forward_method {
    ($(#[$m:meta])*
     pub fn $fnname:ident(&self $($args:tt)*) [$($passargs:tt)*] $(-> $ret:ty)*) => {
        $(#[$m])*
        pub fn $fnname(&self $($args)*) $(-> $ret)* {
            match *self {
                BufImpl::Std(ref buf) => buf.$fnname($($passargs)*),
            }
        }
    };

    ($(#[$m:meta])*
     pub fn $fnname:ident(&mut self $($args:tt)*) [$($passargs:tt)*] $(-> $ret:ty)*) => {
        $(#[$m])*
        pub fn $fnname(&mut self $($args)*) $(-> $ret)* {
            match *self {
                BufImpl::Std(ref mut buf) => buf.$fnname($($passargs)*),
            }
        }
    };

    ($(#[$m:meta])*
     pub unsafe fn $fnname:ident(&self $($args:tt)*) [$($passargs:tt)*] $(-> $ret:ty)*) => {
        $(#[$m])*
        pub unsafe fn $fnname(&self $($args)*) $(-> $ret)* {
            match *self {
                BufImpl::Std(ref buf) => buf.$fnname($($passargs)*),
            }
        }
    };

    ($(#[$m:meta])*
     pub unsafe fn $fnname:ident(&mut self $($args:tt)*) [$($passargs:tt)*] $(-> $ret:ty)*) => {
        $(#[$m])*
        pub unsafe fn $fnname(&mut self $($args)*) $(-> $ret)* {
            match *self {
                BufImpl::Std(ref mut buf) => buf.$fnname($($passargs)*),
            }
        }
    };
}

macro_rules! forward_methods {
    ($($(#[$m:meta])*
     $($qualifiers:ident)+ ($($args:tt)*) [$($passargs:tt)*] $(-> $ret:ty)*);+;) => (
        $(forward_method! {
            $(#[$m])* $($qualifiers)+ ($($args)*) [$($passargs)*] $(-> $ret)*
        })*
    )
}

impl BufImpl {
    pub fn with_capacity(cap: usize) -> Self {
        BufImpl::Std(StdBuf::with_capacity(cap))
    }

    forward_methods! {
        pub fn capacity(&self)[] -> usize;

        pub fn len(&self)[] -> usize;

        pub fn usable_space(&self)[] -> usize;

        pub fn reserve(&mut self, additional: usize)[additional] -> bool;

        pub fn make_room(&mut self)[];

        #[inline]
        pub fn buf(&self)[] -> &[u8];

        pub fn buf_mut(&mut self)[] -> &mut [u8];

        pub unsafe fn write_buf(&mut self)[] -> &mut [MaybeUninit<u8>];

        pub unsafe fn bytes_written(&mut self, add: usize)[add];

        pub fn consume(&mut self, amt: usize)[amt];
    }
}
