use traits::{CodeMap, Span, File, Loc, SpanLoc, LineCol};
use codemap;

impl File for codemap::File {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        self.name().into()
    }

    fn source_line(&self, line: usize) -> &str {
        self.source_line(line)
    }
}

impl Span for codemap::Span {
    type Pos = codemap::Pos;

    fn low(&self) -> Self::Pos {
        self.low()
    }

    fn high(&self) -> Self::Pos {
        self.high()
    }
}

impl CodeMap for codemap::CodeMap {
    type Span = codemap::Span;
    type File = codemap::File;

    /// Gets the file and its line and column ranges represented by a `Span`.
    fn look_up_span(&self, span: Self::Span) -> SpanLoc<Self> {
        let ext = self.look_up_span(span);
        SpanLoc {
            file: ext.file,
            begin: LineCol {
                line: ext.begin.line,
                column: ext.begin.column,
            },
            end: LineCol {
                line: ext.end.line,
                column: ext.end.column,
            }
        }
    }

    /// Gets the file, line, and column represented by a `Pos`.
    fn look_up_pos(&self, pos: <Self::Span as Span>::Pos) -> Loc<Self> {
        let ext = self.look_up_pos(pos);
        Loc {
            file: ext.file,
            position: LineCol {
                line: ext.position.line,
                column: ext.position.column,
            },
        }
    }
}
