#[macro_export]
macro_rules! impl_string {
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
