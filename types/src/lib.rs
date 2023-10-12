#![warn(missing_docs)]
//! # Base Types for Kaitai
//!
//! ```
//! use abbau_types::{KsySchema, TypeRef};
//! use serde::Deserialize;
//!
//! let schema = "
//! meta:
//!   id: gif
//!   file-extension: gif
//!   endian: le
//! seq:
//!   - id: header
//!     type: header
//!   - id: logical_screen
//!     type: logical_screen
//! types:
//!   header:
//!     seq:
//!       - id: magic
//!         contents: 'GIF'
//!       - id: version
//!         size: 3
//!   logical_screen:
//!     seq:
//!       - id: image_width
//!         type: u2
//!       - id: image_height
//!         type: u2
//!       - id: flags
//!         type: u1
//!       - id: bg_color_index
//!         type: u1
//!       - id: pixel_aspect_ratio
//!         type: u1
//! ";
//! let schema: KsySchema = serde_yaml::from_str(schema).unwrap();
//! assert_eq!(&schema.meta.id.0, "gif");
//! assert_eq!(schema.meta.file_extension, Some("gif".into()));
//! assert_eq!(schema.seq[1].ty, Some(TypeRef::Named("logical_screen".to_owned())));
//! assert_eq!(schema.types.get("logical_screen").unwrap().seq.len(), 5);
//! ```

mod attribute;
mod enum_spec;
mod ksy_schema;
mod meta_spec;
mod param_spec;
mod scalar;
mod type_ref;
mod type_spec;
pub use {
    attribute::{Attribute, Contents, Repeat},
    enum_spec::{EnumSpec, EnumValueSpec},
    ksy_schema::KsySchema,
    meta_spec::{Endian, MetaSpec, XRef},
    param_spec::ParamSpec,
    scalar::{AnyScalar, Identifier, StringOrArray},
    type_ref::{EndianSpec, FloatTypeRef, IntTypeRef, TypeRef, WellKnownTypeRef},
    type_spec::TypeSpec,
};
