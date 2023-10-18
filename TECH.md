# Technical details

This file outlines the internals of the game, both the original DOS version and the recreation.

## Basics

The original game consists of:

- `PINBALL.EXE`: the entry point executable, a tiny MZ exe consisting of nothing but some simple code to load and switch between the `.PRG` files
- `INTRO.PRG`: the executable (in MZ exe format) handling the intro sequence, options, and table selection
- `TABLE[1-4].PRG`: the executables handling the actual game, one for each table
- `INTRO.MOD`: soundtrack for the intro sequence (when entered from the command line)
- `MOD2.MOD`: soundtrack for the intro sequence (when entered after exitting the table)
- `TABLE[1-4].MOD`: soundtrack for the actual game
- `*.SDR`: sound drivers; each driver is a MZ exe loaded together with the game / intro sequence
- `SETSOUND.EXE`: a freestanding executable for configuring the sound driver

User data is stored in the following files:

- `PINBALL.CFG`: game options
- `TABLE[1-4].HI`: high scores
- `SOUND.CFG`: selected sound driver and its configuration

When `PINBALL.EXE` is started, it installs a tiny interrupt handler at `int 0x65`, with the following functions:

- retrieval of keyboard scan codes (from the keyboard interrupt hooked by `PINBALL.EXE`) — used only by the intro sequence, the game has its own keyboard handler
- passing game options from intro sequence to the tables
- selecting the table to load

The main executable only handles switching between `*.PRG` files and remains loaded for as long as the game is running.

At any time (except during a switch), there is exactly one `*.PRG` and exactly one `*.SDR` file in memory.

All executable code is hand-written 286 assembly running in real mode.

## Sound and sound drivers

Sound in the game is based on Amiga module files, with custom sequencing logic.  Every sound driver includes a module player, which can be controlled by the game in response to game events.  The sound drivers expose their function via `int 0x66`.

Since some sound drivers require hooking the timer interrupt for their operation, the sound driver is also responsible for providing timer functions to the game.  The sound driver is responsible for providing emulated raster interrupts (ie. interrupts triggered when the CRT electron beam reaches a particular scan line) and vblank interrupts, which are not natively available on VGA.  This is done by carefully calibrating the VGA timing in terms of 8253 cycles, then using 8253 interrupts.

For the intro sequence, the soundtrack is simply a module file, with nothing special to say about it.  For the tables, the module files are tightly coupled with the game logic.

Each table has several main pieces of background music:

- plunger sequence (in game, waiting for the user to start playing)
- main sequence (in game, playing)
- attract sequence (game hasn't started yet)
- special play modes
- silence (used for the "no music" mode instead of plunger and main sequences)

All of the above are contained in the same module file.  The table logic has hardcoded positions (in the module file sense) for every used sequence, and communicates those to the sound driver according to the game state.

The game also has two kinds of sounds: jingles and simple sound effects.  A jingle is a position (or several) in the module file, and temporarily interrupts the background music when played.  A simple sound effect consists of a single note (again, in module file terms) which is played asynchronously without stopping the current background music (or jingle).  The simple sound effects play on channel 4, so the background music and jingles are composed to use only the first three channels (with the exception of the attract sequence, which is guaranteed to be played when no sound effects can happen).

The jingle positions and sound effect notes (as (sample id, period) tuples) are hardcoded in the game logic.

Overall, the sound driver exposes the following functions via `int 0x66`:

- playback control (load file, pause, resume, exit)
- set master sound volume (used for the fadeout in exit sequence)
- get number of sound ticks since start (used in the intro sequence to sync slides with music)
- jump to a position (used for music switching and jingles)
- set a callback for the "jump" effect (used by the game to return to background music when the jingle ends)
- play a single note immediately (used for simple sound effects)
- wait for next vblank
- set vblank interrupt handler
- set raster interrupt handler and scanline position

## Intro sequence

Mostly straightforward, but with a few tricks.

The sequence starts with 3 developer / publisher logos. These are displayed in 320×240 8bpp mode (look up VGA mode X).  Next is the game logo, which is displayed in 640×480 4bpp mode.  The logo display is actually synced to sound playback.  VGA palette manipulation is used for fade-in and fade-out.

The main menu sequence is displayed in 640×240 4bpp mode (with non-square pixels).  Double buffering is used, as well as storing some images in offscreen VGA memory for fast copies.

The left panel initially sliding into position is accomplished via hardware scrolling (ie smoothly moving the start address of the framebuffer within VGA memory).  Normally, when done like that, the part of the panel that is off to the left of the screen would wrap over and be visible on the right of the screen.  To avoid this, the game temporarily lowers the "displayed horizontal characters" CRTC register effectively turning the entire right half of screen into part of the horizontal blanking period as far as the VGA hardware is concerned, causing it to display as black.  This confuses emulators like DOSBox, making them change window size or wrongly center the non-blanked part of the screen.

The 4bpp mode used is quite constraining, since the game needs to display the left panel and two table graphics simultanously.  The raster interrupt is used to obtain more than 16 colors: it is set to fire when in the middle of the screen (in the black gap between the two table graphics), and the raster handler switches the VGA palette to the colors required by the second table.  The subsequent vblank handler switches the VGA palette back to the colors required by the first table.  The colors that are also required by the left panel are shared by the palettes of all 4 tables.  This once again confuses DOSBox, making it use the wrong palette for the bottom table.

On the text screens, the fade-in/fade-out is once again done by manipulating the palette.  However, since only the text needs to be fading (and not the left panel), there is a complication: there are actually two versions of the text fonts (and the "high scores" graphic), low quality and high quality.  The low quality versions use fewer colors, and are used while in fade-out or fade-in.  The high quality versions use more colors, as they can reuse the colors used by the left panel.

All graphics are present in the executable in Amiga PBM / ILBM format.

The recreation instead uses 640×480 8bpp mode all the time, and makes use of the speed of modern computers to just re-render the whole screen every frame, without resorting to the above hacks.

## Main game — graphics

The game has two modes: normal resolution (320×240) and high resolution (320×350).  The playground dimensions are 320×576.  On the bottom of the screen, there is the dot matrix display, which takes up a 320×33 rectangle.  The top of the screen scrolls to follow the ball around the playground, with a window of 320×207 or 320×317 pixels.

The game once again uses hardware scrolling for the main playground: the VGA memory contains the whole playground at all times, and only the starting displayed address changes as the window scrolls around.  The dot matrix display on the bottom is implemented via a VGA feature known as "line compare", which jumps to the beginning of VGA memory when scanline 207 (or 317) is reached.  Utilizing this VGA feature is actually the reason why the dot matrix is on the bottom of the screen (as opposed to the top as in Amiga version).  Funnily enough, in VGA memory, the dot matrix is actually at the top, above the playground.

While the displayed resolution is 320 pixels horizontally, the framebuffer is actually 336 pixels wide in memory, with the extra 16 pixels not displayed.  This is used to simplify character clipping for scrolling messages in dot matrix code: instead of carefully clipping to the right pixel, characters can be simply drawn to the screen slightly out of bounds, with the offscreen portions effectively discarded.

As usual, VGA mode X is used, and the drawing code has to deal with planar memory.

To save main memory, offscreen VGA memory is used for storing rarely-used parts of physmaps.  Surprisingly, this includes using the extra 16 pixels of width in the area bordering the main playground (which is safe from being overwritten by the dot matrix code).

The playground remains mostly static between frames.  The exceptions are: lights, flippers, ball, and spring.

The main playground image is present in the executable in Amiga PBM format, split vertically into 4 chunks (presumably to avoid any single chunk being larger than a 64kiB 8086 segment).

A major complication in physics and other timing-dependent calculations is that all game timing is tied to VGA refresh rate, which in turn differs between the two available resolution modes (nominally 60Hz in normal resolution and 71Hz in high resolution).  For this reason, all timing-related data is either:

- kept in the executable in two versions, for each resolution mode, or
- is fixed up at startup by multiplying with a const factor, or
- is not fixed up in any way, resulting in noticably different timings between resolutions (this tends to apply to most script execution)

The recreation always runs at 60 frames per second no matter the resolution, and uses the normal resolution timing data.

### Playground lights

Playground lights (all the flashing indicators) are implemented by giving each individual light its own range of palette colors (usually 1 or 2, but eg. the Stones n Bones ghost graphics use much more).  Turning the lights on and off is accomplished by simply swapping these colors in the VGA palette between the "off" and "on" versions, without having to modify VGA memory.

The game code potentially maintains a blink state for each light — the frequency and phase of blinking.  Depending on table code, light blinking can either be started asynchronously at phase 0, or synchronously by using a shared blink phase.

### The ball

The ball drawing code is responsible for the main chunk of VGA memory traffic, and is accordingly highly optimized.  The ball drawing routine is completely unrolled — there is no blit loop, each ball pixel is encoded into the executable as several assembly instructions that put it on the screen in the right address, collected into one giant function.

Since the ball can be occluded by playground features "above" it, each table comes with two occlusion bitmaps (one used when the ball is on the "ground" level, the other used when the ball is on the "overhead" level).  The occlusion bitmap simply decides which pixels of the main playground can be drawn over by the ball, and which can't.

Since there is only one copy of the main playground in memory, the ball drawing code saves the previous contents of the pixels currently containing the ball into main memory.  When the ball is moved, the "ball undraw" function is first called, which puts the old contents back.

The recreation recovers the ball image by disassembling the ball drawing code.  See the horrible code in [`extract_ball` function](src/assets/table/gfx.rs#L144).

### The flippers

Drawing the flippers is likewise highly optimized, but in a completely different way.  The main feature of flippers is that they have static position on screen, and hence static address in VGA memory.  So, to make drawing fast, the flipper graphics are diff encoded:

- flipper position is quantised into a small number of distinct frames, and an image is drawn (by the artist) for each of them (there are 22 frames for the main flippers, fewer for the side flippers)
- instead of storing the frames directly in the executable, diffs between the frames are stored instead
- a diff describes how to get frame Y displayed on a screen when it currently has the frame X
- a diff is described as a list of (source, destination) pairs of VGA memory addresses to copy
- the source data for the copies is stored in offscreen VGA memory
- the copy unit is 4 pixels because of VGA planar nature (a blit mode is used for flipper drawing, which copies 1 byte of all 4 planes at a time)

The executable contains precomputed diffs for frame movements of up to ±9 frames at a time.  If a larger movement happens in one game frame, the drawing operation is executed in several steps.

Actually, the above is a little oversimplified.  The executable really contains diffs of ±1 frame, but with extra metadata allowing the engine to quickly compute diffs of up to ±9 frames: for every ±1 frame diff, it sorts the copies within the diff such that the ones that get immediately overwritten by the next diff in the same direction come last, the ones that get overwritten in two frames come before that, and so on.  The length of each such segment is also stored.  This allows the game engine to quickly assemble the full diff with no or minimal overlap.

The recreation doesn't do any of this optimization, and just uses the diff data in the executable to reconstruct the raw pixmaps for every flipper frame, which then get blitted to the screen on each frame.

### The spring

The spring is stored in the executable as a raw pixmap and is redrawn on playground whenever it is moved.  Fairly boring.

## Physics

The ball state is described by several numbers:

- the X and Y position (32-bit fixed point with 10 fractional bits)
- X and Y speed (signed 16-bit, in units of position per physics frame)
- rotational speed (signed 16-bit)
- the 1-bit Z position (the ball can be either on the "ground" level or "overhead" level)
- a "frozen" flag (set when the ball is currently held by some kind of trap)

The ball is 15 pixels across.  The position kept by the game is actually the top left corner of the 15×15 pixel square occupied by the ball, not the ball center.

The physics code runs 4 times per frame in normal resolution (240 physics frames per second), or 3 times per frame in high resolution (213 physics frames per second).  Most physics-related speed data in executable is adjusted on startup according to the frame rate.  Not all of it is.

### Physmaps

For physics purposes, the playground is described by physmaps, ie. images that describe the physics properties of each pixel.  The physmaps have the same 320×576 dimensions as the playground graphics.  There are two physmaps per table, corresponding to the "ground" and "overhead" levels.  The physmaps are used for three purposes:

- collision detection (ie. is there a solid object at this pixel)
- collision handling (ie. what material is the solid object at this pixel made of, for bouncing purposes)
- gravity/ramp handling (ie. is the playground inclined at this pixel, and at what direction and angle)

The physmaps are not fully static — they get modified during play:

- the flipper positions need to be reflected onto the physmap
- some tables contain gates that can be opened or closed
- some tables have drop targets, which can be dropped or raised

In the original executable, each physmap consists of 3 bit planes, and is stored as 3 bitmaps.  However, it is not a simple 3bpp image.  Physmap data is interpretted as follows:

- if bit plane 1 is 0 for a pixel, it's not solid; if it is 1, it is solid
- if a pixel is solid, the bits from planes 0, 1, 2 for that pixel are combined together to obtain the material
  - material 2 is used for flippers and gates (which are not present in the main physmap in the exacutable, and are patched in later)
  - material 3 (rubber) is used for kickers
  - material 6 (steel) is used for most of the playground
  - material 7 (plastic) is used for bumpers
- if a pixel is not solid, it may be part of a ramp
  - if the *byte* containing the pixel is 0 on both bit plane 0 and 1, the byte at bit plane 2 at this location is repurposed to contain the ramp index
  - otherwise, the bytes to the left and to the right are also likewise checked for a valid ramp index

In effect, the same underlying storage is reused for both high-resolution material data and low-resolution ramp data, depending on needs.

Bit plane 1 is also the only one to be modified during play — flippers and gates exist only on bit plane 1 (being made of material 2), while drop targets are made of material 3 or 7 and just temporarily lose their "solid" bits when dropped.

Physmap patches for flippers, gates, and drop targets are single-plane raw bitmaps in the executable.

### Gravity, ramps

Gravity is applied to the ball at all times when it is not frozen.  The ramp index is looked up in the physmap at the ball center.  It is used to index a (per-table) list of ramp inclines, which gives the acceleration to apply to the ball (in X and Y direction).  Index 0 in the list is used for "no ramp".

### Layer switching

Switching between the ground and overhead layers is very simple: the executable contains a list of "transition areas" for each layer, which are simply rectangles that, when entered, will cause the ball to teleport to the given layer.

### Collision handling

Whenever the ball moves, each pixel in a 17×17 pixel circle outline extending 1 pixel past the ball is checked in the physmap.  If a solid pixel is found, a collision is detected.  An arbitrary pixel of the collision is chosen as the representative to sample the material hit.  The average angle (relative to the ball center) of the collision is computed over all hit pixels, and it is used to obtain the hit position for physics computations.

The hit position together with the material are used to detect hitting flippers, bumpers, and kickers — all of them have a static bounding box rectangle, and all pixels with the matching material within that rectangle are assumed to belong to them.

Likewise, all hit triggers on the map have a bounding rectangle used to detect them, but this does not involve a material check.

If a flipper was hit, the current speed of this flipper will be taken into account for collision handling.  Likewise, if the table is currently being pushed (or unpushed), the table push speed will be taken into account.

Each material has somewhat different collision properties (bouncing factor, minimum bounce speed, maximum bounce angle, etc).  If a bumper is hit, or if a kicker is hit and a minimum collision speed is met, a bumper or kicker boost is added to the collision, and a hit is scored.

### Triggers

There are two kinds of triggers on the tables: hit triggers and roll triggers.  Both of them are stored as lists of (rectangle, function pointer) pairs, one list per physics layer.  Hit triggers are registered on collision with a solid pixel within the rectangle, while roll triggers are registered by the ball center simply entering the given rectangle.  The function pointers contain table-dependent game logic.

Kickers and bumpers do not count as hit triggers — they are stored in separate lists.  The information stored for bumpers and kickers is just the bounding rectangle, the score value, and the sound effect to play.

## Tasks

The game contains a simple system of asynchronous task execution — there is a global task list which contains function pointers to be called every frame.  When called, the functions (task handlers) have the opportunity to remove themselves from the list, or remain on it.  Most such tasks make use of timers (in global variables) to delay their execution for a set number of frames, then remove themselves from the list once they actually execute.

This mechanism is used for most background timed behavior (though not all of it — some of it is coded directly into the main per-table frame handling function, with no clear pattern).  For example, when a target is hit, its indicator light often blinks for some frames before settling on the lit state.  This is implemented by the hit trigger handler setting the light to blink, and spawning a delayed task that will stop the blink.  Likewise, tasks are used to handle ball ejection from traps, or (on some tables) to keep high-scoring modes pending while another mode is in progress.

## Scripts and dot matrix

A bunch of game logic is not coded in assembly, but in a form of an interpreted asynchronous "script".  A script opcode consists of a pointer to a native code function handling the opcode, followed by a variable number of arguments to it (and then followed by further opcodes).  There is exactly one script being executed at all times.  Most opcodes will suspend the script execution for one or more frames — some opcodes immediately load the next opcode, but they are in the minority.  Script execution is thus asynchronous.

Internally, executing a script *opcode* will result in a script *task* being loaded.  On every frame, the current script task is run, and it can either request to be executed again the next frame, or it can request the next opcode to be executed (overwriting it with another task).  Scripts end with a special "end" opcode (the null pointer), which will load a special "idle" task, which will then select an "idle" script to execute depending on current game state.

The scripts are tightly coupled to the dot matrix — almost all logic controlling the dot matrix comes in the form of scripts, and it is also the main (but not only) purpose of scripts.

Some of the opcodes available to scripts are:

- delay for given amount of frames
- jump to another script position
  - unconditional
  - if a given score counter (eg. special mode score) is 0
  - if bonus multiplier is 1
  - repeat loops
- play a sound effect or a jingle
- wait until current jingle is done
- do a high-scoring mode: display score and timeout (and block until it's done)
- dot matrix operations
  - set dot matrix blinking
  - clear the dot matrix display (all at once, wipe right, wipe down)
  - show an animation
  - print a string at static position
  - print a score
  - print a string, scroll up/down
  - print a long message, scrolling it left
- accumulate various parts of score as part of the ball drain sequence
- control game state (eg. issue new ball, execute match)
- various table-dependent actions (eg. eject the ball from a trap)

### Dot matrix

The dot matrix is a grid of 160×16 dots, displayed on VGA as 320×33 pixels.

There are 4 fonts in the game:

- 7×5 dots
- 7×8 dots
- 7×11 dots
- 7×13 dots

The character cells are conceptually 8 dots wide in all 4 fonts, with the rightmost dot used as a gap.

For score printing, a comma is printed below and between digits, in two dot lines below the main character.  It is not part of the font.

The characters included in the fonts include uppercase letters, digits, `-`, `(`, `)`, `?` (though `?` is missing in the 7×5 font for unclear reasons).  Curiously, the game uses a custom encoding for most messages which is not quite ASCII:

- `0x00` is end of string
- `0x20` is "no character" (ie. do not print anything, leave old contents of dot matrix at this position)
- `0x2a is "blank" (ie. overwrite the character call with all-unlit dots)
- `0x37-0x40` are the decimal digits `0-9`
- `0x41-0x5a` are the uppercase letters `A-Z` (matching ASCII)
- `0x5b` is `?`
- `0x5c` and `0x5d` is `()`
- `0x5e` is `-`

However, a different encoding is used for long messages:

- `0x01` is "blank"
- `0x20` is "no character"
- letters, digits, and `?()-` have their ASCII codes
- `0xff` is end of string

The recreation converts both encodings to ASCII as part of assets extraction process.  Since the two need to be distinguished, space is used for "no character", while `_` is used for "blank".

### Dot matrix animations

Dot matrix animations are stored as a list of frames, each with an associated duration.  Animations can also have a repeating part at the end, with a given repetition count.  Frames are stored as pointers, and can thus be shared between animations (or used multiple times in an animation).

Animation frames are essentially diffs — they are a list of (pixel position, lit or unlit) pairs with a very simple RLE-like compression.  This helps frame reuse between similar animations.

## Effects and priorities

A bunch of game behavior relies on the "effect" structure.  An "effect" consists of:

- a jingle to play (or none)
- a priority level
- a score to be awarded to the main score
- a score to be awarded to the bonus score
- a script to be run (or none)

Most rewards given to the player come in the form of effects.  An effect will always award the score, but will not necessarily play the jingle and/or run the script.  The game has a concept of priority, which is associated with every jingle and every jingle-less effect as well.  The priority of the currently playing jingle is kept in a global variable.  Whenever an effect is triggered, its priority is compared to the current jingle priority.  If it is less than the current priority, or if there's a high-scoring mode active, the effect is silenced and only the score is given.  Otherwise, the current jingle and script are interrupted by the effect's jingle and script (if applicable).

The code requesting the effect is told whether the effect was silenced, and can take decisions based on this (eg. eject the ball from a trap immediately, or wait for script completion).  It is also possible to temporarily silence all effects, or to force a visible effect regardless of priority and high-scoring mode.