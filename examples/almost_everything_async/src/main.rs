use hyprland::data::{Animations, Client, Clients, Monitors, Workspace};
use hyprland::event_listener::AsyncEventListener;
use hyprland::keyword::*;
use hyprland::prelude::*;
//use hyprland::shared::WorkspaceType;
use hyprland::{async_closure, dispatch::*};

fn main() -> hyprland::Result<()> {
    // It is always a good practice to configure your runtime to your specifications
    tokio::runtime::Builder::new_multi_thread().enable_io().enable_time().worker_threads(1).build()?.block_on(async move {
        // We can call dispatchers with the dispatch function!

        // Here we are telling hyprland to open kitty using the dispatch macro!
        hyprland::dispatch!(async; Exec, "kitty").await?;

        // Here we are adding a keybinding to Hyprland using the bind macro!
        hyprland::bind!(async; SUPER, Key, "i" => ToggleFloating, None).await?;

        // Here we are moving the cursor to the top left corner! We can also just use the Dispatch
        // struct!
        Dispatch::call_async(DispatchType::MoveCursorToCorner(Corner::TopLeft)).await?;

        let border_size = match Keyword::get_async("general:border_size").await?.value {
            OptionValue::Int(i) => i,
            _ => panic!("border size can only be a int"),
        };
        println!("{border_size}");

        // Here we change a keyword, yes its a dispatcher don't complain
        Keyword::set_async("general:border_size", border_size * 2).await?;
        // get all monitors
        let monitors = Monitors::get_async().await?;
        // and the active window
        let win = Client::get_active_async().await?;
        // and all open windows
        let clients = Clients::get_async().await?;
        // and the active workspace
        let work = Workspace::get_active_async().await?;
        // and printing them all out!
        println!("monitors: {monitors:#?},\nactive window: {win:#?},\nclients {clients:#?}\nworkspace:{work:#?}");
        let animations = Animations::get_async().await?;
        println!("{animations:#?}");
        // Create a event listener
        let mut event_listener = AsyncEventListener::new();

        //This changes the workspace to 5 if the workspace is switched to 9
        //this is a performance and mutable state test
        // event_listener.add_workspace_change_handler(async_closure! {|id, state| {
        //     if id == WorkspaceType::Regular('9'.to_string()) {
        //         *state.workspace = '2'.to_string();
        //     }
        // }});
        /*
        event_listener.add_workspace_change_handler(|id, state| {
            Box::pin(async move {
                if id == WorkspaceType::Regular('9'.to_string()) {
                    *state.workspace = '2'.to_string();
                }
            })
        });

        // This makes it so you can't turn on fullscreen lol
        event_listener.add_fullscreen_state_change_handler(async_closure! {|fstate, state| {
            if fstate {
                *state.fullscreen = false;
            }
        }});
        // Makes a monitor unfocusable
        event_listener.add_active_monitor_change_handler(async_closure! {|data, state| {
            let hyprland::event_listener::MonitorEventData{ monitor_name, .. } = data;

            if monitor_name == *"DP-1".to_string() {
                *state.monitor = "eDP-1".to_string()
            }
        }});
        */
        // add event, yes functions and closures both work!

        event_listener.add_workspace_change_handler(
            async_closure! { move |id| println!("workspace changed to {id:#?}")},
        );
        event_listener.add_active_window_change_handler(
            async_closure! { move |data| println!("window changed to {data:#?}")},
        );
        // Waybar example
        // event_listener.add_active_window_change_handler(|data| {
        //     use hyprland::event_listener::WindowEventData;
        //     let string = match data {
        //         Some(WindowEventData(class, title)) => format!("{class}: {title}"),
        //         None => "".to_string()
        //     };
        //     println!(r#"{{"text": "{string}", class: "what is this?"}}"#);
        // });

        // reset your border size back to normal
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            println!("Resetting border size!");
            Keyword::set_async("general:border_size", border_size).await?;
            Ok::<_, hyprland::shared::HyprError>(())
        });

        // and execute the function
        // here we are using the blocking variant
        // but there is a async version too
        event_listener.start_listener_async().await
    })
}
