// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! A button for toggling the navigation side panel.

use crate::{widget, Element};
use apply::Apply;
use derive_setters::Setters;
use iced::Length;

#[derive(Setters)]
pub struct NavBarToggle<Message> {
    active: bool,
    #[setters(strip_option)]
    on_toggle: Option<Message>,
    style: crate::theme::Button,
    selected: bool,
}

#[must_use]
pub fn nav_bar_toggle<Message>() -> NavBarToggle<Message> {
    NavBarToggle {
        active: false,
        on_toggle: None,
        style: crate::theme::Button::Text,
        selected: false,
    }
}

impl<'a, Message: 'static + Clone> From<NavBarToggle<Message>> for Element<'a, Message> {
    fn from(nav_bar_toggle: NavBarToggle<Message>) -> Self {
        let icon = if nav_bar_toggle.active {
            widget::icon::from_svg_bytes(
                &include_bytes!("../../res/icons/close-menu-symbolic.svg")[..],
            )
            .symbolic(true)
        } else {
            widget::icon::from_svg_bytes(
                &include_bytes!("../../res/icons/open-menu-symbolic.svg")[..],
            )
            .symbolic(true)
        };

        widget::button::icon(icon)
            .padding([8, 16])
            .on_press_maybe(nav_bar_toggle.on_toggle)
            .selected(nav_bar_toggle.selected)
            .style(nav_bar_toggle.style)
            .into()
    }
}
