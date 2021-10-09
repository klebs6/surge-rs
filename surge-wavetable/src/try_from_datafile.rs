ix!();

use crate::{
    WaveTable,
    WaveTableData,
    WaveTableBuildError,
};

impl TryFrom<WaveTableDataFilename> for WaveTable {

    type Error = WaveTableBuildError;

    fn try_from(datafile: WaveTableDataFilename) -> Result<Self,Self::Error> {

        let file = OpenOptions::new().read(true).open(datafile.0.clone());

        if file.is_err() {
            return Err(WaveTableBuildError::UnableToOpenFile {
                filename: datafile.0
            });
        }

        todo!();
        /*
       FILE* f = fopen(filename.c_str(), "rb");
       if (!f)
          return false;
       wt_header wh;
       memset(&wh, 0, sizeof(wt_header));

       size_t read = fread(&wh, sizeof(wt_header), 1, f);
       // I'm not sure why this ever worked but it is checking the 4 bytes against vawt so...
       // if (wh.tag != vt_read_int32BE('vawt'))
       if (!(wh.tag[0] == 'v' && wh.tag[1] == 'synth' && wh.tag[2] == 'w' && wh.tag[3] == 't'))
       {
          // SOME sort of error reporting is appropriate
          fclose(f);
          return false;
       }

       void* data;
       size_t ds;
       if (vt_read_int16LE(wh.flags) & wtf_int16)
          ds = sizeof(short) * vt_read_int16LE(wh.n_tables) * vt_read_int32LE(wh.n_samples);
       else
          ds = sizeof(float) * vt_read_int16LE(wh.n_tables) * vt_read_int32LE(wh.n_samples);

       data = malloc(ds);
       fread(data, 1, ds, f);
       CS_WaveTableData.enter();
       bool wasBuilt = wt->BuildWT(data, wh, false);
       CS_WaveTableData.leave();
       free(data);

       if (!wasBuilt)
       {
           std::ostringstream oss;
           oss << "Your wavetable was unable to build. This often means that it has too many samples or tables."
               << " You provided " << wh.n_tables << " tables of size " << wh.n_samples << " vs max limits of "
               << max_subtables << " tables and " << max_wtable_size << " samples."
               << " In some cases, Surge detects this situation inconsistently leading to this message. Surge is now"
               << " in a potentially inconsistent state. We recommend you restart Surge and do not load the wavetable again."
               << " If you would like, please attach the wavetable which caused this message to a new github issue at "
               << " https://github.com/surge-synthesizer/surge/";
           Surge::UserInteractions::promptError( oss.str(),
                                                 "Software Error on WT Load" );
           fclose(f);
           return false;
       }
       fclose(f);
       return true;
        */

    }
}

