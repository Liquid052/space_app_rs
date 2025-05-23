use bevy_inspector_egui::egui::*;
use bevy_inspector_egui::egui::style::{Interaction, ScrollStyle, Selection, Spacing, TextCursorStyle, Widgets, WidgetVisuals};

pub fn style() -> Style {
    Style {
        // override the text styles here:
        // override_text_style: Option<TextStyle>

        // override the font id here:
        // override_font_id: Option<FontId>

        // set your text styles here:
        // text_styles: BTreeMap<TextStyle, FontId>,

        // set your drag value text style:
        // drag_value_text_style: TextStyle,
        spacing: Spacing {
            item_spacing: Vec2 { x: 8.0, y: 3.0 },
            window_margin: Margin {
                left: 6.0,
                right: 6.0,
                top: 6.0,
                bottom: 6.0,
            },
            button_padding: Vec2 { x: 4.0, y: 1.0 },
            menu_margin: Margin {
                left: 6.0,
                right: 6.0,
                top: 6.0,
                bottom: 6.0,
            },
            indent: 18.0,
            interact_size: Vec2 { x: 40.0, y: 18.0 },
            slider_width: 100.0,
            combo_width: 100.0,
            text_edit_width: 280.0,
            icon_width: 14.0,
            icon_width_inner: 8.0,
            icon_spacing: 4.0,
            tooltip_width: 500.0,
            indent_ends_with_horizontal_line: false,
            combo_height: 200.0,
            scroll: ScrollStyle {
                bar_width: 10.0,
                handle_min_length: 12.0,
                bar_inner_margin: 4.0,
                bar_outer_margin: 0.0,
                ..Default::default()
            },
            ..Default::default()
        },
        interaction: Interaction {
            resize_grab_radius_side: 5.0,
            resize_grab_radius_corner: 10.0,
            show_tooltips_only_when_still: true,
            ..Default::default()
        },
        visuals: Visuals {
            dark_mode: true,
            override_text_color: None,
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(36, 36, 36, 255),
                    weak_bg_fill: Color32::from_rgba_premultiplied(27, 27, 27, 255),
                    bg_stroke: Stroke {
                        width: 1.0,
                        color: Color32::from_rgba_premultiplied(70, 70, 70, 255),
                    },
                    rounding: Rounding {
                        nw: 2.0,
                        ne: 2.0,
                        sw: 2.0,
                        se: 2.0,
                    },
                    fg_stroke: Stroke {
                        width: 1.0,
                        color: Color32::from_rgba_premultiplied(142, 142, 142, 255),
                    },
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(60, 60, 60, 255),
                    weak_bg_fill: Color32::from_rgba_premultiplied(60, 60, 60, 255),
                    bg_stroke: Stroke {
                        width: 0.0,
                        color: Color32::from_rgba_premultiplied(0, 0, 0, 0),
                    },
                    rounding: Rounding {
                        nw: 2.0,
                        ne: 2.0,
                        sw: 2.0,
                        se: 2.0,
                    },
                    fg_stroke: Stroke {
                        width: 1.0,
                        color: Color32::from_rgba_premultiplied(180, 180, 180, 255),
                    },
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(70, 70, 70, 255),
                    weak_bg_fill: Color32::from_rgba_premultiplied(70, 70, 70, 255),
                    bg_stroke: Stroke {
                        width: 1.0,
                        color: Color32::from_rgba_premultiplied(150, 150, 150, 255),
                    },
                    rounding: Rounding {
                        nw: 3.0,
                        ne: 3.0,
                        sw: 3.0,
                        se: 3.0,
                    },
                    fg_stroke: Stroke {
                        width: 1.5,
                        color: Color32::from_rgba_premultiplied(240, 240, 240, 255),
                    },
                    expansion: 1.0,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(55, 55, 55, 255),
                    weak_bg_fill: Color32::from_rgba_premultiplied(55, 55, 55, 255),
                    bg_stroke: Stroke {
                        width: 1.0,
                        color: Color32::from_rgba_premultiplied(255, 255, 255, 255),
                    },
                    rounding: Rounding {
                        nw: 2.0,
                        ne: 2.0,
                        sw: 2.0,
                        se: 2.0,
                    },
                    fg_stroke: Stroke {
                        width: 2.0,
                        color: Color32::from_rgba_premultiplied(255, 255, 255, 255),
                    },
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(27, 27, 27, 255),
                    weak_bg_fill: Color32::from_rgba_premultiplied(45, 45, 45, 255),
                    bg_stroke: Stroke {
                        width: 1.0,
                        color: Color32::from_rgba_premultiplied(60, 60, 60, 255),
                    },
                    rounding: Rounding {
                        nw: 2.0,
                        ne: 2.0,
                        sw: 2.0,
                        se: 2.0,
                    },
                    fg_stroke: Stroke {
                        width: 1.0,
                        color: Color32::from_rgba_premultiplied(210, 210, 210, 255),
                    },
                    expansion: 0.0,
                },
            },
            selection: Selection {
                bg_fill: Color32::from_rgba_premultiplied(0, 100, 87, 255),
                stroke: Stroke {
                    width: 1.0,
                    color: Color32::from_rgba_premultiplied(220, 223, 227, 255),
                },
            },
            hyperlink_color: Color32::from_rgba_premultiplied(76, 158, 211, 255),
            faint_bg_color: Color32::from_rgba_premultiplied(5, 5, 5, 0),
            extreme_bg_color: Color32::from_rgba_premultiplied(30, 30, 30, 255),
            code_bg_color: Color32::from_rgba_premultiplied(64, 64, 64, 255),
            warn_fg_color: Color32::from_rgba_premultiplied(255, 143, 0, 255),
            error_fg_color: Color32::from_rgba_premultiplied(255, 0, 0, 255),
            window_rounding: Rounding {
                nw: 2.5999999046325684,
                ne: 2.5999999046325684,
                sw: 2.5999999046325684,
                se: 2.5999999046325684,
            },
            window_shadow: Shadow {
                spread: 7.099999904632568,
                color: Color32::from_rgba_premultiplied(0, 0, 0, 96),
                blur: 15.0,
                offset: Vec2 { x: 10.0, y: 20.0 },
            },
            window_fill: Color32::from_rgba_premultiplied(12, 12, 12, 255),
            window_stroke: Stroke {
                width: 1.399999976158142,
                color: Color32::from_rgba_premultiplied(76, 76, 76, 255),
            },
            menu_rounding: Rounding {
                nw: 6.0,
                ne: 6.0,
                sw: 6.0,
                se: 6.0,
            },
            panel_fill: Color32::from_rgba_premultiplied(23, 23, 23, 255),
            popup_shadow: Shadow {
                spread: 0.0,
                color: Color32::from_rgba_premultiplied(0, 0, 0, 96),
                blur: 8.0,
                offset: Vec2 { x: 6.0, y: 10.0 },
            },
            resize_corner_size: 12.0,
            text_cursor: TextCursorStyle {
                stroke: Stroke {
                    width: 2.0,
                    color: Color32::from_rgba_premultiplied(192, 222, 255, 255),
                },
                preview: false,
                ..Default::default()
            },
            clip_rect_margin: 3.0,
            button_frame: true,
            collapsing_header_frame: false,
            indent_has_left_vline: true,
            striped: false,
            slider_trailing_fill: false,
            ..Default::default()
        },
        animation_time: 0.0833333358168602,
        explanation_tooltips: false,
        ..Default::default()
    }
}