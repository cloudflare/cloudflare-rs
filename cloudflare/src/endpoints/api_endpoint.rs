#[macro_export]
macro_rules! api_results {
    ($name:ident {
        $(
            $(#[$field_meta:meta])*
            $field:ident : $ty:ty
        ),* $(,)?
        }) => {
        paste! {
            #[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
            pub struct [<$name Results>] {
                $(
                    $(#[$field_meta])*
                    pub $field: Option<$ty>,
                )*
            }
        }
    };
}
#[macro_export]
macro_rules! api_gen {
    ($st:ty) => {
        impl ApiResult for $st {}
    };
}
#[macro_export]
macro_rules! api_endpoint {
    // Pattern 1: With params (explicit method and params marker)
    ($(#[$docs:meta])*
        $method:ident, $name:ident => $response:ty , $path:expr; $($field:ident),+; params: $params:ty) => {
        $(#[$docs])*
        #[derive(Debug)]
        pub struct $name<'a> {
            $(pub $field: &'a str,)+
            pub params: $params,
        }

        impl<'a> EndpointSpec for $name<'a> {
            type JsonResponse = $response;
            type ResponseType = ApiSuccess<Self::JsonResponse>;

            fn method(&self) -> Method {
                Method::$method
            }

            fn path(&self) -> String {
                format!($path, $(self.$field),+)
            }

            #[inline]
            fn body(&self) -> Option<RequestBody> {
                Some(RequestBody::Json(
                    serde_json::to_string(&self.params)
                        .expect("Failed to serialize request body")
                ))
            }
        }
    };

    // Pattern 2: Without params (explicit method) and return type
    ($(#[$docs:meta])* $method:ident, $name:ident => $response:ty , $path:expr; $($field:ident),+) => {
        $(#[$docs])*
        #[derive(Debug)]
        pub struct $name<'a> {
            $(pub $field: &'a str,)+
        }

        impl<'a> EndpointSpec for $name<'a> {
            type JsonResponse = $response;
            type ResponseType = ApiSuccess<Self::JsonResponse>;

            fn method(&self) -> Method {
                Method::$method
            }

            fn path(&self) -> String {
                format!($path, $(self.$field),+)
            }

        }
    };

}
