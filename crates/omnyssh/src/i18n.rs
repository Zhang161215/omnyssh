//! Minimal UI language table for the TUI.
//!
//! Default is Simplified Chinese (`zh-CN`). Pass `--lang en` or set
//! `ui.language` in `config.toml` to keep the original English strings.

use std::sync::OnceLock;

/// Supported interface languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    ZhCn,
    En,
}

static LOCALE: OnceLock<Locale> = OnceLock::new();

impl Locale {
    /// Parse a language tag. Unknown values fall back to Simplified Chinese.
    pub fn parse(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().replace('_', "-").as_str() {
            "en" | "en-us" | "en-gb" => Self::En,
            _ => Self::ZhCn,
        }
    }
}

/// Set the process-wide UI language. Safe to call once at startup.
pub fn init(value: &str) {
    let _ = LOCALE.set(Locale::parse(value));
}

pub fn current() -> Locale {
    *LOCALE.get().unwrap_or(&Locale::ZhCn)
}

/// Pick the Simplified Chinese or English UI string.
pub fn t(zh: &'static str, en: &'static str) -> &'static str {
    match current() {
        Locale::ZhCn => zh,
        Locale::En => en,
    }
}

/// Translate a host/snippet form field. The English label is the stable key.
pub fn form_field(en: &str) -> &'static str {
    match en {
        "Name" => t("名称", "Name"),
        "Hostname / IP" => t("主机名 / IP", "Hostname / IP"),
        "User" => t("用户", "User"),
        "Port" => t("端口", "Port"),
        "Identity File" => t("密钥文件", "Identity File"),
        "Password (optional)" => t("密码（可选）", "Password (optional)"),
        "Tags (comma-sep)" => t("标签（逗号分隔）", "Tags (comma-sep)"),
        "Notes" => t("备注", "Notes"),
        "Monitoring (ssh | tcp | tcp:PORT)" => t(
            "监控（ssh | tcp | tcp:端口）",
            "Monitoring (ssh | tcp | tcp:PORT)",
        ),
        "Command" => t("命令", "Command"),
        "Scope (global / host)" => t("范围（global / host）", "Scope (global / host)"),
        "Host (if scope=host)" => t("主机（范围=host 时）", "Host (if scope=host)"),
        "Params (comma-sep)" => t("参数（逗号分隔）", "Params (comma-sep)"),
        _ => t("字段", "Field"),
    }
}

#[cfg(test)]
mod tests {
    use super::Locale;

    #[test]
    fn parse_english_and_default_chinese() {
        assert_eq!(Locale::parse("en"), Locale::En);
        assert_eq!(Locale::parse("en-US"), Locale::En);
        assert_eq!(Locale::parse("zh-CN"), Locale::ZhCn);
        assert_eq!(Locale::parse("nope"), Locale::ZhCn);
    }
}
