use crate::client::user_center::dataobject::Login;
use crate::database::active::property::save_property;
use crate::domain::defines::get_client;

pub async fn login(username: String, password: String) -> anyhow::Result<LoginState> {
    let login = match get_client()
        .login(username.as_str(), password.as_str())
        .await
    {
        Ok(login) => login,
        Err(err) => {
            return Ok(LoginState {
                empty: false,
                error: true,
                error_msg: err.to_string(),
                success: false,
                login: Login::default(),
            });
        }
    };
    save_property("username".to_owned(), username).await?;
    save_property("password".to_owned(), password).await?;
    Ok(LoginState {
        empty: false,
        error: false,
        error_msg: "".to_owned(),
        success: true,
        login,
    })
}

pub struct LoginState {
    pub empty: bool,
    pub error: bool,
    pub error_msg: String,
    pub success: bool,
    pub login: Login,
}

pub async fn re_login() -> anyhow::Result<LoginState> {
    let username = crate::database::active::property::load_property("username".to_owned()).await?;
    let password = crate::database::active::property::load_property("password".to_owned()).await?;
    if username.is_empty() || password.is_empty() {
        return Ok(LoginState {
            empty: true,
            error: false,
            error_msg: "".to_owned(),
            success: false,
            login: Login::default(),
        });
    }
    let login = match get_client()
        .login(username.as_str(), password.as_str())
        .await
    {
        Ok(login) => login,
        Err(err) => {
            return Ok(LoginState {
                empty: false,
                error: true,
                error_msg: err.to_string(),
                success: false,
                login: Login::default(),
            });
        }
    };
    Ok(LoginState {
        empty: false,
        error: false,
        error_msg: "".to_owned(),
        success: true,
        login,
    })
}
