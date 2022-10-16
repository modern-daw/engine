

pub fn second_try() {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use cpal::{Data, Sample, SampleFormat};

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let sample_format = supported_config.sample_format();
    let config = supported_config.into();
    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(&config, write_silence::<f32>, err_fn),
        SampleFormat::I16 => device.build_output_stream(&config, write_silence::<i16>, err_fn),
        SampleFormat::U16 => device.build_output_stream(&config, write_silence::<u16>, err_fn),
    }
    .unwrap();




    
    
    
    fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    
        use std::fs::File;
        use std::io::BufReader;
        use rodio::{Decoder, OutputStream, source::Source};
    
        // Get a output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open("assets/beep.wav").unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
    
        let samples = source.convert_samples()
        for sample in data.iter_mut() {
            //*sample = samples.;
        }
    }

    let res = stream.play();
    let error = res.err();
    let no_error = error.is_none();
    println!("No errors: {}", no_error);
}
