use crate::CoCClient;
use urlencoding::encode;
use crate::models::Clan;

impl CoCClient {
    pub async fn get_clan_information(self: Self, clan_tag: String) -> Clan {
        let encoded_clan_tag = encode(&clan_tag).into_owned();

        let path = format!("{}/clans/{}", self.url, encoded_clan_tag);

        let res = self
            .client
            .unwrap()
            .get(path)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let clan = serde_json::from_str(&res).unwrap();

        return clan;
    }
}
