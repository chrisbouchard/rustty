use std::ops::{
    Index,
    IndexMut,
    Deref,
    DerefMut,
};

/// An array of `Cell`s that represents a terminal display.
///
/// A `CellBuffer` is a two-dimensional array of `Cell`s, each pair of indices correspond to a
/// single point on the underlying terminal.
///
/// The first index, `Cellbuffer[y]`, corresponds to a row, and thus the y-axis. The second
/// index, `Cellbuffer[y][x]`, corresponds to a column within a row and thus the x-axis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellBuffer {
    cols: usize,
    rows: usize,
    buf: Vec<Cell>,
}

impl CellBuffer {
    /// Constructs a new `CellBuffer` with the given number of columns and rows, using the given
    /// `cell` as a blank.
    pub fn new(cols: usize, rows: usize, cell: Cell) -> CellBuffer {
        CellBuffer {
            cols: cols,
            rows: rows,
            buf: vec![cell; cols * rows],
        }
    }

    /// Clears a `CellBuffer`, using the given `Cell` as a blank.
    pub fn clear(&mut self, blank: Cell) {
        for cell in self.buf.iter_mut() {
            *cell = blank;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.cols && y < self.rows {
            let offset = (self.cols * y) + x;
            self.buf.get(offset)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x < self.cols && y < self.rows {
            let offset = (self.cols * y) + x;
            self.buf.get_mut(offset)
        } else {
            None
        }
    }

    /// Resizes the `CellBuffer` to the given number of rows and columns, using the given `Cell` as
    /// a blank.
    pub fn resize(&mut self, newcols: usize, newrows: usize, blank: Cell) {
        let newlen = newcols * newrows;
        let mut newbuf: Vec<Cell> = Vec::with_capacity(newlen);
        for y in 0..newrows {
            for x in 0..newcols {
                let cell = self.get(x, y).unwrap_or(&blank);
                newbuf.push(*cell);
            }
        }
        self.buf = newbuf;
        self.cols = newcols;
        self.rows = newrows;
    }
}

impl Deref for CellBuffer {
    type Target = [Cell];

    fn deref<'a>(&'a self) -> &'a [Cell] {
        &self.buf
    }
}

impl DerefMut for CellBuffer {
    fn deref_mut<'a>(&'a mut self) -> &'a mut [Cell] {
        &mut self.buf
    }
}

impl Index<(usize, usize)> for CellBuffer {
    type Output = Cell;

    fn index<'a>(&'a self, index: (usize, usize)) -> &'a Cell {
        let (x, y) = index;
        self.get(x, y).expect("index out of bounds")
    }
}

impl IndexMut<(usize, usize)> for CellBuffer {
    fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut Cell {
        let (x, y) = index;
        self.get_mut(x, y).expect("index out of bounds")
    }
}

impl Default for CellBuffer {
    /// Constructs a new `CellBuffer` with a size of `(0, 0)`, using the default `Cell` as a blank.
    fn default() -> CellBuffer {
        CellBuffer::new(0, 0, Cell::default())
    }
}

/// A single point on a terminal display.
///
/// A `Cell` contains a character and a set of foreground and background `Style`s.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cell {
    ch: char,
    fg: Style,
    bg: Style,
}

impl Cell {
    /// Creates a new `Cell` with the given `char` and `Style`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Style, Color};
    ///
    /// let mut cell = Cell::new('x', Style::default(), Style::with_color(Color::Green));
    /// assert_eq!(cell.ch(), 'x');
    /// assert_eq!(cell.fg(), Style::default());
    /// assert_eq!(cell.bg(), Style::with_color(Color::Green));
    /// ```
    pub fn new(ch: char, fg: Style, bg: Style) -> Cell {
        Cell {
            ch: ch,
            fg: fg,
            bg: bg,
        }
    }

    /// Creates a new `Cell` with the given `char` and default `Style`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Style};
    ///
    /// let mut cell = Cell::with_char('x');
    /// assert_eq!(cell.ch(), 'x');
    /// assert_eq!(cell.fg(), Style::default());
    /// assert_eq!(cell.bg(), Style::default());
    /// ```
    pub fn with_char(ch: char) -> Cell {
        Cell::new(ch, Style::default(), Style::default())
    }

    /// Creates a new `Cell` with the given `Style`s and a blank `char`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Style, Color};
    ///
    /// let mut cell = Cell::with_styles(Style::default(), Style::with_color(Color::Red));
    /// assert_eq!(cell.fg(), Style::default());
    /// assert_eq!(cell.bg(), Style::with_color(Color::Red));
    /// assert_eq!(cell.ch(), ' ');
    /// ```
    pub fn with_styles(fg: Style, bg: Style) -> Cell {
        Cell::new(' ', fg, bg)
    }

    /// Returns the `Cell`'s character.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::Cell;
    ///
    /// let mut cell = Cell::with_char('x');
    /// assert_eq!(cell.ch(), 'x');
    /// ```
    pub fn ch(&self) -> char {
        self.ch
    }

    /// Sets the `Cell`'s character to the given `char`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::Cell;
    ///
    /// let mut cell = Cell::with_char('x');
    /// assert_eq!(cell.ch(), 'x');
    ///
    /// cell.set_ch('y');
    /// assert_eq!(cell.ch(), 'y');
    /// ```
    pub fn set_ch(&mut self, newch: char) -> &mut Cell {
        self.ch = newch;
        self
    }

    /// Returns the `Cell`'s foreground `Style`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Style, Attr};
    ///
    /// let mut cell = Cell::with_styles(Style::with_attr(Attr::Bold), Style::default());
    /// assert_eq!(cell.fg(), Style::with_attr(Attr::Bold));
    /// ```
    pub fn fg(&self) -> Style {
        self.fg
    }

    /// Returns a mutable reference to the `Cell`'s foreground `Style`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Style};
    ///
    /// let mut cell = Cell::default();
    /// assert_eq!(cell.fg_mut(), &mut Style::default());
    /// ```
    pub fn fg_mut(&mut self) -> &mut Style {
        &mut self.fg
    }

    /// Sets the `Cell`'s foreground `Style` to the given `Style`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Style, Color, Attr};
    ///
    /// let mut cell = Cell::with_styles(Style::with_color(Color::Green), Style::default());
    /// assert_eq!(cell.fg(), Style::with_color(Color::Green));
    ///
    /// cell.set_fg(Style::with_attr(Attr::Underline));
    /// assert_eq!(cell.fg(), Style::with_attr(Attr::Underline));
    /// ```
    pub fn set_fg(&mut self, newfg: Style) -> &mut Cell {
        self.fg = newfg;
        self
    }

    /// Returns the `Cell`'s background `Style`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Style, Attr};
    ///
    /// let mut cell = Cell::with_styles(Style::default(), Style::with_attr(Attr::Bold));
    /// assert_eq!(cell.bg(), Style::with_attr(Attr::Bold));
    /// ```
    pub fn bg(&self) -> Style {
        self.bg
    }

    /// Returns a mutable reference to the `Cell`'s background `Style`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Style};
    ///
    /// let mut cell = Cell::default();
    /// assert_eq!(cell.bg_mut(), &mut Style::default());
    /// ```
    pub fn bg_mut(&mut self) -> &mut Style {
        &mut self.bg
    }

    /// Sets the `Cell`'s background `Style` to the given `Style`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Style, Color, Attr};
    ///
    /// let mut cell = Cell::with_styles(Style::default(), Style::with_color(Color::Green));
    /// assert_eq!(cell.bg(), Style::with_color(Color::Green));
    ///
    /// cell.set_bg(Style::with_attr(Attr::Underline));
    /// assert_eq!(cell.bg(), Style::with_attr(Attr::Underline));
    /// ```
    pub fn set_bg(&mut self, newbg: Style) -> &mut Cell {
        self.bg = newbg;
        self
    }
}

impl Default for Cell {
    /// Constructs a new `Cell` with a blank `char` and default `Style`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Style};
    ///
    /// let mut cell = Cell::default();
    /// assert_eq!(cell.ch(), ' ');
    /// assert_eq!(cell.fg(), Style::default());
    /// assert_eq!(cell.bg(), Style::default());
    /// ```
    fn default() -> Cell {
        Cell::new(' ', Style::default(), Style::default())
    }
}

/// The style of a `Cell`.
///
/// A `Style` has a `Color` and an `Attr` and represents either the foreground or background
/// styling of a `Cell`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Style(Color, Attr);

impl Style {
    /// Constructs a new `Style` with the given `Color` and `Attr`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Style, Color, Attr};
    ///
    /// let mut style = Style::new(Color::Green, Attr::BoldUnderline);
    /// assert_eq!(style.color(), Color::Green);
    /// assert_eq!(style.attr(), Attr::BoldUnderline);
    /// ```
    pub fn new(color: Color, attr: Attr) -> Style {
        Style(color, attr)
    }

    /// Constructs a new `Style` with the given `Color` and the default `Attr`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Style, Color, Attr};
    ///
    /// let mut style = Style::with_color(Color::Cyan);
    /// assert_eq!(style.color(), Color::Cyan);
    /// assert_eq!(style.attr(), Attr::Default);
    /// ```
    pub fn with_color(c: Color) -> Style {
        Style::new(c, Attr::Default)
    }

    /// Constructs a new `Style` with the given `Attr` and the default `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Style, Color, Attr};
    ///
    /// let mut style = Style::with_attr(Attr::UnderlineReverse);
    /// assert_eq!(style.attr(), Attr::UnderlineReverse);
    /// assert_eq!(style.color(), Color::Default);
    /// ```
    pub fn with_attr(a: Attr) -> Style {
        Style::new(Color::Default, a)
    }

    /// Returns the `Style`'s `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Style, Color};
    ///
    /// let mut style = Style::with_color(Color::Yellow);
    /// assert_eq!(style.color(), Color::Yellow);
    /// ```
    pub fn color(&self) -> Color {
        self.0
    }

    /// Sets the `Style`'s `Color` to the given `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Style, Color};
    ///
    /// let mut style = Style::with_color(Color::White);
    /// assert_eq!(style.color(), Color::White);
    ///
    /// style.set_color(Color::Black);
    /// assert_eq!(style.color(), Color::Black);
    /// ```
    pub fn set_color(&mut self, newcolor: Color) -> &mut Style {
        self.0 = newcolor;
        self
    }

    /// Returns the `Style`'s `Attr`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Style, Attr};
    ///
    /// let mut style = Style::with_attr(Attr::Reverse);
    /// assert_eq!(style.attr(), Attr::Reverse);
    /// ```
    pub fn attr(&self) -> Attr {
        self.1
    }

    /// Sets the given `Style`'s `Attr` to the given `Attr`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Style, Attr};
    ///
    /// let mut style = Style::with_attr(Attr::BoldReverse);
    /// assert_eq!(style.attr(), Attr::BoldReverse);
    ///
    /// style.set_attr(Attr::Underline);
    /// assert_eq!(style.attr(), Attr::Underline);
    /// ```
    pub fn set_attr(&mut self, newattr: Attr) -> &mut Style {
        self.1 = newattr;
        self
    }
}

impl Default for Style {
    /// Constructs a new `Style` with the default `Color` and `Attr`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Style, Color, Attr};
    ///
    /// let mut style = Style::default();
    /// assert_eq!(style.color(), Color::Default);
    /// assert_eq!(style.attr(), Attr::Default);
    /// ```
    fn default() -> Style {
        Style::new(Color::Default, Attr::Default)
    }
}

/// The color of a `Cell`.
///
/// `Color::Default` represents the default color of the underlying terminal and may be used to
/// reset a `Style`'s `Color`.
///
/// The eight basic colors may be used directly and correspond to 0x00..0x07 in the 8-bit (256)
/// color range; in addition, the eight basic colors coupled with `Attr::Bold` correspond to
/// 0x08..0x0f in the 8-bit color range.
///
/// `Color::Byte(..)` may be used to specify a color in the 8-bit range.
///
/// # Examples
///
/// ```
/// use rustty::Color;
///
/// // The default color.
/// let default = Color::Default;
///
/// // A basic color.
/// let red = Color::Red;
///
/// // An 8-bit color.
/// let fancy = Color::Byte(0x01);
///
/// // Basic colors are also 8-bit colors (but not vice-versa).
/// assert_eq!(red.as_byte(), fancy.as_byte())
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Byte(u8),
    Default,
}

impl Color {
    /// Returns the `u8` representation of the `Color`.
    pub fn as_byte(&self) -> u8 {
        match *self {
            Color::Black => { 0x00 },
            Color::Red => { 0x01 },
            Color::Green => { 0x02 },
            Color::Yellow => { 0x03 },
            Color::Blue => { 0x04 },
            Color::Magenta => { 0x05 },
            Color::Cyan => { 0x06 },
            Color::White => { 0x07 },
            Color::Byte(b) => { b },
            Color::Default => { panic!("Attempted to cast default color to u8") },
        }
    }
}

/// The attributes of a `Cell`.
///
/// `Attr` enumerates all combinations of attributes a given `Style` may have.
///
/// `Attr::Default` represents no attribute and may be used to reset a `Style`'s `Attr`.
///
/// # Examples
///
/// ```
/// use rustty::Attr;
///
/// // Default attribute.
/// let def = Attr::Default;
///
/// // Base attribute.
/// let base = Attr::Bold;
///
/// // Combination.
/// let comb = Attr::UnderlineReverse;
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Attr {
    Default = 0b000,
    Bold = 0b001,
    Underline = 0b010,
    BoldUnderline = 0b011,
    Reverse = 0b100,
    BoldReverse = 0b101,
    UnderlineReverse = 0b110,
    BoldReverseUnderline = 0b111,
}

