use godot::prelude::*;
use iroh_gossip::{
    Gossip,
    TopicId,
    api::{
        GossipReceiver,
        GossipSender,
    },
};

use crate::{
    iroh_endpoint::IrohEndpoint,
    runtime::IrohRuntime,
};

#[derive(GodotClass)]
#[class(base=Object)]
pub struct IrohGossip {
    base: Base<Object>,
    pub(crate) gossip: Option<Gossip>,
    pub(crate) sender: Option<GossipSender>,
    pub(crate) receiver: Option<GossipReceiver>,
}

#[godot_api]
impl IObject for IrohGossip {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            gossip: None,
            sender: None,
            receiver: None,
        }
    }
}

#[godot_api]
impl IrohGossip {
    #[func]
    fn spawn(&mut self, endpoint: Gd<IrohEndpoint>) {
        let Some(real_endpoint) = endpoint.bind().endpoint.clone() else {
            godot_error!(
                "Attempted to spawn a IrohGossip object with a uninitialized IrohEndpoint as \
                 input, you must first bind the endpoint!"
            );
            return;
        };

        let gossip = IrohRuntime::block_on(async move { Gossip::builder().spawn(real_endpoint) });

        self.gossip = Some(gossip);
    }

    #[func]
    fn subscribe(&mut self, topic: PackedByteArray, peer_public_keys: Vec<GString>) {
        if topic.len() != 32 {
            godot_error!("Topic input must be 32 bytes long");
        }

        let mut keys = vec![];

        for key in peer_public_keys {
            keys.push(
                key.to_string()
                    .parse()
                    .expect("Failed to parse peer public key"),
            );
        }

        let Some(ref real_gossip) = self.gossip else {
            godot_error!(
                "Attempted to call subscribe on a blank gossip. You must first spawn the gossip!"
            );
            return;
        };

        let mut topic_slice = [0u8; 32];
        topic_slice.copy_from_slice(topic.as_slice());

        let (sender, receiver) = IrohRuntime::block_on(async move {
            real_gossip
                .subscribe(TopicId::from_bytes(topic_slice), keys)
                .await
                .expect("Failed to subscribe gossip")
                .split()
        });

        self.sender = Some(sender);
        self.receiver = Some(receiver);
    }

    #[func]
    fn test_connect(&mut self) {
        let Some(ref mut receiver) = self.receiver else {
            godot_error!("fuck");
            return;
        };

        IrohRuntime::block_on(async move {
            receiver
                .joined()
                .await
                .expect("Waiting for connection failed")
        });

        godot_print!("PEER CONNECTED!");
    }
}
