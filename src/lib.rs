#[macro_export]
macro_rules! _build_rest {
    ($n:expr; $words:ident; $($accum:tt)*) => {
        $words
            .by_ref()
            .take($n)
            $($accum)*
            .collect::<Vec<_>>()
    }
}

#[macro_export]
macro_rules! _parse_rest {
    (decr; $n:expr; $words:ident; $($accum:tt)*) => {
        $crate::_build_rest($n; $words; $($accum)* .map(|x| x - 1))
    };
    (decr, $($rest:tt)*) => {
        $crate::_parse_rest!($($rest)* .map(|x| x - 1))
    };
    ($filter:expr; $n:expr; $words:ident; $($accum:tt)*) => {
        $crate::_build_rest!($n; $words; $($accum)* .map($filter))
    };
    ($filter:expr, $($rest:tt)*) => {
        $crate::_parse_rest!($($rest)* .map($filter))
    }
}

#[macro_export]
macro_rules! _define_read {
    ($words:ident, $dollar:tt) => {
        #[allow(unused_macros)]
        macro_rules! read {
            () => {
                $words.next().unwrap().parse().unwrap()
            };
            ($t:ty) => {
                $words.next().unwrap().parse::<$t>().unwrap()
            };
            ($t:ty; $n:expr) => {
                $crate::_build_rest!($n; $words; .map(|s| s.parse::<$t>().unwrap()))
            };
            ($t:ty, $dollar ($dollar rest:tt)*) => {
                $crate::_parse_rest!($dollar ($dollar rest)*; $words; .map(|s| s.parse::<$t>().unwrap()))
            }
        }
    };
}

#[macro_export]
macro_rules! _define_out {
    ($stdout:ident, $dollar:tt) => {
        #[allow(unused_macros)]
        macro_rules! out {
            ($dollar ($dollar arg:tt)*) => {{
                ::std::io::Write::write_fmt(&mut $stdout, format_args!($dollar ($dollar arg)*)).unwrap();
            }};
        }

        #[allow(unused_macros)]
        macro_rules! outln {
            () => {{
                ::std::io::Write::write_all(&mut $stdout, b"\n").unwrap();
            }};
            ($dollar ($dollar arg:tt)*) => {{
                ::std::io::Write::write_fmt(&mut $stdout, format_args!($dollar ($dollar arg)*)).unwrap();
                outln!();
            }};
        }
    }
}

#[macro_export]
macro_rules! _cf_prelude {
    () => {
        let mut input = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();
        let mut words = input.split_whitespace();

        let out = ::std::io::stdout();
        let mut out = ::std::io::BufWriter::new(out.lock());

        $crate::define_read!(words, $);

        $crate::define_out!(out, $);
    };
}

pub use {
    crate::_cf_prelude as cf_prelude,
    crate::_define_out as define_out,
    crate::_define_read as define_read,
};
