use rocket::http::Header;

pub struct TokenHeader {
	token: String
}

impl TokenHeader {
	pub fn new(token: String) -> Self {
		TokenHeader {
			token: token
		}
	}
}

impl<'r> Into<Header<'r>> for TokenHeader {
	fn into(self) -> Header<'r> {
		Header::new("token", self.token)
	}
}