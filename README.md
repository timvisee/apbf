# Android Pattern Brute Force
A tool for brute forcing an Android security pattern through TWRP recovery.

![Screenshot](./res/screenshot.png)

One day I forgot what security pattern I used on my phone. Therefore I build a
tool which brute forces the pattern.

I succeeded to crack my 3x3 pattern in about 1.9 hours.

## Requirements
- A pattern lock
- Android 8.0 (Nougat) or above
- [TWRP][twrp] recovery
- [`adb`][adb] (with connectivity to phone in TWRP)
- [`git`][git]
- [`rust`][rust] `v1.32` or higher (install using [`rustup`][rustup])

## Speed
TWRP recovery enforces a hidden timeout of 10 seconds for each pattern attempt,
all consecutive attempts within that time fail with no warning. Because of this
a brute force attempt will take a long while when the pattern search space is
large.

It is highly recommended to constrain the search space as much as possible if
you partially know the pattern to greatly improve the brute force duration.

In the [`config.rs`](./src/config.rs) file you can tweak a few constants for:
- Minimum pattern length
- Maximum pattern length
- Maximum distance between dots in a pattern
- Dots to attempt patterns on (eliminate all dots that are definitely not used)
- Grid size (as chosen while setting up the pattern, usually `3`)

This tool does brute forcing on the actual device. A brute force attempt could
probably be greatly sped up by performing the attempt locally on a computer,
to work around the timeouts. That's however a lot more work to implement (if
even possible), so it's outside the scope of this project.

## Usage
- Make sure you meet the [requirements](#requirements)
- Clone the repository, and build the project
  ```bash
  # Clone repository
  git clone git@github.com:timvisee/apbf.git
  cd apbf

  # Build project
  cargo build --release
  ```

- Tweak properties for brute force attempt in [`config.rs`](./src/config.rs):
  ```bash
  # Edit constants
  vim src/config.rs
  ```

  Constrain it as much as possible to reduce pattern search space, which greatly
  improves brute force speed. See [speed](#speed).

- Freshly boot phone into TWRP recovery
- Make sure your phone is connected through ADB
  ```bash
  # Device must be visible in list
  adb devices
  ```

- Start brute forcing
  ```bash
  # Run tool
  cargo run --release
  ```

- Wait for a successful attempt, this may take a long while

## License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.

[adb]: https://developer.android.com/studio/command-line/adb
[git]: https://git-scm.com/
[rust]: https://rust-lang.org/
[rustup]: https://rustup.rs/
[twrp]: https://twrp.me/
