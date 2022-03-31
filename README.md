# clymene

The main target is to aid game devs at atlas packing and processing. It doesn't just outputs a packed image, but also a data set about it's sources, which they can be either static images or animations (containing it's frames and tracks data extracted from source).

## Design Goals

* **Fast as possible**: minimize atlas generation time whenever we can.
* **Additive execution**: re-execution should include new or modified sources only, avoid reprocessing unnecessary ones.
* **Project-like configuration**: be able to specify everything from a project config file.
* **Console based**: graphical interfaces only slows down the main goal.

## Features

* Input files formats
    * [X] aseprite, ase
    * [ ] png (with data descriptor)
    * [ ] *others relevant formats*
* Multithreaded sources processing
* Config project-like file
    * [ ] Reads config file from input directory (mixed with system-wide one)
* Output file formats
    * Combined
        * [ ] binary (image + data)
    * Splitted
        * Image
            * [X] png
            * [ ] *others relevant formats*
        * Data
            * [X] json
            * [ ] *others relevant formats*
* Output stats
    * [X] Space usage
    * [ ] Density

## Building

    git clone https://github.com/lucas-miranda/clymene
    cd clymene
    cargo build --release

## Usage

1. Take [config.toml file](/config.toml)
2. Put it aside clymene executable (after building it)
3. Open *config.toml* file and change:
    - `image.input_path` directory to retrieve image sources
4. Optionally, you can modify:
    - `output.path` directory where clymene should outputs
    - `output.name` to give a custom output filename
    - `packer.atlas_size` atlas target size (width and height),
5. Run clymene!

Every option is commented out at [config file](/config.toml), check it out for more settings.

## License

Clymene is under [MIT License](/LICENSE).
