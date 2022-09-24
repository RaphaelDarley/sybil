use std::collections::BTreeMap;

use surrealdb::{sql::Value, Datastore, Session};
use tokio::sync::Mutex;

pub struct DBConfig {
    path: String,
    ns: String,
    db: String,
}

pub struct DSConn {
    ds: Mutex<Datastore>,
    ses: Session,
}

impl DSConn {
    pub async fn execute(
        &self,
        txt: &str,
        vars: Option<BTreeMap<String, Value>>,
        strict: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.ds
            .lock()
            .await
            .execute(txt, &self.ses, vars, strict)
            .await?;

        Ok(())
    }
}

pub async fn make_or_load_ds_and_sess(path: &str) -> Result<DSConn, Box<dyn std::error::Error>> {
    let ds = Datastore::new(path).await?;
    let ses = Session::for_kv().with_ns("test").with_db("test");

    // when using schemaful table, initiate here

    Ok(DSConn {
        ds: Mutex::new(ds),
        ses,
    })
}

// async fn database_init()

pub async fn add_visit_record(dsconn: &DSConn) -> Result<(), Box<dyn std::error::Error>> {
    dsconn
        .ds
        .lock()
        .await
        .execute(
            "CREATE visit SET when = time::now()",
            &dsconn.ses,
            None,
            false,
        )
        .await?;

    Ok(())
}

pub async fn db_visits_as_json(dsconn: &DSConn) -> Result<String, Box<dyn std::error::Error>> {
    let ds = dsconn.ds.lock().await;
    let ses = &dsconn.ses;
    let select_response = ds.execute("SELECT * FROM visit;", ses, None, false).await?;

    let select_result = select_response[0].output().unwrap();

    if let Value::Array(rows) = select_result {
        Ok(serde_json::to_string(rows)?)
    } else {
        panic!("DB vists hasn't returned array of rows")
    }
}

pub async fn print_db_visits(dsconn: &DSConn) -> Result<(), Box<dyn std::error::Error>> {
    let ds = dsconn.ds.lock().await;
    let ses = &dsconn.ses;
    let select_response = ds.execute("SELECT * FROM visit;", ses, None, false).await?;

    let select_result = select_response[0].output().unwrap();

    if let Value::Array(rows) = select_result {
        for row in rows.iter() {
            if let Value::Object(obj) = row {
                println!("{:?}", obj)
            }
        }
    }
    Ok(())
}
