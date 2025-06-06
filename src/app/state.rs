#[derive(Clone)]
pub enum State {
    Timeline(Timeline, usize),
}

#[derive(Clone)]
pub enum Timeline {
    Home,
    Local,
    Public,
}
