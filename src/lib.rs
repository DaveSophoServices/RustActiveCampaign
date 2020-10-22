mod active_client {
    use reqwest::blocking::Client;
    use reqwest::header;
    
    pub struct ACSession {
	client: Client,
	base: String,
	
    }

    pub fn new(ns: &str, token: &str) -> Result<ACSession, String> {
	let mut headers = header::HeaderMap::new();
	let val = match header::HeaderValue::from_str(token) {
	    Ok(x) => x,
	    Err(x) => return Err(format!("Invalid token: {}", x)),
	};
	headers.insert(
	    header::HeaderName::from_static("api-token"),
	    val);
	let client = match Client::builder()
	    .default_headers(headers)
	    .build() {
		Ok(x) => x,
		Err(x) => return Err(format!("Error building client: {}", x)),
	    };
	Ok(ACSession {
	    client,
	    base: format!("https://{}.api1-us1.com/api/3/", ns),
	})	    
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use serde::Deserialize;
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn create_new() {
	match active_client::new("t","123") {
	    Ok(_) => (),
	    Err(_) => panic!("no return value from new!")
	}
    }
    #[derive(Deserialize)]
    struct TestConfigValues {
	namespace: String,
	token: String,
    }

    fn load_config() -> TestConfigValues {
	let s = match fs::read_to_string("test-config.toml") {
	    Ok(x) => x,
	    Err(_) => panic!("Please create config file 'test-config.toml' with namespace and token for testing the API"),
	};
	return match toml::from_str(&s) {
	    Ok(x) => x,
	    Err(x) => panic!(
		r#"Contents of 'test-config.toml' should be 
namespace="..."
token="...""#);

	};
    }
    
    #[test]
    fn create_valid() {
	let c = load_config();
    }

}
