use crate::progress::bar::Bar;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::borrow::Cow;
use std::sync::LazyLock;

static MULTI_PROGRESS: LazyLock<MultiProgress> = LazyLock::new(MultiProgress::new);

pub struct CliProgressBar {
    bar: ProgressBar,
}

impl CliProgressBar {
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        let bar = MULTI_PROGRESS.add(ProgressBar::new(0));

        bar.set_message(message);
        bar.set_style(
            ProgressStyle::with_template("{msg}\t{bar:80.cyan/blue}")
                .unwrap()
                .progress_chars("##-"),
        );

        Self { bar }
    }
}

impl Bar for CliProgressBar {
    async fn set_length(&mut self, length: u64) {
        self.bar.set_length(length);
    }

    async fn update_progress(&mut self, delta: u64) {
        self.bar.inc(delta);
    }

    async fn set_progress(&mut self, progress: u64) {
        self.bar.set_position(progress)
    }

    async fn finish(&mut self) {
        self.bar.finish_and_clear();
    }
}
