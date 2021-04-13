use crate::DebControlError;

use pkgspec::SpecStruct;
use sailfish::TemplateOnce;

#[derive(Clone, Debug, Default, TemplateOnce, PartialEq, SpecStruct)]
#[template(path = "binary.stpl")]
#[spec_error(DebControlError)]
pub struct BinaryDebControl {
    /// The name of the binary package.
    package: String,
    /// The version number of a package. The format is: [epoch:]upstream_version[-debian_revision].
    version: String,
    /// Depending on context and the control file used, the Architecture field can include the following sets of values:
    ///  - A unique single word identifying a Debian machine architecture
    ///  - `any` matches all Debian machine architectures and is the most frequently used
    ///  - `all` which indicates an architecture-independent package
    ///  - `source` which indicates a source package
    architecture: String,
    /// The package maintainer’s name and email address. The name must come first, then the email address inside angle
    /// brackets <> (in RFC822 format).
    maintainer: String,
    /// Description of the package
    description: String,

    /// This field identifies the source package name
    source: Option<String>,
    /// This field specifies an application area into which the package has been classified
    section: Option<String>,
    /// This field represents how important it is that the user have the package installed
    priority: Option<String>,
    /// Estimate of the total amount of disk space required to install the named package
    installed_size: Option<String>,
    /// The URL of the web site for this package
    homepage: Option<String>,
    built_using: Option<String>,

    /// This is a boolean field which may occur only in the control file of a binary package or in a per-package fields
    /// paragraph of a source package control file.
    essential: bool,

    pre_depends: Vec<String>,
    depends: Vec<String>,
    recommends: Vec<String>,
    suggests: Vec<String>,
    breaks: Vec<String>,
    conflicts: Vec<String>,
    provides: Vec<String>,
    replaces: Vec<String>,
    enchances: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DebControlBuilder;

    #[test]
    fn builds_and_renders_binary_control() {
        let expect = BinaryDebControl {
            package: "debcontrol".to_string(),
            version: "1".to_string(),
            architecture: "any".to_string(),
            maintainer: "Wojciech Kępka <wojciech@wkepka.dev>".to_string(),
            description: "crate for DEB/control file generation".to_string(),
            source: Some("package.tar.gz".to_string()),
            section: Some("devel".to_string()),
            priority: None,
            installed_size: None,
            homepage: Some("https://github.com/wojciechkepka/debcontrol".to_string()),
            built_using: Some("rustc".to_string()),
            essential: true,
            pre_depends: vec!["rustc".to_string(), "cargo".to_string()],
            depends: vec!["rustc".to_string(), "cargo".to_string()],
            recommends: vec![],
            suggests: vec![],
            breaks: vec![],
            conflicts: vec!["rustc".to_string(), "cargo".to_string()],
            provides: vec![
                "rustc".to_string(),
                "cargo".to_string(),
                "debcontrol".to_string(),
            ],
            replaces: vec!["rustc".to_string(), "cargo".to_string()],
            enchances: vec!["rustc".to_string(), "cargo".to_string()],
        };
        const OUT: &str = r#"Package:        debcontrol
Version:        1
Architecture:   any
Maintainer:     Wojciech Kępka <wojciech@wkepka.dev>
Description:    crate for DEB/control file generation
Essential:      yes
Source:         package.tar.gz
Section:        devel


Homepage:       https://github.com/wojciechkepka/debcontrol
Built-Using:    rustc

Pre-Depends: rustc
Pre-Depends: cargo

Depends: rustc
Depends: cargo




Conflicts: rustc
Conflicts: cargo

Provides: rustc
Provides: cargo
Provides: debcontrol

Replaces: rustc
Replaces: cargo

Enchances: rustc
Enchances: cargo
"#;
        let got = DebControlBuilder::binary_package_builder("debcontrol")
            .source("package.tar.gz")
            .version("1")
            .architecture("any")
            .maintainer("Wojciech Kępka <wojciech@wkepka.dev>")
            .description("crate for DEB/control file generation")
            .essential(true)
            .section("devel")
            .homepage("https://github.com/wojciechkepka/debcontrol")
            .built_using("rustc")
            .add_pre_depends_entries(vec!["rustc", "cargo"])
            .add_depends_entries(vec!["rustc", "cargo"])
            .add_conflicts_entries(vec!["rustc", "cargo"])
            .add_provides_entries(vec!["rustc", "cargo"])
            .add_replaces_entries(vec!["rustc", "cargo"])
            .add_enchances_entries(vec!["rustc", "cargo"])
            .add_provides_entries(vec!["debcontrol"])
            .build();

        assert_eq!(expect, got);
        assert_eq!(OUT, got.render().unwrap());
    }
}
