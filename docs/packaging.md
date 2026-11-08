# Packaging and releases

How the Linux, Windows and macOS releases are made and published. Building and
checking Yapfast for development is in [building](building.md).

## Linux AppImage

`scripts/build-appimage` makes the Linux release in `target/dist/linux`:
`Yapfast-VERSION-x86_64.AppImage`, its `.zsync` file, the source snapshot and
`SHA256SUMS.txt`. All four go on the release page. AppImageUpdate, and AppImage
managers such as Gear Lever, find a newer release through the `.zsync` file,
named by the update information in the runtime:
`gh-releases-zsync|yapfast|yapfast|latest|Yapfast-*-x86_64.AppImage.zsync`.
The first build compiles every dependency in the Nix sandbox; packing takes
about 10 seconds after that.

Nix builds the binary, `nix/package.nix`'s Discord-only release build, and
`nix/appimage.nix` puts it in an AppDir with the desktop file, the icons and
the AppStream metainfo, `packaging/linux/io.github.yapfast.Yapfast.metainfo.xml`.
It sets the binary's interpreter to `/lib64/ld-linux-x86-64.so.2` and removes
Nix's RUNPATH, so the binary runs on the host's loader, glibc and libstdc++, as
one built on a distribution would. That's what lets the host's GPU driver,
Mesa or NVIDIA's, load into it: a driver needs the glibc it was built for, and
NVIDIA's must match the kernel module, so neither can come in the AppImage.
Nothing else is bundled either. Besides the C and C++ runtimes, the binary
links only GLib, PulseAudio's client, fontconfig and FreeType, which every
desktop has, and it loads EGL, Vulkan, Wayland, X11 and PipeWire at runtime.

The derivation fails if the binary links anything else, or asks for a glibc
newer than 2.39 or a libstdc++ newer than GCC 14's. That covers Ubuntu 24.04,
Fedora 40, Debian 13 and anything newer. nixpkgs' glibc is newer than that, so
a nixpkgs update can raise what the binary asks for. Raise the floors in
`nix/appimage.nix` only knowing which distributions that drops. Older ones,
such as Ubuntu 22.04, would need a build against their own glibc.

`scripts/package-linux.py`, which runs in the development shell, with
squashfs-tools and zsync added from the shell's nixpkgs, adds the
licenses and notices, `THIRD-PARTY.json`, `SOURCE.txt` and `BUILD.json` to
`usr/share/doc/yapfast`, as the Windows package has them, through the helpers
in `scripts/release_package.py` that both packaging scripts use. Skia's
prebuilt Linux archive carries only Skia's own license, so the notices of its
third-party code come from the pinned Skia source,
`nix build path:./nix#skia-source`. Then it packs the AppDir with mksquashfs
(zstd level 19, 1 MiB blocks, fixed owners and times, so the same AppDir packs
to the same file) behind AppImage's official type-2 runtime, pinned by hash.
The runtime is static: it mounts the image with FUSE 3's `fusermount3` and
needs no libfuse, and `--appimage-extract-and-run` runs it without FUSE at all.
The flake sees only the files Git tracks, so add a new file under `nix/` or
`packaging/` before building.

Screen sharing hasn't been tried from the AppImage.

## Arch Linux packages

`packaging/arch/PKGBUILD` builds the Discord-only release from source with
Arch's toolchain, as `nix/package.nix` builds it: `cd packaging/arch && makepkg
-si`. It clones the tag of its `pkgver`, which `scripts/set-version` sets to a
stable version, so run `scripts/set-version 0.2.0` before committing and
tagging `v0.2.0`. The tag's commit holds the PKGBUILD, which can't hold its own
checksum, so that source has none; the libwebrtc and Skia archives keep theirs.
Arch's `llvm21` is a later 21.x than Nix's, and a native Windows host's LLVM
is often newer still, which is why `nix/prepare-skia-native.py` takes LLVM 21
or later. Nothing builds this PKGBUILD automatically: build it in a clean
`archlinux:base-devel` container before a release, about 6 minutes from cold.

The releases carry a package of their own binary instead. `scripts/package-arch`,
run after `scripts/build-appimage`, packs the AppImage's binary, desktop file,
icon, metainfo and notices into `yapfast-VERSION-1-x86_64.pkg.tar.zst` beside
it, through `packaging/arch/release.PKGBUILD`, and adds it to
`SHA256SUMS.txt`. It's the AppImage's binary, which runs on the host's loader
and libraries, so the two packages can't differ. The script runs makepkg in an
`archlinux:base-devel` container with Docker, then installs the package there
and runs `yapfast --smoke-test`, in about 20 seconds. A nightly's version
becomes `0.2.0nightly.20261020.5`, which pacman sorts after 0.1.0 and before
0.2.0, as SemVer sorts the nightly; with a dot before `nightly`, pacman would
sort it after 0.2.0. `sudo pacman -U` installs the file or its link; there's no
repository, so `pacman -Syu` doesn't update it.

## macOS disk image

`scripts/build-macos SKIA_SOURCE` makes the macOS release on a Mac, for Apple
silicon, in `target/dist/macos`: `Yapfast-VERSION-macos-arm64.dmg` and the
source snapshot. It builds the Discord flavour for `aarch64-apple-darwin` with
`scripts/release-env`'s path mappings and macOS 11 as the oldest system, Rust's
own floor for the target. Rust-skia's prebuilt Skia for Metal and OpenGL and
LiveKit's prebuilt libwebrtc come from their crates' downloads; FFmpeg's
hardware encoders have no macOS configuration, so the build goes without them,
as the build scripts allow for any target without a `gen/` folder.
`scripts/package-macos.py` puts the executable in `Yapfast.app` with an
`Info.plist` (bundle ID `io.github.yapfast.Yapfast`, and the reasons macOS
shows when a call first opens the microphone or camera), the icon drawn from
`packaging/icon.svg` by `rsvg-convert` and `iconutil`, and the licenses,
notices, `THIRD-PARTY.json`, `SOURCE.txt` and `BUILD.json` in
`Contents/Resources`. Rust-skia's archive isn't kept by its download, so Skia's
notices, its own among them, come from the pinned Skia source: SKIA_SOURCE is
a folder of its notices, which Linux's `nix build path:./nix#skia-source`
makes. The bundle is signed ad hoc, since Apple silicon runs only signed code,
and isn't notarized, which takes a paid Apple account: macOS asks before
opening it the first time, after a Control-click, Open. `hdiutil` packs it
with a link to `/Applications` into an LZMA disk image. Nobody has run this
build on a Mac yet.

## Releases on GitHub

`.github/workflows/build.yml` builds one commit at one version for every
system, for the two workflows that publish: `scripts/set-version` writes the
version into `Cargo.toml` and `Cargo.lock` before each build, so the app
reports it and the packages and `SOURCE.txt` name it. One job runs
`scripts/build-appimage` and `scripts/package-arch`, another
`scripts/build-windows` in the Windows shell, with the Windows Skia build,
cargo-xwin's MSVC files and the compiled Rust dependencies
(`Swatinem/rust-cache`) cached between runs. The Linux build has no cache: it
compiles in the Nix sandbox, which can't reach one. Wine doesn't start on the
runner, so a job on a Windows runner compiles the installer from the Windows
ZIP, with the Inno Setup `scripts/package-windows.py` pins and the
definitions it passes. A job on a macOS runner (Apple silicon) runs
`scripts/build-macos`, with Skia's notices from a small Linux job that builds
`path:./nix#skia-source`. Every job zips the source snapshot with the commit's
time on every file, so they make the same file;
`.github/scripts/collect-release` checks that, keeps one, and writes one
`SHA256SUMS.txt` for the AppImage, its `.zsync` file, the Arch package, the
Windows installer and ZIP, the macOS disk image and the source snapshot.
`SOURCE.txt` links to the repository the workflow runs in.

Every push to `main` releases it, through `.github/workflows/release.yml`;
Actions, Release, Run workflow releases it by hand. The commits since the
latest release, or before the repository has one, since the commit that set
the current version, choose the version, as Conventional Commits ask: a
breaking change (`feat!:`, or a `BREAKING CHANGE:` line) raises the major
version, or the minor one before 1.0, a `feat` the minor one, and anything
else the patch, so from 0.3.0 a feature makes 0.4.0 and a fix 0.3.1. Run by
hand, it can be told `patch`, `minor` or `major` instead. Merges and the
release's own commits don't count, and with nothing else since the latest
release it releases nothing. One release runs at a time: pushes meanwhile
wait, and the newest of them releases all they brought. It builds that
version, and only once every build has passed commits it to `main` as
`chore(release): VERSION`, with the metainfo's release and the PKGBUILD's
`pkgver` that `scripts/set-version` also writes, tags the commit `vVERSION`
and publishes the files as the latest release. The workflow's own pushes
start no workflow. If commits reached `main` during the build, a merge puts
the release's commit in its history, and the push that brought them
releases them next. The notes, and the release's entry at the top of
`CHANGELOG.md` in the same commit, list the commits since the previous
release, each with its author's GitHub account: features and fixes by their
Conventional Commit type, the rest folded away (`.github/scripts/change-list`,
by `.github/release-notes.jq`).

A nightly of `main` is Actions, Nightly, Run workflow.
`.github/workflows/nightly.yml` names it after the next minor version, the day
(UTC) and the workflow's run number, such as `v0.2.0-nightly.20261020.5`, and
titles it as the stable releases are, with the commit: `Yapfast
0.2.0-nightly.20261020.5 (396abe2)`. The run number puts two nightlies of one
day in order, as a commit hash wouldn't, and gives a second run on one commit
a tag of its own. It commits nothing: it publishes the same files as a
prerelease on that commit, with the commits since the previous nightly, or the
latest release, in its notes. Every nightly stays on the releases page. A
prerelease is never `latest`, so the AppImage's update information leaves
stable installs on stable releases, and takes a nightly AppImage to the latest
stable release, not the next nightly. That information names
`yapfast/yapfast`'s releases wherever the AppImage was built.

## The app icon

`packaging/icon.svg` is Yapfast's icon: the logo in marigold, its dots in
on-accent ink, so it reads on a light taskbar as well as a dark one. The UI
keeps drawing `ui/icons/logo.svg`.

On Linux the window's app ID is `io.github.yapfast.Yapfast` (`src/icon.rs`).
Wayland desktops show a window's icon only through the desktop entry of that
name, `packaging/linux/io.github.yapfast.Yapfast.desktop`;
`scripts/install-desktop-entry [EXECUTABLE]` installs it and the icon for
this user, running `target/release/yapfast` unless told otherwise. The
AppImage installs nothing, so Yapfast run from one writes the entry and the
icon to `~/.local/share` itself before it opens its window
(`src/desktop_entry.rs`), with Exec and TryExec naming the AppImage. It
rewrites them when the AppImage has moved, and leaves them alone when a package
has its own entry in `XDG_DATA_DIRS`, or an AppImage manager such as Gear Lever
made one that runs the same file. On X11 the window also carries the icon
itself. Notifications name the entry, so the notification server shows the
same name and icon.

On Windows, `build.rs` compiles `packaging/windows/yapfast.rc` with `llvm-rc`,
or the Windows SDK's `rc.exe` where there is none,
and links the icon into `yapfast.exe`, for Explorer and shortcuts; the window
loads it from there for its title bar and the taskbar, each at the size
Windows draws it. After changing `icon.svg`, run `scripts/make-windows-icon`
to draw `yapfast.ico` again.
