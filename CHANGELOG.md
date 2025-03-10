# Depthai Viewer changelog


## 0.2.9
- Added RVC4 is not supported pop up.

## 0.2.8
- Fix viewer getting flagged as trojan on windows.

## 0.2.7
- Added stereo depth presets!
- Fixed syncing
- Support latest OAK-T

## 0.2.6
- Fix regression: Without an active stereo config the viewer would crash

## 0.2.5
- Fixed depth aligned camera streams not being rectified and undistorted. Resulting in misaligned visualisation.

## 0.2.4

- Fix OAK-T not streaming.

## 0.2.3

- Fixed https://github.com/luxonis/depthai-viewer/issues/66 , OAK-D-SR not starting correctly.
- Pinned numpy>=1.23,<2.0.0, because numpy 2.0.0 breaks depthai.

## 0.2.2

- Added `--viewer-mode` flag. Mostly intended for internal use, when using the viewer as if it was stock rerun (doesn't install the dependencies and doesn't try to connect to a device.). main benefit over stock rerun are albedo textured depth clouds.

## 0.2.1

- Fixed point cloud coloring for RGB albedo textures.
- Better device defaults.
- Improved OAK-D-SR-POE support.
- Fixed temperature readings on OAK-T.

## 0.2.0

- Fixed point cloud colormap normalization
- Improved the UX for OAK-D-SR-POE

## 0.1.9

- Added the new ToF decoding improvements.
- The default AI model is now Yolo V6 instead of mobilenet-ssd.
- Fixed OAK-D-SR-POE crashing on startup.
- Fixed deformed bounding boxes.

## 0.1.8

- Performance fix.

## 0.1.7

- Added support for OAK-T
- Support multi viewer windows at the same time: `depthai-viewer --port <your-port>`

## 0.1.6

- Added ability to control IR dot projector and flood light brightness.

## 0.1.5

- UI improvements
- Added support for OAK-D-SR POE (tof)
- Fixed some bugs:
  - https://discuss.luxonis.com/d/2425-depthai-viewer-not-working-with-ffc-4p-based-setup/3
  - Blob download failure in dependency installer

## 0.1.4

- Added depdendency installer.
- Added auto device selection on startup:
  - On startup: If a preferred device is set, try to connect to that device. Else (if preferred device not found or not set) connect to the first available device.
  - Whenever a device is selected it is designated as the preferred device. A preferred device is persisted between runs of depthai-viewer.

## 0.1.4-alpha.0

- Added dependency installer.
- Added auto device selection (+preferred device) on initial startup.

## 0.1.3

- Fix default_stereo_pairs for OAK-1 MAX

## 0.1.2

- Fix stereo depth creation for OAK-D-Pro-W 97, and potentially others

## 0.1.1

- Fixes Metal shader compilation on MacOS
- Performance improvement - only create a camera node when a camera is actually needed in any way.
- Added sentry.io error reporting
- Update Windows and MacOS app icons

## 0.1.0

Depthai Viewer official Beta release on pypi!

- Performance improvements
- Better Auto Layouts
- Tweaks to UI for a better UX

## 0.0.8-alpha.0

- Pre-release

## 0.0.7

- Install depthai_sdk from artifactory
- Change logos

## 0.0.6

- App startup bugfixes

## 0.0.5

- App startup bugfixes
- Better default focusing in 3d views

## 0.0.4

- Disable depth settings if intrinsics aren't available.
- App startup bugfixes.

## 0.0.3

- Added support for all devices.

## 0.0.2

Patch release.

## 0.0.1

Beta release of the new Depthai Viewer.
