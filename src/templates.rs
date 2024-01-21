use askama::Template;

#[derive(Template)]
#[template(path = "counter.html")]
pub struct CounterTemplate {
    pub counter: usize,
}
