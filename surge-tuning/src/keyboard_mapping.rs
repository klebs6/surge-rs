crate::ix!();

pub const KEYBOARD_TUNING_FREQUENCY: f64   = 8.175798915 * 32.0;

#[derive(Debug,Clone)]
pub struct KeyboardMapping {
    pub is_valid:               bool,
    pub is_standard_mapping:    bool,
    pub count:                  i32,
    pub first_midi:             i32,
    pub last_midi:              i32,
    pub middle_note:            i32,
    pub tuning_constant_note:   i32,
    pub tuning_frequency:       f32,
    pub octave_degrees:         i32,
    /// rather than an 'x' we use a '-1' for skipped keys
    pub keys:                   Vec<i32>,
    pub raw_text:               MappingData,
    pub name:                   String,
}

impl Initialize for KeyboardMapping {
    fn init(&mut self) {
        self.is_valid             = true;
        self.is_standard_mapping  = true;
        self.count                = 12;
        self.first_midi           = 0;
        self.last_midi            = 127;
        self.middle_note          = 60;
        self.tuning_constant_note = 60;
        self.tuning_frequency     = KEYBOARD_TUNING_FREQUENCY as f32;
        self.octave_degrees       = 12;
        self.keys                 = vec![0,1,2,3,4,5,6,7,8,9,10,11];
        self.raw_text             = MappingData("".to_string());
        self.name                 = "".to_string();
    }
}

impl Default for KeyboardMapping {
    fn default() -> Self {
        Self {
            is_valid:               true,
            is_standard_mapping:    true,
            count:                  12,
            first_midi:             0,
            last_midi:              127,
            middle_note:            60,
            tuning_constant_note:   60,
            tuning_frequency:       KEYBOARD_TUNING_FREQUENCY as f32,
            octave_degrees:         12,
            keys:                   vec![0,1,2,3,4,5,6,7,8,9,10,11],
            raw_text:               MappingData("".to_string()),
            name:                   "".to_string(),
        }
    }
}

impl KeyboardMapping {

    pub fn tune_a69_to(freq: f64) -> Self {
        // There's a couple of ways to do this but since I want it to stream 
        // I will syntheitcally create a KBM file
        let synthetic_kbm_file = formatdoc!["
            ! Surge Synthetic Keyboard Tuning to Retune A69
            !
            ! Map Size
            0
            ! First note
            0
            ! Last note
            127
            ! First mapping
            60
            ! Reference Note
            69
            ! Reference Freqency
            {}
            ! Scale Degree
            0
            ! Mapping
            ",
            freq
        ]; 
            let bytes = synthetic_kbm_file.as_bytes();
            let mut reader = BufReader::new(bytes);
            KeyboardMapping::from(&mut reader)
    }

}

impl From<MappingData> for KeyboardMapping {
    fn from(x: MappingData) -> Self {
        let mut reader = BufReader::new(x.0.as_bytes());
        Self::from(&mut reader)
    }
}

impl From<KbmFileName> for KeyboardMapping {
    fn from(x: KbmFileName) -> Self {
        let file = File::open(x.0).unwrap();
        let mut reader = BufReader::new(file);
        Self::from(&mut reader)
    }
}

impl<R : std::io::Read> From<&mut BufReader<R>> for KeyboardMapping {
    fn from(reader: &mut BufReader<R>) -> Self {

        //TODO: check for porting bugs
        
        #[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
        enum ParsePosition {
            MapSize,
            FirstMidi,
            LastMidi,
            Middle,
            Reference,
            Freq,
            Degree,
            Keys
        }
        impl ParsePosition {
            pub fn next(&self) -> Self {
                match self {
                    ParsePosition::MapSize   => ParsePosition::FirstMidi,
                    ParsePosition::FirstMidi => ParsePosition::LastMidi,
                    ParsePosition::LastMidi  => ParsePosition::Middle,
                    ParsePosition::Middle    => ParsePosition::Reference,
                    ParsePosition::Reference => ParsePosition::Freq,
                    ParsePosition::Freq      => ParsePosition::Degree,
                    ParsePosition::Degree    => ParsePosition::Keys,
                    ParsePosition::Keys      => ParsePosition::Keys,//last state
                }
            }
        }

        let mut res             = KeyboardMapping::default();
        let mut raw_oss          = Vec::<u8>::new();
        res.is_standard_mapping = false;
        res.keys.clear();

        let mut state: ParsePosition = ParsePosition::MapSize;
        let mut line:  String        = String::new();

        while reader.read_line(&mut line).unwrap() != 0 {

            let bytes = line.as_bytes();

            let _written = raw_oss.write(bytes).unwrap();
            let _written = raw_oss.write("\n".as_bytes()).unwrap();

            if bytes[0] == "!".as_bytes()[0] {
                continue;
            }

            if line == "x" {
                line = "-1".to_string() ;
            }
            let i: i32 = line.parse::<i32>().unwrap();
            let v: f32 = line.parse::<f32>().unwrap();
            match state {
                ParsePosition::MapSize => {
                    res.count = i;
                },
                ParsePosition::FirstMidi => {
                    res.first_midi = i;
                },
                ParsePosition::LastMidi => {
                    res.last_midi = i;
                },
                ParsePosition::Middle => {
                    res.middle_note = i;
                },
                ParsePosition::Reference => {
                    res.tuning_constant_note = i;
                },
                ParsePosition::Freq => {
                    res.tuning_frequency = v;
                },
                ParsePosition::Degree => {
                    res.octave_degrees = i;
                },
                ParsePosition::Keys => {
                    res.keys.push(i);
                },
            }
            state = state.next();
        }
        res.raw_text = MappingData(String::from_utf8(raw_oss).unwrap());
        res
    }
}

