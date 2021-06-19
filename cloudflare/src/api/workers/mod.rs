use chrono::{DateTime, Utc};
use crate::{Content, ContentSerializer, Endpoint, JsonResponse, Method, MultipartSerializer};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod kv;
pub mod routes;
pub mod secrets;
pub mod tails;

/// Returns a list of all Workers scripts on the specified account
pub struct List<'a> {
    pub account_id: &'a str,
}

impl Endpoint for List<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<Vec<Worker>>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        format!("accounts/{}/workers/scripts", self.account_id).into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

/// Upload a Workers script with the specified name
pub struct Upload<'a> {
    pub account_id: &'a str,
    pub script_name: &'a str,
    pub form: UploadForm,
}

impl Endpoint for Upload<'_> {
    type Body = UploadForm;
    type Query = ();
    type Response = JsonResponse<WorkerWithScript>;

    const METHOD: Method = Method::Put;

    fn path(&self) -> Cow<str> {
        format!("accounts/{}/workers/scripts/{}", self.account_id, self.script_name).into()
    }
    fn body(&self) -> &Self::Body {
        &self.form
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UploadForm {
    ServiceWorker {
        metadata: ServiceWorkerMetadata,
        files: Vec<ServiceWorkerFile>,
    },
    Modules {
        metadata: ModulesMetadata,
        modules: Vec<Module>,
    },
}

impl Content for UploadForm {
    fn serialize<C: ContentSerializer>(&self, serializer: C) -> Result<C::Ok, C::Error> {
        self.append_to_multipart(serializer.multipart()?)?.end()
    }
}

impl UploadForm {
    /// Appends the upload content to the specified multipart serializer.
    pub fn append_to_multipart<M: MultipartSerializer>(&self, serializer: M) -> Result<M, M::Error> {
        match self {
            UploadForm::ServiceWorker {
                metadata,
                files,
            } => {
                let mut serializer = serializer
                    .add_json(metadata, "metadata.json", Some("metadata.json"))?;

                for file in files {
                    serializer = match file {
                        ServiceWorkerFile::WasmModule { part, data, }
                            => serializer.add_wasm(data, part, None),
                        ServiceWorkerFile::TextBlob { part, text }
                            => serializer.add_plain_text(text, part, None),
                    }?;
                }

                Ok(serializer)
            },
            UploadForm::Modules { metadata, modules } => {
                let mut serializer = serializer
                    .add_json(metadata, "metadata.json", Some("metadata.json"))?;

                for Module { file_name, data } in modules {
                    let file_name = file_name.as_str();
                    serializer = match data {
                        ModuleData::Wasm(data)
                            => serializer.add_wasm(data, file_name, Some(file_name)),
                        ModuleData::EsModule(content)
                            => serializer.add_javascript_module(content, file_name, Some(file_name)),
                        ModuleData::CommonJs(content)
                            => serializer.add_javascript(content, file_name, Some(file_name)),
                        ModuleData::Text(content)
                            => serializer.add_plain_text(content, file_name, Some(file_name)),
                        ModuleData::Data(data)
                            => serializer.add_octet_stream(data, file_name, Some(file_name))
                    }?;
                }

                Ok(serializer)
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ServiceWorkerMetadata {
    /// The usage model of the script. Use none to use the account default setting.
    pub usage_model: Option<UsageModel>,
    /// A set of bindings to use with the Worker.
    pub bindings: Vec<ServiceWorkerBinding>,
    /// The body part that corresponds to the script that should be called as this Worker.
    pub body_part: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UsageModel {
    Bundled,
    Unbound,
}

/// An uploaded file associated with a service worker via a binding
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceWorkerFile {
    WasmModule {
        part: String,
        data: Box<[u8]>,
    },
    TextBlob {
        part: String,
        text: String,
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ServiceWorkerBinding {
    /// A wasm module binding.
    WasmModule {
        name: String,
        part: String,
    },
    /// A KV namespace binding
    KvNamespace {
        name: String,
        namespace_id: String,
    },
    /// A durable object namespace binding.
    DurableObjectNamespace {
        name: String,
        class_name: String,
        script_name: Option<String>,
    },
    /// A text blob binding. This is only valid for service-worker scripts.
    TextBlob {
        name: String,
        part: String,
    },
    PlainText {
        name: String,
        text: String,
    },
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ModulesMetadata {
    /// The usage model of the script. Use none to use the account default setting.
    pub usage_model: Option<UsageModel>,
    /// A set of bindings to use with the Worker.
    pub bindings: Vec<ModulesBinding>,
    /// The module file from the upload form to use as the main module in modules format
    pub main_module: Option<String>,
    /// A Durable Object migration configuration.
    pub migrations: Option<Migration>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub file_name: String,
    pub data: ModuleData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModuleData {
    Wasm(Box<[u8]>),
    EsModule(String),
    CommonJs(String),
    Text(String),
    Data(Box<[u8]>),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ModulesBinding {
    /// A KV namespace binding
    KvNamespace {
        name: String,
        namespace_id: String,
    },
    /// A durable object namespace binding.
    DurableObjectNamespace {
        name: String,
        class_name: String,
        script_name: Option<String>,
    },
    PlainText {
        name: String,
        text: String,
    },
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Migration {
    pub old_tag: Option<String>,
    pub new_tag: Option<String>,

    pub new_classes: Vec<String>,
    pub deleted_classes: Vec<String>,
    pub renamed_classes: Vec<RenameClass>,
    pub transferred_classes: Vec<TransferClass>,    
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RenameClass {
    pub from: String,
    pub to: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TransferClass {
    pub from: String,
    pub from_script: String,
    pub to: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Worker {
    /// The name of the script
    pub id: String,
    pub etag: String,
    /// A set of handlers available on the 
    pub handlers: Vec<String>,
    pub modified_on: DateTime<Utc>,
    pub created_on: DateTime<Utc>,
    pub usage_model: UsageModel,
    pub size: u64,
    pub available_on_subdomain: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WorkerWithScript {
    #[serde(flatten)]
    pub worker: Worker,
    pub script: String,
}