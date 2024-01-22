use askama::Template;

#[derive(Template)]
#[template(path = "counter.html")]
pub struct CounterTemplate {
    pub counter: usize,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}
