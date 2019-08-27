use std::sync::Arc;
use std::fmt::Debug;

#[derive(PartialEq, Copy, Clone)]
pub struct LineCol {
    pub line: usize,
    pub column: usize
}

pub struct SpanLoc<C: CodeMap> {
    pub file: Arc<C::File>,
    pub begin: LineCol,
    pub end: LineCol,
}

impl<C: CodeMap> Clone for SpanLoc<C> {
    fn clone(&self) -> Self {
        SpanLoc {
            file: self.file.clone(),
            begin: self.begin.clone(),
            end: self.end.clone(),
        }
    }
}

pub struct Loc<C: CodeMap> {
    pub file: Arc<C::File>,
    pub position: LineCol,
}

impl<C: CodeMap> Clone for Loc<C> {
    fn clone(&self) -> Self {
        Loc { file: self.file.clone(), position: self.position.clone() }
    }
}

pub trait File {
    fn name(&self) -> std::borrow::Cow<'_, str>;

    /// Gets the source text of a line.
    ///
    /// The string returned does not include the terminating \r or \n characters.
    ///
    /// # Panics
    ///
    ///  * If the line number is out of range
    fn source_line(&self, line: usize) -> &str;
}

pub trait Span: Copy + Debug {
    type Pos: Copy + PartialEq + PartialOrd;

    fn low(&self) -> Self::Pos;
    fn high(&self) -> Self::Pos;
}

pub trait CodeMap: Sized {
    type Span: Span;
    type File: File;

    /// Gets the file and its line and column ranges represented by a `Span`.
    fn look_up_span(&self, span: Self::Span) -> SpanLoc<Self>;

    /// Gets the file, line, and column represented by a `Pos`.
    fn look_up_pos(&self, pos: <Self::Span as Span>::Pos) -> Loc<Self>;
}
