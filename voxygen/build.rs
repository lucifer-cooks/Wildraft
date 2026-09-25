#[cfg(windows)]
fn main() {
    // Windows application identity + icon
    let mut res = winres::WindowsResource::new();
    res.set_icon("app-icon.ico");
    res.set("InternalName", "WILDRAFT");
    res.set("FileDescription", "WILDRAFT");
    res.set("ProductName", "WILDRAFT");
    res.set_version_info(winres::VersionInfo::FILEVERSION, 0x0001000000000000);
    res.set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0001000000000000);
    res.compile()
        .expect("failed to build executable resources.");
}

#[cfg(not(windows))]
fn main() {}
