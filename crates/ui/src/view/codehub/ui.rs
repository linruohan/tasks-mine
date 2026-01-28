use std::{fs::File, io::prelude::*};

use chrono::Days;
use gpui::{
    App, AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
    Styled, Subscription, Window, div, rgb,
};
use gpui_component::{
    ActiveTheme, Icon, IconName, Sizable, StyledExt, WindowExt,
    button::*,
    date_picker::{DatePicker, DatePickerEvent, DatePickerState},
    h_flex,
    input::{Input, InputEvent, InputState},
    v_flex,
};
use tools::MergeRequest;

pub struct CodeHubView {
    mrs: Vec<MergeRequest>,
    date_range_picker: Entity<DatePickerState>,
    date_picker_value: Option<String>,
    search_input: Entity<InputState>,
    search_value: Option<String>,
    _subscriptions: Vec<Subscription>,
}

impl CodeHubView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let now = chrono::Local::now().naive_local().date();
        let date_range_picker = cx.new(|cx| {
            let mut picker = DatePickerState::new(window, cx);
            picker.set_date((now, now.checked_add_days(Days::new(4)).unwrap()), window, cx);
            picker
        });
        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder("搜索 MR..."));

        // 添加示例数据
        let mrs = vec![
            MergeRequest {
                id: "1".to_string(),
                title: "添加用户认证功能".to_string(),
                author: "张三".to_string(),
                created_at: "2026-01-21 10:00:00".to_string(),
                add_lines: 120,
                del_lines: 30,
                status: "merged".to_string(),
            },
            MergeRequest {
                id: "2".to_string(),
                title: "修复登录页面 bug".to_string(),
                author: "李四".to_string(),
                created_at: "2026-01-25 10:00:00".to_string(),
                add_lines: 45,
                del_lines: 15,
                status: "merged".to_string(),
            },
            MergeRequest {
                id: "3".to_string(),
                title: "优化数据库查询性能".to_string(),
                author: "王五".to_string(),
                created_at: "2026-01-26 10:00:00".to_string(),
                add_lines: 80,
                del_lines: 25,
                status: "open".to_string(),
            },
        ];
        // 添加订阅
        let subscriptions = vec![
            cx.subscribe_in(&search_input, window, Self::on_input_event),
            cx.subscribe(&date_range_picker, move |this, _, ev, _| match ev {
                DatePickerEvent::Change(date) => {
                    // Some("2026-01-21 - 2026-01-23")
                    this.date_picker_value = date.format("%Y-%m-%d").map(|s| s.to_string());
                    if let Some(date_range) = &this.date_picker_value {
                        let dates: Vec<&str> = date_range.split(" - ").collect();
                        if dates.len() == 2 {
                            let start_date =
                                chrono::NaiveDate::parse_from_str(dates[0], "%Y-%m-%d")
                                    .unwrap_or(now);
                            let end_date = chrono::NaiveDate::parse_from_str(dates[1], "%Y-%m-%d")
                                .unwrap_or(now);
                            // 过滤 MR 列表
                            this.mrs.retain(|mr| {
                                match chrono::NaiveDateTime::parse_from_str(
                                    &mr.created_at,
                                    "%Y-%m-%d %H:%M:%S",
                                ) {
                                    Ok(mr_datetime) => {
                                        let mr_date = mr_datetime.date();
                                        mr_date >= start_date && mr_date <= end_date
                                    },
                                    Err(_) => true,
                                }
                            });
                        }
                    }
                },
            }),
        ];

        Self {
            mrs,
            date_range_picker,
            date_picker_value: None,
            search_input,
            search_value: None,
            _subscriptions: subscriptions,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn on_input_event(
        &mut self,
        state: &Entity<InputState>,
        event: &InputEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Change => {
                let text = state.read(cx).value();
                if state == &self.search_input {
                    self.search_value = Some(text.into());
                } else {
                    println!("Change: {}", text)
                }
            },
            InputEvent::PressEnter { secondary } => println!("PressEnter secondary: {}", secondary),
            InputEvent::Focus => println!("Focus"),
            InputEvent::Blur => println!("Blur"),
        };
    }

    fn calculate_stats(&self) -> (i32, i32, usize) {
        let total_additions: i32 = self.mrs.iter().map(|mr| mr.add_lines).sum();
        let total_deletions: i32 = self.mrs.iter().map(|mr| mr.del_lines).sum();
        let total_count = self.mrs.len();
        (total_additions, total_deletions, total_count)
    }
}

impl Render for CodeHubView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_query = self.search_input.read(cx).value().trim().to_lowercase();

        // 根据搜索查询过滤MR（日期过滤已在 on_date_picker_change 中完成）
        let filtered_mrs: Vec<&MergeRequest> = self
            .mrs
            .iter()
            .filter(|mr: &&MergeRequest| {
                // 搜索过滤
                search_query.is_empty()
                    || mr.title.to_lowercase().contains(&search_query)
                    || mr.author.to_lowercase().contains(&search_query)
                    || mr.id.contains(&search_query)
            })
            .collect();

        // 计算过滤后的统计数据
        let total_additions: i32 = filtered_mrs.iter().map(|mr| mr.add_lines).sum();
        let total_deletions: i32 = filtered_mrs.iter().map(|mr| mr.del_lines).sum();
        let total_count = filtered_mrs.len();

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
                    .child(self.stat_card("MR 数量", &total_count.to_string(), cx))
                    .child(self.stat_card("新增行数", &format!("+{}", total_additions), cx))
                    .child(self.stat_card("删除行数", &format!("-{}", total_deletions), cx))
                    .child(self.stat_card(
                        "净变化",
                        &format!("{:+}", total_additions + total_deletions),
                        cx,
                    )),
            )
            .child(
                // 搜索区
                v_flex()
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
                        // 筛选区
                        h_flex()
                            .w_full()
                            .gap_4()
                            .items_end()
                            .child(v_flex().gap_1().child(DatePicker::new(&self.date_range_picker)))
                            .child(
                                Button::new("filter-btn")
                                    .primary()
                                    .label("查询")
                                    .icon(Icon::new(IconName::Search))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        // 使用 update 方法来访问 DatePickerState 的可变引用
                                        this.date_range_picker.update(cx, |_picker_state, _cx| {
                                            // 在这里可以访问 picker_state 的方法
                                            println!("Picker state in update: [DatePickerState]");
                                            // 尝试调用可能存在的方法来获取日期范围
                                        });
                                    })),
                            ),
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
                            .child(mr.created_at.clone()),
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
                            .text_sm()
                            .child(format!("+{}", mr.add_lines)),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0xef4444))
                            .text_sm()
                            .child(format!("-{}", mr.del_lines)),
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
                    writeln!(file, "创建时间: {}", mr.created_at).unwrap();
                    writeln!(file, "新增行数: +{}", mr.add_lines).unwrap();
                    writeln!(file, "删除行数: -{}", mr.del_lines).unwrap();
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
