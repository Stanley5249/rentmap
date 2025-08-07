use clap::Args;
use spider_chrome::browser::BrowserConfig;
use spider_chrome::handler::viewport::Viewport;

use super::error::SpiderChromeError;

#[derive(Debug, Default, Args)]
#[command(next_help_heading = "Spider Chrome")]
pub struct SpiderChromeArgs {
    /// Run browser in head mode (non-headless)
    #[arg(long)]
    pub head: bool,
}

impl TryFrom<SpiderChromeArgs> for BrowserConfig {
    type Error = SpiderChromeError;

    fn try_from(args: SpiderChromeArgs) -> Result<Self, Self::Error> {
        let mut config = BrowserConfig::builder()
            .viewport(Some(Viewport {
                width: 1920,
                height: 1080,
                ..Default::default()
            }))
            .enable_request_intercept();

        if args.head {
            config = config.with_head();
        }

        let mut browser_config = config.build().map_err(SpiderChromeError::Config)?;

        browser_config.ignore_ads = true;
        browser_config.ignore_analytics = true;
        browser_config.ignore_javascript = false;
        browser_config.ignore_stylesheets = false;
        browser_config.ignore_visuals = false;

        Ok(browser_config)
    }
}
