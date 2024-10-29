//The goal of this file is to create a library of functions for the podcontroller to use
//
use rumqttc::{AsyncClient, ClientError, Event, EventLoop, Incoming, MqttOptions, QoS};
use tokio::time;
use std::sync::{Arc, Mutex};
use std::time::Duration;

//This function can be called after the establishclient() function. Pass this function the event where 
//declaration of event : let event = eventloop.poll().await.unwrap();
pub async fn pollevents(event: Event) {
    let messages = Arc::new(Mutex::new(Vec::new()));
    let messages_clone = Arc::clone(&messages);
    //create async thread for the eventloop
    tokio::spawn(async move {
            match event {
                Event::Incoming(Incoming::Publish(publish)) => {
                    // Store received message in shared state
                    let payload = String::from_utf8_lossy(&publish.payload).to_string();
                    // Print payload
                    println!("Payload: {}", payload);
                    messages_clone.lock().unwrap().push(payload);
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
                //Break from events other than incoming
                //Other events can be handled here (Connection lost, etc..)
                _ => {}
            }
        //}
    });
}

//This is the first function that should be called when starting MQTT connection
//Make sure to seperate client and eventloop when calling this function
//declaration of client and eventloop : let (client, mut eventloop) = establishclient(pod.clone(), broker_address, port).await;
pub async fn establishclient(pod: String, address: String, port: u16) -> (rumqttc::AsyncClient, EventLoop) {
    //establish client with options
    let mut mqttoptions = MqttOptions::new(pod, address, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    //return AsyncClient (Client details for publishing messages) and eventloop
    (client, eventloop)
}

pub async fn sub (client: AsyncClient, topic: String) {
    //subscribe to topic
    if let Err(e) = client.subscribe(topic, QoS::AtMostOnce).await {
        eprintln!("Failed to subscribe: {:?}", e)
    } else {
        println!("Subscribed");
    }
 
}

pub async fn publishmessage (message: String, client: AsyncClient, topic: String) {
    let payload = message.trim();
    client.publish(topic, QoS::ExactlyOnce, false, payload).await.unwrap();
    time::sleep(Duration::from_millis(1000)).await;
}