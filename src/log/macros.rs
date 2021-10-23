#[allow(unused_macros)]
macro_rules! is_trace_enabled {
    () => {{
        match $crate::log::logger() {
            Some(logger) => logger.is_module_verbose(module_path!()),
            None => false,
        }
    }};
}

#[cfg(debug_assertions)]
#[allow(unused_macros)]
macro_rules! is_debug_enabled {
    () => {{
        match $crate::log::logger() {
            Some(logger) => logger.is_debug(),
            None => false,
        }
    }};
}

#[allow(unused_macros)]
macro_rules! info {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        print!("{}", tree_decorator::tree_item!($first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* , $str $( $arg )*))
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        print!("{}", tree_decorator::tree_item!($first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*))
    };

    ($str:literal $($arg:tt)*) => {
        print!("{}", tree_decorator::tree_item!($str $( $arg )*))
    };

    () => {
        print!("{}", tree_decorator::tree_item!())
    };
}

#[allow(unused_macros)]
macro_rules! infoln {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        println!("{}", tree_decorator::tree_item!($first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* , $str $( $arg )*))
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        println!("{}", tree_decorator::tree_item!($first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*))
    };

    ($str:literal $($arg:tt)*) => {
        println!("{}", tree_decorator::tree_item!($str $( $arg )*))
    };

    () => {
        println!("{}", tree_decorator::tree_item!())
    };
}

#[allow(unused_macros)]
macro_rules! warn {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        print!("{}", tree_decorator::tree_item!($first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* , $str $( $arg )*))
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        print!("{}", tree_decorator::tree_item!($first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*))
    };

    ($str:literal $($arg:tt)*) => {
        print!("{}", tree_decorator::tree_item!($str $( $arg )*))
    };

    () => {
        print!("{}", tree_decorator::tree_item!())
    };
}

#[allow(unused_macros)]
macro_rules! warnln {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        println!("{}", tree_decorator::tree_item!($first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* , $str $( $arg )*))
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        println!("{}", tree_decorator::tree_item!($first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*))
    };

    ($str:literal $($arg:tt)*) => {
        println!("{}", tree_decorator::tree_item!($str $( $arg )*))
    };

    () => {
        println!("{}", tree_decorator::tree_item!())
    };
}

#[allow(unused_macros)]
macro_rules! error {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        print!("{}", tree_decorator::tree_item!($first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* , $str $( $arg )*))
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        print!("{}", tree_decorator::tree_item!($first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*))
    };

    ($str:literal $($arg:tt)*) => {
        print!("{}", tree_decorator::tree_item!($str $( $arg )*))
    };

    () => {
        print!("{}", tree_decorator::tree_item!())
    };
}

#[allow(unused_macros)]
macro_rules! errorln {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        println!("{}", tree_decorator::tree_item!($first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* , $str $( $arg )*))
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        println!("{}", tree_decorator::tree_item!($first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*))
    };

    ($str:literal $($arg:tt)*) => {
        println!("{}", tree_decorator::tree_item!($str $( $arg )*))
    };

    () => {
        println!("{}", tree_decorator::tree_item!())
    };
}

#[cfg(debug_assertions)]
#[allow(unused_macros)]
macro_rules! debug {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        if is_debug_enabled!() {
            info!($first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* , $str $( $arg )*);
        }
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        if is_debug_enabled!() {
            info!($first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*);
        }
    };

    ($str:literal $($arg:tt)*) => {
        if is_debug_enabled!() {
            info!($str $( $arg )*);
        }
    };

    () => {
        if is_debug_enabled!() {
            info!();
        }
    };
}

#[cfg(debug_assertions)]
#[allow(unused_macros)]
macro_rules! debugln {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        if is_debug_enabled!() {
            infoln!($first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* , $str $( $arg )*);
        }
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        if is_debug_enabled!() {
            infoln!($first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*);
        }
    };

    ($str:literal $($arg:tt)*) => {
        if is_debug_enabled!() {
            infoln!($str $( $arg )*);
        }
    };

    () => {
        if is_debug_enabled!() {
            infoln!();
        }
    };
}

#[cfg(debug_assertions)]
#[allow(unused_macros)]
macro_rules! trace {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {{
        if is_trace_enabled!() {
            info!(dashed; $first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* , $str $( $arg )*);
        }
    }};

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        if is_trace_enabled!() {
            info!(dashed; $first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*);
        }
    };

    ($str:literal $($arg:tt)*) => {
        if is_trace_enabled!() {
            info!(dashed, $str $( $arg )*);
        }
    };

    () => {
        if is_trace_enabled!() {
            info!();
        }
    };
}

#[cfg(debug_assertions)]
#[allow(unused_macros)]
macro_rules! traceln {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        if is_trace_enabled!() {
            infoln!(dashed; $first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* , $str $( $arg )*);
        }
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        if is_trace_enabled!() {
            infoln!(dashed; $first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*);
        }
    };

    ($str:literal $($arg:tt)*) => {
        if is_trace_enabled!() {
            infoln!(dashed, $str $( $arg )*);
        }
    };

    () => {
        if is_trace_enabled!() {
            infoln!();
        }
    };
}
