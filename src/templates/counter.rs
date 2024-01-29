use askama::Template;

#[derive(Template)]
#[template(path = "counter.html")]
pub struct Counter {
    pub counter: usize,
}
