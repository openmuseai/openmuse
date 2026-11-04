"""Package an independently named ONNX library; never shadow another plugin."""
import hashlib
import json
import os
from pathlib import Path
import struct
import urllib.request
import zipfile

ROOT = Path(__file__).resolve().parents[1]


def rename_soname(data, name):
    data = bytearray(data)
    if data[:6] != b'\x7fELF\x02\x01':
        raise ValueError('Expected a 64-bit little-endian ELF library')
    offset = struct.unpack_from('<Q', data, 32)[0]
    size, count = struct.unpack_from('<HH', data, 54)
    segments = [struct.unpack_from('<IIQQQQQQ', data, offset + i * size) for i in range(count)]
    dynamic = next(segment for segment in segments if segment[0] == 2)
    tags = dict(struct.unpack_from('<qQ', data, i) for i in range(dynamic[2], dynamic[2] + dynamic[5], 16))
    strings = tags[5]
    load = next(segment for segment in segments if segment[0] == 1 and segment[3] <= strings < segment[3] + segment[5])
    start = load[2] + strings - load[3] + tags[14]
    end = data.index(0, start)
    if bytes(data[start:end]) != b'libonnxruntime.so' or len(name.encode()) > end - start:
        raise ValueError('Unexpected ONNX ELF identity')
    data[start:end] = name.encode().ljust(end - start, b'\0')
    return bytes(data)


def prepare():
    spec = json.loads((ROOT / 'native/android-onnx.json').read_text())
    cache = Path(os.path.expanduser(os.environ.get('GRADLE_USER_HOME', '~/.gradle')))
    candidates = list((cache / f'caches/modules-2/files-2.1/com.microsoft.onnxruntime/onnxruntime-android/{spec["version"]}').glob('*/*.aar'))
    package = next((path for path in candidates if hashlib.sha256(path.read_bytes()).hexdigest() == spec['sha256']), None)
    if package is None:
        package = ROOT / '.build' / f'onnxruntime-android-{spec["version"]}.aar'
        package.parent.mkdir(parents=True, exist_ok=True)
        if not package.exists() or hashlib.sha256(package.read_bytes()).hexdigest() != spec['sha256']:
            temporary = package.with_suffix('.downloading')
            urllib.request.urlretrieve(spec['url'], temporary)
            if hashlib.sha256(temporary.read_bytes()).hexdigest() != spec['sha256']:
                temporary.unlink()
                raise ValueError('ONNX Runtime archive digest mismatch')
            temporary.replace(package)
    with zipfile.ZipFile(package) as archive:
        original = archive.read('jni/arm64-v8a/libonnxruntime.so')
    binary = rename_soname(original, spec['library'])
    output = ROOT / 'flutter_runtime/android/src/main/jniLibs/arm64-v8a' / spec['library']
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_bytes(binary)
    (output.parent / 'onnxruntime.source.json').write_text(json.dumps({**spec, 'artifactSha256': hashlib.sha256(binary).hexdigest()}, indent=2) + '\n')
    print(output, flush=True)


if __name__ == '__main__':
    prepare()
