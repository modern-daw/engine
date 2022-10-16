cargo build --target-dir ./native/target
mkdir -p ./lib/audio_engine/libopendaw_engine
cp ./native/target/debug/libopendaw_engine.so ./lib/audio_engine/libopendaw_engine/libopendaw_engine.so