ix!();

use crate::{
    SurgeSynthesizer,
    CS_MOD_ROUTING
};

#[derive(Debug)]
#[repr(align(16))]
pub struct TwoByTwoBlock<const LEN: usize>  {
    pub buf: Align16<[[[f32; LEN]; 2]; 2]>,
}

impl<const LEN: usize> TwoByTwoBlock<LEN> {
    pub fn clear(&mut self) {
        unsafe {
            clear_block_antidenormalnoise::<LEN>(self.buf[0][0].as_mut_ptr());
            clear_block_antidenormalnoise::<LEN>(self.buf[0][1].as_mut_ptr());
            clear_block_antidenormalnoise::<LEN>(self.buf[1][0].as_mut_ptr());
            clear_block_antidenormalnoise::<LEN>(self.buf[1][1].as_mut_ptr());
        }
    }
}

impl<const LEN: usize> Default for TwoByTwoBlock<LEN> {
    fn default() -> Self {
        let mut x = Self {
            buf: Align16([[[f32::zero(); LEN]; 2]; 2]), 
        };
        x.clear();
        x
    }
}

impl SurgeSynthesizer<'plugin_layer> {

    #[inline] pub fn set_amp_from_volume(&mut self) {
        let volume_db = pvalf![self.active_patch.params[PatchParam::Volume]];
        self.amp.set_target_smoothed(self.tables.db_to_linear(volume_db));
    }

    #[inline] pub fn set_amp_mute_from_masterfade(&mut self) {
        let masterfade = self.synth_out.masterfade();
        self.amp_mute.set_target(masterfade);
    }

    #[inline] pub fn maybe_apply_sends(&mut self) {
        let bypass_type = self.get_fx_bypass_type();

        if bypass_type == FxBypassType::AllFX {
            self.maybe_do_send_a();
            self.maybe_do_send_b();
        }
    }

    #[inline] pub fn sum_scenes(&mut self){

        let scene_a_l: *mut f32 = self.active_patch.scene[0].out.buf[0].as_mut_ptr();
        let scene_a_r: *mut f32 = self.active_patch.scene[0].out.buf[1].as_mut_ptr();

        let scene_b_l: *mut f32 = self.active_patch.scene[1].out.buf[0].as_mut_ptr();
        let scene_b_r: *mut f32 = self.active_patch.scene[1].out.buf[1].as_mut_ptr();

        copy_block(scene_a_l,       self.synth_out.out_l(), BLOCK_SIZE_QUAD);
        copy_block(scene_a_r,       self.synth_out.out_r(), BLOCK_SIZE_QUAD);
        accumulate_block(scene_b_l, self.synth_out.out_l(), BLOCK_SIZE_QUAD);
        accumulate_block(scene_b_r, self.synth_out.out_r(), BLOCK_SIZE_QUAD);
    }

    #[inline] pub fn apply_inserts<const N: usize>(&mut self, sc_a: &mut bool, sc_b: &mut bool) {

        let bypass_type = self.get_fx_bypass_type();

        // apply insert effects
        if bypass_type != FxBypassType::NoSendFX {

            let disable_flags = pvali![self.active_patch.params[PatchParam::FxDisable]];

            let scene_a_l: *mut f32 = self.active_patch.scene[0].out.buf[0].as_mut_ptr();
            let scene_a_r: *mut f32 = self.active_patch.scene[0].out.buf[1].as_mut_ptr();
            let scene_b_l: *mut f32 = self.active_patch.scene[1].out.buf[0].as_mut_ptr();
            let scene_b_r: *mut f32 = self.active_patch.scene[1].out.buf[1].as_mut_ptr();

            unsafe {
                if (disable_flags & (1 << 0)) == 0 {
                    *sc_a = self.fx_unit.fx[0].process_ringout::<N>(
                        scene_a_l, scene_a_r, *sc_a);
                }

                if (disable_flags & (1 << 1)) == 0 {
                    *sc_a = self.fx_unit.fx[1].process_ringout::<N>(
                        scene_a_l, scene_a_r, *sc_a);
                }

                if (disable_flags & (1 << 2))  == 0 {
                    *sc_b = self.fx_unit.fx[2].process_ringout::<N>(
                        scene_b_l, scene_b_r, *sc_b);
                }

                if (disable_flags & (1 << 3)) == 0 {
                    *sc_b = self.fx_unit.fx[3].process_ringout::<N>(
                        scene_b_l, scene_b_r, *sc_b);
                }
            }
        }
    }

    #[inline] pub fn get_fx_disable(&self) -> i32 {
        pvali![self.active_patch.params[PatchParam::FxDisable]]
    }

    #[inline] pub fn apply_effects<const N: usize>(&mut self, sc_a: bool, sc_b: bool, 
        sceneout: &mut TwoByTwoBlock::<BLOCK_SIZE_OS>,
        fxsendout: &mut TwoByTwoBlock::<BLOCK_SIZE_OS>) 
    {
        let bypass_type = self.get_fx_bypass_type();

        let mut added_send_a: bool = false;
        let mut added_send_b: bool = false;

        // add send effects
        if bypass_type == FxBypassType::AllFX {
            added_send_a = self.maybe_add_send_a::<N>(sc_a, sc_b, sceneout, fxsendout);
            added_send_b = self.maybe_add_send_b::<N>(sc_a, sc_b, sceneout, fxsendout);
        }

        // apply global effects
        match bypass_type {
            FxBypassType::AllFX | FxBypassType::NoSendFX => {

                let l: *mut f32 = self.synth_out.out_l();
                let r: *mut f32 = self.synth_out.out_r();

                let glob: bool = sc_a || sc_b || added_send_a || added_send_b;

                let fx_disable = self.get_fx_disable();

                unsafe {
                    //TODO: we shouldn't need to try_into().unwrap() below because 
                    //we should know N at compile time
                    if (fx_disable & (1 << 6)) == 0 { 
                        self.fx_unit.fx[6].process_ringout::<N>(l, r, glob);
                    }

                    if (fx_disable & (1 << 7)) == 0 { 
                        self.fx_unit.fx[7].process_ringout::<N>(l, r, glob);
                    }
                }
            },
            _ => {},
        }
    }

    #[inline] pub fn apply_amp(&mut self) {
        let l: *mut f32 = self.synth_out.out_l();
        let r: *mut f32 = self.synth_out.out_r();

        unsafe {
            self.amp.multiply_2_blocks(l, r, BLOCK_SIZE_QUAD);
        }
    }
    #[inline] pub fn apply_amp_mute(&mut self) {
        let l: *mut f32 = self.synth_out.out_l();
        let r: *mut f32 = self.synth_out.out_r();

        unsafe {
            self.amp_mute.multiply_2_blocks(l, r, BLOCK_SIZE_QUAD);
        }
    }

    pub fn maybe_do_send_a(&mut self) {

        let patch = &mut self.active_patch;

        if self.fx_unit.fx_enable[4] {

            let send_0a = 
                pvalf![patch.scene[0].params[SceneParam::SendLevelA]];

            let send_1a = 
                pvalf![patch.scene[1].params[SceneParam::SendLevelA]];

            let returnlevel = patch.fx[4].returnlevel();

            self.active_patch.scene[0].returnfx.set_target_smoothed( amp_to_linear( returnlevel ));

            self.active_patch.scene[0].send[0].set_target_smoothed( amp_to_linear( send_0a ));
            self.active_patch.scene[0].send[1].set_target_smoothed( amp_to_linear( send_1a ));
        }
    }

    pub fn maybe_do_send_b(&mut self) {

        let patch = &mut self.active_patch;

        if self.fx_unit.fx_enable[5] {

            let send_0b = 
                pvalf![patch.scene[0].params[SceneParam::SendLevelB]];

            let send_1b = 
                pvalf![patch.scene[1].params[SceneParam::SendLevelB]];

            let returnlevel = patch.fx[5].returnlevel();

            self.active_patch.scene[1].returnfx.set_target_smoothed( amp_to_linear( returnlevel ));

            self.active_patch.scene[1].send[0].set_target_smoothed( amp_to_linear( send_0b ));
            self.active_patch.scene[1].send[1].set_target_smoothed( amp_to_linear( send_1b ));
        }
    }

    #[inline] pub fn maybe_add_send_a<const N: usize>(&mut self,
        sc_a:     bool,
        sc_b:     bool,
        sceneout:  &mut TwoByTwoBlock::<BLOCK_SIZE_OS>,
        fxsendout: &mut TwoByTwoBlock::<BLOCK_SIZE_OS>) -> bool {

        let patch = &mut self.active_patch;

        let mut added_send_a: bool = false;

        if (pvali![patch.params[PatchParam::FxDisable]] & (1 << 4)) == 0
        { 
            unsafe {
                self.active_patch.scene[0].send[0].mac_2_blocks_to(
                    sceneout.buf[0][0].as_mut_ptr(), 
                    sceneout.buf[0][1].as_mut_ptr(), 
                    fxsendout.buf[0][0].as_mut_ptr(),
                    fxsendout.buf[0][1].as_mut_ptr(), 
                    BLOCK_SIZE_QUAD);

                self.active_patch.scene[0].send[1].mac_2_blocks_to(
                    sceneout.buf[1][0].as_mut_ptr(), 
                    sceneout.buf[1][1].as_mut_ptr(), 
                    fxsendout.buf[0][0].as_mut_ptr(), 
                    fxsendout.buf[0][1].as_mut_ptr(), 
                    BLOCK_SIZE_QUAD);

                added_send_a = self.fx_unit.fx[4].process_ringout::<N>(
                    fxsendout.buf[0][0].as_mut_ptr(), 
                    fxsendout.buf[0][1].as_mut_ptr(), 
                    sc_a || sc_b);

                self.active_patch.scene[0].returnfx.mac_2_blocks_to(
                    fxsendout.buf[0][0].as_mut_ptr(), 
                    fxsendout.buf[0][1].as_mut_ptr(), 
                    self.synth_out.out_l(), 
                    self.synth_out.out_r(),
                    BLOCK_SIZE_QUAD);
            }
        }

        added_send_a
    }

    #[inline] pub fn maybe_add_send_b<const N: usize>(&mut self, 
        sc_a: bool,
        sc_b: bool,
        sceneout:  &mut TwoByTwoBlock::<BLOCK_SIZE_OS>,
        fxsendout: &mut TwoByTwoBlock::<BLOCK_SIZE_OS>) -> bool {

        let patch = &mut self.active_patch;

        let mut added_send_b: bool = false;

        if (pvali![patch.params[PatchParam::FxDisable]] & (1 << 5)) == 0
        { 
            unsafe {
                self.active_patch.scene[1].send[0].mac_2_blocks_to(
                    sceneout.buf[0][0].as_mut_ptr(), 
                    sceneout.buf[0][1].as_mut_ptr(), 
                    fxsendout.buf[1][0].as_mut_ptr(), 
                    fxsendout.buf[1][1].as_mut_ptr(), 
                    BLOCK_SIZE_QUAD);

                self.active_patch.scene[1].send[1].mac_2_blocks_to(
                    sceneout.buf[1][0].as_mut_ptr(), 
                    sceneout.buf[1][1].as_mut_ptr(), 
                    fxsendout.buf[1][0].as_mut_ptr(), 
                    fxsendout.buf[1][1].as_mut_ptr(), 
                    BLOCK_SIZE_QUAD);

                added_send_b = self.fx_unit.fx[5].process_ringout::<N>(
                    fxsendout.buf[1][0].as_mut_ptr(), 
                    fxsendout.buf[1][1].as_mut_ptr(), 
                    sc_a || sc_b);

                self.active_patch.scene[1].returnfx.mac_2_blocks_to(
                    fxsendout.buf[1][0].as_mut_ptr(), 
                    fxsendout.buf[1][1].as_mut_ptr(), 
                    self.synth_out.out_l(), 
                    self.synth_out.out_r(), 
                    BLOCK_SIZE_QUAD);
            }
        }

        added_send_b
    }

    #[inline] pub fn do_halt(&mut self) {
        unsafe {
            clear_block::<BLOCK_SIZE_QUAD>(self.synth_out.out_l());
            clear_block::<BLOCK_SIZE_QUAD>(self.synth_out.out_r());
        }
    }

    #[inline] pub fn do_process_inputs(&mut self) {

        unsafe {
            //process inputs (upsample & halfrate);
            hardclip_block8(
                self.synth_in.in_left(), 
                BLOCK_SIZE_QUAD);

            hardclip_block8(
                self.synth_in.in_right(), 
                BLOCK_SIZE_QUAD);
        }

        copy_block(
            self.synth_in.in_left(), 
            self.synth_in.non_os_audio_in_left(),    
            BLOCK_SIZE_QUAD);

        copy_block(
            self.synth_in.in_right(), 
            self.synth_in.non_os_audio_in_right(),    
            BLOCK_SIZE_QUAD);

        self.halfband_in.process_block_upsample_by_two(
            self.synth_in.in_left(), 
            self.synth_in.in_right(), 
            self.synth_in.audio_in_left(), 
            self.synth_in.audio_in_right(),
            Some(BLOCK_SIZE_QUAD));
    }

    #[inline] fn clear_inputs(&mut self) {
        unsafe {
            clear_block_antidenormalnoise::<BLOCK_SIZE_OS_QUAD>(self.synth_in.audio_in_left());
            clear_block_antidenormalnoise::<BLOCK_SIZE_OS_QUAD>(self.synth_in.audio_in_right());
            clear_block_antidenormalnoise::<BLOCK_SIZE_QUAD>(self.synth_in.non_os_audio_in_right());
            clear_block_antidenormalnoise::<BLOCK_SIZE_QUAD>(self.synth_in.non_os_audio_in_right());
        }
    }

    fn process<const N: usize>(&mut self) {

        if self.controller.halt_engine {
            self.do_halt();
            return;
        }

        if self.patchid_queue >= Some(0) {
            self.handle_patchid_queue();
        }

        match self.controller.process_input {
            true  => self.do_process_inputs(),
            false => self.clear_inputs(),
        }

        let mut sceneout  = TwoByTwoBlock::<BLOCK_SIZE_OS>::default();
        let mut fxsendout = TwoByTwoBlock::<BLOCK_SIZE_OS>::default();

        let (playscene, fb_entry) = {

            let _critical_section = CS_MOD_ROUTING.lock().unwrap();

            self.process_control();
            self.set_amp_from_volume();
            self.set_amp_mute_from_masterfade();
            self.maybe_apply_sends();

            let playscene: [bool; 2] = [
                self.active_patch.scene[0].playscene(),
                self.active_patch.scene[1].playscene(),
            ];

            let mut fb_entry: [i32; 2] = [0; 2];
            let mut vcount = 0;

            (fb_entry[0], vcount) = self.active_patch.scene[0].do_routing_critical_secion(fb_entry[0], vcount);
            (fb_entry[1], vcount) = self.active_patch.scene[1].do_routing_critical_secion(fb_entry[1], vcount);

            self.controller.polydisplay.store(vcount, atomic::Ordering::SeqCst);

            (playscene, fb_entry)
        };

        self.active_patch.scene[0].process(fb_entry[0]);
        self.active_patch.scene[1].process(fb_entry[1]);

        if playscene[0] {
            self.active_patch.scene[0].post_process();
        }

        if playscene[1] {
            self.active_patch.scene[1].post_process();
        }

        self.active_patch.scene[0].apply_highpass();
        self.active_patch.scene[1].apply_highpass();

        let mut sc_a: bool = playscene[0];
        let mut sc_b: bool = playscene[1];

        self.apply_inserts::<N>(&mut sc_a, &mut sc_b);
        self.sum_scenes();
        self.apply_effects::<N>(sc_a, sc_b, &mut sceneout, &mut fxsendout);

        self.apply_amp();
        self.apply_amp_mute();

        let vu_falloff: f32 = self.srunit.vu_falloff();

        let mut vu_peak0 = self.synth_out.get_vu_peak(0);
        let mut vu_peak1 = self.synth_out.get_vu_peak(1);

        self.synth_out.set_vu_peak(0,minf(2.0, vu_falloff * vu_peak0));
        self.synth_out.set_vu_peak(1,minf(2.0, vu_falloff * vu_peak1));

        vu_peak0 = self.synth_out.get_vu_peak(0);
        vu_peak1 = self.synth_out.get_vu_peak(1);

        let out_l = self.synth_out.out_l();
        let out_r = self.synth_out.out_r();

        self.synth_out.set_vu_peak(0,maxf(
                vu_peak0, 
                get_absmax(out_l, BLOCK_SIZE_QUAD)
        ));

        self.synth_out.set_vu_peak(1,maxf(
            vu_peak1, 
            get_absmax(out_r, BLOCK_SIZE_QUAD)
        ));

        unsafe {
            hardclip_block8(out_l, BLOCK_SIZE_QUAD);
            hardclip_block8(out_r, BLOCK_SIZE_QUAD);
        }
    }
}
