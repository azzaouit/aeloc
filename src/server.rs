use crate::{nominatim, overpass};
use ethers::prelude::*;
use ethers::{
    core::types::{Address, U256},
    providers::Provider,
};
use eyre::Result;
use hex;
use log::info;
use std::sync::Arc;

/// Coordinate multiplier
const COORDINATE_MULTIPLIER: f64 = 1e8_f64;

abigen!(AelocDispatcher, "contracts/AelocDispatcher.json");

pub async fn serve(
    provider: String,
    nominatim_uri: String,
    overpass_uri: String,
    dispatcher: Address,
    wallet: LocalWallet,
) -> Result<()> {
    let provider = Provider::<Ws>::connect(&provider).await?;
    let client = SignerMiddleware::new_with_provider_chain(provider, wallet).await?;

    let c = Arc::new(client.clone());
    let events = AelocDispatcher::new(dispatcher, c).events();
    let mut stream = events.stream().await?;

    while let Some(Ok(event)) = stream.next().await {
        let tx;
        let client = Arc::new(client.clone());
        match event {
            AelocDispatcherEvents::GeocodeFilter(e) => {
                let (caller, data) = geocode_handler(&e, nominatim_uri.clone()).await?;
                let instance = AelocDispatcher::new(caller, client);
                tx = instance.geocode_callback(data);
            }
            AelocDispatcherEvents::ReverseGeocodeFilter(e) => {
                let (caller, data) = reverse_geocode_handler(&e, nominatim_uri.clone()).await?;
                let instance = AelocDispatcher::new(caller, client);
                tx = instance.reverse_geocode_callback(data);
            }
            AelocDispatcherEvents::BoundingBoxFilter(e) => {
                let (caller, data) = bounding_box_handler(&e, overpass_uri.clone()).await?;
                let instance = AelocDispatcher::new(caller, client);
                tx = instance.bounding_box_callback(data);
            }
        };
        let tx_rcpt = tx.send().await?.await?;
        let tx_hash = tx_rcpt.unwrap().transaction_hash;
        info!("Dispatcher: sent transaction {}", hex::encode(tx_hash));
    }

    Ok(())
}

pub async fn geocode_handler(e: &GeocodeFilter, nominatim_uri: String) -> Result<(Address, U256)> {
    info!("Entered geocode handler: {}", e);
    let c = nominatim::Config {
        url: format!("{}/{}", nominatim_uri, "search"),
        timeout: 25,
    };

    let s = String::from_utf8(e.location.as_bytes().to_vec())?;
    let g = nominatim::Geocode::new(s);
    let resp = g.search(&c).await?;
    let top_id = U256::from_big_endian(&resp[0].osm_id.to_be_bytes());

    info!("Returning from geocode handler");
    Ok((e.caller, top_id))
}

pub async fn reverse_geocode_handler(
    e: &ReverseGeocodeFilter,
    nominatim_uri: String,
) -> Result<(Address, U256)> {
    info!("Entered reverse geocode handler: {}", e);
    let c = nominatim::Config {
        url: format!("{}/{}", nominatim_uri, "reverse"),
        timeout: 25,
    };

    let g = nominatim::ReverseGeocode {
        lat: e.lat.as_i64() as f64 / COORDINATE_MULTIPLIER,
        lon: e.lon.as_i64() as f64 / COORDINATE_MULTIPLIER,
    };
    let resp = g.search(&c).await?;

    let top_id = U256::from_big_endian(&resp.osm_id.to_be_bytes());
    Ok((e.caller, top_id))
}

pub async fn bounding_box_handler(
    e: &BoundingBoxFilter,
    overpass_uri: String,
) -> Result<(Address, Vec<U256>)> {
    info!("Entered bounding box handler: {}", e);
    let c = overpass::Config {
        url: &overpass_uri,
        timeout: 25,
        key: &e.key.to_string(),
        val: &e.val.to_string(),
    };

    let b = overpass::BoundingBox {
        xmin: e.xmin.as_i64() as f64 / COORDINATE_MULTIPLIER,
        ymin: e.ymin.as_i64() as f64 / COORDINATE_MULTIPLIER,
        xmax: e.xmax.as_i64() as f64 / COORDINATE_MULTIPLIER,
        ymax: e.ymax.as_i64() as f64 / COORDINATE_MULTIPLIER,
    };

    let resp = b.search(&c).await?;
    let r: Vec<U256> = resp
        .elements
        .iter()
        .take(e.limit.as_u64() as usize)
        .map(|i| U256::from(i.id))
        .collect();
    Ok((e.caller, r))
}
