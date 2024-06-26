// This file is @generated by prost-build.
/// AnyValue is used to represent any type of attribute value. AnyValue may contain a
/// primitive value such as a string or integer or it may contain an arbitrary nested
/// object containing arrays, key-value lists and primitives.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnyValue {
    /// The value is one of the listed fields. It is valid for all values to be unspecified
    /// in which case this AnyValue is considered to be "empty".
    #[prost(oneof = "any_value::Value", tags = "1, 2, 3, 4, 5, 6, 7")]
    pub value: ::core::option::Option<any_value::Value>,
}

/// Nested message and enum types in `AnyValue`.
pub mod any_value {
    /// The value is one of the listed fields. It is valid for all values to be unspecified
    /// in which case this AnyValue is considered to be "empty".
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag = "1")]
        StringValue(::prost::alloc::string::String),
        #[prost(bool, tag = "2")]
        BoolValue(bool),
        #[prost(int64, tag = "3")]
        IntValue(i64),
        #[prost(double, tag = "4")]
        DoubleValue(f64),
        #[prost(message, tag = "5")]
        ArrayValue(super::ArrayValue),
        #[prost(message, tag = "6")]
        KvlistValue(super::KeyValueList),
        #[prost(bytes, tag = "7")]
        BytesValue(::prost::alloc::vec::Vec<u8>),
    }
}

/// ArrayValue is a list of AnyValue messages. We need ArrayValue as a message
/// since oneof in AnyValue does not allow repeated fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrayValue {
    /// Array of values. The array may be empty (contain 0 elements).
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<AnyValue>,
}

/// KeyValueList is a list of KeyValue messages. We need KeyValueList as a message
/// since `oneof` in AnyValue does not allow repeated fields. Everywhere else where we need
/// a list of KeyValue messages (e.g. in Span) we use `repeated KeyValue` directly to
/// avoid unnecessary extra wrapping (which slows down the protocol). The 2 approaches
/// are semantically equivalent.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValueList {
    /// A collection of key/value pairs of key-value pairs. The list may be empty (may
    /// contain 0 elements).
    /// The keys MUST be unique (it is not allowed to have more than one
    /// value with the same key).
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<KeyValue>,
}

/// KeyValue is a key-value pair that is used to store Span attributes, Link
/// attributes, etc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValue {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<AnyValue>,
}

/// InstrumentationScope is a message representing the instrumentation scope information
/// such as the fully qualified name and version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentationScope {
    /// An empty instrumentation scope name means the name is unknown.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// Additional attributes that describe the scope. \[Optional\].
    /// Attribute keys MUST be unique (it is not allowed to have more than one
    /// attribute with the same key).
    #[prost(message, repeated, tag = "3")]
    pub attributes: ::prost::alloc::vec::Vec<KeyValue>,
    #[prost(uint32, tag = "4")]
    pub dropped_attributes_count: u32,
}

/// Resource information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// Set of attributes that describe the resource.
    /// Attribute keys MUST be unique (it is not allowed to have more than one
    /// attribute with the same key).
    #[prost(message, repeated, tag = "1")]
    pub attributes: ::prost::alloc::vec::Vec<KeyValue>,
    /// dropped_attributes_count is the number of dropped attributes. If the value is 0, then
    /// no attributes were dropped.
    #[prost(uint32, tag = "2")]
    pub dropped_attributes_count: u32,
}

/// LogsData represents the logs data that can be stored in a persistent storage,
/// OR can be embedded by other protocols that transfer OTLP logs data but do not
/// implement the OTLP protocol.
///
/// The main difference between this message and collector protocol is that
/// in this message there will not be any "control" or "metadata" specific to
/// OTLP protocol.
///
/// When new fields are added into this message, the OTLP request MUST be updated
/// as well.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogsData {
    /// An array of ResourceLogs.
    /// For data coming from a single resource this array will typically contain
    /// one element. Intermediary nodes that receive data from multiple origins
    /// typically batch the data before forwarding further and in that case this
    /// array will contain multiple elements.
    #[prost(message, repeated, tag = "1")]
    pub resource_logs: ::prost::alloc::vec::Vec<ResourceLogs>,
}

/// A collection of ScopeLogs from a Resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLogs {
    /// The resource for the logs in this message.
    /// If this field is not set then resource info is unknown.
    #[prost(message, optional, tag = "1")]
    pub resource: ::core::option::Option<Resource>,
    /// A list of ScopeLogs that originate from a resource.
    #[prost(message, repeated, tag = "2")]
    pub scope_logs: ::prost::alloc::vec::Vec<ScopeLogs>,
    /// The Schema URL, if known. This is the identifier of the Schema that the resource data
    /// is recorded in. To learn more about Schema URL see
    /// <https://opentelemetry.io/docs/specs/otel/schemas/#schema-url>
    /// This schema_url applies to the data in the "resource" field. It does not apply
    /// to the data in the "scope_logs" field which have their own schema_url field.
    #[prost(string, tag = "3")]
    pub schema_url: ::prost::alloc::string::String,
}

/// A collection of Logs produced by a Scope.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopeLogs {
    /// The instrumentation scope information for the logs in this message.
    /// Semantically when InstrumentationScope isn't set, it is equivalent with
    /// an empty instrumentation scope name (unknown).
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<InstrumentationScope>,
    /// A list of log records.
    #[prost(message, repeated, tag = "2")]
    pub log_records: ::prost::alloc::vec::Vec<LogRecord>,
    /// The Schema URL, if known. This is the identifier of the Schema that the log data
    /// is recorded in. To learn more about Schema URL see
    /// <https://opentelemetry.io/docs/specs/otel/schemas/#schema-url>
    /// This schema_url applies to all logs in the "logs" field.
    #[prost(string, tag = "3")]
    pub schema_url: ::prost::alloc::string::String,
}

/// A log record according to OpenTelemetry Log Data Model:
/// <https://github.com/open-telemetry/oteps/blob/main/text/logs/0097-log-data-model.md>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogRecord {
    /// time_unix_nano is the time when the event occurred.
    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    /// Value of 0 indicates unknown or missing timestamp.
    #[prost(fixed64, tag = "1")]
    pub time_unix_nano: u64,
    /// Time when the event was observed by the collection system.
    /// For events that originate in OpenTelemetry (e.g. using OpenTelemetry Logging SDK)
    /// this timestamp is typically set at the generation time and is equal to Timestamp.
    /// For events originating externally and collected by OpenTelemetry (e.g. using
    /// Collector) this is the time when OpenTelemetry's code observed the event measured
    /// by the clock of the OpenTelemetry code. This field MUST be set once the event is
    /// observed by OpenTelemetry.
    ///
    /// For converting OpenTelemetry log data to formats that support only one timestamp or
    /// when receiving OpenTelemetry log data by recipients that support only one timestamp
    /// internally the following logic is recommended:
    ///    - Use time_unix_nano if it is present, otherwise use observed_time_unix_nano.
    ///
    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    /// Value of 0 indicates unknown or missing timestamp.
    #[prost(fixed64, tag = "11")]
    pub observed_time_unix_nano: u64,
    /// Numerical value of the severity, normalized to values described in Log Data Model.
    /// \[Optional\].
    #[prost(enumeration = "SeverityNumber", tag = "2")]
    pub severity_number: i32,
    /// The severity text (also known as log level). The original string representation as
    /// it is known at the source. \[Optional\].
    #[prost(string, tag = "3")]
    pub severity_text: ::prost::alloc::string::String,
    /// A value containing the body of the log record. Can be for example a human-readable
    /// string message (including multi-line) describing the event in a free form or it can
    /// be a structured data composed of arrays and maps of other values. \[Optional\].
    #[prost(message, optional, tag = "5")]
    pub body: ::core::option::Option<AnyValue>,
    /// Additional attributes that describe the specific event occurrence. \[Optional\].
    /// Attribute keys MUST be unique (it is not allowed to have more than one
    /// attribute with the same key).
    #[prost(message, repeated, tag = "6")]
    pub attributes: ::prost::alloc::vec::Vec<KeyValue>,
    #[prost(uint32, tag = "7")]
    pub dropped_attributes_count: u32,
    /// Flags, a bit field. 8 least significant bits are the trace flags as
    /// defined in W3C Trace Context specification. 24 most significant bits are reserved
    /// and must be set to 0. Readers must not assume that 24 most significant bits
    /// will be zero and must correctly mask the bits when reading 8-bit trace flag (use
    /// flags & LOG_RECORD_FLAGS_TRACE_FLAGS_MASK). \[Optional\].
    #[prost(fixed32, tag = "8")]
    pub flags: u32,
    /// A unique identifier for a trace. All logs from the same trace share
    /// the same `trace_id`. The ID is a 16-byte array. An ID with all zeroes OR
    /// of length other than 16 bytes is considered invalid (empty string in OTLP/JSON
    /// is zero-length and thus is also invalid).
    ///
    /// This field is optional.
    ///
    /// The receivers SHOULD assume that the log record is not associated with a
    /// trace if any of the following is true:
    ///    - the field is not present,
    ///    - the field contains an invalid value.
    #[prost(bytes = "vec", tag = "9")]
    pub trace_id: ::prost::alloc::vec::Vec<u8>,
    /// A unique identifier for a span within a trace, assigned when the span
    /// is created. The ID is an 8-byte array. An ID with all zeroes OR of length
    /// other than 8 bytes is considered invalid (empty string in OTLP/JSON
    /// is zero-length and thus is also invalid).
    ///
    /// This field is optional. If the sender specifies a valid span_id then it SHOULD also
    /// specify a valid trace_id.
    ///
    /// The receivers SHOULD assume that the log record is not associated with a
    /// span if any of the following is true:
    ///    - the field is not present,
    ///    - the field contains an invalid value.
    #[prost(bytes = "vec", tag = "10")]
    pub span_id: ::prost::alloc::vec::Vec<u8>,
}

/// Possible values for LogRecord.SeverityNumber.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SeverityNumber {
    /// UNSPECIFIED is the default SeverityNumber, it MUST NOT be used.
    Unspecified = 0,
    Trace = 1,
    Trace2 = 2,
    Trace3 = 3,
    Trace4 = 4,
    Debug = 5,
    Debug2 = 6,
    Debug3 = 7,
    Debug4 = 8,
    Info = 9,
    Info2 = 10,
    Info3 = 11,
    Info4 = 12,
    Warn = 13,
    Warn2 = 14,
    Warn3 = 15,
    Warn4 = 16,
    Error = 17,
    Error2 = 18,
    Error3 = 19,
    Error4 = 20,
    Fatal = 21,
    Fatal2 = 22,
    Fatal3 = 23,
    Fatal4 = 24,
}

impl SeverityNumber {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SeverityNumber::Unspecified => "SEVERITY_NUMBER_UNSPECIFIED",
            SeverityNumber::Trace => "SEVERITY_NUMBER_TRACE",
            SeverityNumber::Trace2 => "SEVERITY_NUMBER_TRACE2",
            SeverityNumber::Trace3 => "SEVERITY_NUMBER_TRACE3",
            SeverityNumber::Trace4 => "SEVERITY_NUMBER_TRACE4",
            SeverityNumber::Debug => "SEVERITY_NUMBER_DEBUG",
            SeverityNumber::Debug2 => "SEVERITY_NUMBER_DEBUG2",
            SeverityNumber::Debug3 => "SEVERITY_NUMBER_DEBUG3",
            SeverityNumber::Debug4 => "SEVERITY_NUMBER_DEBUG4",
            SeverityNumber::Info => "SEVERITY_NUMBER_INFO",
            SeverityNumber::Info2 => "SEVERITY_NUMBER_INFO2",
            SeverityNumber::Info3 => "SEVERITY_NUMBER_INFO3",
            SeverityNumber::Info4 => "SEVERITY_NUMBER_INFO4",
            SeverityNumber::Warn => "SEVERITY_NUMBER_WARN",
            SeverityNumber::Warn2 => "SEVERITY_NUMBER_WARN2",
            SeverityNumber::Warn3 => "SEVERITY_NUMBER_WARN3",
            SeverityNumber::Warn4 => "SEVERITY_NUMBER_WARN4",
            SeverityNumber::Error => "SEVERITY_NUMBER_ERROR",
            SeverityNumber::Error2 => "SEVERITY_NUMBER_ERROR2",
            SeverityNumber::Error3 => "SEVERITY_NUMBER_ERROR3",
            SeverityNumber::Error4 => "SEVERITY_NUMBER_ERROR4",
            SeverityNumber::Fatal => "SEVERITY_NUMBER_FATAL",
            SeverityNumber::Fatal2 => "SEVERITY_NUMBER_FATAL2",
            SeverityNumber::Fatal3 => "SEVERITY_NUMBER_FATAL3",
            SeverityNumber::Fatal4 => "SEVERITY_NUMBER_FATAL4",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SEVERITY_NUMBER_UNSPECIFIED" => Some(Self::Unspecified),
            "SEVERITY_NUMBER_TRACE" => Some(Self::Trace),
            "SEVERITY_NUMBER_TRACE2" => Some(Self::Trace2),
            "SEVERITY_NUMBER_TRACE3" => Some(Self::Trace3),
            "SEVERITY_NUMBER_TRACE4" => Some(Self::Trace4),
            "SEVERITY_NUMBER_DEBUG" => Some(Self::Debug),
            "SEVERITY_NUMBER_DEBUG2" => Some(Self::Debug2),
            "SEVERITY_NUMBER_DEBUG3" => Some(Self::Debug3),
            "SEVERITY_NUMBER_DEBUG4" => Some(Self::Debug4),
            "SEVERITY_NUMBER_INFO" => Some(Self::Info),
            "SEVERITY_NUMBER_INFO2" => Some(Self::Info2),
            "SEVERITY_NUMBER_INFO3" => Some(Self::Info3),
            "SEVERITY_NUMBER_INFO4" => Some(Self::Info4),
            "SEVERITY_NUMBER_WARN" => Some(Self::Warn),
            "SEVERITY_NUMBER_WARN2" => Some(Self::Warn2),
            "SEVERITY_NUMBER_WARN3" => Some(Self::Warn3),
            "SEVERITY_NUMBER_WARN4" => Some(Self::Warn4),
            "SEVERITY_NUMBER_ERROR" => Some(Self::Error),
            "SEVERITY_NUMBER_ERROR2" => Some(Self::Error2),
            "SEVERITY_NUMBER_ERROR3" => Some(Self::Error3),
            "SEVERITY_NUMBER_ERROR4" => Some(Self::Error4),
            "SEVERITY_NUMBER_FATAL" => Some(Self::Fatal),
            "SEVERITY_NUMBER_FATAL2" => Some(Self::Fatal2),
            "SEVERITY_NUMBER_FATAL3" => Some(Self::Fatal3),
            "SEVERITY_NUMBER_FATAL4" => Some(Self::Fatal4),
            _ => None,
        }
    }
}

/// LogRecordFlags represents constants used to interpret the
/// LogRecord.flags field, which is protobuf 'fixed32' type and is to
/// be used as bit-fields. Each non-zero value defined in this enum is
/// a bit-mask.  To extract the bit-field, for example, use an
/// expression like:
///
///    (logRecord.flags & LOG_RECORD_FLAGS_TRACE_FLAGS_MASK)
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LogRecordFlags {
    /// The zero value for the enum. Should not be used for comparisons.
    /// Instead use bitwise "and" with the appropriate mask as shown above.
    DoNotUse = 0,
    /// Bits 0-7 are used for trace flags.
    TraceFlagsMask = 255,
}

impl LogRecordFlags {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LogRecordFlags::DoNotUse => "LOG_RECORD_FLAGS_DO_NOT_USE",
            LogRecordFlags::TraceFlagsMask => "LOG_RECORD_FLAGS_TRACE_FLAGS_MASK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOG_RECORD_FLAGS_DO_NOT_USE" => Some(Self::DoNotUse),
            "LOG_RECORD_FLAGS_TRACE_FLAGS_MASK" => Some(Self::TraceFlagsMask),
            _ => None,
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportLogsServiceRequest {
    /// An array of ResourceLogs.
    /// For data coming from a single resource this array will typically contain one
    /// element. Intermediary nodes (such as OpenTelemetry Collector) that receive
    /// data from multiple origins typically batch the data before forwarding further and
    /// in that case this array will contain multiple elements.
    #[prost(message, repeated, tag = "1")]
    pub resource_logs: ::prost::alloc::vec::Vec<ResourceLogs>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportLogsServiceResponse {
    /// The details of a partially successful export request.
    ///
    /// If the request is only partially accepted
    /// (i.e. when the server accepts only parts of the data and rejects the rest)
    /// the server MUST initialize the `partial_success` field and MUST
    /// set the `rejected_<signal>` with the number of items it rejected.
    ///
    /// Servers MAY also make use of the `partial_success` field to convey
    /// warnings/suggestions to senders even when the request was fully accepted.
    /// In such cases, the `rejected_<signal>` MUST have a value of `0` and
    /// the `error_message` MUST be non-empty.
    ///
    /// A `partial_success` message with an empty value (rejected_<signal> = 0 and
    /// `error_message` = "") is equivalent to it not being set/present. Senders
    /// SHOULD interpret it the same way as in the full success case.
    #[prost(message, optional, tag = "1")]
    pub partial_success: ::core::option::Option<ExportLogsPartialSuccess>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportLogsPartialSuccess {
    /// The number of rejected log records.
    ///
    /// A `rejected_<signal>` field holding a `0` value indicates that the
    /// request was fully accepted.
    #[prost(int64, tag = "1")]
    pub rejected_log_records: i64,
    /// A developer-facing human-readable message in English. It should be used
    /// either to explain why the server rejected parts of the data during a partial
    /// success or to convey warnings/suggestions during a full success. The message
    /// should offer guidance on how users can address such issues.
    ///
    /// error_message is an optional field. An error_message with an empty value
    /// is equivalent to it not being set.
    #[prost(string, tag = "2")]
    pub error_message: ::prost::alloc::string::String,
}

/// Generated client implementations.
pub mod logs_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]

    use tonic::codegen::*;
    use tonic::codegen::http::Uri;

    /// Service that can be used to push logs between one Application instrumented with
    /// OpenTelemetry and an collector, or between an collector and a central collector (in this
    /// case logs are sent/received to/from multiple Applications).
    #[derive(Debug, Clone)]
    pub struct LogsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }

    impl LogsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
            where
                D: TryInto<tonic::transport::Endpoint>,
                D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }

    impl<T> LogsServiceClient<T>
        where
            T: tonic::client::GrpcService<tonic::body::BoxBody>,
            T::Error: Into<StdError>,
            T::ResponseBody: Body<Data=Bytes> + Send + 'static,
            <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> LogsServiceClient<InterceptedService<T, F>>
            where
                F: tonic::service::Interceptor,
                T::ResponseBody: Default,
                T: tonic::codegen::Service<
                    http::Request<tonic::body::BoxBody>,
                    Response=http::Response<
                        <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                    >,
                >,
                <T as tonic::codegen::Service<
                    http::Request<tonic::body::BoxBody>,
                >>::Error: Into<StdError> + Send + Sync,
        {
            LogsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// For performance reasons, it is recommended to keep this RPC
        /// alive for the entire life of the application.
        pub async fn export(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportLogsServiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExportLogsServiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/opentelemetry.proto.collector.logs.v1.LogsService/Export",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "opentelemetry.proto.collector.logs.v1.LogsService",
                        "Export",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}

/// Generated server implementations.
pub mod logs_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]

    use tonic::codegen::*;

    /// Generated trait containing gRPC methods that should be implemented for use with LogsServiceServer.
    #[async_trait]
    pub trait LogsService: Send + Sync + 'static {
        /// For performance reasons, it is recommended to keep this RPC
        /// alive for the entire life of the application.
        async fn export(
            &self,
            request: tonic::Request<super::ExportLogsServiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExportLogsServiceResponse>,
            tonic::Status,
        >;
    }

    /// Service that can be used to push logs between one Application instrumented with
    /// OpenTelemetry and an collector, or between an collector and a central collector (in this
    /// case logs are sent/received to/from multiple Applications).
    #[derive(Debug)]
    pub struct LogsServiceServer<T: LogsService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }

    struct _Inner<T>(Arc<T>);

    impl<T: LogsService> LogsServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
            where
                F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }

    impl<T, B> tonic::codegen::Service<http::Request<B>> for LogsServiceServer<T>
        where
            T: LogsService,
            B: Body + Send + 'static,
            B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/opentelemetry.proto.collector.logs.v1.LogsService/Export" => {
                    #[allow(non_camel_case_types)]
                    struct ExportSvc<T: LogsService>(pub Arc<T>);
                    impl<
                        T: LogsService,
                    > tonic::server::UnaryService<super::ExportLogsServiceRequest>
                    for ExportSvc<T> {
                        type Response = super::ExportLogsServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExportLogsServiceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LogsService>::export(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExportSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }

    impl<T: LogsService> Clone for LogsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }

    impl<T: LogsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }

    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl<T: LogsService> tonic::server::NamedService for LogsServiceServer<T> {
        const NAME: &'static str = "opentelemetry.proto.collector.logs.v1.LogsService";
    }
}
