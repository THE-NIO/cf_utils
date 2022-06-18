#[macro_export]
#[rustfmt::skip]
macro_rules! define_read {
    ($words:ident) => {
        macro_rules! read {
            () => {
                $words.next().unwrap().parse().unwrap()
            };
            ($t:ty) => {
                $words.next().unwrap().parse::<$t>().unwrap()
            };
            ($t:ty; $n:expr) => {
                read!($t, |a| a; $n)
            };
            ($t:ty, decr; $n: expr) => {
                read!($t, |a| a - 1; $n)
            };
            ($t:ty, $f:expr; $n: expr) => {
                $words
                    .by_ref()
                    .take($n)
                    .map(|s| s.parse::<$t>().unwrap())
                    .map($f)
                    .collect::<Vec<_>>()
            };
        }
    };
}

#[macro_export]
#[rustfmt::skip]
macro_rules! define_out {
    ($stdout:ident, $dollar:tt) => {
        macro_rules! out {
            ($dollar ($dollar arg:tt)*) => {
                ::std::io::Write::write_fmt(&mut $stdout, format_args!($dollar ($dollar arg)*)).unwrap();
            };
        }

        macro_rules! outln {
            () => {
                ::std::io::Write::write_all(&mut $stdout, b"\n").unwrap();
            };
            ($dollar ($dollar arg:tt)*) => {
                ::std::io::Write::write_fmt(&mut $stdout, format_args!($dollar ($dollar arg)*)).unwrap();
                outln!();
            };
        }
    }
}

#[macro_export]
macro_rules! cf_prelude {
    () => {
        let mut input = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();
        let mut words = input.split_whitespace();

        let out = ::std::io::stdout();
        let mut out = ::std::io::BufWriter::new(out.lock());

        define_read!(words);

        define_out!(out, $);
    };
}
