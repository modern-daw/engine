import 'dart:ffi' as ffi;
import 'dart:io' show Platform, Directory;
import 'package:path/path.dart' as path;

typedef audio_engine_func = ffi.Void Function();
typedef AudioEngine = void Function();

var libraryPath = path.join(Directory.current.path,
    'lib/audio_engine/libopendaw_engine', 'libopendaw_engine.so');

final dylib = ffi.DynamicLibrary.open(libraryPath);
final AudioEngine play =
    dylib.lookup<ffi.NativeFunction<audio_engine_func>>('play').asFunction();
