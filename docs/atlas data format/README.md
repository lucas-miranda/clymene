# Atlas Data Format Specification

Holds data about every source file, it's frames and tracks.

## Formats

To see how format works in general terms, please look at [Base](./base-format.md).

### Custom
- [JSON](./json-format.md)

## Concepts

Some concepts which clymene uses to improve it's usage.

### Mixed Indices

Frame indices entries may use more than one entry format.

When reading from source files, clymene will group as many as possible indices into ranges.

For example, entries `1` and `2` will never be seen as single entries side by side, it'll always turn into a range `{ "from": 1, "to": 2 }`.

A track which uses frames: `1`, `3 to 5` and `8`, will be represented as following, using [JSON Specification](./json-format.md):

```json
"indices": [
    1,
    { "from": 3, "to": 5 },
    8
],
```
*Entirely track entry, tracks, frames data and everything else is omitted to simplify.*

### Nested Tracks

Source formats may support nested tracks (e.g [aseprite tags](https://www.aseprite.org/docs/tags/)), where a track can have multiple inner tracks. They can be nested without a limit.

To be considered a nested track, the top track must fully contains the bottom track.

It can be omitted if it's empty.

A simple way to consume the values is just by combining the labels from top track to bottom track, an example achieving that using [JSON Specification](./json-format.md):

```json
"tracks": [
    {
        "label": "walk",
        "indices": [
            { "from": 0, "to": 2 }
        ]
    },
    {
        "label": "attack",
        "indices": [
            { "from": 3, "to": 10 }
        ],
        "tracks": [
            {
                "label": "prepare",
                "indices": [
                    { "from": 3, "to": 5 }
                ]
            },
            {
                "label": "hit",
                "indices": [
                    { "from": 6, "to": 10 }
                ]
            }
        ]
    }
]
```
*Frames data and everything else is omitted to simplify.*

Data above can be interpreted with any separator to combine nested tracks, resulting in `walk`, `attack.prepare`, `attack.hit` tracks.
`attack` can be used also, if it makes sense to play all frames, or played inner parts separated as `attack.prepare` and `attack.hit`.

This is just an example, so it's the consumer job to handle it the way it feels appropriated.
