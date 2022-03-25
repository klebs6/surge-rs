ix!();

use crate::*;

//TODO: can give these better names after
//listening to them 
enhanced_enum![
    ReverbPreset {
        A,
        B,
        C,
        D,
    }
];

impl ReverbPreset {
    pub fn delay_time_buffer(&self) -> A1d::<usize> {
        match self {
            ReverbPreset::A => {
                array![
                    1339934, 0962710, 1004427, 1103966,
                    1198575, 1743348, 1033425, 0933313,
                    0949407, 1402754, 1379894, 1225304,
                    1135598, 1402107, 0956152, 1137737,
                ]
            },
            ReverbPreset::B => {
                array![
                    1265607, 0844703, 0856159, 1406425,
                    0786608, 1163557, 1091206, 1129434,
                    1270379, 0896997, 1415393, 0782808,
                    0868582, 1234463, 1000336, 0968299,
                ]
            },
            ReverbPreset::C => {
                array![
                    1293101, 1334867, 1178781, 1850949,
                    1663760, 1982922, 1211021, 1824481,
                    1520266, 1351822, 1102711, 1513696,
                    1057618, 1671799, 1406360, 1170468,
                ]
            },
            ReverbPreset::D => {
                array![
                    1833435, 2462309, 2711583, 2219764,
                    1664194, 2109157, 1626137, 1434473,
                    2271242, 1621375, 1831218, 2640903,
                    1577737, 1871624, 2439164, 1427343,
                ]
            },
        }
    }
}

impl LoadPreset for Reverb {

    type PresetType = ReverbPreset;

    fn load_preset(&mut self, preset: Self::PresetType) {

        self.preset = preset;

        self.clear_buffers();

        self.delay_time = Align16(preset.delay_time_buffer());

        let room_size: f32 = self.pvalf(ReverbParam::RoomSize);

        for t in 0..REVERB_TAPS
        {
            self.delay_time[t] = 
                (
                    2.0_f32 * room_size * (self.delay_time[t] as f32)
                ) as usize;
        }
        self.lastf[ReverbParam::RoomSize as usize] = room_size;
        self.update_rtime();
    }
}
