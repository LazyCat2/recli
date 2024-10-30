use rive::prelude::Authentication;

pub fn auth() -> Authentication {
    Authentication::SessionToken(
		std::fs::read_to_string("./token")
			.expect("Check if your current pwd has the token file")
    )
}

//Not sure what return type must be
/*
pub async fn auth() -> &'static mut Result<Rive, std::error::Error> {
    let auth = Authentication::SessionToken(
		std::fs::read_to_string("./token")
			.expect("Check if your current pwd has the token file")
	);
    Rive::new(auth).await
}
*/