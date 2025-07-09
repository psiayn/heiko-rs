use iroh::{Endpoint, protocol::Router};
use iroh_blobs::{ALPN as BLOBS_ALPN, net_protocol::Blobs};
use iroh_docs::{protocol::Docs, store::{Query, QueryBuilder}, ALPN as DOCS_ALPN};
use iroh_gossip::{ALPN as GOSSIP_ALPN, net::Gossip};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create an endpoint, it allows creating and accepting
    // connections in the iroh p2p world
    let endpoint = Endpoint::builder().discovery_n0().bind().await?;

    let builder = Router::builder(endpoint);

    // build the blobs protocol
    let blobs = Blobs::memory().build(builder.endpoint());

    // build the gossip protocol
    let gossip = Gossip::builder().spawn(builder.endpoint().clone()).await?;

    // build the docs protocol
    let docs = Docs::memory().spawn(&blobs, &gossip).await?;

    let docs_client = docs.client();

    let author_id = docs_client.authors().default().await.unwrap();
    let new_doc = docs_client.create().await.unwrap();
    let hash = new_doc.set_bytes(author_id, "aaa", "bbb").await.unwrap();

    println!(
        "added new key with hash: {hash} on doc with id: {}",
        new_doc.id()
    );

    let added_doc = docs_client.open(new_doc.id()).await.unwrap().unwrap();
    let added_doc = added_doc.get_one(QueryBuilder::default().key_exact("aaa").build()).await.unwrap().unwrap();

    tokio::signal::ctrl_c().await?;

    // setup router
    let router = builder
        .accept(BLOBS_ALPN, blobs)
        .accept(GOSSIP_ALPN, gossip)
        .accept(DOCS_ALPN, docs)
        .spawn();

    // Gracefully shut down the router
    println!("Shutting down.");
    router.shutdown().await?;

    Ok(())
}
