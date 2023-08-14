use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn progress() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.red} {msg}")
            .unwrap()
            .tick_strings(&[
                "                              🚶",
                "                              🚶",
                "                             🏃 ",
                "                             🏃 ",
                "                            🚶  ",
                "                            🚶  ",
                "                           🏃   ",
                "                           🏃   ",
                "                          🚶    ",
                "                          🚶    ",
                "                         🏃     ",
                "                         🏃     ",
                "                        🚶      ",
                "                        🚶      ",
                "                       🏃       ",
                "                       🏃       ",
                "                      🚶        ",
                "                      🚶        ",
                "                     🏃         ",
                "                     🏃         ",
                "                    🚶          ",
                "                    🚶          ",
                "                   🏃           ",
                "                   🏃           ",
                "                  🚶            ",
                "                  🚶            ",
                "                 🏃             ",
                "                 🏃             ",
                "                🚶              ",
                "                🚶              ",
                "               🏃               ",
                "               🏃               ",
                "              🚶                ",
                "              🚶                ",
                "             🏃                 ",
                "             🏃                 ",
                "            🚶                  ",
                "            🚶                  ",
                "           🏃                   ",
                "           🏃                   ",
                "          🚶                    ",
                "          🚶                    ",
                "         🏃                     ",
                "         🏃                     ",
                "        🚶                      ",
                "        🚶                      ",
                "       🏃                       ",
                "       🏃                       ",
                "      🚶                        ",
                "      🚶                        ",
                "     🏃                         ",
                "     🏃                         ",
                "    🚶                          ",
                "    🚶                          ",
                "   🏃                           ",
                "   🏃                           ",
                "  🚶                            ",
                "  🚶                            ",
                " 🏃                             ",
                " 🏃                             ",
                "🚶                              ",
                "🚶                              ",
            ]),
    );
    pb.set_message("Processing...");

    pb
}
