# Android Pattern Brute Force
A tool for brute forcing an Android security pattern through TWRP recovery.

![Screenshot](./res/screenshot.png)

One day I forgot what security pattern I used on my phone. Therefore I build a
tool which brute forces the pattern.

## Requirements
- A pattern lock
- Android Nougat or above
- [TWRP][twrp] recovery
- [`adb`][adb] (with connectivity to phone in TWRP)
- [`git`][git]
- [`rust`][rust] `v1.32` or higher (install using [`rustup`][rustup])

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

- Boot phone into TWRP recovery
- Make sure your phone is connected through ADB
  ```bash
  # Device must be visible in list
  adb devices
  ```

- Show logs (tap nav-bar button on the bottom-right)
- Start brute forcing
  ```bash
  # Run tool
  cargo run --release
  ```

- Wait for a successful attempt, this may take a while

## License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.

[adb]: https://developer.android.com/studio/command-line/adb
[git]: https://git-scm.com/
[rust]: https://rust-lang.org/
[rustup]: https://rustup.rs/
[twrp]: https://twrp.me/
