pub(crate) type EditorResult<T, E> = Result<T, E>;

pub(crate) enum ResultCode {
    KeyReadFail
}

#[derive(Default, Copy, Clone)]
pub(crate) struct Position {
    pub(crate) x: u16,
    pub(crate) y: u16,
}