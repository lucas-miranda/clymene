# JSON Atlas Data Format Specification

Specification when using JSON as the output format.
It's based on [format specification](./base-format.md).

## Differences

Changes from [format specification](./base-format.md), which makes sense to this format.

- `source filename`: Instead of a field, it's the key value at `graphics` dictionary.
- `nothing`: When a value will be `nothing`, `null` is used.
- `rect`: A simple key-value entry is used.
    ```json
    {
        "x": 0,
        "y": 0,
        "width": 0,
        "height": 0
    }
    ```

## Format

```json
{
    "graphics": {
        source filename which yield this data
        "source filename": {
            "frames": [
                {
                    "atlas": {
                        "x": 0,
                        "y": 0,
                        "width": 0,
                        "height": 0
                    },
                    "duration": 0,
                    "source": {
                        "x": 0,
                        "y": 0,
                        "width": 0,
                        "height": 0
                    }
                }
            ],
            "tracks": [
                {
                    "label": "",
                    "indices": [
                        single entry
                        0,

                        range entry
                        { "from": 0, "to": 0 }
                    ],
                    empty (omitted) or nested track entries
                    "tracks": [
                    ]
                }
            ]
        }
    },
    "meta": {
        "app": "repo url",
        "version": "major.minor.patch"
    }
}
```

*Parse errors above are intentional to highlight important information*
