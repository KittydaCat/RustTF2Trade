use std::num::NonZeroU16;
use reqwest::{Client};
use serde_json::{json, Number, Value};
use tf2_sku::{SKU, SKUString};
use std::time::Duration;

#[derive(Debug)]
struct PriceGrabber {
    token: String,
    client: Client,
}

impl PriceGrabber {

    async fn new() -> PriceGrabber {
        let client = Client::new();

        let res = client
            .post("https://api2.prices.tf/auth/access")
            .send()
            .await
            .unwrap();

        let body: Value = res.json().await.unwrap();

        let Value::Object(map) = body else {panic!()};

        dbg!(&map);

        let Value::String(token) = map.get("accessToken").unwrap() else {panic!()};

        PriceGrabber {
            token: token.clone(),
            client,
        }

    }

    async fn get_price(&self, sku: SKU) -> u64 {

        let request = self.client.get(format!("https://api2.prices.tf/prices/{}", sku.to_sku_string()))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await;

        if let Err(err) = request {
            dbg!(err);
            todo!();
        }

        let Ok(response) = request else {unreachable!()};

        let Value::Object(map) = response.json::<Value>().await.unwrap() else { panic!() };
        let Value::Number(x) = map.get("buyHalfScrap").unwrap() else { panic!() };

        dbg!(x);

        x.as_u64().unwrap()

    }
}

#[derive(Debug)]
struct Listing {

}

#[derive(Debug)]
struct BackpackGrabber {
    key: String,
    client: Client,
    time_since_last_request: Duration
}

impl BackpackGrabber {
    fn new(key: String) -> BackpackGrabber {
        BackpackGrabber {
            key,
            client: Client::new(),
            time_since_last_request: Duration::new(60,0), // might set higher
        }
    }

    async fn get_listings(&mut self, item_name: &str) -> Vec<Listing> {
        let res = self.client.get("https://backpack.tf/api/classifieds/listings/snapshot")
            .json(&json!({
                "sku": item_name,
                "appid": "440",
                "token": self.key,
                }))
            .
            .send()
            .await
            .unwrap();

        dbg!(res.json::<Value>().await.unwrap());
        todo!()
    }

}

#[cfg(test)]
mod tests {
    use std::fs;
    use tf2_sku::tf2_enum::Quality;
    use super::*;

    #[tokio::test]
    async fn init_client() {
        let grab = PriceGrabber::new().await;

        dbg!(grab);
    }

    #[tokio::test]
    async fn get_pan_price() {
        let grab = PriceGrabber::new().await;

        dbg!(grab.get_price(SKU::new(264, Quality::Unique)).await);
    }
    #[tokio::test]
    async fn get_all_prices() {

        let item_names = fs::read_to_string("killstreakable_weapons_names.txt").unwrap()
            .lines().filter(|x| !x.is_empty()).map(String::from).collect::<Vec<String>>();

        for name in item_names {
            // SKU::
        }
    }

    #[tokio::test]
    async fn init_backpack_client() {
        let mut grab = BackpackGrabber::new(fs::read_to_string("secrets.txt").unwrap());

        dbg!(&grab);

        grab.get_listings("Frying Pan").await;
    }



}