use serde_json::{Value, to_value};

use exonum::storage::Fork;
use exonum::blockchain::Transaction;
use exonum::messages::Message;

use blockchain::dto::{TxTimestamp, TimestampEntry};
use blockchain::schema::Schema;


impl Transaction for TxTimestamp {

    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, view: &mut Fork) {
        let mut schema = Schema::new(view);        
        let entry = TimestampEntry::new(self.content(), &self.hash());
        schema.add_timestamp(entry);       
        trace!("Timestamp added: {:?}", self); 
    }

    fn info(&self) -> Value {
        to_value(self).unwrap()
    }
}