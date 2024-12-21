use std::error::Error;
use std::thread;
use std::time::Duration;
use openrgb::{data::Color, OpenRGB};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // connect to local server
    let client = OpenRGB::connect_to(("localhost", 6742)).await?;

    // set client name
    client.set_name("my client").await?;

    // print protocol version
    println!("connected using protocol version {}", client.get_protocol_version());

    // query controllers count
    let controllers = client.get_controller_count().await?;

    let color = Color {
        r: 255,
        g: 0,
        b: 0,
    };
    // query and print each controller data
    for controller_id in 0..controllers {
        println!("controller {}: {:#?}", controller_id, client.get_controller(controller_id).await?);
        client.update_led(controller_id, 5, color).await?;
    }

    // // get profiles names
    // println!("profiles: {:?}", client.get_profiles().await?);

    // // save the current configuration to a new profile
    // client.save_profile("my profile").await?;

    // // load a profile by name
    // client.load_profile("my profile").await?;

    // // delete a profile by name
    // client.delete_profile("my profile").await?;

    // client.update_led()
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}

