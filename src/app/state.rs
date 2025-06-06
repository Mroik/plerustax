pub enum State {
    Timeline(Timeline, usize),
}

pub enum Timeline {
    Home,
    Public,
    Known,
}
