# 🗺️ clymene

<p align="center">
  <a href="https://asciinema.org/a/493885" target="_blank"><img width="600" src="./images/usage.svg" /></a>
</p>

An atlas image and data generation tool.

The main target is to aid game devs at atlas packing and data processing. It doesn't just outputs a packed image, but also a data set about it's sources, which can be either static images or animations.

## Design Goals

* **Fast as possible**: minimize atlas generation time whenever is possible.
* **Additive execution**: re-execution should be smart enough to only process the differences.
* **Configurable**: be able to specify everything from a project config file.
* **Command-line based**: serve as a command-line tool and nothing else.

## Features

* Input file formats
    * [X] [.aseprite, .ase](https://www.aseprite.org)
* Output formats
    * [X] .png + [.json](./docs/atlas%20data%20format/json%20format.md)
* Multithreaded sources processing
* Cached data to speed up next executions
* Configuration `.toml` file (cli options always overrides it, more info at `--help`)

## Building

```bash
git clone https://github.com/lucas-miranda/clymene
cd clymene
cargo build --release
```

## Usage

1. [Build](#Building) or [grab latest release](https://github.com/lucas-miranda/clymene/releases/latest)
2. Get config file, by **one** of following methods:
    1. Run clymene, it'll generate a default config
    2. Take [config.toml file](./config.toml) and put it aside clymene executable
4. Open `config.toml` and change:
    - `image.input_path`: directory to retrieve image sources
5. Additionally, you can modify:
    - `output.path`: directory where clymene should outputs
    - `output.name`: to give a custom output filename
    - `packer.atlas_size`: atlas target size (width and height),
6. Run clymene!

More options are commented out at [config file](./config.toml), check it out for more settings.

## Documentation

See [docs/](./docs/) to more in-depth details about formats and other things.

## License

Clymene is under [MIT License](./LICENSE).
