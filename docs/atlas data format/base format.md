# Base Atlas Data Format Specification

Output atlas data format, some details may change between different data formats.

## Reference

* **rect**
    ```
    u32             x
    u32             y
    u32             width
    u32             height
    ```

* **nothing**
    Some values or entries may be empty, but must be provided ir order to keep track of their entry position, as it's used as index somewhere else, for example.

    E.g: [Frame](#Format#Frame) index position is refered at [Track's Index](#Format#Graphic#Track#Index).

## Format

```
[graphic]       graphic entries
metadata
```

### Graphic

```
string          source filename

[frame]         frames
                Every frame extracted sequentially from source file.

[track]         tracks
```

#### Frame

It can be `nothing`, each specification can use a properly indication when this happens.

```
rect            atlas
                Region which the frame occupies at output atlas

u32             duration
                Frame duration (in milliseconds).
                It may be omitted (if specification supports it) when duration
                is meaningless (e.g at single frame source image).

rect            source
                Extracted region from frame at source file.
                It's needed because clymene completely strips empty spaces at every frame
                to be able to pack them tightly, so you should use x and y to reconstruct
                where this frame were.
```

#### Track

Tagged set of frames (sequentially or disjoint).

```
string          label
                Track tag name, useful to identify it, duplicates are allowed.

[index]         frame indices
                List of frame positions (at frames list).
                May happen as a range entry or single entry, both can be used together,
                at multiple times and in any order.

[track]         tracks
                A track may have inner tracks, there is no limit how deep it can go.
                This value may be omitted if there is no entries.
```

##### Index

1. single entry

    Single frame index.

    ```
    u32             index
    ```

3. range entry

    Range of sequential indices, inclusive at both sides.

    ```
    u32             from index
    u32             to index
    ```

### Metadata

Data about atlas generation, it can be completely ignored or used as source to check tool version.

```
string          repo url
                Where you can find about the Clymene (will always be a link to this repo).

string          version
                Clymene version which generated it.
                Format: Major.Minor.Patch
```
