use alloy_primitives::{Bytes, B128, B64};
use alloy_sol_types::SolValue;
use postgres::{types::ToSql, Client, NoTls};
use spec::Vm::{
    commitTransactionCall, commitTransactionsCall, connectDbCall, executeCall,
    executeInTransactionCall, openTransactionCall, queryCall, queryInTransactionCall, queryOptCall,
    queryOptInTransactionCall, rollbackTransactionCall, rollbackTransactionsCall,
    PostgresDb as VmSQLDatabse,
};
use uuid::Uuid;

use crate::{Cheatcode, Cheatcodes, Result};

mod client;
mod error;
pub use client::{DatabaseClient, PostgresClient};
use error::Error;

#[derive(Debug, Clone)]
pub(crate) struct SQLDatabase {
    pub host: String,
    pub port: u64,
    pub user: String,
    pub password: String,
    pub name: String,
}

trait ToParam {
    fn with_params<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&[&(dyn ToSql + Sync)]) -> R;
}

impl ToParam for Vec<Bytes> {
    #[inline]
    fn with_params<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&[&(dyn ToSql + Sync)]) -> R,
    {
        let owned_params: Vec<Vec<u8>> = self.iter().map(|p| p.to_vec()).collect();

        let param_refs: Vec<&(dyn ToSql + Sync)> =
            owned_params.iter().map(|p| p as &(dyn ToSql + Sync)).collect();

        f(param_refs.as_slice())
    }
}

impl SQLDatabase {
    pub fn new<DBClient: DatabaseClient>(&self) -> Result<DBClient, Error> {
        let conn_str = format!(
            "host={} port={} user={} password={} dbname={}",
            self.host, self.port, self.user, self.password, self.name
        );
        let client = Client::connect(&conn_str, NoTls)?;
        Ok(DBClient::new(client))
    }
}

impl From<VmSQLDatabse> for SQLDatabase {
    fn from(vmsqldb: VmSQLDatabse) -> Self {
        SQLDatabase {
            host: vmsqldb.host,
            port: vmsqldb.port.to(),
            user: vmsqldb.user,
            password: vmsqldb.password,
            name: vmsqldb.name,
        }
    }
}

impl Cheatcode for connectDbCall {
    fn apply(&self, state: &mut Cheatcodes) -> Result {
        let Self { sqldb: db } = self;
        let sql_db = SQLDatabase::from(db.clone());
        let client = sql_db.new().unwrap();
        state.set_db_client(client);
        Ok(Default::default())
    }
}

impl Cheatcode for openTransactionCall {
    fn apply(&self, state: &mut Cheatcodes) -> Result {
        let db_client = state.db_client.as_mut().unwrap();
        let id = db_client.build_transaction().unwrap();
        Ok(B128::from(id.to_u128_le()).abi_encode())
    }
}

impl Cheatcode for commitTransactionCall {
    fn apply(&self, state: &mut Cheatcodes) -> Result {
        let Self { transactionId } = self;
        let db_client = state.db_client.as_mut().unwrap();
        let uuid = Uuid::from_u128_le(*transactionId);
        db_client.commit(uuid).unwrap();
        Ok(Default::default())
    }
}

impl Cheatcode for commitTransactionsCall {
    fn apply(&self, state: &mut Cheatcodes) -> Result {
        let db_client = state.db_client.as_mut().unwrap();
        db_client.commit_all_transcation().unwrap();
        Ok(Default::default())
    }
}

impl Cheatcode for rollbackTransactionCall {
    fn apply(&self, state: &mut Cheatcodes) -> Result {
        let Self { transactionId } = self;
        let db_client = state.db_client.as_mut().unwrap();
        let uuid = Uuid::from_u128_le(*transactionId);
        db_client.rollback(uuid).unwrap();
        Ok(Default::default())
    }
}

impl Cheatcode for rollbackTransactionsCall {
    fn apply(&self, state: &mut Cheatcodes) -> Result {
        let db_client = state.db_client.as_mut().unwrap();
        db_client.rollback_all_transcation().unwrap();
        Ok(Default::default())
    }
}

impl Cheatcode for executeCall {
    fn apply(&self, state: &mut Cheatcodes) -> Result {
        let Self { query, params } = self;
        let db_client = state.db_client.as_mut().unwrap();
        let rows_updated =
            params.with_params(|param_refs| db_client.execute(query, param_refs).unwrap());

        Ok(B64::from(rows_updated).abi_encode())
    }
}

impl Cheatcode for executeInTransactionCall {
    fn apply(&self, state: &mut Cheatcodes) -> Result {
        let Self { transactionId, query, params } = self;
        let db_client = state.db_client.as_mut().unwrap();
        let uuid = Uuid::from_u128_le(*transactionId);
        let rows_updated = params.with_params(|param_refs| {
            db_client.execute_in_transaction(uuid, query, param_refs).unwrap()
        });
        Ok(B64::from(rows_updated).abi_encode())
    }
}

impl Cheatcode for queryOptCall {
    fn apply(&self, state: &mut Cheatcodes) -> Result {
        let Self { query, params, selects } = self;
        let db_client = state.db_client.as_mut().unwrap();
        let row = params.with_params(|param_refs| db_client.query_opt(query, param_refs).unwrap());
        if let Some(r) = row {
            let results: Vec<Vec<u8>> = selects.iter().map(|s| r.get(s.as_str())).collect();
            return Ok(results.abi_encode());
        }
        Ok(Default::default())
    }
}

impl Cheatcode for queryCall {
    fn apply(&self, state: &mut Cheatcodes) -> Result {
        let Self { query, params, selects } = self;
        let db_client = state.db_client.as_mut().unwrap();
        let rows = params.with_params(|param_refs| db_client.query(query, param_refs).unwrap());
        let results: Vec<Vec<Vec<u8>>> =
            rows.iter().map(|r| selects.iter().map(|s| r.get(s.as_str())).collect()).collect();
        return Ok(results.abi_encode());
    }
}

impl Cheatcode for queryOptInTransactionCall {
    fn apply(&self, state: &mut Cheatcodes) -> Result {
        let Self { transactionId, query, params, selects } = self;
        let db_client = state.db_client.as_mut().unwrap();
        let uuid = Uuid::from_u128_le(*transactionId);
        let row = params.with_params(|param_refs| {
            db_client.query_opt_in_transaction(uuid, query, param_refs).unwrap()
        });
        if let Some(r) = row {
            let results: Vec<Vec<u8>> = selects.iter().map(|s| r.get(s.as_str())).collect();
            return Ok(results.abi_encode());
        }
        Ok(Default::default())
    }
}

impl Cheatcode for queryInTransactionCall {
    fn apply(&self, state: &mut Cheatcodes) -> Result {
        let Self { transactionId, query, params, selects } = self;
        let db_client = state.db_client.as_mut().unwrap();
        let uuid = Uuid::from_u128_le(*transactionId);
        let rows = params.with_params(|param_refs| {
            db_client.query_in_transaction(uuid, query, param_refs).unwrap()
        });
        let results: Vec<Vec<Vec<u8>>> =
            rows.iter().map(|r| selects.iter().map(|s| r.get(s.as_str())).collect()).collect();
        return Ok(results.abi_encode());
    }
}
