#[macro_use]
pub mod parsing {

    macro_rules! conv {
        ($i:expr, $t:tt :: $t2:tt($($e2:tt)*)) => (do_parse!($i, r: $($e2)* >> ($t :: $t2(r))));
        ($i:expr, $t:tt($($e2:tt)*)) => (do_parse!($i, r: $($e2)* >> ($t(r))));
    }

    macro_rules! keyword {
        ($i:expr, $e:expr) => (keyword($i, $e))
    }

    use nom::{IResult, Needed};

    pub fn box_opt<T>(i: Option<T>) -> Option<Box<T>> {
        match i {
            Some(v) => Some(Box::new(v)),
            None => None,
        }
    }

    pub fn is_whitespace(c: char) -> bool {
        c.is_whitespace()
    }

    pub fn is_whitespace_not_line_terminator(c: char) -> bool {
        match c {
            '\r' | '\n' | '\u{2028}' | '\u{2029}' => false,
            other => is_whitespace(other),
        }
    }

    pub fn keyword<'a>(i: &'a str, kw: &str) -> IResult<&'a str, &'a str> {
        do_parse!(i,
            take_while_s!(is_whitespace) >>
            ret: tag_s!(kw) >>
            (ret)
        )
    }

    pub fn anychar_s(input:&str) -> IResult<&str, char> {
        if input.is_empty() {
            IResult::Incomplete(Needed::Size(1))
        } else {
            IResult::Done(&input[1..], input.chars().nth(0).unwrap())
        }
    }

    named!(pub xid_start<&str, char>, verify!(
        anychar_s,
        |val:char| val.is_xid_start()
    ));

    named!(pub xid_continue<&str, char>, verify!(
        anychar_s,
        |val:char| val.is_xid_continue()
    ));

}