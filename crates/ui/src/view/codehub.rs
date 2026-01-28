use std::{fs::File, io::prelude::*};

use chrono::{DateTime, Utc};
use gpui::{
    App, AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
    Styled, Subscription, Window, div, px, rgb,
};
use gpui_component::{
    ActiveTheme, Icon, IconName, Sizable, StyledExt, WindowExt,
    button::*,
    h_flex,
    input::{Input, InputEvent, InputState},
    v_flex,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRequest {
    pub id: String,
    pub title: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub additions: i32,
    pub deletions: i32,
    pub status: String,
}

pub struct CodeHubView {
    mrs: Vec<MergeRequest>,
    filter_start_date: String,
    filter_end_date: String,
    start_date_input: Entity<InputState>,
    end_date_input: Entity<InputState>,
    search_input: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
}

impl CodeHubView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let start_date_input = cx.new(|cx| InputState::new(window, cx).placeholder("YYYY-MM-DD"));
        let end_date_input = cx.new(|cx| InputState::new(window, cx).placeholder("YYYY-MM-DD"));
        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder("搜索 MR..."));

        // 添加示例数据
        let mrs = vec![
            MergeRequest {
                id: "1".to_string(),
                title: "添加用户认证功能".to_string(),
                author: "张三".to_string(),
                created_at: Utc::now(),
                additions: 120,
                deletions: 30,
                status: "merged".to_string(),
            },
            MergeRequest {
                id: "2".to_string(),
                title: "修复登录页面 bug".to_string(),
                author: "李四".to_string(),
                created_at: Utc::now(),
                additions: 45,
                deletions: 15,
                status: "merged".to_string(),
            },
            MergeRequest {
                id: "3".to_string(),
                title: "优化数据库查询性能".to_string(),
                author: "王五".to_string(),
                created_at: Utc::now(),
                additions: 80,
                deletions: 25,
                status: "open".to_string(),
            },
        ];

        let mut this = Self {
            mrs,
            filter_start_date: String::new(),
            filter_end_date: String::new(),
            start_date_input,
            end_date_input,
            search_input,
            _subscriptions: vec![],
        };

        // 添加订阅
        let subscription = cx.subscribe(&this.search_input, |_this, _, event, cx| {
            if matches!(event, InputEvent::Change) {
                cx.notify();
            }
        });
        this._subscriptions.push(subscription);

        let subscription = cx.subscribe(&this.start_date_input, |this, _, event, cx| {
            if matches!(event, InputEvent::Change) {
                this.filter_start_date = this.start_date_input.read(cx).value().to_string();
                cx.notify();
            }
        });
        this._subscriptions.push(subscription);

        let subscription = cx.subscribe(&this.end_date_input, |this, _, event, cx| {
            if matches!(event, InputEvent::Change) {
                this.filter_end_date = this.end_date_input.read(cx).value().to_string();
                cx.notify();
            }
        });
        this._subscriptions.push(subscription);

        this
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn calculate_stats(&self) -> (i32, i32, usize) {
        let total_additions: i32 = self.mrs.iter().map(|mr| mr.additions).sum();
        let total_deletions: i32 = self.mrs.iter().map(|mr| mr.deletions).sum();
        let total_count = self.mrs.len();
        (total_additions, total_deletions, total_count)
    }
}

impl Render for CodeHubView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_query = self.search_input.read(cx).value().trim().to_lowercase();

        // 根据搜索查询过滤MR
        let filtered_mrs: Vec<&MergeRequest> = if search_query.is_empty() {
            self.mrs.iter().collect()
        } else {
            self.mrs
                .iter()
                .filter(|mr| {
                    mr.title.to_lowercase().contains(&search_query)
                        || mr.author.to_lowercase().contains(&search_query)
                        || mr.id.contains(&search_query)
                })
                .collect()
        };

        let (additions, deletions, count) = self.calculate_stats();

        v_flex()
            .size_full()
            .p_4()
            .gap_4()
            .child(
                // 标题区
                h_flex().w_full().items_center().justify_between().child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(Icon::new(IconName::GitMerge).size_6())
                        .child(
                            div()
                                .text_2xl()
                                .font_bold()
                                .text_color(cx.theme().foreground)
                                .child("CodeHub - MR 管理"),
                        ),
                ),
            )
            .child(
                // 统计卡片区
                h_flex()
                    .w_full()
                    .gap_4()
                    .child(self.stat_card("MR 数量", &count.to_string(), cx))
                    .child(self.stat_card("新增行数", &format!("+{}", additions), cx))
                    .child(self.stat_card("删除行数", &format!("-{}", deletions), cx))
                    .child(self.stat_card("净变化", &format!("{:+}", additions - deletions), cx)),
            )
            .child(
                // 搜索区
                h_flex()
                    .w_full()
                    .gap_4()
                    .child(
                        div().flex_1().child(
                            Input::new(&self.search_input)
                                .cleanable(true)
                                .prefix(Icon::new(IconName::Search).size_4()),
                        ),
                    )
                    .child(
                        Button::new("open-file-btn")
                            .primary()
                            .label("打开文件")
                            .icon(Icon::new(IconName::FolderOpen))
                            .on_click(cx.listener(
                                |this: &mut CodeHubView, _event, window: &mut Window, cx| {
                                    this.handle_open_file(window, cx);
                                },
                            )),
                    ),
            )
            .child(
                // 筛选区
                h_flex()
                    .w_full()
                    .gap_4()
                    .items_end()
                    .child(
                        v_flex()
                            .gap_1()
                            .child(div().text_sm().child("开始日期"))
                            .child(Input::new(&self.start_date_input).w(px(200.0)).cleanable(true)),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .child(div().text_sm().child("结束日期"))
                            .child(Input::new(&self.end_date_input).w(px(200.0)).cleanable(true)),
                    )
                    .child(
                        Button::new("filter-btn")
                            .primary()
                            .label("查询")
                            .icon(Icon::new(IconName::Search)),
                    )
                    .child(
                        Button::new("add-mr-btn")
                            .primary()
                            .label("添加 MR")
                            .icon(Icon::new(IconName::Plus)),
                    ),
            )
            .child(
                // MR 列表
                v_flex()
                    .flex_1()
                    .w_full()
                    .gap_2()
                    .child(div().text_lg().font_semibold().child("MR 列表"))
                    .child(
                        div()
                            .w_full()
                            .h_full()
                            .border_1()
                            .border_color(cx.theme().border)
                            .rounded_md()
                            .p_4()
                            .child(if filtered_mrs.is_empty() {
                                v_flex()
                                    .size_full()
                                    .items_center()
                                    .justify_center()
                                    .gap_2()
                                    .child(
                                        Icon::new(IconName::Inbox)
                                            .size_16()
                                            .text_color(cx.theme().muted_foreground),
                                    )
                                    .child(div().text_color(cx.theme().muted_foreground).child(
                                        if search_query.is_empty() {
                                            "暂无 MR 数据"
                                        } else {
                                            "没有找到匹配的 MR"
                                        },
                                    ))
                            } else {
                                div().child(self.render_mr_list(filtered_mrs, cx))
                            }),
                    ),
            )
    }
}

impl CodeHubView {
    fn stat_card(&self, label: &str, value: &str, cx: &Context<Self>) -> impl IntoElement {
        let label = label.to_string();
        let value = value.to_string();
        v_flex()
            .flex_1()
            .p_4()
            .gap_2()
            .bg(cx.theme().background)
            .border_1()
            .border_color(cx.theme().border)
            .rounded_lg()
            .child(div().text_sm().text_color(cx.theme().muted_foreground).child(label))
            .child(div().text_2xl().font_bold().text_color(cx.theme().foreground).child(value))
    }

    fn render_mr_list(&self, mrs: Vec<&MergeRequest>, cx: &Context<Self>) -> impl IntoElement {
        v_flex().w_full().gap_2().children(mrs.into_iter().map(|mr| self.render_mr_item(mr, cx)))
    }

    fn render_mr_item(&self, mr: &MergeRequest, cx: &Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .p_3()
            .gap_3()
            .bg(cx.theme().background)
            .border_1()
            .border_color(cx.theme().border)
            .rounded_md()
            .hover(|this| this.bg(cx.theme().background))
            .child(
                v_flex()
                    .flex_1()
                    .gap_1()
                    .child(
                        div()
                            .font_semibold()
                            .text_color(cx.theme().foreground)
                            .child(mr.title.clone()),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(format!("#{}", mr.id))
                            .child("•")
                            .child(mr.author.clone())
                            .child("•")
                            .child(mr.created_at.format("%Y-%m-%d %H:%M").to_string()),
                    ),
            )
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0x22c55e))
                            .text_color(rgb(0x22c55e))
                            .text_sm()
                            .child(format!("+{}", mr.additions)),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0xef4444))
                            .text_color(rgb(0xef4444))
                            .text_sm()
                            .child(format!("-{}", mr.deletions)),
                    ),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Button::new(format!("edit-{}", mr.id))
                            .small()
                            .ghost()
                            .icon(Icon::new(IconName::PenTool)),
                    )
                    .child(
                        Button::new(format!("delete-{}", mr.id))
                            .small()
                            .ghost()
                            .icon(Icon::new(IconName::Trash)),
                    ),
            )
    }

    fn handle_open_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // 这里可以实现文件选择对话框
        // 由于gpui框架可能没有直接的文件对话框API
        // 这里我们使用一个简单的示例，创建一个测试文件

        let test_file_path = "test_mr_data.txt";

        // 创建测试文件
        match File::create(test_file_path) {
            Ok(mut file) => {
                writeln!(file, "MR 数据导出").unwrap();
                writeln!(file, "=============").unwrap();

                for mr in &self.mrs {
                    writeln!(file, "ID: {}", mr.id).unwrap();
                    writeln!(file, "标题: {}", mr.title).unwrap();
                    writeln!(file, "作者: {}", mr.author).unwrap();
                    writeln!(file, "创建时间: {}", mr.created_at.format("%Y-%m-%d %H:%M:%S"))
                        .unwrap();
                    writeln!(file, "新增行数: +{}", mr.additions).unwrap();
                    writeln!(file, "删除行数: -{}", mr.deletions).unwrap();
                    writeln!(file, "状态: {}", mr.status).unwrap();
                    writeln!(file, "--------------").unwrap();
                }

                // 显示成功通知
                struct FileSaved;
                let note = gpui_component::notification::Notification::new()
                    .message(format!("文件已保存到: {}", test_file_path))
                    .id::<FileSaved>();
                window.push_notification(note, cx);
            },
            Err(e) => {
                // 显示错误通知
                struct FileError;
                let note = gpui_component::notification::Notification::new()
                    .message(format!("保存文件失败: {}", e))
                    .id::<FileError>();
                window.push_notification(note, cx);
            },
        }
    }
}
