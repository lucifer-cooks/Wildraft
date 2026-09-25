use super::{FILL_FRAC_ONE, FILL_FRAC_TWO, Imgs, LoginInfo, Message, Showing};
use crate::ui::{
    fonts::IcedFonts as Fonts,
    ice::{
        Element,
        component::neat_button,
        style,
        widget::{
            AspectRatioContainer, BackgroundContainer, Image, Padding,
            compound_graphic::{CompoundGraphic, Graphic},
        },
    },
};

use i18n::{LanguageMetadata, Localization};
use iced::{
    Align, Button, Column, Container, Length, Row, Scrollable, Space, Text, TextInput, button,
    scrollable, text_input,
};
use vek::*;

const INPUT_WIDTH: u16 = 230;
const INPUT_TEXT_SIZE: u16 = 20;

/// Login screen for the main menu
#[derive(Default)]
pub struct Screen {
    quit_button: button::State,
    // settings_button: button::State,
    servers_button: button::State,
    credits_button: button::State,
    language_select_button: button::State,

    error_okay_button: button::State,

    pub banner: LoginBanner,
    language_selection: LanguageSelectBanner,
}

impl Screen {
    pub(super) fn view(
        &mut self,
        fonts: &Fonts,
        imgs: &Imgs,
        server_field_locked: bool,
        login_info: &LoginInfo,
        error: Option<&str>,
        i18n: &Localization,
        show: &Showing,
        selected_language_index: Option<usize>,
        language_metadatas: &[LanguageMetadata],
        button_style: style::button::Style,
    ) -> Element<'_, Message> {
        let mut buttons = Vec::new();

        // PLAY DEMO button (primary action — stronger visual weight)
        buttons.push(
            Container::new(neat_button(
                &mut self.servers_button,
                i18n.get_msg("wildraft-play_demo"),
                FILL_FRAC_ONE,
                button_style,
                Some(Message::Singleplayer),
            ))
            .width(Length::Units(260))
            .padding(4)
            .into(),
        );

        // CREDITS button (secondary — restrained)
        buttons.push(
            Container::new(neat_button(
                &mut self.credits_button,
                i18n.get_msg("wildraft-credits"),
                FILL_FRAC_ONE,
                button_style,
                Some(Message::ShowCredits),
            ))
            .width(Length::Units(260))
            .padding(4)
            .into(),
        );

        // QUIT button (secondary — restrained)
        buttons.push(
            Container::new(neat_button(
                &mut self.quit_button,
                i18n.get_msg("wildraft-quit"),
                FILL_FRAC_ONE,
                button_style,
                Some(Message::Quit),
            ))
            .width(Length::Units(260))
            .padding(4)
            .into(),
        );

        let buttons = Container::new(
            Column::with_children(buttons)
                .width(Length::Fill)
                .max_width(280)
                .spacing(8),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_y(Align::Center);

        // WILDRAFT branding — logo directly above buttons, left-aligned
        let logo_section = Column::with_children(vec![
            // Logo at top — real image, enlarged, aspect preserved
            Image::new(imgs.logo)
                .width(Length::Units(380)) // Increased size for visibility
                .fix_aspect_ratio()
                .into(),
            Space::new(Length::Fill, Length::Units(4)).into(),
        ])
        .width(Length::Fill)
        .max_width(460);

        // Logo moved down to sit just above Play Demo button
        let left_column = Column::with_children(vec![
            Space::new(Length::Fill, Length::Fill).into(),
            logo_section.into(),
            buttons.into(),
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(30)
        .into();

        let central_content: Element<'_, Message> = if let Some(error) = error {
            Container::new(
                Column::with_children(vec![
                    Container::new(Text::new(error)).height(Length::Fill).into(),
                    Container::new(neat_button(
                        &mut self.error_okay_button,
                        i18n.get_msg("wildraft-ok"),
                        FILL_FRAC_ONE,
                        button_style,
                        Some(Message::CloseError),
                    ))
                    .width(Length::Units(200))
                    .height(Length::Units(40))
                    .center_x()
                    .into(),
                ])
                .height(Length::Fill)
                .width(Length::Fill),
            )
            .style(
                style::container::Style::color_with_double_cornerless_border(
                    (15, 25, 35, 255).into(),
                    (8, 15, 25, 255).into(),
                    (30, 45, 55, 255).into(),
                ),
            )
            .width(Length::Units(420))
            .height(Length::Units(200))
            .padding(25)
            .into()
        } else {
            // WILDRAFT demo landing: clean menu, no login banner visible
            Container::new(Space::new(Length::Fill, Length::Fill)).into()
        };

        let central_column = Container::new(central_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        // Right column: cleared — no visible text
        let right_column = Container::new(Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y();

        Row::with_children(vec![
            left_column,
            central_column.into(),
            right_column.into(),
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(20)
        .into()
    }
}

#[derive(Default)]
pub struct LanguageSelectBanner {
    okay_button: button::State,
    language_buttons: Vec<button::State>,

    selection_list: scrollable::State,
}

impl LanguageSelectBanner {
    fn view(
        &mut self,
        fonts: &Fonts,
        imgs: &Imgs,
        i18n: &Localization,
        language_metadatas: &[LanguageMetadata],
        selected_language_index: Option<usize>,
        button_style: style::button::Style,
    ) -> Element<'_, Message> {
        // Reset button states if languages were added / removed
        if self.language_buttons.len() != language_metadatas.len() {
            self.language_buttons = vec![Default::default(); language_metadatas.len()];
        }

        let title = Text::new(i18n.get_msg("main-login-select_language"))
            .size(fonts.cyri.scale(35))
            .horizontal_alignment(iced::HorizontalAlignment::Center);

        let mut list = Scrollable::new(&mut self.selection_list)
            .spacing(8)
            .height(Length::Fill)
            .align_items(Align::Start);

        let list_items = self
            .language_buttons
            .iter_mut()
            .zip(language_metadatas)
            .enumerate()
            .map(|(i, (state, lang))| {
                let color = if Some(i) == selected_language_index {
                    (97, 255, 18)
                } else {
                    (97, 97, 25)
                };
                let button = Button::new(
                    state,
                    Row::with_children(vec![
                        Space::new(Length::FillPortion(5), Length::Units(0)).into(),
                        Text::new(lang.language_name.clone())
                            .width(Length::FillPortion(95))
                            .font(fonts.universal.id)
                            .size(fonts.universal.scale(25))
                            .vertical_alignment(iced::VerticalAlignment::Center)
                            .into(),
                    ]),
                )
                .style(
                    style::button::Style::new(imgs.selection)
                        .hover_image(imgs.selection_hover)
                        .press_image(imgs.selection_press)
                        .image_color(Rgba::new(color.0, color.1, color.2, 192)),
                )
                .min_height(56)
                .on_press(Message::LanguageChanged(i));
                Row::with_children(vec![
                    Space::new(Length::FillPortion(3), Length::Units(0)).into(),
                    button.width(Length::FillPortion(92)).into(),
                    Space::new(Length::FillPortion(5), Length::Units(0)).into(),
                ])
            });

        for item in list_items {
            list = list.push(item);
        }

        let okay_button = Container::new(neat_button(
            &mut self.okay_button,
            i18n.get_msg("common-okay"),
            FILL_FRAC_TWO,
            button_style,
            Some(Message::OpenLanguageMenu),
        ))
        .center_x()
        .max_width(200);

        let content = Column::with_children(vec![title.into(), list.into(), okay_button.into()])
            .spacing(8)
            .width(Length::Fill)
            .height(Length::FillPortion(38))
            .align_items(Align::Center);

        let selection_menu = BackgroundContainer::new(
            CompoundGraphic::from_graphics(vec![
                Graphic::image(imgs.banner_top, [138, 17], [0, 0]),
                Graphic::rect(Rgba::new(0, 0, 0, 230), [130, 165], [4, 17]),
                // TODO: use non image gradient
                Graphic::gradient(Rgba::new(0, 0, 0, 230), Rgba::zero(), [130, 50], [4, 182]),
            ])
            .fix_aspect_ratio()
            .height(Length::Fill),
            content,
        )
        .padding(Padding::new().horizontal(5).top(15).bottom(50))
        .max_width(350);

        selection_menu.into()
    }
}

#[derive(Default)]
pub struct LoginBanner {
    pub username: text_input::State,
    pub password: text_input::State,
    pub server: text_input::State,

    multiplayer_button: button::State,
    #[cfg(feature = "singleplayer")]
    singleplayer_button: button::State,

    unlock_server_field_button: button::State,
}

impl LoginBanner {
    fn view(
        &mut self,
        fonts: &Fonts,
        imgs: &Imgs,
        server_field_locked: bool,
        login_info: &LoginInfo,
        i18n: &Localization,
        button_style: style::button::Style,
    ) -> Element<'_, Message> {
        let input_text_size = fonts.cyri.scale(INPUT_TEXT_SIZE);

        let server_field: Element<Message> = if server_field_locked {
            let unlock_style = style::button::Style::new(imgs.unlock)
                .hover_image(imgs.unlock_hover)
                .press_image(imgs.unlock_press);

            let unlock_button = Button::new(
                &mut self.unlock_server_field_button,
                Space::new(Length::Fill, Length::Fill),
            )
            .style(unlock_style)
            .width(Length::Fill)
            .height(Length::Fill)
            .on_press(Message::UnlockServerField);

            let container = AspectRatioContainer::new(unlock_button);
            let container = match unlock_style.active().0 {
                Some((img, _)) => container.ratio_of_image(img),
                None => container,
            };

            Row::with_children(vec![
                Text::new(&login_info.server)
                    .size(input_text_size)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .into(),
                container.into(),
            ])
            .align_items(Align::Center)
            .height(Length::Fill)
            .into()
        } else {
            TextInput::new(
                &mut self.server,
                &i18n.get_msg("main-server"),
                &login_info.server,
                Message::Server,
            )
            .size(input_text_size)
            .on_submit(Message::Multiplayer)
            .into()
        };

        let banner_content = Column::with_children(vec![
            Column::with_children(vec![
                BackgroundContainer::new(
                    Image::new(imgs.input_bg)
                        .width(Length::Units(INPUT_WIDTH))
                        .fix_aspect_ratio(),
                    TextInput::new(
                        &mut self.username,
                        &i18n.get_msg("main-username"),
                        &login_info.username,
                        Message::Username,
                    )
                    .size(input_text_size)
                    .on_submit(Message::FocusPassword),
                )
                .padding(Padding::new().horizontal(7).top(5))
                .into(),
                BackgroundContainer::new(
                    Image::new(imgs.input_bg)
                        .width(Length::Units(INPUT_WIDTH))
                        .fix_aspect_ratio(),
                    TextInput::new(
                        &mut self.password,
                        &i18n.get_msg("main-password"),
                        &login_info.password,
                        Message::Password,
                    )
                    .size(input_text_size)
                    .password()
                    .on_submit(Message::Multiplayer),
                )
                .padding(Padding::new().horizontal(7).top(5))
                .into(),
                BackgroundContainer::new(
                    Image::new(imgs.input_bg)
                        .width(Length::Units(INPUT_WIDTH))
                        .fix_aspect_ratio(),
                    server_field,
                )
                .padding(Padding::new().horizontal(7).vertical(5))
                .into(),
            ])
            .spacing(5)
            .into(),
            Column::with_children(vec![
                neat_button(
                    &mut self.multiplayer_button,
                    i18n.get_msg("common-multiplayer"),
                    FILL_FRAC_ONE,
                    button_style,
                    Some(Message::Multiplayer),
                ),
                #[cfg(feature = "singleplayer")]
                neat_button(
                    &mut self.singleplayer_button,
                    i18n.get_msg("common-singleplayer"),
                    FILL_FRAC_ONE,
                    button_style,
                    Some(Message::Singleplayer),
                ),
            ])
            .max_width(170)
            .height(Length::Units(200))
            .spacing(8)
            .into(),
        ])
        .width(Length::Fill)
        .align_items(Align::Center);

        Container::new(banner_content)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}
// Steam asset check result: NO existing Steam logo in checkout
// (assets/voxygen/element/ lacks it). Right-side promo laid out transparent;
// Steam slot reserved — asset required before insertion.

// Co-Authored-By: Claude Code <noreply@anthropic.com>
