# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## v0.1.0 - 2022-05-14
#### Bug Fixes
- build ci - (5563766)
- discard empty sources earlier at packer - (413a909)
- missing "default" when deserializing - (24267ef)
- format_handlers shouldn't unwrap directly - (9a8b3e3)
- rectangle now always uses inner values - (fbbe757)
- indices range when from == to should be value - (3e47105)
- properly handle log macros at release - (9f434a4)
- clarify README output file formats section - (8f3b301)
- lowercase title - (ef9afe8)
- readme to improve readability - (17daabf)
- processing getting stuck when reusing cacheCache loaded files aren't counted as processed even when it should - (edb45dd)
- execution to effectively use cache - (6184119)
- crop single images at aseprite raw file - (1a54523)
- message for cache skip by being forced - (e22b6e4)
- messages breaking tree level - (1923993)
- cache reusability not working - (2cef0da)
- cache importing not working as expected - (f3f3950)
- cache stagesIt was not correct detecting cache status and regenerating needed files from source - (3eae407)
- cache processor - (9ede1d1)
- .gitignore to ignore dirs at root only - (ce076d4)
- some references - (66ae9d3)
- custom packer algorithm - (937473a)
#### Build system
- update dependencies version - (33b71f8)
- update to Rust 2021 - (782e261)
- clippy and format - (e4d52b6)
- upgrade dependencies version - (61e707e)
- clippy complainings - (09a15ff)
- clippy warnings - (2847c83)
- panic! warnings (for rust 2021) - (65561bf)
#### Continuous Integration
- trying to fix release - (fcf0d56)
- add cocogitto configuration - (04d251c)
#### Documentation
- fix usage.svg - (f0df23c)
- update README usage svg - (235b6ef)
- update usage.svg at README - (b6a2251)
- reduce images/usage.svg columns display - (373b202)
- add usage.svg to README - (46d6466)
- add usage.svg - (0836621)
- add docs - (337c11f)
- add jobs to config.toml file - (d758cb0)
- update README - (9ea1b74)
#### Features
- update ci to remove double compression - (a55bc58)
- improve cache mode - (e146f93)
- add eyre to handle error report - (b59f558)
- temporary switching to custom asefile repo - (a497953)
- simplify frame data's field names - (f428b58)
- more meaningful default values - (05d7238)
- improve animation's track tags - (fb645e0)
- add github workflows - (e90e676)
- improve README main description - (0807e3a)
- update README to reflect current status - (961a6a9)
- add ability to retry with larger atlas size - (7580e75)
- add packer's validation error type - (fb04e96)
- report remaining free space (at image) - (1f441c7)
- improve image processor thread interact - (3836822)
- add multithreading to image processor - (ecd4c48)
- make packer->optimize work as expected - (5834f3d)
- make image.aseprite.bin_path optional - (cf36750)
- ensure cache verification is smart - (6496ce8)
- force regenerate when metadata changes - (49e7cb7)
- move output to it's own config sub-section - (e5105d5)
- output results to the right dir - (a46723d)
- add cache mode - (0837f26)
- reorganize subcommands using modes - (4249af3)
- avoid exporting images everytime - (7ad2d2a)
- keep processors result buffers in memoryJust to speed up execution, loading images on demand is very slow - (ed478ab)
- update default packer algorithm - (bec28e5)
- add timer to track some operations - (7a6ac83)
- args to be handled by clap - (b54d6e1)
- add an execution duration at the end - (11c08b3)
- add raw file aseprite format handler - (5106420)
- add a placeholder aseprite raw file processor - (7bad023)
- improve filetype handling - (248a92e)
- always backup cache file when exporting - (cde8e0a)
- add --force parameter - (3738113)
- update all messages - (213e412)
- update frame_indices_data serde representation - (0da9fa7)
- add prettify json to data processor - (9ee8d3e)
- remove prettify_json from image.aseprite config - (ead8543)
- mark processor as verbose automatically - (76ab799)
- replace logger crate - (325e65b)
- add data processor - (3f5be4a)
- update image_processor error message - (199a24f)
- add everything - (9c593aa)
#### Miscellaneous Chores
- **(config)** update toml file to match program - (90033c1)
- fix README - (52a31c5)
- update README usage.svg ref - (0a53a81)
- update usage.svg - (5370854)
- update README - (6c23779)
- update usage.svg - (688e5a1)
- update dependencies - (546a628)
- update dependencies version - (46ed4c7)
- fix some messages to fit debug and verbose - (5b7f603)
- remove config test file - (12effbf)
- remove some unused things - (396a172)
- update .gitignore and .fdignore - (629f6e0)
#### Refactoring
- reorganize mod files - (068b36f)
- code format - (1ace742)
- aseprite_handler to support sub processor - (3eface9)
- improve code readability - (a03ad1d)
#### Style
- clippy suggestions - (180dbf2)
- adjust error report at cache importer - (bf2541f)
- rename cache config 'root_path()' - (0a4ae98)
- fix clippy complains - (4ff8944)
- clippy - (6aa9558)
- adjust some names - (e6fe8a5)
- adjust clippy suggestions - (6f8f942)
- add check.sh to help testing style - (44e4d18)
- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).