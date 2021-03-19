#[macro_use]
extern crate penrose;

use penrose::{
    core::{
        bindings::KeyEventHandler, config::Config, helpers::index_selectors,
        manager::WindowManager, ring::Selector,
    },
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More, Result,
};

use simplelog::{LevelFilter, SimpleLogger};

const TERMINAL: &str = "alacritty";
const LAUNCHER: &str = "dmenu_run";

fn main() -> Result<()> {
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("Unable to set log level: {}", e);
    };

    let config = Config::default();
    let key_bindings = gen_keybindings! {
        "M-p" => run_external!(LAUNCHER);
        "M-Return" => run_external!(TERMINAL);

        "M-C-q" => run_internal!(exit);


        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-S-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-S-w" => run_internal!(kill_client);


        "M-S-t" => run_internal!(toggle_workspace);
        "M-Tab" => run_internal!(cycle_workspace, Forward);
        "M-A-Tab" => run_internal!(cycle_workspace, Backward);


        "M-Space" => run_internal!(cycle_layout, Forward);
        "M-A-Space" => run_internal!(cycle_layout, Backward);
        "M-A-k" => run_internal!(update_max_main, More);
        "M-A-j" => run_internal!(update_max_main, Less);
        "M-A-l" => run_internal!(update_main_ratio, More);
        "M-A-h" => run_internal!(update_main_ratio, Less);

        refmap [ config.ws_range() ] in {
            "M-{}" => focus_workspace [ index_selectors(config.workspaces().len()) ];
            "M-S-{}" => client_to_workspace [ index_selectors(config.workspaces().len()) ];
        };

        let mut wm = new_xcb_backed_window_manager(config, vec![], logging_error_handler())?;
        wm.grab_keys_and_run(key_bindings, map!{})
    };
}
