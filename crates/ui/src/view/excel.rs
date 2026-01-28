use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription,
    Window, div,
};
use gpui_component::{
    ActiveTheme, Icon, IconName, Sizable, StyledExt,
    button::*,
    h_flex,
    input::{Input, InputEvent, InputState},
    v_flex,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExcelData {
    pub id: String,
    pub name: String,
    pub file_type: String,
    pub size: String,
    pub created_at: String,
    pub description: String,
}

pub struct ExcelView {
    file_path: String,
    data_list: Vec<ExcelData>,
    search_input: Entity<InputState>,
    search_value: Option<String>,
    _subscriptions: Vec<Subscription>,
}

impl ExcelView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("搜索 Excel 文件..."));

        // 添加测试数据
        let data_list = vec![
            ExcelData {
                id: "EXCEL001".to_string(),
                name: "Q1销售数据".to_string(),
                file_type: "xlsx".to_string(),
                size: "2.5 MB".to_string(),
                created_at: "2025-01-20".to_string(),
                description: "第一季度销售统计".to_string(),
            },
            ExcelData {
                id: "EXCEL002".to_string(),
                name: "员工信息表".to_string(),
                file_type: "xlsx".to_string(),
                size: "1.2 MB".to_string(),
                created_at: "2025-01-22".to_string(),
                description: "公司员工基本信息".to_string(),
            },
            ExcelData {
                id: "EXCEL003".to_string(),
                name: "项目预算".to_string(),
                file_type: "xls".to_string(),
                size: "0.8 MB".to_string(),
                created_at: "2025-01-25".to_string(),
                description: "2025年项目预算表".to_string(),
            },
            ExcelData {
                id: "EXCEL004".to_string(),
                name: "库存清单".to_string(),
                file_type: "xlsx".to_string(),
                size: "3.1 MB".to_string(),
                created_at: "2025-01-26".to_string(),
                description: "仓库库存统计".to_string(),
            },
        ];

        let subscriptions = vec![cx.subscribe_in(&search_input, window, Self::on_input_event)];

        Self {
            file_path: String::new(),
            data_list,
            search_input,
            search_value: None,
            _subscriptions: subscriptions,
        }
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
                }
            },
            _ => {},
        };
    }
}

impl Render for ExcelView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_query = self.search_input.read(cx).value().trim().to_lowercase();

        // 根据搜索查询过滤数据
        let filtered_data: Vec<&ExcelData> = self
            .data_list
            .iter()
            .filter(|data: &&ExcelData| {
                search_query.is_empty()
                    || data.name.to_lowercase().contains(&search_query)
                    || data.description.to_lowercase().contains(&search_query)
                    || data.id.to_lowercase().contains(&search_query)
            })
            .collect();

        v_flex()
            .size_full()
            .p_4()
            .gap_4()
            .child(
                // 标题区
                h_flex().w_full().items_center().child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(Icon::new(IconName::FileSpreadsheet).size_6())
                        .child(
                            div()
                                .text_2xl()
                                .font_bold()
                                .text_color(cx.theme().foreground)
                                .child("Excel 工具"),
                        ),
                ),
            )
            .child(
                // 搜索区
                h_flex()
                    .w_full()
                    .gap_4()
                    .items_center()
                    .child(
                        div().flex_1().child(
                            Input::new(&self.search_input)
                                .cleanable(true)
                                .prefix(Icon::new(IconName::Search).size_4()),
                        ),
                    )
                    .child(
                        Button::new("read-excel-btn")
                            .primary()
                            .label("选择文件读取")
                            .icon(Icon::new(IconName::FolderOpen)),
                    )
                    .child(
                        Button::new("write-excel-btn")
                            .ghost()
                            .label("导出数据")
                            .icon(Icon::new(IconName::Download)),
                    ),
            )
            .child(
                // 数据列表
                v_flex()
                    .flex_1()
                    .w_full()
                    .gap_2()
                    .child(div().text_lg().font_semibold().child("Excel 文件列表"))
                    .child(
                        div()
                            .w_full()
                            .h_full()
                            .border_1()
                            .border_color(cx.theme().border)
                            .rounded_md()
                            .p_4()
                            .child(if filtered_data.is_empty() {
                                v_flex()
                                    .size_full()
                                    .items_center()
                                    .justify_center()
                                    .gap_2()
                                    .child(
                                        Icon::new(IconName::FileSpreadsheet)
                                            .size_16()
                                            .text_color(cx.theme().muted_foreground),
                                    )
                                    .child(div().text_color(cx.theme().muted_foreground).child(
                                        if search_query.is_empty() {
                                            "暂无 Excel 文件"
                                        } else {
                                            "没有找到匹配的文件"
                                        },
                                    ))
                            } else {
                                div().child(self.render_data_list(filtered_data, cx))
                            }),
                    ),
            )
    }
}
impl ExcelView {
    fn render_data_list(&self, data: Vec<&ExcelData>, cx: &Context<Self>) -> impl IntoElement {
        v_flex().w_full().gap_2().children(data.iter().map(|item| self.render_data_item(item, cx)))
    }

    fn render_data_item(&self, data: &ExcelData, cx: &Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .p_3()
            .gap_3()
            .bg(cx.theme().background)
            .border_1()
            .border_color(cx.theme().border)
            .rounded_md()
            .child(
                v_flex()
                    .flex_1()
                    .gap_1()
                    .child(
                        div()
                            .font_semibold()
                            .text_color(cx.theme().foreground)
                            .child(data.name.clone()),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(format!("#{}", data.id))
                            .child("•")
                            .child(data.description.clone())
                            .child("•")
                            .child(data.created_at.clone()),
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
                            .bg(gpui::rgb(0x3b82f6))
                            .text_sm()
                            .child(data.file_type.clone()),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(data.size.clone()),
                    ),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Button::new(format!("view-{}", data.id))
                            .small()
                            .ghost()
                            .icon(Icon::new(IconName::Eye)),
                    )
                    .child(
                        Button::new(format!("download-{}", data.id))
                            .small()
                            .ghost()
                            .icon(Icon::new(IconName::Download)),
                    ),
            )
    }
}
