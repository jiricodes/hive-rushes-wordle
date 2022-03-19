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

You can use option `DICT` to alter path to used dictionary.
### Assistant
Launch with `make assistant` and provide the last guessed letter followed by *status string*. The assistant then suggests top 25 words to try next.

The *status string* is simply a sequence of letters corresponding to the color response of the game.
Where 'X' == grey, 'Y' == yellow and 'G' == green. 

**Example**
```
Insert current guess:crane
Insert status string [GYX]:gyxxy

Showing 7 out of 7 suggestions
Suggestion    Unique chars  Avg. frequency score
corer         4             38.760002
cheer         4             38.168
cider         5             34.870003
cower         5             32.346
cover         5             32.058
clerk         5             30.448002
cyber         5             27.654001

Insert current guess:cover
Insert status string [GYX]:ggxgg

Showing 2 out of 2 suggestions
Suggestion    Unique chars  Avg. frequency score
corer         4             38.760002
cower         5             32.346

Insert current guess:cower
Insert status string [GYX]:ggxgg

Showing 1 out of 1 suggestions
Suggestion    Unique chars  Avg. frequency score
corer         4             38.760002

Insert current guess:corer
Insert status string [GYX]:ggggg
Wordle solved.
```


### Player
Launch player with `make player`  command and use words it suggest to play the game, then provide feedback in format of *status string*.

The *status string* is simply a sequence of letters corresponding to the color response of the game.
Where 'X' == grey, 'Y' == yellow and 'G' == green. 

**Example**
```
Try this next: binge
Insert status string [GYX]:xyxyx

Try this next: grimy
Insert status string [GYX]:yyyxx

Try this next: sprig
Insert status string [GYX]:ggggg
Out of suggestions, did you win?
```

### Game
Clone of the Wordle game, for your pleasure. Launch it with `make game` to use default dictionary or with  `make game DICT=path/to/dict` to use custom one.

The game is pretty straight forward. Text is captured in tiles and can be deleted with backspace or submitted with enter (return). Game restarts with escape.


## Contributors
[Jiri Novotny (jiricodes.com)](jiricodes.com)

[Carl Nysten (github.com/crl-n)](https://github.com/crl-n)
