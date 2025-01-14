use crate::input_file::InputFile;

/// An "anchored" element is one that knows which input file it is in.
/// Eventually, the plan is to have it also know its *offset* within
/// that file (often stored relative to some other anchored thing).
pub trait Anchored {
    fn input_file(&self) -> InputFile;
}

impl<A: ?Sized> Anchored for &A
where
    A: Anchored,
{
    fn input_file(&self) -> InputFile {
        A::input_file(&self)
    }
}

#[derive(Debug, PartialEq)]
pub struct FileSpan {
    pub input_file: InputFile,
    pub start: Offset,
    pub end: Offset,
}

impl FileSpan {
    // self não possui lifetime no original
    pub fn snippet<'a>(&'a self) -> &'a str {
        &self.input_file.source_text[usize::from(self.start.clone())..usize::from(self.end.clone())]
    }

    /// True if the given character falls within this span.
    pub fn contains(&self, offset: Offset) -> bool {
        self.start <= offset && offset < self.end
    }
}

#[derive(PartialEq)]
pub struct Span {
    pub start: Offset,
    pub end: Offset,
}

impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}..{:?})", self.start.0, self.end.0)
    }
}

/// 0-based byte offset within a file.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Offset(u32);

#[derive(Debug, PartialEq, Clone)]
pub struct LineColumn {
    /// 0-based line number
    line0: u32,

    /// 0-based column number
    column0: u32,
}

impl LineColumn {
    /// Create from 1-based line/column numbers
    pub fn new1(line1: u32, column1: u32) -> Self {
        assert!(line1 > 0);
        assert!(column1 > 0);
        Self::new0(line1 - 1, column1 - 1)
    }

    /// Create from 0-based line/column numbers
    pub fn new0(line0: impl U32OrUsize, column0: impl U32OrUsize) -> Self {
        Self {
            line0: line0.into_u32(),
            column0: column0.into_u32(),
        }
    }

    pub fn line0(&self) -> u32 {
        self.line0
    }

    pub fn line1(&self) -> u32 {
        self.line0 + 1
    }

    pub fn line0_usize(&self) -> usize {
        usize::try_from(self.line0).unwrap()
    }

    pub fn column0(&self) -> u32 {
        self.column0
    }

    pub fn column1(&self) -> u32 {
        self.column0 + 1
    }
}

impl From<FileSpan> for Span {
    fn from(fs: FileSpan) -> Span {
        Span {
            start: fs.start,
            end: fs.end,
        }
    }
}

impl Span {
    #[track_caller]
    pub fn from(start: impl Into<Offset>, end: impl Into<Offset>) -> Self {
        let this = Self {
            start: start.into(),
            end: end.into(),
        };
        assert!(this.start <= this.end);
        this
    }

    // self não possui reference no original
    pub fn anchor_to(&self, anchored: impl Anchored) -> FileSpan {
        let input_file = anchored.input_file();
        FileSpan {
            input_file,
            start: self.start.clone(),
            end: self.end.clone(),
        }
    }

    // RESOLVE
    pub fn snippet<'a>(&self, anchored: impl Anchored) -> &'a str {
        /*let fs = self.anchor_to(anchored);
        let s = fs.snippet();
        s*/
        self
            .anchor_to(anchored)
            .snippet()
    }

    /// Returns a 0-length span at the start of this span
    #[must_use]
    pub fn span_at_start(&self) -> Span {
        Span {
            start: self.start.clone(),
            end: self.start.clone(),
        }
    }

    pub fn zero() -> Self {
        Span {
            start: Offset(0),
            end: Offset(0),
        }
    }

    pub fn len(self) -> u32 {
        self.end - self.start
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn to(self, other: Span) -> Span {
        assert!(self.start <= other.start && other.end >= self.end);
        Span {
            start: self.start,
            end: other.end,
        }
    }
}

impl std::ops::Add<u32> for Offset {
    type Output = Offset;

    fn add(self, other: u32) -> Offset {
        Offset(self.0 + other)
    }
}

impl std::ops::Add<usize> for Offset {
    type Output = Offset;

    fn add(self, other: usize) -> Offset {
        assert!(other < std::u32::MAX as usize);
        self + (other as u32)
    }
}

impl std::ops::Sub<Offset> for Offset {
    type Output = u32;

    fn sub(self, other: Offset) -> u32 {
        self.0 - other.0
    }
}

impl From<usize> for Offset {
    fn from(value: usize) -> Offset {
        assert!(value < std::u32::MAX as usize);
        Offset(value as u32)
    }
}

impl From<u32> for Offset {
    fn from(value: u32) -> Offset {
        Offset(value)
    }
}

impl From<Offset> for u32 {
    fn from(offset: Offset) -> Self {
        offset.0
    }
}

impl From<Offset> for usize {
    fn from(offset: Offset) -> Self {
        offset.0 as usize
    }
}

pub trait U32OrUsize {
    fn into_u32(self) -> u32;
    fn from_u32(n: u32) -> Self;
    fn into_usize(self) -> usize;
    fn from_usize(n: usize) -> Self;
}

impl U32OrUsize for u32 {
    fn into_u32(self) -> u32 {
        self
    }

    fn from_u32(n: u32) -> Self {
        n
    }

    fn into_usize(self) -> usize {
        usize::try_from(self).unwrap()
    }

    fn from_usize(n: usize) -> Self {
        u32::try_from(n).unwrap()
    }
}

impl U32OrUsize for usize {
    fn into_u32(self) -> u32 {
        u32::try_from(self).unwrap()
    }

    fn from_u32(n: u32) -> Self {
        usize::try_from(n).unwrap()
    }

    fn into_usize(self) -> usize {
        self
    }

    fn from_usize(n: usize) -> Self {
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test LineColumn
    #[test]
    fn test_linecolumn_new0() {
        assert_eq!(
            LineColumn {
                line0: 4,
                column0: 3
            },
            LineColumn::new0(4u32, 3u32)
        )
    }

    #[test]
    fn test_linecolumn_new1() {
        assert_eq!(
            LineColumn {
                line0: 3,
                column0: 2
            },
            LineColumn::new1(4, 3)
        )
    }

    #[test]
    fn test_linecolumn_line_column() {
        let lc = LineColumn::new0(4u32, 3u32);
        assert_eq!(4, lc.line0());
        assert_eq!(5, lc.line1());
        assert_eq!(3, lc.column0());
        assert_eq!(4, lc.column1());
        assert_eq!(4, lc.line0_usize());
    }

    // Test Offset as Add<u32>
    #[test]
    fn test_offset_as_add_u32_add() {
        assert_eq!(Offset(5), Offset(3) + 2u32)
    }

    // Test Offset as Add<usize>
    #[test]
    fn test_offset_as_add_usize_add() {
        assert_eq!(Offset(5), Offset(3) + 2usize)
    }

    // Test Offset as Sub<Offset>
    #[test]
    fn test_offset_as_sub_offset_sub() {
        assert_eq!(3, Offset(7) - Offset(4))
    }

    // Test Offset as From<usize>
    #[test]
    fn test_offset_as_from_usize_from() {
        assert_eq!(Offset(7), <Offset as From<usize>>::from(7))
    }

    // Test Offset as From<u32>
    #[test]
    fn test_offset_as_from_u32_from() {
        assert_eq!(Offset(8), <Offset as From<u32>>::from(8))
    }

    // Test u32 as From<Offset>
    #[test]
    fn test_u32_as_from_offset_from() {
        assert_eq!(4, u32::from(Offset(4)))
    }

    // Test usize as From<Offset>
    #[test]
    fn test_usize_as_from_offset_from() {
        assert_eq!(7, usize::from(Offset(7)))
    }

    // Test u32 as U32OrUsize
    #[test]
    fn test_u32_as_u32orusize_into_u32() {
        assert_eq!(7, 7u32.into_u32())
    }

    #[test]
    fn test_u32_as_u32orusize_from_u32() {
        assert_eq!(8, u32::from_u32(8))
    }

    #[test]
    fn test_u32_as_u32orusize_into_usize() {
        assert_eq!(9, 9u32.into_usize())
    }

    #[test]
    fn test_u32_as_u32orusize_from_usize() {
        assert_eq!(6, u32::from_usize(6))
    }

    // Test usize as U32OrUsize
    #[test]
    fn test_usize_as_u32orusize_into_u32() {
        assert_eq!(7, 7usize.into_u32())
    }

    #[test]
    fn test_usize_as_u32orusize_from_u32() {
        assert_eq!(8, usize::from_u32(8))
    }

    #[test]
    fn test_usize_as_u32orusize_into_usize() {
        assert_eq!(9, 9usize.into_usize())
    }

    #[test]
    fn test_usize_as_u32orusize_from_usize() {
        assert_eq!(6, usize::from_usize(6))
    }

    // Test Span
    #[test]
    fn test_span_from() {
        assert_eq!(
            Span {
                start: Offset(7),
                end: Offset(8)
            },
            Span::from(Offset(7), Offset(8))
        )
    }

    #[test]
    fn test_span_anchor_to() {
        let s = Span {
            start: Offset(7),
            end: Offset(8),
        };

        let w = crate::word::Word {
            string: "test.dada".to_owned(),
        };
        let input_file = InputFile {
            name: w,
            source_text: "fn main() {\n print(\"hello world\").await\n}".to_owned(),
            breakpoint_locations: vec![LineColumn::new1(4, 3)],
        };
        assert_eq!(
            FileSpan {
                input_file: input_file.clone(),
                start: Offset(7),
                end: Offset(8)
            },
            s.anchor_to(&input_file)
        )
    }

    #[test]
    fn test_span_snippet() {
        let s = Span {
            start: Offset(7),
            end: Offset(8),
        };

        let w = crate::word::Word {
            string: "test.dada".to_owned(),
        };
        let input_file = InputFile {
            name: w,
            source_text: "fn main() {\n print(\"hello world\").await\n}".to_owned(),
            breakpoint_locations: vec![LineColumn::new1(4, 3)],
        };
        println!("{:?}", s.snippet(&input_file))
    }

    #[test]
    fn test_span_zero() {
        assert_eq!(
            Span {
                start: Offset(0),
                end: Offset(0)
            },
            Span::zero()
        )
    }

    #[test]
    fn test_span_len() {
        let s = Span::from(Offset(14), Offset(32));

        assert_eq!(18, s.len())
    }

    #[test]
    fn test_span_is_empty() {
        let s = Span::from(Offset(54), Offset(54));

        assert_eq!(true, s.is_empty())
    }

    #[test]
    fn test_span_span_at_start() {
        let s = Span::from(Offset(3), Offset(4));

        assert_eq!(
            Span {
                start: Offset(3),
                end: Offset(3)
            },
            s.span_at_start()
        )
    }

    #[test]
    fn test_span_to() {
        let s = Span::from(Offset(3), Offset(8));

        assert_eq!(
            Span {
                start: Offset(3),
                end: Offset(9)
            },
            s.to(Span::from(Offset(4), Offset(9)))
        )
    }

    // Test FileSpan
    #[test]
    fn test_filespan_snippet_contains() {
        let w = crate::word::Word {
            string: "test.dada".to_owned(),
        };
        let input_file = InputFile {
            name: w,
            source_text: "fn main() {\n print(\"hello world\").await\n}".to_owned(),
            breakpoint_locations: vec![LineColumn::new1(4, 3)],
        };
        let fs = FileSpan {
            input_file,
            start: Offset(3),
            end: Offset(11),
        };
        assert_eq!("main() {", fs.snippet());
        assert!(fs.contains(Offset(5)))
    }

    // Test Span as From<FileSpan>
    #[test]
    fn test_span_as_from_filespan_from() {
        let w = crate::word::Word {
            string: "test.dada".to_owned(),
        };
        let input_file = InputFile {
            name: w,
            source_text: "fn main() {\n print(\"hello world\").await\n}".to_owned(),
            breakpoint_locations: vec![LineColumn::new1(4, 3)],
        };
        let fs = FileSpan {
            input_file,
            start: Offset(3),
            end: Offset(11),
        };
        assert_eq!(
            Span::from(Offset(3), Offset(11)),
            <Span as From<FileSpan>>::from(fs)
        )
    }
}
