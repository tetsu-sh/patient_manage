use crate::{
    domain::patient::{Patient, PatientRepository},
    utils::errors::MyError,
};
use async_trait::async_trait;
use log::info;
use sqlx::MySqlPool;

pub struct PatientRepositoryImpl<'a> {
    pub conn: &'a MySqlPool,
}

#[async_trait]
impl PatientRepository for PatientRepositoryImpl<'_> {
    async fn save(&self, patient: &Patient) -> Result<(), MyError> {
        info!("patient save");
        sqlx::query!(
            "insert into patients(id,code,name)
            values(?,?,?)
            ",
            patient.id,
            patient.code,
            patient.name,
        )
        .execute(self.conn)
        .await?;

        Ok(())
    }

    async fn fetch_one(&self, id: &String) -> Result<Patient, MyError> {
        let record = sqlx::query!("select id, code,name from patients where id=?", id)
            .fetch_one(self.conn)
            .await?;
        Ok(Patient::from(record.id, record.code, record.name))
    }

    async fn fetch_by_code(&self, code: &String) -> Result<Patient, MyError> {
        let record = sqlx::query!("select id, code,name from patients where code=?", code)
            .fetch_one(self.conn)
            .await?;
        Ok(Patient::from(record.id, record.code, record.name))
    }

    async fn fetch_all(&self) -> Result<Vec<Patient>, MyError> {
        let records = sqlx::query!(
            "select id,code,name
            from patients"
        )
        .fetch_all(self.conn)
        .await?
        .into_iter()
        .map(|record| Patient::from(record.id, record.code, record.name))
        .collect::<Vec<Patient>>();
        Ok(records)
    }
}
