//Sends json messages to server and listens for messages
mod mqtt;
use mqtt::{establishclient, publishmessage, sub, pollevents};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, ACCEPT_LANGUAGE, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    battery_percentage: u8,
    mission_text: String,
    state_text: String
}

#[tokio::main]
async fn main() {
    let pod = "pod1".to_string();
    let port = 4832;
    let broker_address = "localhost".to_string();

    //Format pod information for the controller's topic
    //let topic = format!("podc/v1/devices/{}", pod.clone());
    let topic = format!("test/topic");
    //Set up client
    let (client, mut eventloop) = establishclient(pod.clone(), broker_address, port).await;
    sub(client.clone(), topic.clone()).await;

    //Uncomment to publish a message that the pod is online
    //let online_msg = format!("{} is online", pod.clone());

    //Construct headers for API call
    fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static("Basic ZGlzdHJpYnV0b3I6NjJmMmYwZjFlZmYxMGQzMTUyYzk1ZjZmMDU5NjU3NmU0ODJiYjhlNDQ4MDY0MzNmNGNmOTI5NzkyODM0YjAxNA=="));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }
    //Link to MiR API
    let url = format!("http://192.168.0.11/api/v2.0.0/status");
    //build client using headers
    let api_client = reqwest::Client::builder()
    .default_headers(construct_headers())
    .build()
    .unwrap();

    //Get API respones
    let response = api_client
    .get(url)
    .send()
    .await
    .unwrap();

    //Publish API response to MQTT
    let json_response = response.text().await.unwrap();
    publishmessage(json_response.clone(), client.clone(), topic.clone()).await;

    //Poll for events
    loop {
        let event = eventloop.poll().await.unwrap();
        pollevents(event).await;
    }
 
}
