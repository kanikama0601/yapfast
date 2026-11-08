# Changelog

Each release's changes since the one before, as its release notes list them.

## 0.4.0 (2026-11-08)

#### Features

- feat(chat): right-click a picture or video shown large by @wiredtenshi in 19f01fdd7e15a600f0a13a0cb0d6a96e68943f30
- feat(discord): load a reply's original when the reply shows by @wiredtenshi in cc0424928411b83a59f72448a8e5d5bfb1ccb581
- feat(discord): suggest channels for a # typed in the composer by mistycalpi in 939e84f36e8f299909f7d566f00614e8a1ec0a87
- feat(chat): animate emoji, and fill in a server's card by @blockgameentity in 4f4963630d7999459b5f2d644f9cd0e73f98b150
- feat(chat): draw message emoji from Twemoji, and open their card on a click by @blockgameentity in f625a3f625879246a216b65079c328dd8de0b461
- feat(discord): keep the GIF picker's home for a day by @blockgameentity in 06d27016808e4fd04736db0696d5ca3c9c0d1cbc
- feat(discord): ask for the home before the favourites at a start by @blockgameentity in 90ff863eb0019d5dbfb264369d180567cbfed98d
- feat(discord): fetch the GIFs ahead at a start, and only the home's first GIFs by @blockgameentity in 6219d852ad4b28ce3db010cee136f5cc137c521c
- feat(discord): fetch the home's GIFs ahead, and let go of the plan by @blockgameentity in 7fbdb2907df84c688bf05514cfd810f4e8883530
- feat(discord): keep a favourite's GIF itself, not a still of it by @blockgameentity in 323d5d4f424c33d231e090ec1a636ba15e663145
- feat(discord): keep a thumbnail of every favourite GIF by @blockgameentity in e51e88a81c22875df1933d753840ac1255a56789
- feat(chat): draw the picker's emoji from Twemoji, and fetch them ahead by @blockgameentity in ff99ef7646f9c33ea5bfb3ef0ad4813c188378e9
- feat(chat): resize the emoji picker with the GIFs' window by @blockgameentity in 9b59f3cc739abf34c00974e642912c86e028f84f

#### Fixes

- fix(x264): assemble x264's x86-64 code for Intel Macs by @kanikama0601 in 4c4d3c01df65ff1a7945a840ca4c855abafc2fa9
- fix(ui): build the drag pointer for macOS, where winit gives drops no position by @kanikama0601 in cb1ccbdc45957c361cdee91f1f1e4d40128a975c
- fix(ui): build Slint's Skia renderer for macOS without wgpu by @kanikama0601 in 8e7d78dc1056547754fb89d131d86ec1735f24ab
- fix(ui): send keys nothing took where Discord's client sends them by @wiredtenshi in a5480959dae0da7fd982b7d6f572cd28581f4afc
- fix(ui): type into the composer from anywhere, and let Escape cancel a reply by @6gah in 7a5164781185d0d61af38851e4850c40a10e6399
- fix(discord): play videos in the thread beside the channel by @wiredtenshi in 223096455a6e1d8a4199175bcb5eb1c7ff989a82
- fix(discord): highlight mentions in the thread's composer by @wiredtenshi in 83b46d783421b2363d07e734b016f17711b6b8a9
- fix: build without Discord again by @wiredtenshi in 494157438ae16837af97411c865fb71405fe6a53
- fix(discord): open a sent channel mention, and tell channels apart by @wiredtenshi in 3c6db4c22f20626d1dad03edcde8bccea55aade3
- fix(discord): keep thumbnails of favourite GIFs, and fetch ahead only what's cheap by @wiredtenshi in a3cb5e16b7e335ee1bf587401695785ca2dec16a
- fix(chat): stop cutting the edges off emoji in text by @wiredtenshi in 34119c014c6f029eada770191dd23195fb95afba
- fix(chat): draw message emoji from one font the program carries by @wiredtenshi in bf5f991e457e6e0a69247a9c51aaf1fb5676bf4c
- fix(chat): find the system emoji font for a picture on Linux by @wiredtenshi in f72eeb968c26b4dd37cb0e6cd8d205d6d062e6be
- fix(chat): let go of the picker's pictures when they're far off or closed by @wiredtenshi in 23c544bda70848d1a77bff514b19ec03652c1c82
- fix(discord): don't offer friend requests to bots by @wiredtenshi in 432a9368d7ff908a678e10a6f8c3cb41c209ecd4
- fix(ui): mentions, emoji and fitting previews in the thread beside a channel by mistycalpi in a6c64d79ab26b2c82eff11953b8206ce2de35bd9
- fix(discord): place the "New" divider once the newest page came by @wiredtenshi in a381418876d0ebd73eeda60a25e3214790c73ba5
- fix(discord): look an original up again for a reply that comes later by @wiredtenshi in d5eb592875d3278397c598e64b104a3cb4ea2556
- fix(chat): ask Discord for an animated emoji's GIF by @blockgameentity in 02a6237ff16257727f5a957cd73c53153c763c33
- fix(chat): play animation in a window that opens focused by @blockgameentity in 00d122f4711681090ef559a8cc45851fb0e0d839
- fix(discord): keep a reply's original through a preview's update by mistycalpi in db87413a20e1f389d7e75f0280085917bd703cb3
- fix(windows): hand Direct3D 12 frames back in the PRESENT state by @wiredtenshi in 4530ebbd5b779f244c8de0b63fa15cfa0a8b5cdc
- fix(ui): read a short unread channel as soon as it opens by mistycalpi in 677b55db902d746720cf51238383349d775a2fcb
- fix(chat): read an emoji's run in the text that was shaped by @blockgameentity in a9c699a69d99621b2a2bb4cbdc0da260e46ca212
- fix(chat): draw an emoji's frame as it plays, and ask for a card's picture by @blockgameentity in fc5d10b1e5327697e1d605412f55a2ca9a9f8b5b
- fix(chat): fill the emoji the grid is scrolled to when it is resized by @blockgameentity in 39ddfc0702246026955857f84d325a4010aa08b2
- fix(chat): draw the picker's emoji as pictures, not colour-font glyphs by @blockgameentity in fae97565e7d3bb49fcec3508a072b60705693552

<details><summary>Other changes (8)</summary>

- ci: build macOS for Intel too by @kanikama0601 in 75cbf353f17d0e6e8e02d1920a0497b5d957c816
- ci: release every push to main, at the version its commits ask for by @kanikama0601 in 682faba1f98052df21373d4e60fa345f5131f04e
- ci: build and release Linux, Windows and macOS from Actions by @kanikama0601 in 925c00744fb40e083f3ab35ec32d6d60c7cc4346
- feat:スタンプに対応 by @kanikama0601 in 13d99038a29af972b60e86c3c4f4dfa65ba106ba
- docs: add CONTRIBUTING.md, and make Nix optional for the checks by @wiredtenshi in ab7de6f55928be264f15453a807f55c4fe8eb3dc
- ui: mute a server, channel or DM at once when its row is clicked by @blockgameentity in e6440d2cb52a66cc48043aee00916174d5767ff5
- ui: let a menu row choose its own id rather than open its submenu by @blockgameentity in 1229be8aee944e847330938aa7939e790b79f020
- docs: plan caching every favourite GIF by @blockgameentity in 6923451dc5d4b3fd30bec86e9efd6779b26146b4

</details>

All changes: https://github.com/kanikama0601/yapfast/compare/03862f6394fad09b541d731181b819a3c9713d3f...91a65790534716ae77d600e0fe8d273135857451
