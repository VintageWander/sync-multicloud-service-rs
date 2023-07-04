use mongodb::{bson::doc, Collection, Cursor};

use crate::{error::Error, models::proxy::Proxy};

#[derive(Clone)]
pub struct ProxyService {
    collection: Collection<Proxy>,
}

impl ProxyService {
    pub fn init(collection: &Collection<Proxy>) -> Self {
        Self {
            collection: collection.clone(),
        }
    }

    pub async fn get_proxies(&self) -> Result<Cursor<Proxy>, Error> {
        let proxies = self.collection.find(None, None).await?;
        Ok(proxies)
    }

    pub async fn add_proxy(&self, url: &str) -> Result<Proxy, Error> {
        // Find if the proxy already exists
        let exists_proxy = self
            .collection
            .count_documents(doc! {"url": url}, None)
            .await?
            > 0;

        if exists_proxy {
            return Err(Error::ProxyAlreadyExists);
        }

        // Add the proxy into the database and gets its ID
        let new_proxy_id = self
            .collection
            .insert_one(Proxy { url: url.into() }, None)
            .await?
            .inserted_id
            .as_object_id()
            .ok_or_else(|| Error::Generic)?;

        // Get the new proxy
        let new_proxy = self
            .collection
            .find_one(doc! {"_id": new_proxy_id}, None)
            .await?
            .ok_or_else(|| Error::CannotCreateProxy)?;

        Ok(new_proxy)
    }

    pub async fn delete_proxy(&self, url: &str) -> Result<(), Error> {
        // Just delete the proxy
        self.collection.delete_one(doc! {"url": url}, None).await?;
        Ok(())
    }
}
