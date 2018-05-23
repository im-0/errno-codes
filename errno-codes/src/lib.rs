#![cfg_attr(feature = "unstable", warn(unreachable_pub))]
#![forbid(unsafe_code)]
#![warn(unused_results)]
#![cfg_attr(feature = "cargo-clippy", warn(empty_line_after_outer_attr))]
#![cfg_attr(feature = "cargo-clippy", warn(filter_map))]
#![cfg_attr(feature = "cargo-clippy", warn(if_not_else))]
#![cfg_attr(feature = "cargo-clippy", warn(mut_mut))]
#![cfg_attr(feature = "cargo-clippy", warn(non_ascii_literal))]
#![cfg_attr(feature = "cargo-clippy", warn(option_map_unwrap_or))]
#![cfg_attr(feature = "cargo-clippy", warn(option_map_unwrap_or_else))]
#![cfg_attr(feature = "cargo-clippy", warn(single_match_else))]
#![cfg_attr(feature = "cargo-clippy", warn(wrong_pub_self_convention))]
#![cfg_attr(feature = "cargo-clippy", warn(use_self))]
#![cfg_attr(feature = "cargo-clippy", warn(used_underscore_binding))]
#![cfg_attr(feature = "cargo-clippy", warn(print_stdout))]
#![cfg_attr(feature = "cargo-clippy", warn(else_if_without_else))]

extern crate phf;

#[cfg(feature = "serialization")]
extern crate serde;
#[cfg(feature = "serialization")]
#[macro_use]
extern crate serde_derive;

/// errno codes for Unix and Unix-like OSes.
pub mod unix;
/// errno codes for Windows.
pub mod windows;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
/// errno code description.
pub struct ErrnoCode {
    /// Numeric value.
    pub num: std::os::raw::c_int,
    /// Human-readable message.
    pub msg: &'static str,
    /// Name of identifier.
    pub id: &'static str,
}

pub type ErrnoCodes = &'static [ErrnoCode];

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
/// Known Linux architectures.
pub enum LinuxArch {
    Alpha,
    ARC,
    ARM,
    ARM64,
    C6x,
    H8300,
    Hexagon,
    IA64,
    M68k,
    Microblaze,
    MIPS,
    NDS32,
    Nios2,
    OpenRISC,
    PARISC,
    PowerPC,
    RISCV,
    S390,
    SH,
    SPARC,
    UM,
    Unicore32,
    X86,
    Xtensa,

    Any,

    #[doc(hidden)]
    /// This enum may be extended in future, use catch-all `_` arm to match future variants.
    __NonExhaustive,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
/// Unix and Unix-like OSes.
pub enum UnixOs {
    Linux(LinuxArch),

    Any,

    #[doc(hidden)]
    /// This enum may be extended in future, use catch-all `_` arm to match future variants.
    __NonExhaustive,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
/// OS family.
pub enum Family {
    Unix(UnixOs),
    Windows,

    Any,

    #[doc(hidden)]
    /// This enum may be extended in future, use catch-all `_` arm to match future variants.
    __NonExhaustive,
}
