# Contributing to rust-GameDig
This project is very open to new suggestions, additions and/or changes, these 
can come in the form of *discussions* about the project's state, *proposing a 
new feature*, *holding a few points on why we shall do X breaking change* or
*submitting a fix*.

## Communications
GitHub is the place we use to track bugs and discuss new features/changes,
although we have a [Discord](https://discord.gg/NVCMn3tnxH) server for the
community, all bugs, suggestions and changes will be reported on GitHub 
alongside with their backing points to ensure the transparency of the project's
development.

## Issues
Before opening an issue, check if there is an existing relevant issue first, 
someone might just have had your issue already, or you might find something 
related that could be of help.

When opening a new issue, make sure to fill the issue template. They are made
to make the subject to be as understandable as possible, not doing so may result 
in your issue not being managed right away, if you don't understand something
(be it regarding your own problem/the issue template/the library), please state 
so.

## Development
Note before contributing that everything done here is under the [MIT](https://opensource.org/license/mit/) license.
### Naming
Naming is an important matter, and it shouldn't be changed unless necessary.

A game's identificator shall be created following these rules:
1. Names composed of a maximum of two words will result in an id where the 
words are concatenated (`Dead Cells` -> `deadcells`), acronyms in the name 
count as a single word (`S.T.A.L.K.E.R.` -> `stalker`).
2. Names of more than two words shall be made into an acronym made of the 
initial letters (`The Binding of Isaac` -> `tboi`), [hypenation composed words](https://prowritingaid.com/hyphenated-words) 
don't count as a single word, but of how many parts they are made of 
(`Dino D-Day`, 3 words, so `ddd`).
3. If a game has the exact name as a previously existing id's game 
(`Star Wars Battlefront 2`, the 2005 and 2017 one), append the release name to
the newer id (2005 would be `swbf2` and 2017 would be `swbf22017`).
4. If a new id (`Day of Dragons` -> `dod`) results in an id that already exists 
(`Day of Defeat` -> `dod`), then the new name should ignore rule #2 
(`Day of Dragons` -> `dayofdragons`).
5. Always append the sequel number at the end (`Team Fortress 2` -> 
`teamfortress2`) and the year (`Unreal Torunament 2004` -> 
`unrealtournament2004`), note that the number (or the year) don't count to rule
#2, roman numbering will be converted to arabic numbering (`Final Fantasy XIV` -> `finalfantasy14`).

### Priorities
Game suggestions will be prioritized by maintainers based on whether the game
uses a protocol already implemented in the library (games that use already
implemented protocols will be added first), except in the case where a
contribution is made with the protocol needed to implement the game.

The same goes for protocols, if 2 were to be requested, the one implemented in 
the most games will be prioritized.

### Releases
Currently, there is no release schedule.
Releases are made when the team decides one will be fitting to be done.