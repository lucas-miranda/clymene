use super::OutputStats;
use colored::Colorize;

pub struct AtlasOutputStats {
    free_space_percent: f32,
}

impl AtlasOutputStats {
    pub fn new(free_space_percent: f32) -> Self {
        Self { free_space_percent }
    }
}

impl OutputStats for AtlasOutputStats {
    fn display_stats(&self) {
        let used_space_percent = (100.0f32 - self.free_space_percent).clamp(0.0, 100.0);
        infoln!(block, "Used space");

        let bar_length = 16;
        let completed_bar_length =
            ((used_space_percent / 100.0) * bar_length as f32).round() as usize;

        let empty_bar_length = bar_length - completed_bar_length;

        infoln!(
            last,
            "{:.2}%  [{}{}]",
            used_space_percent.to_string().bold(),
            "=".repeat(completed_bar_length),
            " ".repeat(empty_bar_length),
        );
    }
}
