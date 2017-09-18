pub mod parser;

use iron::prelude::*;
use router::Router;
use bodyparser;

use serde::Deserialize;

use params;
use params::{Params, Value};

use exonum::crypto::Hash;
use exonum::blockchain::{Blockchain, Transaction, BlockProof, Schema as CoreSchema};
use exonum::node::TransactionSend;
use exonum::api::{Api, ApiError};
use exonum::storage::MapProof;
use exonum::crypto::{PublicKey, SecretKey};
use exonum::crypto::HexValue;

use exonum::storage::StorageValue;

use TIMESTAMPING_SERVICE;
use blockchain::schema::Schema;
use blockchain::dto::{Timestamp, TxTimestamp, TimestampEntry};
use api::parser::RequestParser;

#[derive(Debug, Serialize)]
pub struct TimestampProof {
    pub block_info: BlockProof,
    pub state_proof: MapProof<Hash>,
    pub timestamp_proof: MapProof<TimestampEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemsTemplate<T> {
    pub total_count: u64,
    pub items: Vec<T>,
}

#[derive(Clone)]
pub struct PublicApi<T: TransactionSend + Clone + 'static> {
    channel: T,
    blockchain: Blockchain,
    pub secret_key: SecretKey,
    pub public_key: PublicKey
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimestampData{
    pub content_hash: String,
    pub metadata: String
}

impl<T> PublicApi<T>
where
    T: TransactionSend + Clone + 'static,
{
    pub fn new(blockchain: Blockchain, channel: T, public_key: PublicKey, secret_key: SecretKey) -> PublicApi<T> {        
        PublicApi {
            blockchain: blockchain,
            channel: channel,
            secret_key: secret_key,
            public_key: public_key
        }
    }

    pub fn put_transaction<Tx: Transaction>(&self, tx: Tx) -> Result<Hash, ApiError> {
        let hash = tx.hash();
        self.channel.send(Box::new(tx))?;
        Ok(hash)
    }

    pub fn timestamp_proof(&self, content_hash: &Hash) -> Result<TimestampProof, ApiError> {
        let snap = self.blockchain.snapshot();
        let (state_proof, block_info) = {
            let core_schema = CoreSchema::new(&snap);

            let last_block_height = self.blockchain.last_block().height();
            let block_proof = core_schema.block_and_precommits(last_block_height).unwrap();
            let state_proof = core_schema.get_proof_to_service_table(TIMESTAMPING_SERVICE, 0);
            (state_proof, block_proof)
        };

        let schema = Schema::new(&snap);
        let timestamp_proof = schema.timestamps().get_proof(content_hash);

        Ok(TimestampProof {
            block_info,
            state_proof,
            timestamp_proof,
        })
    }

    pub fn timestamp(&self, content_hash: &Hash) -> Result<Option<TimestampEntry>, ApiError> {
        let snap = self.blockchain.snapshot();
        let schema = ::blockchain::schema::Schema::new(&snap);
        Ok(schema.timestamps().get(content_hash))
    }    

    fn make_post_request<Tx>(&self, router: &mut Router, endpoint: &str, name: &str)
    where
        Tx: Clone, for<'a> Tx: Deserialize<'a>,
    {
        let api = self.clone();
        let put_content = move |req: &mut Request| -> IronResult<Response> {            
            match req.get::<bodyparser::Struct<TimestampData>>() {
                Ok(Some(td)) => {             
                    let hash = Hash::from_hex(td.content_hash).unwrap();
                    let timestamp = Timestamp::new(&hash, &td.metadata.to_string());
                    let tx = TxTimestamp::new(&self.public_key, timestamp, &self.secret_key);
                    let hash = api.put_transaction(tx)?;
                    api.ok_response(&json!(hash))
                }
                Ok(None) => Err(ApiError::IncorrectRequest("Empty request body".into()))?,
                Err(e) => Err(ApiError::IncorrectRequest(Box::new(e)))?,
            }
        };
        router.post(endpoint, put_content, name);
    }
}

impl<T> Api for PublicApi<T>
where
    T: TransactionSend + Clone + 'static,
{
    fn wire(&self, router: &mut Router) {
        
        let api = self.clone();
        let get_timestamp_proof = move |req: &mut Request| -> IronResult<Response> {
            let parser = RequestParser::new(req);
            let content_hash = parser.route_param("content_hash")?;

            let proof = api.timestamp_proof(&content_hash)?;
            api.ok_response(&json!(proof))
        };

        let api = self.clone();
        let get_timestamp = move |req: &mut Request| -> IronResult<Response> {
            let parser = RequestParser::new(req);
            let content_hash = parser.route_param("content_hash")?;

            let timestamp = api.timestamp(&content_hash)?;
            api.ok_response(&json!(timestamp))
        };        

        self.make_post_request::<TimestampData>(router, "/v1/timestamps", "post_timestamp");
        
        router.get(
            "/v1/timestamps/value/:content_hash",
            get_timestamp,
            "get_timestamp",
        );
        router.get(
            "/v1/timestamps/proof/:content_hash",
            get_timestamp_proof,
            "get_timestamp_proof",
        );
    }
}