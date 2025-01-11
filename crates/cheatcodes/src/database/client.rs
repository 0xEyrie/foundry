use std::{
    collections::HashMap,
    fmt,
    mem::transmute,
    sync::{Arc, Mutex},
};

use postgres::{types::ToSql, Client, Row, Transaction};
use uuid::Uuid;

use super::error::Error;

struct TransactionWrapper(Transaction<'static>);
impl TransactionWrapper {
    fn new(transaction: Transaction<'_>) -> Self {
        // Safety: We're ensuring the transaction lives as long as needed through Arc<Mutex<>>
        let transaction =
            unsafe { transmute::<Transaction<'_>, Transaction<'static>>(transaction) };
        Self(transaction)
    }
}

pub struct PostgresClient {
    client: Arc<Mutex<Client>>,
    transactions: Arc<Mutex<HashMap<Uuid, TransactionWrapper>>>,
}

impl fmt::Debug for PostgresClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DbClient").field("client", &"Client").finish()
    }
}

impl Clone for PostgresClient {
    fn clone(&self) -> Self {
        PostgresClient {
            client: Arc::clone(&self.client),
            transactions: Arc::clone(&self.transactions),
        }
    }
}

impl DatabaseClient for PostgresClient {
    fn new(client: Client) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
            transactions: Arc::new(Mutex::new(HashMap::default())),
        }
    }

    fn build_transaction(&mut self) -> Result<Uuid, Error> {
        let mut client = self.client.lock().unwrap();
        let transaction = client.transaction()?;
        let transaction_id = Uuid::new_v4();
        self.transactions
            .lock()
            .unwrap()
            .insert(transaction_id, TransactionWrapper::new(transaction));
        Ok(transaction_id)
    }

    fn execute(&mut self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error> {
        let mut client = self.client.lock().unwrap();
        client.execute(query, params).map_err(Error::Postgres)
    }

    fn query(&mut self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error> {
        let mut client = self.client.lock().unwrap();
        client.query(query, params).map_err(Error::Postgres)
    }

    fn query_opt(
        &mut self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error> {
        let mut client = self.client.lock().unwrap();
        client.query_opt(query, params).map_err(Error::Postgres)
    }

    fn execute_in_transaction(
        &mut self,
        id: Uuid,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, Error> {
        let mut transactions = self.transactions.lock().unwrap();
        if let Some(transaction) = transactions.get_mut(&id) {
            return transaction.0.execute(query, params).map_err(Error::Postgres);
        }
        Err(Error::NotFound("Transaction not found".to_string()))
    }

    fn query_in_transaction(
        &mut self,
        id: Uuid,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, Error> {
        let mut transactions = self.transactions.lock().unwrap();
        if let Some(transaction) = transactions.get_mut(&id) {
            return transaction.0.query(query, params).map_err(Error::Postgres);
        }
        Err(Error::NotFound("Transaction not found".to_string()))
    }

    fn query_opt_in_transaction(
        &mut self,
        id: Uuid,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error> {
        let mut transactions = self.transactions.lock().unwrap();
        if let Some(transaction) = transactions.get_mut(&id) {
            return transaction.0.query_opt(query, params).map_err(Error::Postgres);
        }
        Err(Error::NotFound("Transaction not found".to_string()))
    }

    fn commit(&mut self, id: Uuid) -> Result<(), Error> {
        let mut transactions = self.transactions.lock().unwrap();
        if let Some(transaction) = transactions.remove(&id) {
            return transaction.0.commit().map_err(Error::Postgres);
        }
        Err(Error::NotFound("Transaction not found".to_string()))
    }

    fn commit_all_transcation(&mut self) -> Result<(), Error> {
        let mut transactions = self.transactions.lock().unwrap();
        for (_, transaction) in transactions.drain() {
            transaction.0.commit().map_err(Error::Postgres)?;
        }
        Ok(())
    }

    fn rollback(&mut self, id: Uuid) -> Result<(), Error> {
        let mut transactions = self.transactions.lock().unwrap();
        if let Some(transaction) = transactions.remove(&id) {
            return transaction.0.rollback().map_err(Error::Postgres);
        }
        Err(Error::NotFound("Transaction not found".to_string()))
    }

    fn rollback_all_transcation(&mut self) -> Result<(), Error> {
        let mut transactions = self.transactions.lock().unwrap();
        for (_, transaction) in transactions.drain() {
            transaction.0.rollback().map_err(Error::Postgres)?;
        }
        Ok(())
    }
}

pub trait DatabaseClient: fmt::Debug {
    fn new(client: Client) -> Self;
    fn build_transaction(&mut self) -> Result<Uuid, Error>;
    fn execute(&mut self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error>;
    fn query(&mut self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>;
    fn query_opt(
        &mut self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error>;
    fn execute_in_transaction(
        &mut self,
        id: Uuid,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, Error>;
    fn query_in_transaction(
        &mut self,
        id: Uuid,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, Error>;
    fn query_opt_in_transaction(
        &mut self,
        id: Uuid,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error>;
    fn commit(&mut self, id: Uuid) -> Result<(), Error>;
    fn commit_all_transcation(&mut self) -> Result<(), Error>;
    fn rollback(&mut self, id: Uuid) -> Result<(), Error>;
    fn rollback_all_transcation(&mut self) -> Result<(), Error>;
}
