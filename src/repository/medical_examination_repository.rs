use crate::domain::medical_examination::{MedicalExamination, MedicalExaminationRepository};
use crate::utils::errors::MyError;

use chrono::{Local, TimeZone};
use serde_json::json;

use async_trait::async_trait;

use sqlx::MySqlPool;

pub struct MedicalExaminationRepositoryImpl<'a> {
    pub conn: &'a MySqlPool,
}

#[async_trait]
impl MedicalExaminationRepository for MedicalExaminationRepositoryImpl<'_> {
    async fn save(
        &self,
        user_id: &String,
        patient_code: &String,
        medical_examination: &MedicalExamination,
    ) -> Result<(), MyError> {
        sqlx::query!(
            "insert into medical_examinations(id,user_id,patient_code,interviewed_at,symptom)
            values(?,?,?,?,?)
            ",
            medical_examination.id,
            user_id,
            patient_code,
            medical_examination
                .interviewed_at
                .unwrap()
                .format("%Y/%m/%d %H:%M:%S")
                .to_string(),
            medical_examination.symptom,
        )
        .execute(self.conn)
        .await?;
        Ok(())
    }

    async fn fetch_by_patient_id(
        &self,
        patient_code: &String,
    ) -> Result<Vec<MedicalExamination>, MyError> {
        struct MedicalExaminationQuery {
            id: String,
            symptom: String,
            interviewed_at: String,
        }
        let records = sqlx::query!(
            "select id,symptom,interviewed_at from medical_examinations where patient_code=?",
            patient_code
        )
        .fetch_all(self.conn)
        .await?;
        let mut medical_examinations = vec![];
        for record in records {
            medical_examinations.push(MedicalExamination::from(
                record.id,
                record.symptom,
                Some(
                    Local
                        .datetime_from_str(&record.interviewed_at.to_string(), "%Y/%m/%d %H:%M:%S")
                        .unwrap(),
                ),
            )?)
        }

        Ok(medical_examinations)
    }

    async fn fetch_one(&self, id: &String) -> Result<MedicalExamination, MyError> {
        let record = sqlx::query!(
            "select id, interviewed_at,symptom 
            from medical_examinations
            where id=? 
            ",
            id
        )
        .fetch_optional(self.conn)
        .await?;
        if let Some(record) = record {
            let medical_examination = MedicalExamination::from(
                record.id,
                record.symptom,
                Some(
                    Local
                        .datetime_from_str(&record.interviewed_at.to_string(), "%Y/%m/%d %H:%M:%S")
                        .unwrap(),
                ),
            )?;
            return Ok(medical_examination);
        } else {
            return Err(MyError::BadRequest(json!({
                "error": format!("no record of id={}.", id)
            })));
        }
    }
}
