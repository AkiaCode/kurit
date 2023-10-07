pub trait Template {
    fn css() -> String;
    fn html(title: String, body: String) -> String;
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
