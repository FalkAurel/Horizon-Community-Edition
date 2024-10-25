use plugin_test_api::{BaseAPI, GameEvent, Plugin, PluginContext, PluginInformation, PluginMetadata, RpcPlugin, SayHello, PLUGIN_API_VERSION};
use std::{any::Any, sync::Arc};
use async_trait::async_trait;
use crate::recipe_smith::{self, RecipeSmith};


#[derive(Debug, Clone)]
pub struct StarsBeyond {
    recipe_smith: Arc<RecipeSmith>,
}

impl StarsBeyond {
    pub fn new() -> Self {
        StarsBeyond {
            recipe_smith: Arc::new(RecipeSmith::new()),
        }
    }
}

#[async_trait]
impl BaseAPI for StarsBeyond {
    async fn on_game_event(&self, event: &GameEvent) {
        match event {
            GameEvent::PlayerJoined(player) => {
                println!("Stars Beyond: Welcome, explorer {}! The universe awaits.", player.id);
            }
            GameEvent::PlayerMoved { player, new_position } => {
                println!("Stars Beyond: Explorer {} moved to {:?}", player.id, new_position);
            }
            _ => {}
        }
        // Forward events to RecipeSmith
        self.recipe_smith.on_game_event(event).await;
    }

    async fn on_game_tick(&self, delta_time: f64) {
        println!("Stars Beyond: Simulating space, delta time: {:.2}", delta_time);
        // Forward tick to RecipeSmith
        self.recipe_smith.on_game_tick(delta_time).await;

        let player_id = "sdaljhn398";

        let params: Box<dyn Any + Send + Sync> = Box::new((player_id.to_string(), 18u32));
        let _ = self.recipe_smith.call_rpc("create_player_inventory", &*params);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Plugin for StarsBeyond {
    fn on_load(&self) {
        println!("Stars Beyond plugin loaded");
        self.recipe_smith.on_load();
    }

    fn on_unload(&self) {
        println!("Stars Beyond plugin unloaded");
        self.recipe_smith.on_unload();
    }

    fn execute(&self) {
        println!("Stars Beyond plugin executed");
        self.recipe_smith.execute();
    }

    fn initialize(&self, context: &mut PluginContext) {
        println!("Stars Beyond plugin initializing");
        self.recipe_smith.initialize(context);
    }

    fn shutdown(&self, context: &mut PluginContext) {
        println!("Stars Beyond plugin shutting down");
        self.recipe_smith.shutdown(context);
    }

    fn on_enable(&self, context: &mut PluginContext) {
        println!("Stars Beyond plugin enabled");
        self.recipe_smith.on_enable(context);
    }

    fn on_disable(&self, context: &mut PluginContext) {
        println!("Stars Beyond plugin disabled");
        self.recipe_smith.on_disable(context);
    }
}

impl PluginInformation for StarsBeyond {
    fn name(&self) -> String {
        "Stars Beyond".to_string()
    }

    fn get_instance(&self) -> Box<dyn SayHello> {
        Box::new(self.clone())
    }
}

impl SayHello for StarsBeyond {
    fn say_hello(&self) -> String {
        format!("Calculating shortest path to universal domination... ETA: 42 millennia. Warning: Excessive galactic conquest may lead to space papercuts. Pack bandages accordingly. {}",
                self.recipe_smith.say_hello())
    }
}

// Instead of a const PLUGIN_METADATA, we'll use a function to create it
pub fn get_plugin() -> StarsBeyond {
    StarsBeyond::new()
}

pub fn get_plugin_metadata() -> PluginMetadata {
    PluginMetadata {
        name: "Stars Beyond".to_string(),
        version: "1.0.0".to_string(),
        description: "A plugin for space exploration and cosmic adventures, with integrated crafting system".to_string(),
        api_version: PLUGIN_API_VERSION,
    }
}