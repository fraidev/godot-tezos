use godot::prelude::*;
use godot::engine::Sprite2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    #[base]
    sprite: Base<Sprite2D>
}

use godot::engine::Sprite2DVirtual;
use tezos_rpc::client::TezosRpc;
use tezos_rpc::models::block::BlockId;

async fn my_async_function() {
    let rpc = TezosRpc::new("https://ghostnet.tezos.marigold.dev/".to_string());
    let block_result = rpc
        .get_block()
        .block_id(&BlockId::Head)
        .send().await;

    match block_result {
        Ok(block) => {
            godot_print!("block hash: {:?}", block.hash); // Prints to the Godot console
        }
        Err(e) => {
            godot_print!("Error: {:?}", e); // Prints to the Godot console
        }
    }
}

#[godot_api]
impl Sprite2DVirtual for Player {
    fn init(sprite: Base<Sprite2D>) -> Self {

        let mut runtime_builder = tokio::runtime::Builder::new_current_thread();
        runtime_builder.enable_all();
        let runtime = runtime_builder.build().unwrap();
        runtime.block_on(async {
            my_async_function().await;
        });


        
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            sprite
        }
    }

}

