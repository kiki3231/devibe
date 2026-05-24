use ratatui::style::Color;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
    Gruvbox,
    Nord,
    Catppuccin,
    Monokai,
    OneDark,
}

impl Theme {
    pub fn all() -> &'static [Theme] {
        &[
            Theme::Dark,
            Theme::Light,
            Theme::Gruvbox,
            Theme::Nord,
            Theme::Catppuccin,
            Theme::Monokai,
            Theme::OneDark,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            Theme::Gruvbox => "Gruvbox",
            Theme::Nord => "Nord",
            Theme::Catppuccin => "Catppuccin",
            Theme::Monokai => "Monokai",
            Theme::OneDark => "OneDark",
        }
    }

    pub fn next(self) -> Self {
        let all = Self::all();
        let idx = all.iter().position(|t| *t == self).unwrap_or(0);
        all[(idx + 1) % all.len()]
    }

    pub fn prev(self) -> Self {
        let all = Self::all();
        let idx = all.iter().position(|t| *t == self).unwrap_or(0);
        all[(idx + all.len() - 1) % all.len()]
    }

    pub fn bg(&self) -> Color {
        match self {
            Theme::Dark => Color::Rgb(18, 18, 18),
            Theme::Light => Color::Rgb(250, 250, 250),
            Theme::Gruvbox => Color::Rgb(40, 40, 40),
            Theme::Nord => Color::Rgb(46, 52, 64),
            Theme::Catppuccin => Color::Rgb(30, 30, 46),
            Theme::Monokai => Color::Rgb(39, 40, 34),
            Theme::OneDark => Color::Rgb(40, 44, 52),
        }
    }

    pub fn surface(&self) -> Color {
        match self {
            Theme::Dark => Color::Rgb(30, 30, 30),
            Theme::Light => Color::Rgb(240, 240, 240),
            Theme::Gruvbox => Color::Rgb(50, 48, 47),
            Theme::Nord => Color::Rgb(59, 66, 82),
            Theme::Catppuccin => Color::Rgb(49, 50, 68),
            Theme::Monokai => Color::Rgb(46, 48, 42),
            Theme::OneDark => Color::Rgb(33, 37, 43),
        }
    }

    pub fn text(&self) -> Color {
        match self {
            Theme::Dark | Theme::Gruvbox | Theme::Nord | Theme::Catppuccin | Theme::Monokai | Theme::OneDark => Color::Rgb(220, 220, 220),
            Theme::Light => Color::Rgb(40, 40, 40),
        }
    }

    pub fn text_dim(&self) -> Color {
        match self {
            Theme::Dark => Color::Rgb(120, 120, 120),
            Theme::Light => Color::Rgb(140, 140, 140),
            Theme::Gruvbox => Color::Rgb(146, 131, 116),
            Theme::Nord => Color::Rgb(129, 161, 193),
            Theme::Catppuccin => Color::Rgb(108, 112, 134),
            Theme::Monokai => Color::Rgb(117, 113, 94),
            Theme::OneDark => Color::Rgb(92, 99, 112),
        }
    }

    pub fn accent(&self) -> Color {
        match self {
            Theme::Dark => Color::Rgb(0, 220, 200),
            Theme::Light => Color::Rgb(0, 140, 180),
            Theme::Gruvbox => Color::Rgb(184, 187, 38),
            Theme::Nord => Color::Rgb(136, 192, 208),
            Theme::Catppuccin => Color::Rgb(137, 180, 250),
            Theme::Monokai => Color::Rgb(166, 226, 46),
            Theme::OneDark => Color::Rgb(97, 175, 239),
        }
    }

    pub fn border(&self) -> Color {
        match self {
            Theme::Dark => Color::Rgb(64, 64, 64),
            Theme::Light => Color::Rgb(200, 200, 200),
            Theme::Gruvbox => Color::Rgb(102, 92, 84),
            Theme::Nord => Color::Rgb(76, 86, 106),
            Theme::Catppuccin => Color::Rgb(69, 71, 90),
            Theme::Monokai => Color::Rgb(73, 72, 62),
            Theme::OneDark => Color::Rgb(51, 56, 64),
        }
    }

    pub fn bar_colors(&self) -> [Color; 4] {
        match self {
            Theme::Dark => [
                Color::Rgb(60, 80, 60),
                Color::Rgb(80, 160, 60),
                Color::Rgb(0, 180, 80),
                Color::Rgb(0, 220, 100),
            ],
            Theme::Light => [
                Color::Rgb(160, 200, 160),
                Color::Rgb(80, 180, 80),
                Color::Rgb(20, 160, 60),
                Color::Rgb(0, 130, 50),
            ],
            Theme::Gruvbox => [
                Color::Rgb(104, 157, 106),
                Color::Rgb(142, 192, 124),
                Color::Rgb(184, 187, 38),
                Color::Rgb(250, 189, 47),
            ],
            Theme::Nord => [
                Color::Rgb(76, 86, 106),
                Color::Rgb(129, 161, 193),
                Color::Rgb(136, 192, 208),
                Color::Rgb(143, 188, 187),
            ],
            Theme::Catppuccin => [
                Color::Rgb(166, 227, 161),
                Color::Rgb(148, 226, 213),
                Color::Rgb(137, 180, 250),
                Color::Rgb(180, 190, 254),
            ],
            Theme::Monokai => [
                Color::Rgb(73, 72, 62),
                Color::Rgb(166, 226, 46),
                Color::Rgb(230, 219, 116),
                Color::Rgb(253, 151, 31),
            ],
            Theme::OneDark => [
                Color::Rgb(92, 99, 112),
                Color::Rgb(152, 195, 121),
                Color::Rgb(97, 175, 239),
                Color::Rgb(198, 120, 221),
            ],
        }
    }

    pub fn heatmap_empty(&self) -> Color {
        match self {
            Theme::Dark => Color::Rgb(30, 30, 30),
            Theme::Light => Color::Rgb(230, 230, 230),
            Theme::Gruvbox => Color::Rgb(50, 48, 47),
            Theme::Nord => Color::Rgb(46, 52, 64),
            Theme::Catppuccin => Color::Rgb(30, 30, 46),
            Theme::Monokai => Color::Rgb(39, 40, 34),
            Theme::OneDark => Color::Rgb(40, 44, 52),
        }
    }

    pub fn heatmap_colors(&self) -> [Color; 4] {
        match self {
            Theme::Dark => [
                Color::Rgb(20, 80, 40),
                Color::Rgb(0, 140, 60),
                Color::Rgb(0, 190, 80),
                Color::Rgb(0, 240, 100),
            ],
            Theme::Light => [
                Color::Rgb(198, 228, 198),
                Color::Rgb(120, 200, 120),
                Color::Rgb(40, 168, 80),
                Color::Rgb(0, 124, 46),
            ],
            Theme::Gruvbox => [
                Color::Rgb(69, 133, 136),
                Color::Rgb(104, 157, 106),
                Color::Rgb(142, 192, 124),
                Color::Rgb(184, 187, 38),
            ],
            Theme::Nord => [
                Color::Rgb(59, 66, 82),
                Color::Rgb(76, 86, 106),
                Color::Rgb(129, 161, 193),
                Color::Rgb(143, 188, 187),
            ],
            Theme::Catppuccin => [
                Color::Rgb(69, 71, 90),
                Color::Rgb(137, 180, 250),
                Color::Rgb(148, 226, 213),
                Color::Rgb(166, 227, 161),
            ],
            Theme::Monokai => [
                Color::Rgb(73, 72, 62),
                Color::Rgb(102, 217, 239),
                Color::Rgb(166, 226, 46),
                Color::Rgb(230, 219, 116),
            ],
            Theme::OneDark => [
                Color::Rgb(40, 44, 52),
                Color::Rgb(86, 182, 194),
                Color::Rgb(152, 195, 121),
                Color::Rgb(97, 175, 239),
            ],
        }
    }
}
