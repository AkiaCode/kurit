pub trait Template {
    fn new() -> Self
    where
        Self: Sized;
    fn css(&self) -> String;
    fn html(&self, title: String, body: String) -> String;
}

// TODO: Make Template System
pub enum Templates {
    Default,
    Kafu,
}

impl Templates {
    pub fn check(name: String) -> Option<Templates> {
        match name.to_lowercase().as_str() {
            "default" => Some(Templates::Default),
            "kafu" => Some(Templates::Kafu),
            _ => None,
        }
    }

    pub fn to_tmpl(tmpl: Templates) -> Box<dyn Template> {
        match tmpl {
            Templates::Default => Box::new(KuritDefault::new()),
            Templates::Kafu => Box::new(Kafu::new()),
        }
    }
}

pub struct Kafu {}

impl Template for Kafu {
    fn new() -> Self {
        Kafu {}
    }
    fn css(&self) -> String {
        include_str!("deps/normailze.css/normalize.css").into()
    }

    fn html(&self, title: String, body: String) -> String {
        format!(
            "<html>
    <head>
        <style>
{css}
        </style>
        <title>{title}</title>
    </head>
    <body>
        {body}
    </body>
</html>",
            css = self.css()
        )
    }
}

pub struct KuritDefault {}

impl Template for KuritDefault {
    fn new() -> Self {
        KuritDefault {}
    }
    fn css(&self) -> String {
        "".into()
    }
    fn html(&self, title: String, body: String) -> String {
        format!(
            "<html>
    <head>
        <style>
{css}
        </style>
        <title>{title}</title>
    </head>
    <body>
        {body}
    </body>
</html>",
            css = self.css()
        )
    }
}
