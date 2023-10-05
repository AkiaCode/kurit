pub const NORMAILZE: &'static str = include_str!("normailze.css");

pub trait Template {
    fn css() -> String;
}

pub struct KuritDefault {}

impl Template for KuritDefault {
    fn css() -> String {
        NORMAILZE.into()
    }
}
