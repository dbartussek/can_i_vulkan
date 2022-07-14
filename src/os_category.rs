use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Hash, Ord, PartialOrd, Derivative)]
#[derivative(Debug)]
pub enum OsCategory {
    Windows,
    Linux,

    OSX,
    IOS,

    Android,
    FreeBSD,

    Unknown,

    #[derivative(Debug = "transparent")]
    Other(String),
}

impl OsCategory {
    pub fn parse(s: &str) -> Self {
        match s {
            "windows" => Self::Windows,
            "osx" => Self::OSX,
            "ios" => Self::IOS,

            "android" => Self::Android,

            "FreeBSD" | "freebsd" => Self::FreeBSD,

            "unknown" => Self::Unknown,

            "arch"
            | "archarm"
            | "ubuntu"
            | "manjaro"
            | "manjaro-arm"
            | "opensuse-tumbleweed"
            | "fedora"
            | "linuxmint"
            | "gentoo"
            | "debian"
            | "antergos"
            | "opensuse-leap"
            | "opensuse"
            | "endeavouros"
            | "neon"
            | "solus"
            | "pop"
            | "artix"
            | "arcolinux"
            | "elementary"
            | "RebornOS"
            | "anarchy"
            | "garuda"
            | "kali"
            | "opensuse-microos"
            | "centos"
            | "slackware"
            | "void"
            | "zorin"
            | "calculate"
            | "ArchLabs"
            | "mageia"
            | "Deepin"
            | "deepin"
            | "swagarch"
            | "ArchMerge"
            | "SolydXK"
            | "arcolinuxd"
            | "avouch"
            | "crux"
            | "devuan"
            | "funtoo"
            | "rocky"
            | "uos"
            | "iglunix"
            | "steamos" => Self::Linux,

            other => Self::Other(other.to_string()),
        }
    }
}
