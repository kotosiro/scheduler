#[macro_export]
macro_rules! impl_string {
    ( $type:tt ) => {
        impl $type {
            pub fn new(value: String) -> anyhow::Result<Self> {
                let object = Self { value: value };
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
