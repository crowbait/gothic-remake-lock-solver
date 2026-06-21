#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl VersionInfo {
    /** Expects (e.g.) "`0.1.2`" or "`v0.1.2`". Automatically removes all characters not a digit or "`.`". */
    pub fn from_str(str: &str) -> Self {
        let parts: Vec<u32> = str
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.')
            .collect::<String>()
            .split('.')
            .map(|s| s.parse().unwrap())
            .collect();
        VersionInfo {
            major: parts[0],
            minor: parts[1],
            patch: parts[2],
        }
    }
    /** Returns -1 of the passed version is older, 0 if same, 1 if passed version is newer. */
    pub fn compare(self: &Self, other: &VersionInfo) -> i8 {
        if self.major != other.major {
            if self.major < other.major { 1 } else { -1 }
        } else if self.minor != other.minor {
            if self.minor < other.minor { 1 } else { -1 }
        } else if self.patch != other.patch {
            if self.patch < other.patch { 1 } else { -1 }
        } else {
            0
        }
    }
    pub fn to_string(self: &Self) -> String {
        format!("v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Clone)]
pub struct AvailableUpdate {
    pub version_info: VersionInfo,
    /** URL to release page (open in browser) */
    pub release_page_url: String,
    /** URL to exe */
    pub exe_direct_download_url: String,
}

pub fn get_available_version() -> Result<Option<AvailableUpdate>, String> {
    let client = reqwest::blocking::Client::new();
    let release: Option<AvailableUpdate> = client
        .get("https://api.github.com/repos/crowbait/gothic-remake-lock-solver/releases/latest")
        .header(
            reqwest::header::USER_AGENT,
            "crowbait/gothic-remake-lock-solver",
        )
        .send()
        .ok()
        .and_then(|res| res.text().ok())
        .and_then(|res| serde_json::from_str(&res).ok())
        .and_then(|res: serde_json::Value| {
            let ver = res.get("tag_name").unwrap().as_str().unwrap().to_string();
            let rel = res.get("html_url").unwrap().as_str().unwrap().to_string();
            let exe = res.get("assets").unwrap().as_array().unwrap()[0]
                .get("browser_download_url")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();

            Some(AvailableUpdate {
                version_info: VersionInfo::from_str(&ver),
                release_page_url: rel,
                exe_direct_download_url: exe,
            })
        });

    if let Some(available_release) = release {
        let cur_ver = VersionInfo::from_str(env!("CARGO_PKG_VERSION"));
        let compared = cur_ver.compare(&available_release.version_info);

        if compared == 1 {
            Ok(Some(available_release))
        } else {
            Ok(None)
        }
    } else {
        Err(String::from("Failed checking for updates"))
    }
}
