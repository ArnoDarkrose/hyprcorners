use crate::config::Config;

use std::time::{Duration, Instant};

use hyprland::data::CursorPosition;
use hyprland::shared::HyprData;
use hyprland::Result;

pub struct App;
impl App {
    //noinspection ALL
    pub async fn run(config: Config) -> Result<()> {
        let sticky = config.sticky_timeout.is_some();

        let mut last_switch = None;
        let mut inside_corner = false;

        loop {
            let mut modified_inside_corner = false;

            let cursor_pos = CursorPosition::get_async().await?;
            let (x, y) = (cursor_pos.x, cursor_pos.y);

            let since_last_sticky_switch =
                last_switch.map(|last_switch| Instant::now().duration_since(last_switch));

            if !sticky
                || since_last_sticky_switch.is_none_or(|since_last_sticky_switch| {
                    since_last_sticky_switch.as_millis() >= config.sticky_timeout.unwrap() as u128
                })
            {
                let dispatch_permitted = sticky || !inside_corner;

                if let Some(ref corner) = config.top_right {
                    if x > config.screen_width - corner.radius && y < corner.radius {
                        if dispatch_permitted {
                            corner.dispatch(&sticky, &mut last_switch).await?;
                        }

                        inside_corner = true;
                        modified_inside_corner = true;
                    }
                }

                if let Some(ref corner) = config.top_left {
                    if x < corner.radius && y < corner.radius {
                        if dispatch_permitted {
                            corner.dispatch(&sticky, &mut last_switch).await?;
                        }

                        inside_corner = true;
                        modified_inside_corner = true;
                    }
                }

                if let Some(ref corner) = config.bottom_right {
                    if x > config.screen_width - corner.radius
                        && y > config.screen_height - corner.radius
                    {
                        if dispatch_permitted {
                            corner.dispatch(&sticky, &mut last_switch).await?;
                        }

                        inside_corner = true;
                        modified_inside_corner = true;
                    }
                }

                if let Some(ref corner) = config.bottom_left {
                    if x < corner.radius && y > config.screen_height - corner.radius {
                        if dispatch_permitted {
                            corner.dispatch(&sticky, &mut last_switch).await?;
                        }

                        inside_corner = true;
                        modified_inside_corner = true;
                    }
                }

                if let Some(ref edge) = config.top {
                    if y < edge.distance {
                        if dispatch_permitted {
                            edge.dispatch(&sticky, &mut last_switch).await?;
                        }

                        inside_corner = true;
                        modified_inside_corner = true;
                    }
                }

                if let Some(ref edge) = config.bottom {
                    if y > config.screen_height - edge.distance {
                        if dispatch_permitted {
                            edge.dispatch(&sticky, &mut last_switch).await?;
                        }

                        inside_corner = true;
                        modified_inside_corner = true;
                    }
                }

                if let Some(ref edge) = config.left {
                    if x < edge.distance {
                        if dispatch_permitted {
                            edge.dispatch(&sticky, &mut last_switch).await?;
                        }

                        inside_corner = true;
                        modified_inside_corner = true;
                    }
                }

                if let Some(ref edge) = config.right {
                    if x > config.screen_width - edge.distance {
                        if dispatch_permitted {
                            edge.dispatch(&sticky, &mut last_switch).await?;
                        }

                        inside_corner = true;
                        modified_inside_corner = true;
                    }
                }

                if !modified_inside_corner {
                    inside_corner = false;
                }
            }

            std::thread::sleep(Duration::from_millis(config.timeout));
        }
    }
}
