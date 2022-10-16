pub fn first_try() {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use cpal::Data;

    // Finding host
    let host = cpal::default_host();

    // Finding device
    let device = host
        .default_output_device()
        .expect("no output device available");

    // Selecting a config
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let config = supported_config.into();

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // react to stream events and read or write stream data here.
        },
        move |err| {
            // react to errors here.
        },
    );

    let _res = stream.and_then(|stream| {
        stream.play();
        Ok(1)
    });
}
