
/// Represents a character escape code in a type-safe manner.
pub enum CharEscape{
  /// An escaped quote `"`
  Quote,
  /// An escaped reverse solidus `\`
  ReverseSolidus,
  /// An escaped solidus `/`
  Solidus,
  /// An escaped backspace character (usually escaped as `\b`)
  Backspace,
  /// An escaped form feed character (usually escaped as `\f`)
  FormFeed,
  /// An escaped line feed character (usually escaped as `\n`)
  LineFeed,
  /// An escaped carriage return character (usually escaped as `\r`)
  CarriageReturn,
  /// An escaped tab character (usually escaped as `\t`)
  Tab,
  /// An escaped ASCII plane control character (usually escaped as
  /// `\u00XX` where `XX` are two hex characters)
  AsciiControl(u8),
}

/// This trait abstracts away serializing the JSON control characters, which allows the user to
/// optionally pretty print the JSON output.
pub trait Formatter{
  /// Writes a `null` value.
  #[inline]
  fn write_null(&mut self, output: &mut Vec<u8>){
    output.extend_from_slice(b"null");
  }

  /// Writes a `true` or `false` value.
  #[inline]
  fn write_bool(&mut self, output: &mut Vec<u8>, value: bool){
    output.extend_from_slice(if value { b"true" } else { b"false" });
  }

  /// Writes an integer value like `-123`.
  #[inline]
  fn write_i8(&mut self, output: &mut Vec<u8>, value: i8){
    let mut buffer = itoa::Buffer::new();
    let s = buffer.format(value);
    output.extend_from_slice(s.as_bytes());
  }

  /// Writes an integer value like `-123`.
  #[inline]
  fn write_i16(&mut self, output: &mut Vec<u8>, value: i16){
    let mut buffer = itoa::Buffer::new();
    let s = buffer.format(value);
    output.extend_from_slice(s.as_bytes());
  }

  /// Writes an integer value like `-123`.
  #[inline]
  fn write_i32(&mut self, output: &mut Vec<u8>, value: i32){
    let mut buffer = itoa::Buffer::new();
    let s = buffer.format(value);
    output.extend_from_slice(s.as_bytes());
  }

  /// Writes an integer value like `-123`.
  #[inline]
  fn write_i64(&mut self, output: &mut Vec<u8>, value: i64){
    let mut buffer = itoa::Buffer::new();
    let s = buffer.format(value);
    output.extend_from_slice(s.as_bytes());
  }

  /// Writes an integer value like `-123`.
  #[inline]
  fn write_i128(&mut self, output: &mut Vec<u8>, value: i128){
    let mut buffer = itoa::Buffer::new();
    let s = buffer.format(value);
    output.extend_from_slice(s.as_bytes());
  }

  /// Writes an integer value like `123`.
  #[inline]
  fn write_u8(&mut self, output: &mut Vec<u8>, value: u8){
    let mut buffer = itoa::Buffer::new();
    let s = buffer.format(value);
    output.extend_from_slice(s.as_bytes());
  }

  /// Writes an integer value like `123`.
  #[inline]
  fn write_u16(&mut self, output: &mut Vec<u8>, value: u16){
    let mut buffer = itoa::Buffer::new();
    let s = buffer.format(value);
    output.extend_from_slice(s.as_bytes());
  }

  /// Writes an integer value like `123`.
  #[inline]
  fn write_u32(&mut self, output: &mut Vec<u8>, value: u32){
    let mut buffer = itoa::Buffer::new();
    let s = buffer.format(value);
    output.extend_from_slice(s.as_bytes());
  }

  /// Writes an integer value like `123`.
  #[inline]
  fn write_u64(&mut self, output: &mut Vec<u8>, value: u64){
    let mut buffer = itoa::Buffer::new();
    let s = buffer.format(value);
    output.extend_from_slice(s.as_bytes());
  }

  /// Writes an integer value like `123`.
  #[inline]
  fn write_u128(&mut self, output: &mut Vec<u8>, value: u128){
    let mut buffer = itoa::Buffer::new();
    let s = buffer.format(value);
    output.extend_from_slice(s.as_bytes());
  }

  /// Writes a floating point value like `-31.26e+12`.
  ///
  /// # Special cases
  ///
  /// This function **does not** check for NaN or infinity. If the input
  /// number is not a finite float, the printed representation will be some
  /// correctly formatted but unspecified numerical value.
  ///
  /// Please check [`is_finite`] yourself before calling this function, or
  /// check [`is_nan`] and [`is_infinite`] and handle those cases yourself
  /// with a different `Formatter` method.
  ///
  /// [`is_finite`]: f32::is_finite
  /// [`is_nan`]: f32::is_nan
  /// [`is_infinite`]: f32::is_infinite
  #[inline]
  fn write_f32(&mut self, output: &mut Vec<u8>, value: f32){
    let mut buffer = ryu::Buffer::new();
    let s = buffer.format_finite(value);
    output.extend_from_slice(s.as_bytes());
  }

  /// Writes a floating point value like `-31.26e+12`.
  ///
  /// # Special cases
  ///
  /// This function **does not** check for NaN or infinity. If the input
  /// number is not a finite float, the printed representation will be some
  /// correctly formatted but unspecified numerical value.
  ///
  /// Please check [`is_finite`] yourself before calling this function, or
  /// check [`is_nan`] and [`is_infinite`] and handle those cases yourself
  /// with a different `Formatter` method.
  ///
  /// [`is_finite`]: f64::is_finite
  /// [`is_nan`]: f64::is_nan
  /// [`is_infinite`]: f64::is_infinite
  #[inline]
  fn write_f64(&mut self, output: &mut Vec<u8>, value: f64){
    let mut buffer = ryu::Buffer::new();
    let s = buffer.format_finite(value);
    output.extend_from_slice(s.as_bytes());
  }

  /// Writes a number that has already been rendered to a string.
  #[inline]
  fn write_number_str(&mut self, output: &mut Vec<u8>, value: &str){
    output.extend_from_slice(value.as_bytes());
  }

  /// Called before each series of `write_string_fragment` and
  /// `write_char_escape`.  Writes a `"`.
  #[inline]
  fn begin_string(&mut self, output: &mut Vec<u8>){
    output.push(b'"')
  }

  /// Called after each series of `write_string_fragment` and
  /// `write_char_escape`.  Writes a `"`.
  #[inline]
  fn end_string(&mut self, output: &mut Vec<u8>){
    output.push(b'"')
  }

  /// Writes a string fragment that doesn't need any escaping.
  #[inline]
  fn write_string_fragment(&mut self, output: &mut Vec<u8>, fragment: &str){
    output.extend_from_slice(fragment.as_bytes())
  }

  /// Writes a character escape code.
  #[inline]
  fn write_char_escape(&mut self, output: &mut Vec<u8>, char_escape: CharEscape){
    let escape_char = match char_escape {
      CharEscape::Quote => b'"',
      CharEscape::ReverseSolidus => b'\\',
      CharEscape::Solidus => b'/',
      CharEscape::Backspace => b'b',
      CharEscape::FormFeed => b'f',
      CharEscape::LineFeed => b'n',
      CharEscape::CarriageReturn => b'r',
      CharEscape::Tab => b't',
      CharEscape::AsciiControl(_) => b'u',
    };

    match char_escape{
      CharEscape::AsciiControl(byte) => {
        static HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";
        let bytes = &[
          b'\\',
          escape_char,
          b'0',
          b'0',
          HEX_DIGITS[(byte >> 4) as usize],
          HEX_DIGITS[(byte & 0xF) as usize],
        ];
        output.extend_from_slice(bytes)
      }
      _ => output.extend_from_slice(&[b'\\', escape_char]),
    }
  }

  /// Writes the representation of a byte array. Formatters can choose whether
  /// to represent bytes as a JSON array of integers (the default), or some
  /// JSON string encoding like hex or base64.
  fn write_byte_array(&mut self, output: &mut Vec<u8>, value: &[u8]){
    self.begin_array(output);
    let mut first = true;
    for byte in value{
      self.begin_array_value(output, first);
      self.write_u8(output, *byte);
      self.end_array_value(output);
      first = false;
    }
    self.end_array(output)
  }

  /// Called before every array.  Writes a `[`.
  #[inline]
  fn begin_array(&mut self, output: &mut Vec<u8>){
    output.push(b'[');
  }

  /// Called after every array.  Writes a `]`.
  #[inline]
  fn end_array(&mut self, output: &mut Vec<u8>){
    output.push(b']');
  }

  /// Called before every array value.  Writes a `,` if needed.
  #[inline]
  fn begin_array_value(&mut self, output: &mut Vec<u8>, first: bool){
    if !first {
      output.push(b',');
    }
  }

  /// Called after every array value.
  #[inline]
  fn end_array_value(&mut self, output: &mut Vec<u8>){

  }

  /// Called before every object.  Writes a `{`.
  #[inline]
  fn begin_object(&mut self, output: &mut Vec<u8>){
    output.push(b'{');
  }

  /// Called after every object.  Writes a `}`.
  #[inline]
  fn end_object(&mut self, output: &mut Vec<u8>){
    output.push(b'}');
  }

  /// Called before every object key.
  #[inline]
  fn begin_object_key(&mut self, output: &mut Vec<u8>, first: bool){
    if !first {
      output.push(b',');
    }
  }

  /// Called after every object key.  A `:` should be written
  /// by either this method or `begin_object_value`.
  #[inline]
  fn end_object_key(&mut self, output: &mut Vec<u8>){

  }

  /// Called before every object value.  A `:` should be written
  /// by either this method or `end_object_key`.
  #[inline]
  fn begin_object_value(&mut self, output: &mut Vec<u8>){
    output.push(b':');
  }

  /// Called after every object value.
  #[inline]
  fn end_object_value(&mut self, output: &mut Vec<u8>){

  }

  /// Called before every idMap.  Writes a `[:`.
  #[inline]
  fn begin_id_map(&mut self, output: &mut Vec<u8>){
    output.extend_from_slice(b"[:");
  }

  /// Called after every idMap.  Writes a `]`.
  #[inline]
  fn end_id_map(&mut self, output: &mut Vec<u8>){
    output.push(b']');
  }

  /// Called before every idMap key.
  #[inline]
  fn begin_id_map_key(&mut self, output: &mut Vec<u8>, first: bool){
    if !first {
      output.push(b',');
    }
  }

  /// Called after every idMap key.  A `:` should be written
  /// by either this method or `begin_id_map_value`.
  #[inline]
  fn end_id_map_key(&mut self, output: &mut Vec<u8>){

  }

  /// Called before every idMap value.  A `:` should be written
  /// by either this method or `end_id_map_key`.
  #[inline]
  fn begin_id_map_value(&mut self, output: &mut Vec<u8>){
    output.push(b':');
  }

  /// Called after every idMap value.
  #[inline]
  fn end_id_map_value(&mut self, output: &mut Vec<u8>){

  }

  /// Writes a raw JSON fragment that doesn't need any escaping.
  #[inline]
  fn write_raw_fragment(&mut self, output: &mut Vec<u8>, fragment: &str){
    output.extend_from_slice(fragment.as_bytes());
  }
}




/// This structure compacts a JSON value with no extra whitespace.
#[derive(Clone, Debug, Default)]
pub struct CompactFormatter;

impl Formatter for CompactFormatter{

}

/// This structure pretty prints a JSON value to make it human readable.
#[derive(Clone, Debug)]
pub struct PrettyFormatter<'a>{
  current_indent: usize,
  has_value: bool,
  indent: &'a [u8],
}

impl<'a> PrettyFormatter<'a>{
  /// Construct a pretty printer formatter that defaults to using two spaces for indentation.
  pub fn new() -> Self {
    PrettyFormatter::with_indent(b"  ")
  }

  /// Construct a pretty printer formatter that uses the `indent` string for indentation.
  pub fn with_indent(indent: &'a [u8]) -> Self{
    PrettyFormatter{
      current_indent: 0,
      has_value: false,
      indent,
    }
  }
}

impl<'a> Default for PrettyFormatter<'a>{
  fn default() -> Self{
    PrettyFormatter::new()
  }
}

impl<'a> Formatter for PrettyFormatter<'a>{
  #[inline]
  fn begin_array(&mut self, output: &mut Vec<u8>){
    self.current_indent += 1;
    self.has_value = false;
    output.push(b'[');
  }

  #[inline]
  fn end_array(&mut self, output: &mut Vec<u8>){
    self.current_indent -= 1;

    if self.has_value {
      output.push(b'\n');
      indent(output, self.current_indent, self.indent);
    }

    output.push(b']');
  }

  #[inline]
  fn begin_array_value(&mut self, output: &mut Vec<u8>, first: bool){
    output.extend_from_slice(if first { b"\n" } else { b",\n" });
    indent(output, self.current_indent, self.indent)
  }

  #[inline]
  fn end_array_value(&mut self, output: &mut Vec<u8>){
    self.has_value = true;
  }

  #[inline]
  fn begin_object(&mut self, output: &mut Vec<u8>){
    self.current_indent += 1;
    self.has_value = false;
    output.push(b'{');
  }

  #[inline]
  fn end_object(&mut self, output: &mut Vec<u8>){
    self.current_indent -= 1;

    if self.has_value {
      output.push(b'\n');
      indent(output, self.current_indent, self.indent);
    }

    output.push(b'}');
  }

  #[inline]
  fn begin_object_key(&mut self, output: &mut Vec<u8>, first: bool){
    output.extend_from_slice(if first { b"\n" } else { b",\n" });
    indent(output, self.current_indent, self.indent)
  }

  #[inline]
  fn begin_object_value(&mut self, output: &mut Vec<u8>){
    output.extend_from_slice(b": ");
  }

  #[inline]
  fn end_object_value(&mut self, output: &mut Vec<u8>){
    self.has_value = true;
  }

  #[inline]
  fn begin_id_map(&mut self, output: &mut Vec<u8>){
    self.current_indent += 1;
    self.has_value = false;
    output.extend_from_slice(b"[:");
  }

  #[inline]
  fn end_id_map(&mut self, output: &mut Vec<u8>){
    self.current_indent -= 1;

    if self.has_value {
      output.push(b'\n');
      indent(output, self.current_indent, self.indent);
    }

    output.push(b']');
  }

  #[inline]
  fn begin_id_map_key(&mut self, output: &mut Vec<u8>, first: bool){
    output.extend_from_slice(if first { b"\n" } else { b",\n" });
    indent(output, self.current_indent, self.indent)
  }

  #[inline]
  fn begin_id_map_value(&mut self, output: &mut Vec<u8>){
    output.extend_from_slice(b": ");
  }

  #[inline]
  fn end_id_map_value(&mut self, output: &mut Vec<u8>){
    self.has_value = true;
  }
}



fn indent(output: &mut Vec<u8>, depth: usize, indent_s: &[u8]){
  for _ in 0..depth {
    output.extend_from_slice(indent_s);
  }
}



