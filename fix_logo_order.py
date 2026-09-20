#!/usr/bin/env python3
with open("voxygen/src/menu/main/ui/login.rs") as f:
    content = f.read()

old = """        // WILDRAFT branding — real logo inserted using imgs.logo
        // (assets/voxygen/element/logo.png)
        let info_window = BackgroundContainer::new(
            CompoundGraphic::from_graphics(vec![
                // Subtle atmospheric overlay — preserves background readability
                Graphic::rect(Rgba::new(5, 7, 10, 120), [460, 300], [0, 0]),
                Graphic::rect(Rgba::new(20, 25, 35, 90), [460, 4], [0, 10]), // top hairline
                // Logo slot — uses the real logo.png provided (not v_logo)
                Graphic::image(imgs.logo, [300, 130], [150, 30])
                    .color(Rgba::new(240, 240, 235, 255)),
            ])
            .fix_aspect_ratio()
            .height(Length::Shrink),
            Column::with_children(vec![
                // Logo only — no old text title; enlarged image replaces title entirely
                Space::new(Length::Fill, Length::Units(20)).into(),
                // Subtitle — clearly subordinate to the logo
                Text::new(subtitle)"""

new = """        // WILDRAFT branding — logo above subtitle/demo, left-aligned
        let info_window = BackgroundContainer::new(
            CompoundGraphic::from_graphics(vec![
                // Subtle atmospheric overlay — preserves background readability
                Graphic::rect(Rgba::new(5, 7, 10, 120), [460, 300], [0, 0]),
                Graphic::rect(Rgba::new(20, 25, 35, 90), [460, 4], [0, 10]), // top hairline
            ])
            .fix_aspect_ratio()
            .height(Length::Shrink),
            Column::with_children(vec![
                // Logo at top — real image, enlarged, aspect preserved
                Image::new(imgs.logo)
                    .width(Length::Units(280))
                    .fix_aspect_ratio()
                    .into(),
                Space::new(Length::Fill, Length::Units(12)).into(),
                // Subtitle — clearly subordinate to the logo
                Text::new(subtitle)"""

content = content.replace(old, new)
with open("voxygen/src/menu/main/ui/login.rs", "w") as f:
    f.write(content)
print("done")
