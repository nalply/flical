// Helper macro for debugging.
macro_rules! d {
  ( $( $tt:tt )+ ) => {
    if ENABLE_D {
      let s = format!( $( $tt )+ );
      eprintln!("{}:{} d! {s}", file!(), line!());
    }
  }
}

/// Pretty print and shorten a byte slice which might contain invalid UTF-8:
///
/// - shorten to width with ellipsis in the middle
/// - escape invalid UTF-8 as `\Uhh..hh;`
/// - escape control codes and whitespace (except space and newline) as one of:
///   `\xhh`, `\Xhhh;`, `\Xhhhh;` or `\Xhhhhh;`
///
/// Note that the resulting character count might vary by up to 6 characters
/// because escapes take more space and are not truncated.
///
/// This function should never panic. If it does, it is a bug. This said, this
/// function is not optimized and has many implicit assertions (like Rust's
/// out-of-bound checks). This is by design (stay safe and slow).
/// ```
/// # use pretty::pretty;
/// let s = b"012\x01456789\xff";
/// assert_eq!("012\\x01456789\\Uff;", pretty(s, 0));
/// assert_eq!("012\\x01⠤89\\Uff;", pretty(s, 10));
/// assert_eq!("01⠤\\Uff;", pretty(s, 5));
/// assert_eq!("01⠤\\Uff;", pretty(s, 4));
/// assert_eq!("0⠤\\Uff;", pretty(s, 3));
/// ```
pub fn pretty(input: &[u8], width: usize) -> String {
  let len = input.len();
  if len == 0 {
    return String::new();
  }

  let shortened = width > 0 && width < len;
  let width = width.max(3).min(len); // not shorter than 3 except len < 3
  let width2 = (width / 2).max(1); // half width not shorter than 1

  // The first part (or the main part if not shortened)
  let mut output = Vec::new();
  let mut char_count = 0;
  let mut part1_len = 0;
  for item in OutputIterator(input) {
    if shortened && char_count >= width2 {
      break;
    }
    char_count += item.char_count();
    part1_len += item.input_len;
    output.push(item);
  }
  let mut pretty = coalesced(&output);

  d!("w{width} {width2} cc{char_count} i{part1_len} l{len} {shortened}");

  if shortened {
    // Estimate where the second part will start. It's tricky because we don't
    // know the bytes and we don't want to read the whole byte slice, might be
    // long. Go backwards from the end of the slice, but UTF-8, ugh. Let's try.
    //
    // One to four bytes get converted to: a char, or be escaped: \xhh, \Xhhh;
    // or \u{hhhhh}. A special case are invalid bytes, sequences of them get
    // coalesced, like this: \Uffffff;
    //
    // The most cautious case to go back as far as neccessary is assuming that
    // all bytes are four-byte chars. We need to go back four times the bytes
    // of width2 (half width). The worst that could happen is that invalid bytes
    // and ASCII control codes strictly alternate, then in that case one byte
    // gets expanded to five chars on average. This means we go back 4 * 5 = 20
    // times too far. But because we ant to shorten the display and will work
    // with the only bytes that get possibly displayed as passed in by the
    // parameter `width`, it's not too bad. Of course if someone wants to
    // shorten to a few thousand characters, this might take a bit more time.
    let start2 = len.saturating_sub(4 * width2).max(part1_len);
    let output = OutputIterator(&input[start2..]).collect::<Vec<_>>();
    let output_count = output.len();
    let width2 = width2 + (width % 2) - 1;
    d!("s{start2} w{width2} o{output_count}");

    // Now reverse and count the chars
    let mut char_count = 0;
    let mut part2_len = 0;
    let shortened_count = output
      .iter()
      .rev()
      .take_while(|&item| {
        char_count += item.char_count();
        part2_len += item.input_len;
        d!("  item {item:?} cc{char_count} i{part2_len}");
        char_count <= width2
      })
      .count()
      .max(1)
      .min(width2);

    let start2 = output_count - shortened_count;
    let consumed = part1_len + part2_len >= len;
    let start2 = if consumed { 0 } else { start2 };
    d!("s{shortened_count} cc{char_count} i{part2_len} s{start2}");

    // If all bytes both from part1 and part2 are consumed omit gap indicator
    if !consumed {
      pretty.push('⠤');
    }

    pretty.push_str(&coalesced(&output[start2..]))
  }

  pretty
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Output {
  contents: String,
  input_len: usize,
  valid_utf8: bool,
}

impl Output {
  fn char_count(&self) -> usize {
    self.contents.chars().count()
  }
}

struct OutputIterator<'b>(&'b [u8]);

const SAFE_UTF8: &str = "unexpected: already validated as utf8";
const NON_EMPTY: &str = "unexpected: already made sure not empty";

impl<'b> Iterator for OutputIterator<'b> {
  type Item = Output;

  fn next(&mut self) -> Option<Self::Item> {
    fn ascii_x_esc(c: char) -> bool {
      c.is_ascii_control()
    }

    fn x_esc(c: char) -> bool {
      c.is_control() && c.is_whitespace() && c > '\u{ff}'
    }

    // read valid char, escape and unshift from bytes then return Output item
    fn valid_char(bytes: &mut &[u8], n: usize) -> Output {
      let c = std::str::from_utf8(&bytes[..n])
        .expect(SAFE_UTF8)
        .chars()
        .next()
        .expect(NON_EMPTY);
      let input_len = c.len_utf8();
      *bytes = &bytes[input_len..];

      let contents = match c {
        '\\' => r"\\".to_owned(),
        '\r' => r"\r".to_owned(),
        '\n' => r"\n".to_owned(),
        '\t' => r"\t".to_owned(),
        '\0' => r"\0".to_owned(),
        c if ascii_x_esc(c) => format!("\\x{:02x}", c as u32),
        c if x_esc(c) => format!("\\X{:x};", c as u32),
        c => c.to_string(),
      };
      Output { contents, input_len, valid_utf8: true }
    }

    if self.0.is_empty() {
      return None;
    }

    let n = self.0.len().min(4);
    let slice = std::str::from_utf8(&self.0[..n]);

    Some(match slice {
      Ok(_) => valid_char(&mut self.0, n),
      Err(err) => {
        let valid_len = err.valid_up_to();
        if valid_len > 0 {
          valid_char(&mut self.0, valid_len.min(4))
        } else {
          let contents = format!("{:02x}", self.0[0]);
          self.0 = &self.0[1..];
          Output { contents, input_len: 1, valid_utf8: false }
        }
      }
    })
  }
}

fn coalesced(output: &[Output]) -> String {
  let mut invalid = false;
  let mut result = String::new();
  for item in output {
    if item.valid_utf8 && invalid {
      invalid = false;
      result.push(';');
    } else if !item.valid_utf8 && !invalid {
      invalid = true;
      result.push_str("\\U");
    }
    result.push_str(&item.contents);
  }

  if invalid {
    result.push(';')
  }
  result
}

// Enable the d!() macro or not
const ENABLE_D: bool = false;
// const ENABLE_D: bool = true;

#[cfg(test)]
pub mod tests {
  use super::pretty;

  #[test]
  fn test_pretty_some() {
    assert_eq!(pretty(b"", 0), "");
    assert_eq!(pretty(b"", 10), "");

    assert_eq!(pretty(b"normal text", 0), "normal text");
    assert_eq!(pretty("Schönen Tag!".as_bytes(), 0), "Schönen Tag!");
    assert_eq!(pretty(b"\0\x01\x07\x13\\\x1f", 0), r"\0\x01\x07\x13\\\x1f");
    assert_eq!(pretty(b"ASCII text\tand tab", 0), r"ASCII text\tand tab");
    assert_eq!(pretty(b"-\xf0\x80-\xee", 0), r"-\Uf080;-\Uee;");
    assert_eq!(pretty(b"abcd\x00ef\xfegh", 0), r"abcd\0ef\Ufe;gh");

    assert_eq!(pretty(b"normal text", 8), "norm⠤ext");
    assert_eq!(pretty("Schönen Tag!".as_bytes(), 8), "Schö⠤ag!");
    assert_eq!(pretty(b"\0\x01\x07\x13\\\x1f", 3), r"\0⠤\x1f");
    assert_eq!(pretty(b"\0\x01\x07\x13\\\x1f", 4), r"\0⠤\x1f");
    assert_eq!(pretty(b"\0\x01\x07\x13\\\x1f", 5), r"\0⠤\x1f");
    assert_eq!(pretty(b"\0\x01\x07\x13\\\x1f", 6), r"\0\x01\x07\x13\\\x1f");
    assert_eq!(pretty(b"ASCII text\tand tab", 8), r"ASCI⠤tab");
    assert_eq!(pretty(b"-\xf0\x80-\xee", 3), r"-⠤\Uee;");
    assert_eq!(pretty(b"-\xf0\x80-\xee", 4), r"-\Uf0;⠤\Uee;");
    assert_eq!(pretty(b"-\xf0\x80-\xee", 5), r"-\Uf080;-\Uee;");
    assert_eq!(pretty(b"abcd\x00ef\xfegh", 5), r"ab⠤gh");
    assert_eq!(pretty(b"abcd\x00ef\xfegh", 6), r"abc⠤gh");
    assert_eq!(pretty(b"abcd\x00ef\xfegh", 7), r"abc⠤gh");
    assert_eq!(pretty(b"abcd\x00ef\xfegh", 8), r"abcd⠤gh");
    assert_eq!(pretty(b"abcd\x00ef\xfegh", 9), r"abcd⠤\Ufe;gh");
    assert_eq!(pretty(b"abcd\x00ef\xfegh", 10), r"abcd\0ef\Ufe;gh");
  }

  #[test]
  fn test_pretty_long() {
    let long = "x".repeat(1000000);
    let long_bytes = long.as_bytes();
    let x9 = "x".repeat(9);
    let x99 = "x".repeat(99);
    let x999 = "x".repeat(999);
    let x9999 = "x".repeat(9999);
    let x99999 = "x".repeat(99999);
    assert_eq!(pretty(long_bytes, 0), long);
    assert_eq!(pretty(long_bytes, 20), format!("{x9}x⠤{x9}"));
    assert_eq!(pretty(long_bytes, 200), format!("{x99}x⠤{x99}"));

    // switching to assert!() because of too long error messages
    assert!(pretty(long_bytes, 2000) == format!("{x999}x⠤{x999}"));
    assert!(pretty(long_bytes, 20000) == format!("{x9999}x⠤{x9999}"));
    assert!(pretty(long_bytes, 200000) == format!("{x99999}x⠤{x99999}"));
  }

  #[test]
  fn test_pretty_short() {
    assert_eq!(pretty(b"a", 0), "a");
    assert_eq!(pretty("ä".as_bytes(), 0), "ä");
    assert_eq!(pretty(b"ab", 0), "ab");
    assert_eq!(pretty("äa".as_bytes(), 0), "äa");
    assert_eq!(pretty("aä".as_bytes(), 0), "aä");
    assert_eq!(pretty("äö".as_bytes(), 0), "äö");
    assert_eq!(pretty(b" \n", 0), " \\n");

    assert_eq!(pretty(b"a", 1), "a");
    assert_eq!(pretty("ä".as_bytes(), 1), "ä");
    assert_eq!(pretty(b"ab", 1), "ab");
    assert_eq!(pretty("äa".as_bytes(), 1), "äa");
    assert_eq!(pretty("aä".as_bytes(), 1), "aä");
    assert_eq!(pretty("äö".as_bytes(), 1), "äö");
    assert_eq!(pretty(b" \n", 1), " \\n");

    assert_eq!(pretty(b"a", 2), "a");
    assert_eq!(pretty("ä".as_bytes(), 2), "ä");
    assert_eq!(pretty(b"ab", 2), "ab");
    assert_eq!(pretty("äa".as_bytes(), 2), "äa");
    assert_eq!(pretty("aä".as_bytes(), 2), "aä");
    assert_eq!(pretty("äö".as_bytes(), 2), "äö");
    assert_eq!(pretty(b" \n", 2), " \\n");

    assert_eq!(pretty(b"a", 3), "a");
    assert_eq!(pretty("ä".as_bytes(), 3), "ä");
    assert_eq!(pretty(b"ab", 3), "ab");
    assert_eq!(pretty("äa".as_bytes(), 3), "äa");
    assert_eq!(pretty("aä".as_bytes(), 3), "aä");
    assert_eq!(pretty("äö".as_bytes(), 3), "äö");
    assert_eq!(pretty(b" \n", 3), " \\n");
  }

  #[test]
  fn test_pretty_iterative() {
    let s = ["abcdefghijklmn", "Höflichkeit 💩 été à Li 李"];
    let t = ["a⠤n", "H⠤李"];
    let b = s.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
    for i in 0..s.len() {
      assert_eq!(pretty(b[i], 0), s[i], "string {i} iteration 0");
      for j in 1..4 {
        assert_eq!(pretty(b[i], j), t[i], "string {i} iteration {j}");
      }
    }

    assert_eq!(pretty(b[0], 1), "a⠤n");
    assert_eq!(pretty(b[0], 2), "a⠤n");
    assert_eq!(pretty(b[0], 3), "a⠤n");
    assert_eq!(pretty(b[0], 4), "ab⠤n");
    assert_eq!(pretty(b[0], 5), "ab⠤mn");
    assert_eq!(pretty(b[0], 6), "abc⠤mn");
    assert_eq!(pretty(b[0], 7), "abc⠤lmn");
    assert_eq!(pretty(b[0], 8), "abcd⠤lmn");
    assert_eq!(pretty(b[0], 9), "abcd⠤klmn");
    assert_eq!(pretty(b[0], 10), "abcde⠤klmn");
    assert_eq!(pretty(b[0], 11), "abcde⠤jklmn");
    assert_eq!(pretty(b[0], 12), "abcdef⠤jklmn");
    assert_eq!(pretty(b[0], 13), "abcdef⠤ijklmn");
    assert_eq!(pretty(b[0], 14), "abcdefghijklmn");
    assert_eq!(pretty(b[1], 10), "Höfli⠤Li 李");
    assert_eq!(pretty(b[1], 11), "Höfli⠤ Li 李");
    assert_eq!(pretty(b[1], 12), "Höflic⠤ Li 李");
    assert_eq!(pretty(b[1], 13), "Höflic⠤à Li 李");
    assert_eq!(pretty(b[1], 14), "Höflich⠤à Li 李");
    assert_eq!(pretty(b[1], 15), "Höflich⠤ à Li 李");
    assert_eq!(pretty(b[1], 16), "Höflichk⠤ à Li 李");
    assert_eq!(pretty(b[1], 17), "Höflichk⠤é à Li 李");
    assert_eq!(pretty(b[1], 18), "Höflichke⠤é à Li 李");
    assert_eq!(pretty(b[1], 19), "Höflichke⠤té à Li 李");
    assert_eq!(pretty(b[1], 20), "Höflichkei⠤té à Li 李");
    assert_eq!(pretty(b[1], 21), "Höflichkei⠤été à Li 李");
    assert_eq!(pretty(b[1], 22), "Höflichkeit⠤été à Li 李");
    assert_eq!(pretty(b[1], 23), "Höflichkeit⠤ été à Li 李");
    assert_eq!(pretty(b[1], 24), "Höflichkeit 💩 été à Li 李");
    assert_eq!(pretty(b[1], 25), "Höflichkeit 💩 été à Li 李");
  }

  // Simply fuzzer to try to tigger panics. Use a simple LCR by D. Knuth to
  // randomize input. The output is not tested, so this fuzzer does not
  // detect bad ouptut, only panics.
  #[test]
  fn fuzzy_pretty() {
    let seed = &mut 0u64;

    // run many times to see whether panics are triggered
    for _ in 0..1000 {
      let s = rand_bytes(seed);
      let bytes = s.as_slice();
      pretty(bytes, 0);

      let n = bytes.len() as u8;
      pretty(bytes, (rand::lcr(seed) % n) as usize);

      if n > 10 {
        pretty(bytes, (10 + rand::lcr(seed) % (n - 10)) as usize);
      }
    }
    let _ = pretty(b"asdf", 0);
  }

  fn rand_bytes(seed: &mut u64) -> Vec<u8> {
    let n = rand::lcr(seed) as usize | (rand::lcr(seed) as usize) >> 8;
    let mut v = Vec::with_capacity(n as usize);
    for _ in 0..n {
      v.push(rand::lcr(seed))
    }
    v
  }

  mod rand {
    use std::num::Wrapping;

    // by Donald Knuth
    const A: Wrapping<u64> = Wrapping(6364136223846793005);
    const B: Wrapping<u64> = Wrapping(1442695040888963407);

    pub fn lcr(seed: &mut u64) -> u8 {
      let x = (A * Wrapping(*seed) + B).0;
      *seed = x;
      x as u8
    }
  }
}

// Copyright see AUTHORS & LICENSE; SPDX-License-Identifier: ISC+
