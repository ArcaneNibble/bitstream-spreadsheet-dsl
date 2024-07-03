# Domain specific language for FPGA bitstreams

Powered by spreadsheets!

## Context and reasoning

### Old, busted attempts

I performed some of the reverse-engineering of the bitstream format for Xilinx/AMD CoolRunner-II CPLDs and the complete reverse engineering of the Altera/Intel MAX V CPLDs. I built an initial Rust crate for the former but never got around to building a "proper, cleaned-up" crate for the latter. There were a number of things we learned from doing this:

* The initial CoolRunner-II crate tried to model the entirety of the chip within the Rust type system -- an enum of variants (for each chip capacity) containing structs, sub-structs, and arrays of `bool`s and C-like enums. This wasn't great for a number of reasons:
    * Rust (especially before `min_const_generics`) doesn't handle arrays that well.
    * This structure was pretty big (memory footprint) and would cause issues including running out of stack space to allocate it and weird LLVM slowness due to how big it was.
    * It wasn't particularly _useful_ to model the bitstream this way. Place-and-route/fitter tools tended to want their own model of the chip (often more abstracted in some way) and ended up duplicating most of the structure. This code would then copy its model into the bitstream model just to finally be packed into the final bit patterns for output.
* The initial implementation had all code implemented by hand, with `encode`, `decode`, and "dump in a human-readable way" functions. It was quickly obvious from the start that there was a lot of redundant information which could get out of sync.

The first improvements we tried to make were:

* Use a proc macro to automatically generate functions for C-like enums from a bit pattern (e.g. "001" = `ThingA`, "01x" = `ThingB`, etc.)
* Various attempts to encode "coordinates" for each bit only once, instead of separately for `encode` and `decode` functions.

These first attempts got quite overcomplicated:

* The CoolRunner-II has a "logical" bitstream format (JEDEC file) which is useless for most applications, as mapping from a JEDEC file to JTAG Shift-DR programming instructions is very complicated. However, the vendor tools output this, so supporting it is useful.
* Different CoolRunner-II parts have different features
    * The "larger" parts have more features than the smaller parts
    * In the larger parts, not all macrocells have corresponding IO pads. In the existing structure, this was encoded into the Rust type system.
* When this code first started, function-like procedural macros didn't exist yet! Only `Derive` macros existed.
* We had wanted some way to both write out and _parse_ "human-friendly" text dumps (useful for manual or other low-level bitstream hacking) but never consistently had a good way to do so.
* Several attempts to rewrite this kept being over-generic:
    * JEDEC files are logically a 1-D array of bits, so coordinates at one point allowed arbitrary dimensions
    * In several chips that we've seen, certain structures get "flipped" horizontally and/or vertically in certain situations. This was hacked into the DSL in various ways.
    * We kept trying to represent hierarchical structures in the DSL (after all, you nest structs in Rust), necessitating a lot of reinventing type systems.

### Eventual realization

At some point I consciously realized the following: most of the MAX V bitstream reverse engineering was performed by... turning the bitstream into a 2-D image and then staring at that image. Instinctive similarity-detection and pattern-matching was then used to try and figure out what bits did. The resulting notes were stored in... a spreadsheet.

(Systematic fuzzing was also needed to _confirm_ these guesses, but manual human-powered guessing came *first*. This might not necessarily be how other projects worked.)

What if a DSL could just ingest... a spreadsheet? Wouldn't be the first DSL to does so...

### This attempt

Many of these ideas involved invaluable discussions with [jediminer543](https://github.com/jediminer543).

* The "DSL for creating C-like enums from bit patterns, including `x` to specify an ignored bit" idea has generally been working fine
    * The current implementation supports `x` and `X` to indicate "ignore, but, when writing, store 0" vs "ignore, but, when writing, store 1"
* Spreadsheets turn into a list of fields containing a list of coordinates
    * The coordinates are simply _relative_ to the "upper-left" corner of the spreadsheet. There is no attempt to map these relative coordinates to global coordinates in the DSL. This mapping is done manually in code.
* In general, there is no attempt to encode the connection between fields and coordinates in the DSL. This mapping is done in code. This allows the code to _use normal code_ to handle complexities like flipping, shifting, permuting, special cases, etc.
* "Which field am I accessing?" is no longer encoded using Rust nested structs. Instead, it is encoded _as an object_.
    * The state stored in a "bitstream" struct is... just a collection of bits. One expected implementation is a wrapper around something from the `bitvec` crate.
    * "Which field am I accessing" implements the `PropertyAccessor` trait. This contains all of the information needed to pick one particular field (e.g. "which part capacity", "which tile (x, y)", "which specific field"). The bitstream struct has universal `get`/`set` methods that take this as input and access the appropriate field.
        * By making this a first-class object, it is much easier to handle "dynamic" situations like "parsing a human-friendly text file" or "exposing bitstreams to a scripting language." Previous attempts had used structs which _borrow_ the parent bitstream. This became very annoying to enforce borrowing rules in these "dynamic" contexts
* If there is logical hierarchy in the bitstream, it can be represented using methods on sub-builder structs.
    * e.g. `bitstream.tile(x, y).lut(n).setting_foo()`
* A pile of macro magic to tie this together

## FIXMEs

* Lots of issues with "naming things"
* A bunch of ugly "magic" needed for "enumerate all X so that it can be exhaustively dumped"
* Handling of human-friendly text is still rather clunky and not thought through entirely
* Lots of _awful_ hackery due to lack of specialization in Rust. Need to go through systematically and see how much of this can be tweaked given current restrictions.
