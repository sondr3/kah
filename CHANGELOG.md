<a name="0.2.0"></a>
## 0.2.0
> 2018-09-8

"Initial" release of `kah`, a small little helper utility for Kattis.

#### Changelog:
* [[`0d3e48de7f`](https://github.com/kah/node/commit/0d3e48de7f)] - Add Travis CI badge to README \[ci skip\]
* [[`df8906fa48`](https://github.com/kah/node/commit/df8906fa48)] - Format files with rustfmt and run Clippy
* [[`15cf30a319`](https://github.com/kah/node/commit/15cf30a319)] - Add badges, update package metadata, and add a README
* [[`0bab1fc3a7`](https://github.com/kah/node/commit/0bab1fc3a7)] - Add functionality to parse .kattisrc file
* [[`8ee318b737`](https://github.com/kah/node/commit/8ee318b737)] - Add license
* [[`36c102ed67`](https://github.com/kah/node/commit/36c102ed67)] - Format files with rustfmt
* [[`b0a3574345`](https://github.com/kah/node/commit/b0a3574345)] - Move the rest into their own files
* [[`659a80a7b8`](https://github.com/kah/node/commit/659a80a7b8)] - Move get subcommand to its own file
* [[`00b617f164`](https://github.com/kah/node/commit/00b617f164)] - Format files with rustfmt
* [[`65fcea6bfe`](https://github.com/kah/node/commit/65fcea6bfe)] - Move StructOpt definitions into own file
* [[`a4a289393b`](https://github.com/kah/node/commit/a4a289393b)] - Rename user subcommand to init
* [[`e206a0246d`](https://github.com/kah/node/commit/e206a0246d)] - Add User command, to be used to fetch .kattisrc configuration file
* [[`5a91a23880`](https://github.com/kah/node/commit/5a91a23880)] - Format and fix code according to Clippy
* [[`70424fcb3e`](https://github.com/kah/node/commit/70424fcb3e)] - Add Submit command, fix some comments
* [[`ddc697ec9c`](https://github.com/kah/node/commit/ddc697ec9c)] - Format files with rustfmt
* [[`3225db49e2`](https://github.com/kah/node/commit/3225db49e2)] - Add configuration for Travis CI
* [[`13bc6e817f`](https://github.com/kah/node/commit/13bc6e817f)] - Minor cleanup, we're not using Rust 2018, ignore samples directory
* [[`21c68ddf70`](https://github.com/kah/node/commit/21c68ddf70)] - Name binary file kah -- Kattis Assignment Helper
* [[`b2ae746591`](https://github.com/kah/node/commit/b2ae746591)] - Remove superfluous println, loop from 1 and not 0
* [[`2bf6617a4c`](https://github.com/kah/node/commit/2bf6617a4c)] - Check the proper path, mixed up true/false for path
* [[`d4d2d6833d`](https://github.com/kah/node/commit/d4d2d6833d)] - Don't overwrite files that already exist
* [[`66290f8589`](https://github.com/kah/node/commit/66290f8589)] - Move back to Rust stable instead of nightly
* [[`0ad606fa2a`](https://github.com/kah/node/commit/0ad606fa2a)] - Actually unzip files into samples directory
* [[`82a63608c1`](https://github.com/kah/node/commit/82a63608c1)] - Fix linting errors from Clippy and warnings from rustc
* [[`e9ce91f85d`](https://github.com/kah/node/commit/e9ce91f85d)] - Run cargo fix --edition-idioms
* [[`f8b675d62a`](https://github.com/kah/node/commit/f8b675d62a)] - Formt files with rustfmt
* [[`d5dcaeb3d0`](https://github.com/kah/node/commit/d5dcaeb3d0)] - Add support for unzipping the sample files
* [[`6e8ca9979d`](https://github.com/kah/node/commit/6e8ca9979d)] - Add some documentation, and a stupid way to get a string from the file path
* [[`9e27d9db64`](https://github.com/kah/node/commit/9e27d9db64)] - Copy samples.zip to current directory, it's alive!
* [[`247e9b687a`](https://github.com/kah/node/commit/247e9b687a)] - Add verbose flag, move main CLI part into struct
* [[`8c8b6ff90c`](https://github.com/kah/node/commit/8c8b6ff90c)] - Get and download Kattis files to temporary directory
* [[`84d761e342`](https://github.com/kah/node/commit/84d761e342)] - Use zip and tempfile for upzipping and saving to temp dir
* [[`7306b00f44`](https://github.com/kah/node/commit/7306b00f44)] - Format file with rustfmt
* [[`decf3fb0b3`](https://github.com/kah/node/commit/decf3fb0b3)] - Add ID and problem name fields, method to get path
* [[`2e7d6b748b`](https://github.com/kah/node/commit/2e7d6b748b)] - Holy moly, you can actually ping kattis.com
* [[`1c40dbdfa8`](https://github.com/kah/node/commit/1c40dbdfa8)] - Add structopt, rustfmt for formatting
* [[`90483c0edd`](https://github.com/kah/node/commit/90483c0edd)] - In the beginning there was darkness...
