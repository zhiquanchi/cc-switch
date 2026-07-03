use std::path::{Path, PathBuf};

use crate::app_config::AppType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigTargetKind {
    Primary,
    Wsl,
}

#[derive(Debug, Clone)]
pub struct ConfigTarget {
    pub kind: ConfigTargetKind,
    pub dir: PathBuf,
}

pub fn wsl_home_dir() -> Option<PathBuf> {
    crate::settings::get_wsl_home_dir()
}

pub fn wsl_app_config_dir(app: &AppType) -> Option<PathBuf> {
    let home = wsl_home_dir()?;
    Some(app_config_dir_under_home(&home, app))
}

pub fn wsl_claude_mcp_path() -> Option<PathBuf> {
    Some(wsl_home_dir()?.join(".claude.json"))
}

pub fn app_config_dir_under_home(home: &Path, app: &AppType) -> PathBuf {
    match app {
        AppType::Claude => home.join(".claude"),
        AppType::Codex => home.join(".codex"),
        AppType::Gemini => home.join(".gemini"),
        AppType::OpenCode => home.join(".config").join("opencode"),
        AppType::OpenClaw => home.join(".openclaw"),
        AppType::Hermes => home.join(".hermes"),
        AppType::ClaudeDesktop => home.join(".claude-desktop"),
    }
}

pub fn app_config_targets(app: &AppType, primary_dir: PathBuf) -> Vec<ConfigTarget> {
    let mut targets = vec![ConfigTarget {
        kind: ConfigTargetKind::Primary,
        dir: primary_dir,
    }];

    if let Some(wsl_dir) = wsl_app_config_dir(app) {
        targets.push(ConfigTarget {
            kind: ConfigTargetKind::Wsl,
            dir: wsl_dir,
        });
    }

    targets
}

pub fn is_wsl_sync_warning(err: &crate::error::AppError) -> String {
    format!("WSL 配置同步失败: {err}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_wsl_tool_dirs_from_home() {
        let home = PathBuf::from(r"\\wsl$\Ubuntu\home\alice");
        assert_eq!(
            app_config_dir_under_home(&home, &AppType::Claude),
            home.join(".claude")
        );
        assert_eq!(
            app_config_dir_under_home(&home, &AppType::Codex),
            home.join(".codex")
        );
        assert_eq!(
            app_config_dir_under_home(&home, &AppType::Gemini),
            home.join(".gemini")
        );
        assert_eq!(
            app_config_dir_under_home(&home, &AppType::OpenCode),
            home.join(".config").join("opencode")
        );
        assert_eq!(
            app_config_dir_under_home(&home, &AppType::OpenClaw),
            home.join(".openclaw")
        );
        assert_eq!(
            app_config_dir_under_home(&home, &AppType::Hermes),
            home.join(".hermes")
        );
    }
}
