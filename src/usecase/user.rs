use crate::domain::user::DoctorInChargeRepository;
use crate::middleware::authn::make_jwt;
use crate::utils::hash::{hash_password, verify};
use crate::utils::password::verify_user_password;

use crate::{
    domain::user::{User, UserRepository},
    utils::errors::MyError,
};

pub struct UserUsecase<U: UserRepository, D: DoctorInChargeRepository> {
    pub user_repository: U,
    pub doctor_in_charge_repository: D,
}

impl<U: UserRepository, D: DoctorInChargeRepository> UserUsecase<U, D> {
    pub fn new(user_repository: U, doctor_in_charge_repository: D) -> Self {
        Self {
            user_repository,
            doctor_in_charge_repository,
        }
    }

    pub async fn sign_up(
        &self,
        name: String,
        code: Option<String>,
        raw_password: String,
    ) -> Result<(User, String), MyError> {
        verify_user_password(&raw_password);
        let hashed_password = hash_password(&raw_password)?;
        let user = User::new(name, code, hashed_password)?;
        self.user_repository.save(&user).await?;
        let token = make_jwt(&user.id)?;

        Ok((user, token))
    }

    pub async fn sign_in(&self, name: String, raw_password: String) -> Result<String, MyError> {
        let user = self.user_repository.find_by_name(&name).await?;
        let _ = verify(&raw_password, &user.hashed_password);
        let token = make_jwt(&user.id)?;
        Ok(token)
    }

    pub async fn assign(&self, user_id: String, patient_code: String) -> Result<(), MyError> {
        self.doctor_in_charge_repository
            .save(&user_id, &patient_code)
            .await?;
        Ok(())
    }

    pub async fn fetch(&self, id: &String) -> Result<User, MyError> {
        let user = self.user_repository.fetch_one(id).await?;
        Ok(user)
    }
}

#[cfg(test)]

mod tests {
    use crate::repository::user_repository::{
        DoctorInChargeRepositoryMockImpl, UserRepositoryMockImpl,
    };

    use super::*;

    #[tokio::test]
    async fn test_sign_up() {
        let name = "test_name".to_string();
        let code = "test_code".to_string();
        let raw_password = "test_password".to_string();
        let mock_user_repository = UserRepositoryMockImpl {};
        let mock_doctor_in_charge_repository = DoctorInChargeRepositoryMockImpl {};
        let user_usecase = UserUsecase {
            user_repository: mock_user_repository,
            doctor_in_charge_repository: mock_doctor_in_charge_repository,
        };
        let (user, token) = user_usecase
            .sign_up(name, Some(code), raw_password)
            .await
            .unwrap();
    }
}
