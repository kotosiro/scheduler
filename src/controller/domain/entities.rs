pub mod job;
pub mod project;
pub mod run;
pub mod token;
pub mod workflow;

#[macro_export]
macro_rules! impl_bool_property {
    ( $type:tt ) => {
        impl $type {
            pub fn new(value: bool) -> Self {
                Self { value }
            }

            pub fn as_bool(&self) -> &bool {
                &self.value
            }

            pub fn to_bool(&self) -> bool {
                self.value
            }
        }

        impl sqlx::types::Type<sqlx::postgres::Postgres> for $type {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <bool as sqlx::types::Type<sqlx::postgres::Postgres>>::type_info()
            }

            fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
                <bool as sqlx::types::Type<sqlx::postgres::Postgres>>::compatible(ty)
            }
        }

        impl sqlx::postgres::PgHasArrayType for $type {
            fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                <bool as sqlx::postgres::PgHasArrayType>::array_type_info()
            }

            fn array_compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
                <bool as sqlx::postgres::PgHasArrayType>::array_compatible(ty)
            }
        }

        impl sqlx::encode::Encode<'_, sqlx::postgres::Postgres> for $type {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> sqlx::encode::IsNull {
                <bool as sqlx::encode::Encode<sqlx::postgres::Postgres>>::encode_by_ref(
                    &self.value,
                    buf,
                )
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

            pub fn as_i32(&self) -> &i32 {
                &self.value
            }

            pub fn to_i32(&self) -> i32 {
                self.value
            }
        }
    };
}

#[macro_export]
macro_rules! impl_i64_property {
    ( $type:tt ) => {
        impl $type {
            pub fn new(value: i64) -> anyhow::Result<Self> {
                let object = Self { value };
                object.validate()?;
                Ok(object)
            }

            pub fn as_i64(&self) -> &i64 {
                &self.value
            }

            pub fn to_i64(&self) -> i64 {
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
                &self.value.as_str()
            }

            pub fn to_string(&self) -> String {
                self.value.as_str().to_string()
            }
        }

        impl sqlx::types::Type<sqlx::postgres::Postgres> for $type {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <String as sqlx::types::Type<sqlx::postgres::Postgres>>::type_info()
            }

            fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
                <String as sqlx::types::Type<sqlx::postgres::Postgres>>::compatible(ty)
            }
        }

        impl sqlx::postgres::PgHasArrayType for $type {
            fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                <String as sqlx::postgres::PgHasArrayType>::array_type_info()
            }

            fn array_compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
                <String as sqlx::postgres::PgHasArrayType>::array_compatible(ty)
            }
        }

        impl sqlx::encode::Encode<'_, sqlx::postgres::Postgres> for $type {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> sqlx::encode::IsNull {
                <String as sqlx::encode::Encode<sqlx::postgres::Postgres>>::encode_by_ref(
                    &self.value,
                    buf,
                )
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

            pub fn as_uuid(&self) -> &uuid::Uuid {
                &self.value
            }

            pub fn to_uuid(&self) -> uuid::Uuid {
                self.value.clone()
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

        impl sqlx::types::Type<sqlx::postgres::Postgres> for $type {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <uuid::Uuid as sqlx::types::Type<sqlx::postgres::Postgres>>::type_info()
            }

            fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
                <uuid::Uuid as sqlx::types::Type<sqlx::postgres::Postgres>>::compatible(ty)
            }
        }

        impl sqlx::postgres::PgHasArrayType for $type {
            fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                <uuid::Uuid as sqlx::postgres::PgHasArrayType>::array_type_info()
            }

            fn array_compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
                <uuid::Uuid as sqlx::postgres::PgHasArrayType>::array_compatible(ty)
            }
        }

        impl sqlx::encode::Encode<'_, sqlx::postgres::Postgres> for $type {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> sqlx::encode::IsNull {
                <uuid::Uuid as sqlx::encode::Encode<sqlx::postgres::Postgres>>::encode_by_ref(
                    &self.value,
                    buf,
                )
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

            pub fn as_json(&self) -> &serde_json::Value {
                &self.value
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
