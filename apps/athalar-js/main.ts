import { existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

const { platform, arch } = process;

let nativeBinding = null;
let loadError = null;

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      return readFileSync('/usr/bin/ldd', 'utf8').includes('musl');
    } catch (e) {
      return true;
    }
  } else {
    const { glibcVersionRuntime } = (process.report.getReport() as any).header;
    return !glibcVersionRuntime;
  }
}

const getNativeBinding = (platform: string) => {
  let binding = null;
  let error = null;
  const localBindingExists = existsSync(
    join(__dirname, `athalar-js.${platform}.node`)
  );
  try {
    if (localBindingExists) {
      binding = require(`./athalar-js.${platform}.node`);
    } else {
      binding = require(`@ignisda/athalar-${platform}`);
    }
  } catch (e) {
    error = e;
  }
  return [binding, error];
};

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        [nativeBinding, loadError] = getNativeBinding('android-arm64');
        break;
      case 'arm':
        [nativeBinding, loadError] = getNativeBinding('android-arm-eabi');
        break;
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`);
    }
    break;
  case 'win32':
    switch (arch) {
      case 'x64':
        [nativeBinding, loadError] = getNativeBinding('win32-x64-msvc');
        break;
      case 'ia32':
        [nativeBinding, loadError] = getNativeBinding('win32-ia32-msvc');
        break;
      case 'arm64':
        [nativeBinding, loadError] = getNativeBinding('win32-arm64-msvc');
        break;
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`);
    }
    break;
  case 'darwin':
    switch (arch) {
      case 'x64':
        [nativeBinding, loadError] = getNativeBinding('darwin-x64');
        break;
      case 'arm64':
        [nativeBinding, loadError] = getNativeBinding('darwin-arm64');
        break;
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`);
    }
    break;
  case 'freebsd':
    if (arch !== 'x64')
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`);
    [nativeBinding, loadError] = getNativeBinding('freebsd-x64');
    break;
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl())
          [nativeBinding, loadError] = getNativeBinding('linux-x64-musl');
        else [nativeBinding, loadError] = getNativeBinding('linux-x64-gnu');
        break;
      case 'arm64':
        if (isMusl())
          [nativeBinding, loadError] = getNativeBinding('linux-arm64-musl');
        else [nativeBinding, loadError] = getNativeBinding('linux-arm64-gnu');
        break;
      case 'arm':
        [nativeBinding, loadError] = getNativeBinding('linux-arm-gnueabihf');
        break;
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`);
    }
    break;
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`);
}

if (!nativeBinding) {
  if (loadError) throw loadError;
  throw new Error(`Failed to load native binding`);
}

module.exports = nativeBinding;
