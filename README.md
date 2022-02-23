# hive-rushes-wordle
Rushes are new ephemeral projects at Hive Helsinki. Wordle is the first in this series with 48 hours time window

## Requirements

### Rust
Can be generally installed with command `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.

For Windows and more information see [rust website](https://www.rust-lang.org/tools/install)

### Bevy dependencies
Generally the [`Bevy`](https://bevyengine.org/) game engine should compile and run on any machine. However in some cases the compilation may fail due to missing libraries.

In general it is recommended to have latest GPU drivers and Vulkan installed. If not available `Bevy` should fall back on CPU.

## Usage
There's makefile available in root directory to help users.

### Assistant
Should help Wordle players with showing available words based on given feedback.

### Player
Should play the game itself

### Game
Clone of the Wordle game, for your pleasure.

## Contributors
(Jiri Novotny)[jiricodes.com]
(Carl Nysten)[https://github.com/crl-n]