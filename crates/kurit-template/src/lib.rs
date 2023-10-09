pub trait Template {
    fn css() -> String;
    fn html(title: String, body: String) -> String;
}


// TODO: Make Template System
pub enum Templates {
    Default,
    Kafu
}

impl Templates {
    pub fn check(name: String) -> Option<Templates> {
        match name.to_lowercase().as_str() {
            "default" => Some(Templates::Default),
            "kafu" => Some(Templates::Kafu),
            _ => None
        }
    }

    pub fn to_tmpl(tmpl: Templates) -> impl Template {
        match tmpl {
            Templates::Default => KuritDefault {},
            Templates::Kafu => todo!(),
        }
    }
}

pub struct KuritDefault {}

impl Template for KuritDefault {
    fn css() -> String {
        include_str!("deps/normailze.css/normalize.css").into()
    }
    fn html(title: String, body: String) -> String {
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
            css = KuritDefault::css()
        )
    }
}
