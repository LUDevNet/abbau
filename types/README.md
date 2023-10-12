# kaitai-struct-types

This crate provides types for representing [Kaitai Struct YAML (KSY)][KSY]
files in Rust for use with [`serde`] and [`serde_yaml`].

## Design Considerations

- Preserve enough information to round-trip from yaml to yaml for well-formed spec files.
- Use meaningful enums wherever possible (type ref, endian, repeat, ...)

## Out of scope

- Parsing the expression language (will be a separate crate)

## Not Yet Implemented

- [ ] Cross-References (`xref`)
- [ ] Generic Parameters (`params`)
- [ ] Bit Types (`b1`, `b2`, ...)

PRs are welcome!

## Example

Consider the following KSY-Schema:

```yaml
meta:
  id: gif
  file-extension: gif
  endian: le
seq:
  - id: header
    type: header
  - id: logical_screen
    type: logical_screen
types:
  header:
    seq:
      - id: magic
        contents: 'GIF'
      - id: version
        size: 3
  logical_screen:
    seq:
      - id: image_width
        type: u2
      - id: image_height
        type: u2
      - id: flags
        type: u1
      - id: bg_color_index
        type: u1
      - id: pixel_aspect_ratio
        type: u1
```

this will be turned into the following Rust representation:

```rs
KsySchema {
    meta: MetaSpec {
        id: Identifier("gif"),
        file_extension: Some(StringOrArray::String("gif")),
        endian: Some(LittleEndian),
        ..
    },
    seq: [
        Attribute {
            id: Some("header"),
            ty: Some(TypeRef::Named("header")),
            ..
        },
        Attribute {
            id: Some("logical_screen"),
            ty: Some(TypeRef::Named("logical_screen")),
            ..
        },
    ],
    types: {
        "header": TypeSpec {
            seq: [
                Attribute {
                    id: Some("magic"),
                    contents: Some(Contents::String("GIF")),
                    ..
                },
                Attribute {
                    id: Some("version"),
                    size: Some("3"),
                    ..
                },
            ],
            ..
        },
        "logical_screen": TypeSpec {
            seq: [
                Attribute {
                    id: Some("image_width"),
                    ty: Some(TypeRef::WellKnown(Unsigned(Int2(EndianSpec::Implicit)))),
                    ..
                },
                Attribute {
                    id: Some("image_height"),
                    ty: Some(TypeRef::WellKnown(Unsigned(Int2(EndianSpec::Implicit)))),
                    ..
                },
                Attribute {
                    id: Some("flags"),
                    ty: Some(TypeRef::WellKnown(Unsigned(Int1(EndianSpec::Implicit)))),
                    ..
                },
                Attribute {
                    id: Some("bg_color_index",),
                    ty: Some(TypeRef::WellKnown(Unsigned(Int1(EndianSpec::Implicit)))),
                    ..
                },
                Attribute {
                    id: Some("pixel_aspect_ratio"),
                    ty: Some(TypeRef::WellKnown(Unsigned(Int1(EndianSpec::Implicit)))),
                    ..
                },
            ],
            ..
        },
        ..
    },
    ..
}
```

[KSY]: https://doc.kaitai.io/ksy_diagram.html
[`serde`]: https://serde.rs
[`serde_yaml`]: https://crates.io/crates/serde_yaml