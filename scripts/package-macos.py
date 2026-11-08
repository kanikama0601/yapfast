#!/usr/bin/env python3
"""Package the macOS release: Yapfast.app, with the dependencies' licenses and
notices, BUILD.json and SOURCE.txt in its Resources, signed ad hoc and put in
a disk image beside the source snapshot.

scripts/build-macos builds the executable and runs it on a Mac."""
import argparse
import json
import os
from pathlib import Path
import plistlib
import shutil
import subprocess
import tempfile

from release_package import REPOSITORY, ROOT, dependency_notices, digest, names_home, source_snapshot

TARGET = "aarch64-apple-darwin"
FEATURES = ["--no-default-features", "--features", "desktop,discord"]
# Rust's own minimum for the target, which scripts/build-macos builds for.
MINIMUM_SYSTEM = "11.0"
# Each size Finder and the Dock draw, at 1× and 2×.
ICON_SIZES = [16, 32, 128, 256, 512]


def webrtc_folder():
    """LiveKit's libwebrtc, which webrtc-sys-build downloaded for the build."""
    folders = sorted(path.parent for path in (ROOT / "target/release/build").glob(
        "scratch-*/out/livekit_webrtc/livekit/mac-arm64-release-*/mac-arm64-release/LICENSE.md"))
    if len(folders) != 1:
        raise SystemExit(f"Expected one downloaded macOS libwebrtc, found {folders}")
    return folders[0]


def icon(destination, temporary):
    """Draws packaging/icon.svg at each size into an .icns file."""
    iconset = temporary / "Yapfast.iconset"
    iconset.mkdir()
    for size in ICON_SIZES:
        for scale, suffix in ((1, ""), (2, "@2x")):
            subprocess.run(["rsvg-convert", "--width", str(size * scale), "--height", str(size * scale),
                            "--output", iconset / f"icon_{size}x{size}{suffix}.png", ROOT / "packaging/icon.svg"],
                           check=True, timeout=60)
    subprocess.run(["iconutil", "--convert", "icns", "--output", destination, iconset], check=True, timeout=60)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--exe", type=Path, default=ROOT / "target" / TARGET / "release/yapfast")
    parser.add_argument("--skia-source", type=Path, required=True,
                        help="Skia's source with its third-party code, for their notices")
    parser.add_argument("--output", type=Path, default=ROOT / "target/dist/macos")
    args = parser.parse_args()
    executable = args.exe.resolve()
    architectures = subprocess.check_output(["lipo", "-archs", executable], text=True, timeout=30).split()
    if architectures != ["arm64"]:
        raise SystemExit(f"Expected an arm64 executable, not {architectures}")
    contents = executable.read_bytes()
    if names_home(contents):
        raise SystemExit(f"{executable} names {Path.home()}; build it with scripts/build-macos")
    if b"/matrix-rust-sdk-" in contents:
        raise SystemExit(f"{executable} has Matrix; the macOS release is the Discord flavour")
    del contents
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--locked", "--format-version", "1", "--filter-platform", TARGET, *FEATURES],
        cwd=ROOT, timeout=60,
    ))
    version = next(package["version"] for package in metadata["packages"] if package["name"] == "yapfast")
    output = args.output
    output.mkdir(parents=True, exist_ok=True)
    base = f"Yapfast-{version}"
    image = output / f"{base}-macos-arm64.dmg"
    source_path = output / f"{base}-source.zip"
    with tempfile.TemporaryDirectory(prefix="yapfast-macos-package-") as temporary:
        temporary = Path(temporary)
        if source_snapshot(metadata, base, temporary, source_path):
            raise SystemExit("The macOS release is the Discord flavour, which has no AGPL dependencies")

        volume = temporary / "Yapfast"
        app = volume / "Yapfast.app"
        macos = app / "Contents/MacOS"
        resources = app / "Contents/Resources"
        macos.mkdir(parents=True)
        resources.mkdir()
        shutil.copy2(executable, macos / "yapfast")
        icon(resources / "Yapfast.icns", temporary)
        # The version Finder shows takes only numbers, without a nightly's suffix.
        number = version.split("-")[0]
        with (app / "Contents/Info.plist").open("wb") as stream:
            plistlib.dump({
                "CFBundleDevelopmentRegion": "en",
                "CFBundleDisplayName": "Yapfast",
                "CFBundleExecutable": "yapfast",
                "CFBundleIconFile": "Yapfast",
                "CFBundleIdentifier": "io.github.yapfast.Yapfast",
                "CFBundleInfoDictionaryVersion": "6.0",
                "CFBundleName": "Yapfast",
                "CFBundlePackageType": "APPL",
                "CFBundleShortVersionString": number,
                "CFBundleVersion": number,
                "LSApplicationCategoryType": "public.app-category.social-networking",
                "LSMinimumSystemVersion": MINIMUM_SYSTEM,
                "NSHighResolutionCapable": True,
                # Asked for when a call first opens the microphone or camera.
                "NSMicrophoneUsageDescription": "Yapfast sends your voice in calls.",
                "NSCameraUsageDescription": "Yapfast sends your camera in video calls.",
            }, stream)

        shutil.copy2(ROOT / "LICENSE", resources / "LICENSE.txt")
        (resources / "SOURCE.txt").write_text(
            f"The source code of this build of Yapfast {version} is published with it:\n\n"
            f"  https://github.com/{REPOSITORY}/releases/tag/v{version}\n"
            f"  {source_path.name}, SHA-256 in BUILD.json\n\n"
            f"and in the repository, https://github.com/{REPOSITORY}. The licenses folder\n"
            "holds every dependency's own license and notices.\n",
            encoding="utf-8",
        )
        webrtc = webrtc_folder()
        dependencies = dependency_notices(metadata, resources, resources / "licenses", None,
                                          args.skia_source, webrtc)
        (resources / "THIRD-PARTY.json").write_text(json.dumps(dependencies, indent=2) + "\n", encoding="utf-8")
        build = {
            "version": version, "flavour": "discord", "target": TARGET,
            "cargo_features": " ".join(FEATURES),
            "rustc": subprocess.check_output(["rustc", "--version"], text=True, timeout=10).strip(),
            "macos_minimum": MINIMUM_SYSTEM,
            "sdk": subprocess.check_output(["xcrun", "--show-sdk-version"], text=True, timeout=10).strip(),
            "ui_framework": "Slint 1.18.1",
            "skia_version": "0.153.3",
            "skia_binaries": "rust-skia's prebuilt archive",
            "webrtc": webrtc.parent.name,
            "cargo_lock_sha256": digest(ROOT / "Cargo.lock"),
            "executable_sha256": digest(executable),
            "source_archive_sha256": digest(source_path),
            "signature": "ad hoc, not notarized",
        }
        (resources / "BUILD.json").write_text(json.dumps(build, indent=2) + "\n", encoding="utf-8")

        # Apple silicon runs only signed code. An ad hoc signature seals the
        # bundle's files without an identity; Gatekeeper still asks the first
        # time, as for any app from outside the App Store that isn't notarized.
        subprocess.run(["codesign", "--force", "--sign", "-", "--timestamp=none", app], check=True, timeout=300)
        subprocess.run(["codesign", "--verify", "--strict", "--deep", app], check=True, timeout=300)
        (volume / "Applications").symlink_to("/Applications")
        image.unlink(missing_ok=True)
        subprocess.run(["hdiutil", "create", "-volname", f"Yapfast {version}", "-srcfolder", volume,
                        "-fs", "HFS+", "-format", "ULMO", "-ov", image], check=True, timeout=1800)
        subprocess.run(["hdiutil", "verify", image], check=True, timeout=600)

    hashes = output / "SHA256SUMS.txt"
    artifacts = [image, source_path]
    hashes.write_text("".join(f"{digest(path)}  {path.name}\n" for path in artifacts), encoding="utf-8")
    print(f"Packaged {image} ({image.stat().st_size / 1024 / 1024:.1f} MiB)")
    print(f"Source snapshot: {source_path}")
    print(f"Checksums: {hashes}")


if __name__ == "__main__":
    main()
