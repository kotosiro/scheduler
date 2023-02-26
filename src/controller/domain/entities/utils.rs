#[macro_export]
macro_rules! impl_bool_property {
    ( $type:tt ) => {
        impl $type {
            pub fn new(value: bool) -> Self {
                Self { value }
            }

            pub fn to_bool(&self) -> bool {
                self.value
            }
        }
    };
}

#[macro_export]
macro_rules! impl_i32_property {
    ( $type:tt ) => {
        impl $type {
            pub fn new(value: i32) -> anyhow::Result<Self> {
                let object = Self { value };
                object.validate()?;
                Ok(object)
            }

            pub fn to_i32(&self) -> i32 {
                self.value
            }
        }
    };
}

#[macro_export]
macro_rules! impl_string_property {
    ( $type:tt ) => {
        impl $type {
            pub fn new<S>(value: S) -> anyhow::Result<Self>
            where
                S: Into<String>,
            {
                let object = Self {
                    value: value.into(),
                };
                object.validate()?;
                Ok(object)
            }

            pub fn as_str(&self) -> &str {
                &self.value
            }

            pub fn into_string(self) -> String {
                self.value
            }
        }
    };
}

#[macro_export]
macro_rules! impl_uuid_property {
    ( $type:tt ) => {
        impl $type {
            pub fn new(value: uuid::Uuid) -> Self {
                Self { value }
            }

            pub fn to_uuid(&self) -> uuid::Uuid {
                self.value
            }
        }

        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value.hyphenated())
            }
        }

        impl TryFrom<&str> for $type {
            type Error = anyhow::Error;

            fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
                let value = uuid::Uuid::parse_str(value)?;
                Ok(Self { value })
            }
        }

        impl TryFrom<String> for $type {
            type Error = anyhow::Error;

            fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
                let value = uuid::Uuid::parse_str(value.as_str())?;
                Ok(Self { value })
            }
        }
    };
}

#[macro_export]
macro_rules! impl_json_property {
    ( $type:tt ) => {
        impl $type {
            pub fn new(value: serde_json::Value) -> Self {
                Self { value }
            }

            pub fn to_json(&self) -> serde_json::Value {
                self.value.clone()
            }
        }

        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value.to_string())
            }
        }

        impl TryFrom<&str> for $type {
            type Error = anyhow::Error;

            fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
                let value = serde_json::from_str(value)?;
                Ok(Self { value })
            }
        }

        impl TryFrom<String> for $type {
            type Error = anyhow::Error;

            fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
                let value = serde_json::from_str(value.as_str())?;
                Ok(Self { value })
            }
        }
    };
}
