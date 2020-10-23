mod campaign;
mod user;

mod active_client {
    use reqwest::blocking::Client;
    use reqwest::{header,StatusCode};
    use serde::Deserialize;

    use crate::campaign::{Campaign,Campaigns};
    use crate::user::{User,Users};
    
    pub struct ACSession {
	client: Client,
	base: String,
    }

    #[derive(Deserialize,Debug)]
    pub struct Meta {
	pub total: String,
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
	    base: format!("https://{}.api-us1.com/api/3", ns),
	})	    
    }
    impl ACSession {
	pub fn list_all_users(&self) -> Result<Users,reqwest::Error> {
	    let r = self.get("users")?;
	    let v:Users = match r.json() {
		Ok(x) => x,
		Err(_) => panic!("Invalid response from server"),
	    };
	    Ok(v)
	}
	pub fn list_all_campaigns(&self) -> Result<Campaigns,reqwest::Error> {
	    let v:Campaigns = match self.get("campaigns")?.json() {
		Ok(x) => x,
		Err(x) => panic!("Failed to decode JSON from server: {}",x),
	    };
	    Ok(v)
	}

	fn get(&self, ext:&str) -> Result<reqwest::blocking::Response, reqwest::Error> {
	    let u = format!("{}/{}", self.base, ext);
	    let r = self.client.get(&u).send()?;
	    match r.status() {
		StatusCode::OK => Ok(r),
		_ => panic!("{}: Error calling get on url '{}'", r.status(), r.url())
	    }		    
	}
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
token="...""#),

	};
    }
    
    #[test]
    fn create_valid() {
	let c = load_config();
    }

    #[test]
    fn list_all_users() {
	let c = load_config();
	let ac = match active_client::new(&c.namespace,&c.token) {
	    Ok(x) => x,
	    Err(_) => panic!("no return value from new!"),
	};
	
	let user_list = ac.list_all_users().unwrap();
	assert_eq!(user_list.users.len(), 7);
    }

    #[test]
    fn list_all_campaigns() {
	let c = load_config();
	match active_client::new(&c.namespace,&c.token) {
	    Ok(ac) => {
		let campaign_list = ac.list_all_campaigns().unwrap();
		assert!(campaign_list.campaigns.len() > 0);
	    }
	    Err(_) => panic!("no return value from new"),
	};
    }
}
