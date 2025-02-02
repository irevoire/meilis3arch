use std::time::Duration;

use rusty_s3::{Credentials, UrlStyle};
use url::Url;

use crate::{Bucket, Client, Result};

pub struct MissingCred;
pub struct MissingSecret(String);
pub struct MissingKey(String);
pub struct Complete {
    key: String,
    secret: String,
}

pub struct Builder<State> {
    addr: Url,
    region: Option<String>,
    cred: State,
    url_style: Option<UrlStyle>,
    token: Option<String>,
    actions_expires_in: Option<Duration>,
    timeout: Option<Duration>,
    multipart_size: Option<usize>,
}

impl Builder<MissingCred> {
    /// Create a new `Builder`.
    /// It's currently missing its key and secret.
    ///
    /// # Example
    /// ```
    /// use strois::Builder;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .key("minioadmin")
    ///     .secret("minioadmin")
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    ///
    /// If you try to call `.client()` before setting the key and secret it won't work.
    /// ```compile_fail
    /// use strois::Builder;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    ///
    /// But if you only forgot the secret it should panic as well:
    /// ```compile_fail
    /// use strois::Builder;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .secret("minioadmin")
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    ///
    /// Same for the key:
    /// ```compile_fail
    /// use strois::Builder;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .key("minioadmin")
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    ///
    pub fn new(addr: impl AsRef<str>) -> Result<Self> {
        Ok(Self {
            addr: addr.as_ref().parse()?,
            region: None,
            cred: MissingCred,
            url_style: None,
            token: None,
            actions_expires_in: None,
            timeout: None,
            multipart_size: None,
        })
    }

    /// Create a new `Builder` based on an AWS region
    /// It's currently missing its key and secret.
    ///
    /// # Example
    /// ```
    /// use strois::Builder;
    ///
    /// let client = Builder::new_region(awsregion::Region::UsEast1)
    ///     .key("minioadmin")
    ///     .secret("minioadmin")
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    ///
    /// If you try to call `.client()` before setting the key and secret it won't work.
    /// ```compile_fail
    /// use strois::Builder;
    ///
    /// let client = Builder::new_region(awsregion::Region::UsEast1)
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    ///
    /// But if you only forgot the secret it should panic as well:
    /// ```compile_fail
    /// use strois::Builder;
    ///
    /// let client = Builder::new_region(awsregion::Region::UsEast1)
    ///     .secret("minioadmin")
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    ///
    /// Same for the key:
    /// ```compile_fail
    /// use strois::Builder;
    ///
    /// let client = Builder::new_region(awsregion::Region::UsEast1)
    ///     .key("minioadmin")
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    ///
    #[cfg(feature="aws-region")]
    pub fn new_region(region: awsregion::Region) -> Self {
        Self {
            addr: format!("{}://{}", region.scheme(), region.endpoint()).parse().unwrap(),
            region: Some(region.to_string()),
            cred: MissingCred,
            url_style: None,
            token: None,
            actions_expires_in: None,
            timeout: None,
            multipart_size: None,
        }
    }

    /// Set the key in the `Builder`.
    ///
    /// # Example
    /// ```
    /// use strois::Builder;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .key("minioadmin")
    ///     .secret("minioadmin")
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    pub fn key(self, key: impl Into<String>) -> Builder<MissingSecret> {
        Builder {
            addr: self.addr,
            region: self.region,
            cred: MissingSecret(key.into()),
            url_style: None,
            token: self.token,
            actions_expires_in: self.actions_expires_in,
            timeout: self.timeout,
            multipart_size: None,
        }
    }

    /// Set the secret in the `Builder`.
    ///
    /// # Example
    /// ```
    /// use strois::Builder;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .secret("minioadmin")
    ///     .key("minioadmin")
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    pub fn secret(self, secret: impl Into<String>) -> Builder<MissingKey> {
        Builder {
            addr: self.addr,
            region: self.region,
            cred: MissingKey(secret.into()),
            url_style: None,
            token: self.token,
            actions_expires_in: self.actions_expires_in,
            timeout: self.timeout,
            multipart_size: None,
        }
    }
}

impl Builder<MissingSecret> {
    /// Set the secret in the `Builder`.
    ///
    /// # Example
    /// ```
    /// use strois::Builder;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .key("minioadmin")
    ///     .secret("minioadmin")
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    pub fn secret(self, secret: impl Into<String>) -> Builder<Complete> {
        Builder {
            addr: self.addr,
            region: self.region,
            cred: Complete {
                key: self.cred.0,
                secret: secret.into(),
            },
            url_style: None,
            token: self.token,
            actions_expires_in: self.actions_expires_in,
            timeout: self.timeout,
            multipart_size: None,
        }
    }
}

impl Builder<MissingKey> {
    /// Set the key in the `Builder`.
    ///
    /// # Example
    /// ```
    /// use strois::Builder;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .secret("minioadmin")
    ///     .key("minioadmin")
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    pub fn key(self, key: impl Into<String>) -> Builder<Complete> {
        Builder {
            addr: self.addr,
            region: self.region,
            cred: Complete {
                key: key.into(),
                secret: self.cred.0,
            },
            url_style: None,
            token: self.token,
            actions_expires_in: self.actions_expires_in,
            timeout: self.timeout,
            multipart_size: None,
        }
    }
}

impl Builder<Complete> {
    /// Create a new [`Client`] from the builder.
    ///
    /// # Example
    /// ```
    /// use strois::Builder;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .key("minioadmin")
    ///     .secret("minioadmin")
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    pub fn client(self) -> Client {
        let cred = if let Some(token) = self.token {
            Credentials::new_with_token(self.cred.key, self.cred.secret, token)
        } else {
            Credentials::new(self.cred.key, self.cred.secret)
        };

        Client {
            addr: self.addr,
            region: self.region.unwrap_or_default(),
            cred,
            url_style: self.url_style.unwrap_or(UrlStyle::VirtualHost),
            actions_expires_in: self
                .actions_expires_in
                .unwrap_or(Duration::from_secs(60 * 60)),
            timeout: self.timeout.unwrap_or(Duration::from_secs(60)),
            multipart_size: self.multipart_size.unwrap_or(50 * 1024 * 1024), // 50MiB
        }
    }

    /// Create a new [`Bucket`] from the builder.
    ///
    /// # Example
    /// ```
    /// use strois::Builder;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .key("minioadmin")
    ///     .secret("minioadmin")
    ///     .bucket("tamo");
    /// # Ok::<(), strois::Error>(())
    /// ```
    pub fn bucket(self, name: impl AsRef<str>) -> Result<Bucket> {
        self.client().bucket(name.as_ref())
    }
}

impl<T> Builder<T> {
    /// Enable the url path style.
    ///
    /// # Example
    /// ```
    /// use strois::Builder;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .key("minioadmin")
    ///     .secret("minioadmin")
    ///     .with_url_path_style(true)
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    pub fn with_url_path_style(mut self, path_style: bool) -> Self {
        if path_style {
            self.url_style = Some(UrlStyle::Path);
        }
        self
    }

    /// Set the token.
    ///
    /// # Example
    /// ```
    /// use strois::Builder;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .key("minioadmin")
    ///     .secret("minioadmin")
    ///     .token("tamo")
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Set the region.
    ///
    /// # Example
    /// ```
    /// use strois::Builder;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .key("minioadmin")
    ///     .secret("minioadmin")
    ///     .region("EU-west")
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// Set the size for the parts of the multipart upload in bytes.
    /// By default it's set to 50MiB.
    /// For aws, the value must be contained between 5MiB and 5GiB.
    ///
    /// # Example
    /// ```
    /// use strois::Builder;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .key("minioadmin")
    ///     .secret("minioadmin")
    ///     .multipart_size(5 * 1024 * 1024) // 5MiB
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    pub fn multipart_size(mut self, multipart_size: usize) -> Self {
        self.multipart_size = Some(multipart_size);
        self
    }

    /// Set the token if you have one.
    /// If `None` the token is set to `None` again.
    ///
    /// # Example
    /// ```
    /// use strois::Builder;
    ///
    /// let token = Some("tamo");
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .key("minioadmin")
    ///     .secret("minioadmin")
    ///     .maybe_token(token)
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    pub fn maybe_token(mut self, token: Option<impl Into<String>>) -> Self {
        self.token = token.map(|s| s.into());
        self
    }

    /// Set the time before an action expires.
    /// One hour by default.
    ///
    /// # Example
    /// ```
    /// use strois::Builder;
    /// use std::time::Duration;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .key("minioadmin")
    ///     .secret("minioadmin")
    ///     .actions_expires_in(Duration::from_secs(60))
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    pub fn actions_expires_in(mut self, actions_expires_in: Duration) -> Self {
        self.actions_expires_in = Some(actions_expires_in);
        self
    }

    /// Set the timeout of the http requests.
    /// One minute by default.
    ///
    /// # Example
    /// ```
    /// use strois::Builder;
    /// use std::time::Duration;
    ///
    /// let client = Builder::new("http://localhost:9000")?
    ///     .key("minioadmin")
    ///     .secret("minioadmin")
    ///     .http_timeout(Duration::from_secs(60 * 60))
    ///     .client();
    /// # Ok::<(), strois::Error>(())
    /// ```
    pub fn http_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}
