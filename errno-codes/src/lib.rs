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

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

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

// TODO: Use `macro-attr` crate to automatically generate code for enums.

pub trait GetMappings {
    /// Get identifier string <-> `ErrnoCode` mapping for this type.
    fn get_by_id_mapping(&self) -> &'static phf::Map<&'static str, ErrnoCode>;
    /// Get numeric constant <-> `ErrnoCode` mapping for this type.
    fn get_by_num_mapping(&self) -> &'static phf::Map<std::os::raw::c_int, ErrnoCodes>;
}

trait Expand {
    type Expanded;

    fn expand(&self, f: impl FnMut(&Self::Expanded) -> ());
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
/// Known Linux architectures (search query).
pub enum QueryLinuxArch {
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

impl Expand for QueryLinuxArch {
    type Expanded = LinuxArch;

    fn expand(&self, mut f: impl FnMut(&Self::Expanded) -> ()) {
        match *self {
            QueryLinuxArch::Alpha => f(&LinuxArch::Alpha),
            QueryLinuxArch::ARC => f(&LinuxArch::ARC),
            QueryLinuxArch::ARM => f(&LinuxArch::ARM),
            QueryLinuxArch::ARM64 => f(&LinuxArch::ARM64),
            QueryLinuxArch::C6x => f(&LinuxArch::C6x),
            QueryLinuxArch::H8300 => f(&LinuxArch::H8300),
            QueryLinuxArch::Hexagon => f(&LinuxArch::Hexagon),
            QueryLinuxArch::IA64 => f(&LinuxArch::IA64),
            QueryLinuxArch::M68k => f(&LinuxArch::M68k),
            QueryLinuxArch::Microblaze => f(&LinuxArch::Microblaze),
            QueryLinuxArch::MIPS => f(&LinuxArch::MIPS),
            QueryLinuxArch::NDS32 => f(&LinuxArch::NDS32),
            QueryLinuxArch::Nios2 => f(&LinuxArch::Nios2),
            QueryLinuxArch::OpenRISC => f(&LinuxArch::OpenRISC),
            QueryLinuxArch::PARISC => f(&LinuxArch::PARISC),
            QueryLinuxArch::PowerPC => f(&LinuxArch::PowerPC),
            QueryLinuxArch::RISCV => f(&LinuxArch::RISCV),
            QueryLinuxArch::S390 => f(&LinuxArch::S390),
            QueryLinuxArch::SH => f(&LinuxArch::SH),
            QueryLinuxArch::SPARC => f(&LinuxArch::SPARC),
            QueryLinuxArch::UM => f(&LinuxArch::UM),
            QueryLinuxArch::Unicore32 => f(&LinuxArch::Unicore32),
            QueryLinuxArch::X86 => f(&LinuxArch::X86),
            QueryLinuxArch::Xtensa => f(&LinuxArch::Xtensa),

            QueryLinuxArch::Any => LinuxArch::all_known().iter().for_each(f),

            QueryLinuxArch::__NonExhaustive => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
/// Unix and Unix-like OSes (search query).
pub enum QueryUnix {
    Linux(QueryLinuxArch),

    Any,

    #[doc(hidden)]
    /// This enum may be extended in future, use catch-all `_` arm to match future variants.
    __NonExhaustive,
}

impl Expand for QueryUnix {
    type Expanded = Unix;

    fn expand(&self, mut f: impl FnMut(&Self::Expanded) -> ()) {
        match *self {
            QueryUnix::Linux(arch) => arch.expand(|arch| f(&Unix::Linux(*arch))),

            QueryUnix::Any => Unix::all_known().iter().for_each(f),

            QueryUnix::__NonExhaustive => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
/// OS family (search query).
pub enum QueryFamily {
    Unix(QueryUnix),
    Windows,

    Any,

    #[doc(hidden)]
    /// This enum may be extended in future, use catch-all `_` arm to match future variants.
    __NonExhaustive,
}

impl Expand for QueryFamily {
    type Expanded = Family;

    fn expand(&self, mut f: impl FnMut(&Self::Expanded) -> ()) {
        match *self {
            QueryFamily::Unix(unix) => unix.expand(|unix| f(&Family::Unix(*unix))),
            QueryFamily::Windows => f(&Family::Windows),

            QueryFamily::Any => Family::all_known().iter().for_each(f),

            QueryFamily::__NonExhaustive => unreachable!(),
        }
    }
}

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

    #[doc(hidden)]
    /// This enum may be extended in future, use catch-all `_` arm to match future variants.
    __NonExhaustive,
}

impl LinuxArch {
    /// Return all known Linux architectures.
    pub fn all_known() -> &'static [Self] {
        &[
            LinuxArch::Alpha,
            LinuxArch::ARC,
            LinuxArch::ARM,
            LinuxArch::ARM64,
            LinuxArch::C6x,
            LinuxArch::H8300,
            LinuxArch::Hexagon,
            LinuxArch::IA64,
            LinuxArch::M68k,
            LinuxArch::Microblaze,
            LinuxArch::MIPS,
            LinuxArch::NDS32,
            LinuxArch::Nios2,
            LinuxArch::OpenRISC,
            LinuxArch::PARISC,
            LinuxArch::PowerPC,
            LinuxArch::RISCV,
            LinuxArch::S390,
            LinuxArch::SH,
            LinuxArch::SPARC,
            LinuxArch::UM,
            LinuxArch::Unicore32,
            LinuxArch::X86,
            LinuxArch::Xtensa,
        ]
    }
}

impl GetMappings for LinuxArch {
    fn get_by_id_mapping(&self) -> &'static phf::Map<&'static str, ErrnoCode> {
        match *self {
            LinuxArch::Alpha => &unix::linux::alpha::BY_ID,
            LinuxArch::ARC => &unix::linux::arc::BY_ID,
            LinuxArch::ARM => &unix::linux::arm::BY_ID,
            LinuxArch::ARM64 => &unix::linux::arm64::BY_ID,
            LinuxArch::C6x => &unix::linux::c6x::BY_ID,
            LinuxArch::H8300 => &unix::linux::h8300::BY_ID,
            LinuxArch::Hexagon => &unix::linux::hexagon::BY_ID,
            LinuxArch::IA64 => &unix::linux::ia64::BY_ID,
            LinuxArch::M68k => &unix::linux::m68k::BY_ID,
            LinuxArch::Microblaze => &unix::linux::microblaze::BY_ID,
            LinuxArch::MIPS => &unix::linux::mips::BY_ID,
            LinuxArch::NDS32 => &unix::linux::nds32::BY_ID,
            LinuxArch::Nios2 => &unix::linux::nios2::BY_ID,
            LinuxArch::OpenRISC => &unix::linux::openrisc::BY_ID,
            LinuxArch::PARISC => &unix::linux::parisc::BY_ID,
            LinuxArch::PowerPC => &unix::linux::powerpc::BY_ID,
            LinuxArch::RISCV => &unix::linux::riscv::BY_ID,
            LinuxArch::S390 => &unix::linux::s390::BY_ID,
            LinuxArch::SH => &unix::linux::sh::BY_ID,
            LinuxArch::SPARC => &unix::linux::sparc::BY_ID,
            LinuxArch::UM => &unix::linux::um::BY_ID,
            LinuxArch::Unicore32 => &unix::linux::unicore32::BY_ID,
            LinuxArch::X86 => &unix::linux::x86::BY_ID,
            LinuxArch::Xtensa => &unix::linux::xtensa::BY_ID,

            LinuxArch::__NonExhaustive => unreachable!(),
        }
    }

    fn get_by_num_mapping(&self) -> &'static phf::Map<std::os::raw::c_int, ErrnoCodes> {
        match *self {
            LinuxArch::Alpha => &unix::linux::alpha::BY_NUM,
            LinuxArch::ARC => &unix::linux::arc::BY_NUM,
            LinuxArch::ARM => &unix::linux::arm::BY_NUM,
            LinuxArch::ARM64 => &unix::linux::arm64::BY_NUM,
            LinuxArch::C6x => &unix::linux::c6x::BY_NUM,
            LinuxArch::H8300 => &unix::linux::h8300::BY_NUM,
            LinuxArch::Hexagon => &unix::linux::hexagon::BY_NUM,
            LinuxArch::IA64 => &unix::linux::ia64::BY_NUM,
            LinuxArch::M68k => &unix::linux::m68k::BY_NUM,
            LinuxArch::Microblaze => &unix::linux::microblaze::BY_NUM,
            LinuxArch::MIPS => &unix::linux::mips::BY_NUM,
            LinuxArch::NDS32 => &unix::linux::nds32::BY_NUM,
            LinuxArch::Nios2 => &unix::linux::nios2::BY_NUM,
            LinuxArch::OpenRISC => &unix::linux::openrisc::BY_NUM,
            LinuxArch::PARISC => &unix::linux::parisc::BY_NUM,
            LinuxArch::PowerPC => &unix::linux::powerpc::BY_NUM,
            LinuxArch::RISCV => &unix::linux::riscv::BY_NUM,
            LinuxArch::S390 => &unix::linux::s390::BY_NUM,
            LinuxArch::SH => &unix::linux::sh::BY_NUM,
            LinuxArch::SPARC => &unix::linux::sparc::BY_NUM,
            LinuxArch::UM => &unix::linux::um::BY_NUM,
            LinuxArch::Unicore32 => &unix::linux::unicore32::BY_NUM,
            LinuxArch::X86 => &unix::linux::x86::BY_NUM,
            LinuxArch::Xtensa => &unix::linux::xtensa::BY_NUM,

            LinuxArch::__NonExhaustive => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
/// Unix and Unix-like OSes.
pub enum Unix {
    Linux(LinuxArch),

    #[doc(hidden)]
    /// This enum may be extended in future, use catch-all `_` arm to match future variants.
    __NonExhaustive,
}

impl Unix {
    /// Return all known Unix and Unix-like OSes.
    pub fn all_known() -> &'static [Self] {
        &[
            Unix::Linux(LinuxArch::Alpha),
            Unix::Linux(LinuxArch::ARC),
            Unix::Linux(LinuxArch::ARM),
            Unix::Linux(LinuxArch::ARM64),
            Unix::Linux(LinuxArch::C6x),
            Unix::Linux(LinuxArch::H8300),
            Unix::Linux(LinuxArch::Hexagon),
            Unix::Linux(LinuxArch::IA64),
            Unix::Linux(LinuxArch::M68k),
            Unix::Linux(LinuxArch::Microblaze),
            Unix::Linux(LinuxArch::MIPS),
            Unix::Linux(LinuxArch::NDS32),
            Unix::Linux(LinuxArch::Nios2),
            Unix::Linux(LinuxArch::OpenRISC),
            Unix::Linux(LinuxArch::PARISC),
            Unix::Linux(LinuxArch::PowerPC),
            Unix::Linux(LinuxArch::RISCV),
            Unix::Linux(LinuxArch::S390),
            Unix::Linux(LinuxArch::SH),
            Unix::Linux(LinuxArch::SPARC),
            Unix::Linux(LinuxArch::UM),
            Unix::Linux(LinuxArch::Unicore32),
            Unix::Linux(LinuxArch::X86),
            Unix::Linux(LinuxArch::Xtensa),
        ]
    }
}

impl GetMappings for Unix {
    fn get_by_id_mapping(&self) -> &'static phf::Map<&'static str, ErrnoCode> {
        match *self {
            Unix::Linux(arch) => arch.get_by_id_mapping(),

            Unix::__NonExhaustive => unreachable!(),
        }
    }

    fn get_by_num_mapping(&self) -> &'static phf::Map<std::os::raw::c_int, ErrnoCodes> {
        match *self {
            Unix::Linux(arch) => arch.get_by_num_mapping(),

            Unix::__NonExhaustive => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
/// OS family.
pub enum Family {
    Unix(Unix),
    Windows,

    #[doc(hidden)]
    /// This enum may be extended in future, use catch-all `_` arm to match future variants.
    __NonExhaustive,
}

impl Family {
    /// Return all known OS families.
    pub fn all_known() -> &'static [Self] {
        &[
            Family::Unix(Unix::Linux(LinuxArch::Alpha)),
            Family::Unix(Unix::Linux(LinuxArch::ARC)),
            Family::Unix(Unix::Linux(LinuxArch::ARM)),
            Family::Unix(Unix::Linux(LinuxArch::ARM64)),
            Family::Unix(Unix::Linux(LinuxArch::C6x)),
            Family::Unix(Unix::Linux(LinuxArch::H8300)),
            Family::Unix(Unix::Linux(LinuxArch::Hexagon)),
            Family::Unix(Unix::Linux(LinuxArch::IA64)),
            Family::Unix(Unix::Linux(LinuxArch::M68k)),
            Family::Unix(Unix::Linux(LinuxArch::Microblaze)),
            Family::Unix(Unix::Linux(LinuxArch::MIPS)),
            Family::Unix(Unix::Linux(LinuxArch::NDS32)),
            Family::Unix(Unix::Linux(LinuxArch::Nios2)),
            Family::Unix(Unix::Linux(LinuxArch::OpenRISC)),
            Family::Unix(Unix::Linux(LinuxArch::PARISC)),
            Family::Unix(Unix::Linux(LinuxArch::PowerPC)),
            Family::Unix(Unix::Linux(LinuxArch::RISCV)),
            Family::Unix(Unix::Linux(LinuxArch::S390)),
            Family::Unix(Unix::Linux(LinuxArch::SH)),
            Family::Unix(Unix::Linux(LinuxArch::SPARC)),
            Family::Unix(Unix::Linux(LinuxArch::UM)),
            Family::Unix(Unix::Linux(LinuxArch::Unicore32)),
            Family::Unix(Unix::Linux(LinuxArch::X86)),
            Family::Unix(Unix::Linux(LinuxArch::Xtensa)),
            Family::Windows,
        ]
    }
}

impl GetMappings for Family {
    fn get_by_id_mapping(&self) -> &'static phf::Map<&'static str, ErrnoCode> {
        match *self {
            Family::Unix(unix) => unix.get_by_id_mapping(),
            Family::Windows => &windows::BY_ID,

            Family::__NonExhaustive => unreachable!(),
        }
    }

    fn get_by_num_mapping(&self) -> &'static phf::Map<std::os::raw::c_int, ErrnoCodes> {
        match *self {
            Family::Unix(unix) => unix.get_by_num_mapping(),
            Family::Windows => &windows::BY_NUM,

            Family::__NonExhaustive => unreachable!(),
        }
    }
}

/// Search errno information by string identifier.
pub fn search_by_id(query: QueryFamily, err_id: &str) -> Vec<(Family, &'static ErrnoCode)> {
    let err_id = err_id.to_uppercase();

    let mut result = Vec::new();
    query.expand(|family| {
        let _ = family
            .get_by_id_mapping()
            .get(err_id.as_str())
            .map(|code| result.push((*family, code)));
    });
    result
}

/// Search errno information by numeric constant.
pub fn search_by_num(query: QueryFamily, errnum: std::os::raw::c_int) -> Vec<(Family, &'static ErrnoCode)> {
    let mut result = Vec::new();
    query.expand(|family| {
        let _ = family
            .get_by_num_mapping()
            .get(&errnum)
            .map(|codes| result.extend(codes.iter().map(|code| (*family, code))));
    });
    result
}

#[cfg(test)]
mod tests {
    use std;

    #[test]
    fn expand() {
        use super::Expand;

        let mut s = std::collections::HashSet::new();
        super::QueryFamily::Any.expand(|family| {
            assert!(s.insert(*family));
        });
        assert_eq!(s, super::Family::all_known().iter().cloned().collect());

        let mut s = std::collections::HashSet::new();
        super::QueryFamily::Unix(super::QueryUnix::Any).expand(|family| {
            assert!(s.insert(*family));
        });
        assert_eq!(
            s,
            super::Unix::all_known()
                .iter()
                .map(|unix| super::Family::Unix(*unix))
                .collect()
        );
    }

    #[test]
    fn search_by_id() {
        assert_eq!(
            super::search_by_id(super::QueryFamily::Any, "EDOM").len(),
            super::Family::all_known().len()
        );
    }

    #[test]
    fn search_by_num() {
        assert!(super::search_by_num(super::QueryFamily::Any, 1).len() > 1);
    }
}
