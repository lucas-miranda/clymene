macro_rules! doneln {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        println!(
            "{}",
            tree_decorator::tree_item!(
                last;
                $first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* ,
                "{} {}",
                "Done".green(),
                format!($str $( $arg )*),
            ),
        );
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        println!(
            "{}",
            tree_decorator::tree_item!(
                last;
                $first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*,
                "{}",
                "Done".green(),
            ),
        );
    };

    ($str:literal $($arg:tt)*) => {
        println!(
            "{}",
            tree_decorator::tree_item!(
                last,
                "{} {}",
                "Done".green(),
                format!($str $( $arg )*),
            ),
        );
    };

    () => {
        println!(
            "{}",
            tree_decorator::tree_item!(
                last,
                "{}",
                "Done".green(),
            ),
        );
    };
}

macro_rules! doneln_with_timer {
    ($timer:expr) => {
        doneln!(" {}", format!("{}s", $timer.end_secs_str()).bright_black());
    };
}
