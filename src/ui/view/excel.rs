use gpui::{div, Context, IntoElement, ParentElement, Render, Styled, Window};
use gpui_component::{button::*, h_flex, v_flex, ActiveTheme, Icon, IconName, StyledExt};

pub struct ExcelView {
    file_path: String,
}

impl ExcelView {
    pub fn new() -> Self {
        Self { file_path: String::new() }
    }
}

impl Render for ExcelView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                // 功能卡片区
                h_flex()
                    .w_full()
                    .gap_4()
                    .child(
                        v_flex()
                            .flex_1()
                            .p_6()
                            .gap_4()
                            .bg(cx.theme().background)
                            .border_1()
                            .border_color(cx.theme().border)
                            .rounded_lg()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .child(Icon::new(IconName::FileDown).size_5())
                                    .child(div().text_lg().font_semibold().child("读取 Excel")),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("支持读取 .xlsx 和 .xls 格式文件"),
                            )
                            .child(
                                Button::new("read-excel-btn")
                                    .primary()
                                    .label("选择文件读取")
                                    .icon(Icon::new(IconName::FolderOpen)),
                            ),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .p_6()
                            .gap_4()
                            .bg(cx.theme().background)
                            .border_1()
                            .border_color(cx.theme().border)
                            .rounded_lg()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .child(Icon::new(IconName::FileUp).size_5())
                                    .child(div().text_lg().font_semibold().child("写入 Excel")),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("导出数据到 .xlsx 格式文件"),
                            )
                            .child(
                                Button::new("write-excel-btn")
                                    .primary()
                                    .label("导出数据")
                                    .icon(Icon::new(IconName::Download)),
                            ),
                    ),
            )
            .child(
                // 数据预览区
                v_flex()
                    .flex_1()
                    .w_full()
                    .gap_2()
                    .child(
                        h_flex()
                            .w_full()
                            .items_center()
                            .justify_between()
                            .child(div().text_lg().font_semibold().child("数据预览"))
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Button::new("export-mr-btn").ghost().label("导出 MR 数据"),
                                    )
                                    .child(
                                        Button::new("export-dts-btn")
                                            .ghost()
                                            .label("导出 DTS 数据"),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .w_full()
                            .h_full()
                            .border_1()
                            .border_color(cx.theme().border)
                            .rounded_md()
                            .p_4()
                            .child(
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
                                    .child(
                                        div()
                                            .text_color(cx.theme().muted_foreground)
                                            .child("请选择 Excel 文件进行读取或导出数据"),
                                    ),
                            ),
                    ),
            )
            .child(
                // 快速操作区
                v_flex()
                    .w_full()
                    .gap_2()
                    .child(div().text_lg().font_semibold().child("快速操作"))
                    .child(
                        h_flex()
                            .w_full()
                            .gap_2()
                            .child(
                                Button::new("template-mr")
                                    .ghost()
                                    .label("下载 MR 模板")
                                    .icon(Icon::new(IconName::Download)),
                            )
                            .child(
                                Button::new("template-dts")
                                    .ghost()
                                    .label("下载 DTS 模板")
                                    .icon(Icon::new(IconName::Download)),
                            )
                            .child(
                                Button::new("batch-import")
                                    .ghost()
                                    .label("批量导入")
                                    .icon(Icon::new(IconName::Upload)),
                            ),
                    ),
            )
    }
}
