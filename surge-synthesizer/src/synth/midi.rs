ix!();

use crate::SurgeSynthesizer;

impl SurgeSynthesizer<'plugin_layer> {

    pub fn on_nrpn(&mut self, 
        channel: u8, 
        _lsb_nrpn: i32, 
        _msb_nrpn: i32, 
        _lsb_value: i32, 
        _msb_value: i32) 
    {
        let _channel: i32 = channel.try_into().unwrap();
        todo!();
    }

    /*
      |OK there are two things we are dealing with
      |here
      |
      |1: The MPE specification v1.0 section 2.1.1
      |says the message format for RPN is
      |
      |Bn 64 06
      |Bn 65 00
      |Bn 06 mm
      |
      |where n = 0 is lower zone, n=f is upper
      |zone, all others are invalid, and mm=0
      |means no MPE and mm=1->f is zone.  So you
      |would think the Roli Seaboard would send,
      |since it is one zone
      |
      |B0 64 06 B0 65 00 B0 06 0F
      |
      |and be done with it. If it did this code
      |would work.  But it doesn't. At least with
      |Roli Seaboard Firmware 1.1.7.  Instead on
      |*each* channel it sends
      |
      |Bn 64 04 Bn 64 0 Bn 06 00
      |Bn 64 03 Bn 64 0 Bn 06 00
      |
      |for each channel. Which seems unrelated to
      |the spec. But as a result the original
      |on_rpn code means you get no MPE with
      |a Roli Seaboard.
      |
      |Hey one year later an edit: Those aren't
      |coming from ROLI they are coming from Logic
      |PRO and now that I correct modify and
      |stream MPE state, we should not listen to
      |those messages.
      */
    pub fn on_rpn(&mut self, 
        channel: u8, 
        lsb_rpn: i32, 
        msb_rpn: i32, 
        _lsb_value: i32, 
        msb_value: i32) 
    {
        let channel: i32 = channel.try_into().unwrap();

        let pitchbend_range:         bool = lsb_rpn == 0 && msb_rpn == 0;
        let mpe_mode:                 bool = lsb_rpn == 6 && msb_rpn == 0;
        let handle_logicpro_special:  bool = lsb_rpn == 4 && msb_rpn == 0 && channel != 0 && channel != 0xF ;

       if pitchbend_range {
           match channel {
               0 => { self.mpe_unit.set_global_pitchbend_range(msb_value as f32); },
               1 => { self.mpe_unit.set_pitchbend_range(msb_value as f32); },
               _ => {},
           }

       } else if mpe_mode {

          self.mpe_unit.set_enabled(msb_value > 0);
          self.mpe_unit.set_num_voices((msb_value & 0xF) as u32);

          if self.mpe_unit.pitchbend_range().0 < 0.0  {
             self.mpe_unit.set_pitchbend_range(48.0);
          }

          self.mpe_unit.set_global_pitchbend_range(0.0);

       } else if handle_logicpro_special {

           /*
             | This is code sent by logic in all
             | cases for some reason. input ancient
             | times I thought it came from
             | a roli. But I since changed the MPE
             | state management so with 1.6.5 do
             | this:
             */
           if false {

               /*
                 This is the invalid message
                 which the ROLI sends.  Rather
                 than have the Roli not work
                 */
               self.mpe_unit.set_enabled(true);

               self.mpe_unit.set_num_voices((msb_value & 0xF) as u32);

               self.mpe_unit.set_pitchbend_range(48.0);

               println!("{:?} {:?} MPEE={:?}, MPEPBR={:?}",
                   line!(), file!(), 
                   self.mpe_unit.enabled(), 
                   self.mpe_unit.pitchbend_range()
               );

               self.mpe_unit.set_global_pitchbend_range(0.0);
           }
       }
    }

    pub fn program_change(&mut self, _channel: u8, value: i32) 
    {
       self.pch = value;
       // load_patch((cc0<<7) + pch);
       self.patchid_queue = Some((self.cc0 << 7) + self.pch);
    }
}

