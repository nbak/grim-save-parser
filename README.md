Parser for Grim Dawn save files.

Tested on save files from 1.1.9.8 and 1.2.0.0 game versions with all DLCs enabled.

There are 3 modules:
* parser - library with parsers
* console-app - console application
  * list of options can be viewed using `-h` (`--help`) option
  * file can be passed in stdin or using `-f` (`--filepath`) option
* server-app - web server
  * starts on port 3000
  * POST endpoint for each type of parser

Parsers:

| name | struct | expected file |
|-|-|-|
| `character` | [parser::CharacterReader](./parser/src/parser/reader/character.rs) | `character.gdc` |
| `formulas` | [parser::FormulasReader](./parser/src/parser/reader/formulas.rs) | `formulas.gst` |
| `stash` | [parser::StashFileReader](./parser/src/parser/reader/stash.rs) | `transfer.gst` |


Stash and character parsers based on https://github.com/AaronHutchinson/Grim-Dawn-Save-Decryption and https://github.com/wr8fdy/yagde

Formulas parser based on https://github.com/marius00/iagd
