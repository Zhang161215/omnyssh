use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::{AppState, FileManagerPopup, Screen, SnippetPopup, ViewState};
use crate::i18n::t;

/// Renders the bottom status bar with context-sensitive key hints.
pub fn render(frame: &mut Frame, area: Rect, state: &AppState, view: &ViewState) {
    // Apply active theme colours.
    let key_style = Style::default()
        .fg(view.theme.key_badge_fg)
        .bg(view.theme.key_badge_bg)
        .add_modifier(Modifier::BOLD);
    let sep_style = Style::default().fg(view.theme.separator_fg);
    let hint_style = Style::default().fg(view.theme.hint_fg);

    macro_rules! key {
        ($k:expr) => {
            Span::styled(format!(" {} ", $k), key_style)
        };
    }
    macro_rules! hint {
        ($h:expr) => {
            Span::styled(format!(" {} ", $h), hint_style)
        };
    }
    macro_rules! sep {
        () => {
            Span::styled(" │ ", sep_style)
        };
    }

    // If a status message is set, show it instead of key hints.
    if let Some(msg) = &view.status_message {
        let line = Line::from(Span::styled(
            format!(" {}", msg),
            Style::default().fg(Color::Yellow),
        ));
        frame.render_widget(
            Paragraph::new(line).style(Style::default().bg(Color::Reset)),
            area,
        );
        return;
    }

    let hlv = &view.host_list;

    // Search mode: show search-specific hints.
    if hlv.search_mode {
        let line = Line::from(vec![
            Span::styled(
                t(" [搜索] ", " [SEARCH] "),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                t(" 输入以筛选  ", " type to filter  "),
                Style::default().fg(Color::Gray),
            ),
            key!("Enter"),
            hint!(t("确认", "confirm")),
            sep!(),
            key!("Esc"),
            hint!(t("清除", "clear")),
        ]);
        frame.render_widget(
            Paragraph::new(line).style(Style::default().bg(Color::Reset)),
            area,
        );
        return;
    }

    // Popup mode: show popup-specific hints.
    if let Some(popup) = &hlv.popup {
        use crate::app::HostPopup;
        let line = match popup {
            HostPopup::Add(_) | HostPopup::Edit { .. } => Line::from(vec![
                key!("Tab"),
                hint!(t("下一字段", "next field")),
                sep!(),
                key!("Shift+Tab"),
                hint!(t("上一字段", "prev field")),
                sep!(),
                key!("Enter"),
                hint!(t("保存", "save")),
                sep!(),
                key!("Esc"),
                hint!(t("取消", "cancel")),
            ]),
            HostPopup::DeleteConfirm(_) => Line::from(vec![
                key!("y"),
                hint!(t("确认删除", "confirm delete")),
                sep!(),
                key!("n / Esc"),
                hint!(t("取消", "cancel")),
            ]),
            HostPopup::KeySetupConfirm(_) => Line::from(vec![
                key!("y / Enter"),
                hint!(t("确认配置", "confirm setup")),
                sep!(),
                key!("n / Esc"),
                hint!(t("取消", "cancel")),
            ]),
            HostPopup::KeySetupProgress { .. } => Line::from(vec![
                Span::styled(
                    t(" 正在配置 SSH 密钥… ", " Setting up SSH keys… "),
                    hint_style,
                ),
                sep!(),
                key!("Esc"),
                hint!(t("关闭", "close")),
            ]),
        };
        frame.render_widget(
            Paragraph::new(line).style(Style::default().bg(Color::Reset)),
            area,
        );
        return;
    }

    // Snippets search mode.
    if view.snippets_view.search_mode {
        let line = Line::from(vec![
            Span::styled(
                t(" [搜索] ", " [SEARCH] "),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                t(" 输入以筛选  ", " type to filter  "),
                Style::default().fg(Color::Gray),
            ),
            key!("Enter"),
            hint!(t("确认", "confirm")),
            sep!(),
            key!("Esc"),
            hint!(t("清除", "clear")),
        ]);
        frame.render_widget(
            Paragraph::new(line).style(Style::default().bg(Color::Reset)),
            area,
        );
        return;
    }

    // Snippets popup hints.
    if let Some(popup) = &view.snippets_view.popup {
        let line = match popup {
            SnippetPopup::Add(_) | SnippetPopup::Edit { .. } => Line::from(vec![
                key!("Tab"),
                hint!(t("下一字段", "next field")),
                sep!(),
                key!("Shift+Tab"),
                hint!(t("上一字段", "prev field")),
                sep!(),
                key!("Enter"),
                hint!(t("保存", "save")),
                sep!(),
                key!("Esc"),
                hint!(t("取消", "cancel")),
            ]),
            SnippetPopup::DeleteConfirm(_) => Line::from(vec![
                key!("y"),
                hint!(t("确认删除", "confirm delete")),
                sep!(),
                key!("n / Esc"),
                hint!(t("取消", "cancel")),
            ]),
            SnippetPopup::ParamInput { .. } => Line::from(vec![
                key!("Tab"),
                hint!(t("下一参数", "next param")),
                sep!(),
                key!("Enter"),
                hint!(t("运行", "run")),
                sep!(),
                key!("Esc"),
                hint!(t("取消", "cancel")),
            ]),
            SnippetPopup::BroadcastPicker { .. } => Line::from(vec![
                key!("j/k"),
                hint!(t("浏览", "navigate")),
                sep!(),
                key!("Space"),
                hint!(t("切换", "toggle")),
                sep!(),
                key!("Enter"),
                hint!(t("运行", "run")),
                sep!(),
                key!("Esc"),
                hint!(t("取消", "cancel")),
            ]),
            SnippetPopup::QuickExecuteInput { .. } => Line::from(vec![
                key!("Enter"),
                hint!(t("运行", "run")),
                sep!(),
                key!("Esc"),
                hint!(t("取消", "cancel")),
            ]),
            SnippetPopup::Results { .. } => Line::from(vec![
                key!("j/k"),
                hint!(t("滚动", "scroll")),
                sep!(),
                key!("Esc"),
                hint!(t("关闭", "close")),
            ]),
        };
        frame.render_widget(
            Paragraph::new(line).style(Style::default().bg(Color::Reset)),
            area,
        );
        return;
    }

    // File manager popup hints.
    if matches!(state.screen, Screen::FileManager) {
        if let Some(fm_popup) = &view.file_manager.popup {
            let line = match fm_popup {
                FileManagerPopup::HostPicker { .. } => Line::from(vec![
                    key!("j/k"),
                    hint!(t("浏览", "navigate")),
                    sep!(),
                    key!("Enter"),
                    hint!(t("连接", "connect")),
                    sep!(),
                    key!("Esc"),
                    hint!(t("取消", "cancel")),
                ]),
                FileManagerPopup::DeleteConfirm { .. } => Line::from(vec![
                    key!("y"),
                    hint!(t("确认删除", "confirm delete")),
                    sep!(),
                    key!("n / Esc"),
                    hint!(t("取消", "cancel")),
                ]),
                FileManagerPopup::MkDir(_) | FileManagerPopup::Rename { .. } => Line::from(vec![
                    key!("Enter"),
                    hint!(t("确认", "confirm")),
                    sep!(),
                    key!("Esc"),
                    hint!(t("取消", "cancel")),
                ]),
                FileManagerPopup::TransferProgress {
                    filename,
                    done,
                    total,
                    ..
                } => {
                    let pct = if *total > 0 {
                        ((*done as f64 / *total as f64) * 100.0) as u64
                    } else {
                        0
                    };
                    Line::from(vec![Span::styled(
                        format!(" {}: {}  {}% ", t("传输中", "Transferring"), filename, pct),
                        hint_style,
                    )])
                }
            };
            frame.render_widget(
                Paragraph::new(line).style(Style::default().bg(Color::Reset)),
                area,
            );
            return;
        }
    }

    // Tag popup: show its own hints.
    if hlv.tag_popup_open {
        let line = Line::from(vec![
            key!("j/k"),
            hint!(t("浏览", "navigate")),
            sep!(),
            key!("Enter"),
            hint!(t("选择", "select")),
            sep!(),
            key!("Esc"),
            hint!(t("关闭", "close")),
        ]);
        frame.render_widget(
            Paragraph::new(line).style(Style::default().bg(Color::Reset)),
            area,
        );
        return;
    }

    // Global hints only - screen-specific hints are now in page headers.
    let spans = vec![
        key!("1"),
        hint!(t("仪表盘", "Dashboard")),
        sep!(),
        key!("2"),
        hint!(t("文件", "Files")),
        sep!(),
        key!("3"),
        hint!(t("片段", "Snippets")),
        sep!(),
        key!("4"),
        hint!(t("终端", "Terminal")),
        sep!(),
        key!("?"),
        hint!(t("帮助", "Help")),
        sep!(),
        key!("q"),
        hint!(t("退出", "Quit")),
    ];

    frame.render_widget(
        Paragraph::new(Line::from(spans)).style(Style::default().bg(Color::Reset)),
        area,
    );
}
