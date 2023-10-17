# Pinball Fantasies Reassembled

This is a game engine recreation of the 1993 Pinball Fantasies game for DOS.

## Building

Obtain a Rust toolchain, then:

```
cargo build --release
```

## Obtaining assets

The game requires some files from the original game to work.  The required files and their sha256 sums are:

```
619723e39acc003c64ae5f10159ae9da6192a28642c348f455bac447a1184967  INTRO.PRG
3b897533f11163934b8e4da038143e8f7339224421803ab4108c9d10f0a7bb4a  TABLE1.PRG
37019f7bd41d896a8f5a6383a2dffc3b3e4fc63fdf1aec1cc15db99110e581eb  TABLE2.PRG
da83ef5a7a471e6a6ad759126907076c81e92ffde6dec8e3de8e6052c6a98858  TABLE3.PRG
88f63edd4c7b50bd057397016d7aa962f0ed1c858f4a746f1ccf976f67494ebf  TABLE4.PRG
[..............................................................]  INTRO.MOD
aa5003c275b494062f37f44e8c77105b8a420555f4bd6ff53d7698f89c540f21  MOD2.MOD
a0877e4372abe64b70d9e361bf257ea5a84c948771f0eace3433d5f6399060b5  TABLE1.MOD
728629c54311386781271308e181ac0435f0582e90870accff0a42270d467529  TABLE2.MOD
fb7bfd1c96a462cb03999d2e6f843a20d3de69ba05fcbd384a9f1c131b9a563a  TABLE3.MOD
31ad7e671ae77c07c3d075e2f1fecd3d918fd921fa23acd9a1b0b6fc07fbbcea  TABLE4.MOD
```

The `INTRO.MOD` file is modified by the DOS game as part of its DRM scheme, so the sha256 is unlikely to match.  Don't worry about it.

There are several slightly different versions of the game files, and this game will only work with the exact above versions.  If your copy has a different version of some files, you can obtain the correct versions from https://archive.org/details/000323-PinballFantasies

## Running

To play the game, run:

```
target/release/pfr <path to data file directory>
```

To play the game and immediately load a table without going through the intro, run:

```
target/release/pfr <path to data file directory> <1-4>
```

The game will use (and store) configuration and high scores in the data directory, in a format compatible with the DOS version.