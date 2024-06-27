crate::ix!();

/**
  | From:
  | http://huygens-fokker.org/scala/scl_format.html
  |
  | The files are human readable ASCII or 8-bit
  | character text-files.
  |
  | The file type is .scl .
  |
  | There is one scale per file.
  |
  | Lines beginning with an exclamation mark are
  | regarded as comments and are to be ignored.
  |
  | The first (non comment) line contains a short
  | description of the scale, but long lines are
  | possible and should not give a read error. The
  | description is only one line. If there is no
  | description, there should be an empty
  | line. The second line contains the number of
  | notes. This number indicates the number of
  | lines with pitch values that follow. In
  | principle there is no upper limit to this, but
  | it is allowed to reject files exceeding
  | a certain size. The lower limit is 0, which is
  | possible since degree 0 of 1/1 is
  | implicit. Spaces before or after the number
  | are allowed.
  |
  | After that come the pitch values, each on
  | a separate line, either as a ratio or as
  | a value in cents. If the value contains
  | a period, it is a cents value, otherwise
  | a ratio. Ratios are written with a slash, and
  | only one. Integer values with no period or
  | slash should be regarded as such, for example
  | "2" should be taken as "2/1". Numerators and
  | denominators should be supported to at least
  | 231-1 = 2147483647. Anything after a valid
  | pitch value should be ignored. Space or
  | horizontal tab characters are allowed and
  | should be ignored. Negative ratios are
  | meaningless and should give a read error. For
  | a description of cents, go here. The first
  | note of 1/1 or 0.0 cents is implicit and not
  | in the files. Files for which Scala gives
  | Error in file format are incorrectly
  | formatted. They should give a read error and
  | be rejected.
  */
#[derive(Debug,Clone)]
pub struct Scale {
    pub name: String,
    pub description: String,
    pub raw_text: TuningData,
    pub count: usize,
    pub tones: Vec<Tone>,
}

impl Initialize for Scale {

    fn init(&mut self) -> Result<(),SurgeError> {
        self.name         = "empty scale".to_string();
        self.description  = "".to_string();
        self.raw_text     = TuningData("".to_string());
        self.count        = 0;
        self.tones        = vec![];

        Ok(())
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self {
            name: "empty scale".to_string(),
            description: "".to_string(),
            raw_text: TuningData("".to_string()),
            count: 0,
            tones: vec![],
        }
    }
}

enhanced_enum!{
    ParsePosition {
        ReadHeader,
        ReadCount,
        ReadNote,
    }
}

impl ParsePosition {
    pub fn next(&self) -> Self {
        match self {
            ParsePosition::ReadHeader => ParsePosition::ReadCount,
            ParsePosition::ReadCount  => ParsePosition::ReadNote,
            ParsePosition::ReadNote   => ParsePosition::ReadNote,//last state
        }
    }
}

impl<R: std::io::Read> From<&mut BufReader<R>> for Scale {

    fn from(reader: &mut BufReader<R>) -> Self {

        let mut res    = Scale::default();
        let mut raw_oss = Vec::<u8>::new();
        let mut state: ParsePosition = ParsePosition::ReadHeader;
        let mut line:  String        = String::new();

        while reader.read_line(&mut line).unwrap() != 0 {

            let bytes = line.as_bytes();

            let _written = raw_oss.write(bytes).unwrap();
            let _written = raw_oss.write("\n".as_bytes()).unwrap();

            if bytes[0] == "!".as_bytes()[0] {
                continue;
            }

            let _i: i32 = line.parse::<i32>().unwrap();
            let _v: f32 = line.parse::<f32>().unwrap();

            match state {

                ParsePosition::ReadHeader => {
                    res.description = line.clone();
                },

                ParsePosition::ReadCount => {
                    res.count = line.parse::<usize>().unwrap();
                },

                ParsePosition::ReadNote => {

                    let mut t = Tone {
                        repr: line.clone(),
                        .. Default::default()
                    };

                    if let Some(_pos) = line.find('.') {
                        t.ty = ToneType::Cents;
                        t.cents = line.parse::<f32>().unwrap();
                    } else {

                        t.ty = ToneType::Ratio;

                        if let Some(slash_pos) = line.find('/') {
                            t.ratio_n = line[0..slash_pos].parse::<i32>().unwrap() as f32;
                            t.ratio_d = line[slash_pos..].parse::<i32>().unwrap() as f32;
                        } else {
                            t.ratio_n = line.parse::<i32>().unwrap() as f32;
                            t.ratio_d = 1.0;
                        }

                        t.cents = 1200.0 * (1.0 * t.ratio_n / t.ratio_d).log2();
                    }

                    t.val = t.cents / 1200.0 + 1.0;
                    res.tones.push(t);
                },

            }
            state = state.next();
        }
        res.raw_text = TuningData(String::from_utf8(raw_oss).unwrap());
        res
    }
}

impl Scale {

    pub fn is_valid(&self) -> bool {
        //TODO: try to factor this out
        //maybe check more things
        if self.count == 0 {
            return false;
        }
        true
    }

    pub fn even_temperament_12_note_scale() -> Scale {
        let scl = crate::TWELVE_TONE_EQUAL_TEMPERAMENT.as_bytes();
        let mut reader = BufReader::new(scl);

        Scale::from(&mut reader)
    }

    pub fn scale_432() -> Scale {
        let scl = crate::ED2_25.as_bytes();
        let mut reader = BufReader::new(scl);
        Scale::from(&mut reader)
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "Scale\n {}\n {}\n --- {} tones ---\n{:?}",
            self.name,
            self.description,
            self.count,
            self.tones)
    }
}

impl From<SclFileName> for Scale {
    fn from(x: SclFileName) -> Self {
        let file = File::open(x.0).unwrap();
        let mut reader = BufReader::new(file);
        Self::from(&mut reader)
    }
}

impl From<TuningData> for Scale {
    fn from(x: TuningData) -> Self {
        let mut reader = BufReader::new(x.0.as_bytes());
        Self::from(&mut reader)
    }
}

