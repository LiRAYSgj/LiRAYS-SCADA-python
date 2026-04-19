#![allow(clippy::type_complexity)]
#![allow(clippy::useless_conversion)]

use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

use futures_util::StreamExt;
use lirays_rust::types::errors::ClientError;
use lirays_rust::{
    BooleanVar as RsBooleanVar, Client as RsClient, ConnectionOptions as RsConnectionOptions,
    FloatVar as RsFloatVar, IntegerVar as RsIntegerVar, TextVar as RsTextVar,
    VariableMetadataPatch as RsVariableMetadataPatch,
};
use lirays_scada_proto::namespace::v1::{self as namespace, value};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

fn build_runtime() -> PyResult<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|err| PyRuntimeError::new_err(format!("failed to create tokio runtime: {err}")))
}

fn lock_err(what: &str) -> PyErr {
    PyRuntimeError::new_err(format!("{what} lock poisoned"))
}

fn to_py_err(err: ClientError) -> PyErr {
    PyRuntimeError::new_err(err.to_string())
}

fn typed_to_object(py: Python<'_>, typed: Option<value::Typed>) -> PyObject {
    match typed {
        Some(value::Typed::IntegerValue(v)) => v.into_py(py),
        Some(value::Typed::FloatValue(v)) => v.into_py(py),
        Some(value::Typed::TextValue(v)) => v.into_py(py),
        Some(value::Typed::BooleanValue(v)) => v.into_py(py),
        None => py.None(),
    }
}

fn var_data_type_name(raw: i32) -> &'static str {
    match namespace::VarDataType::try_from(raw).ok() {
        Some(namespace::VarDataType::Integer) => "integer",
        Some(namespace::VarDataType::Float) => "float",
        Some(namespace::VarDataType::Text) => "text",
        Some(namespace::VarDataType::Boolean) => "boolean",
        _ => "invalid",
    }
}

type SharedRuntime = Arc<Mutex<tokio::runtime::Runtime>>;

#[pyclass(module = "lirays._lirays")]
#[derive(Clone, Debug)]
pub struct ConnectionOptions {
    #[pyo3(get, set)]
    pub host: String,
    #[pyo3(get, set)]
    pub port: i64,
    #[pyo3(get, set)]
    pub tls: bool,
    #[pyo3(get, set)]
    pub allow_insecure_tls: bool,
    #[pyo3(get, set)]
    pub pat_token: Option<String>,
}

impl ConnectionOptions {
    fn to_rust(&self) -> RsConnectionOptions {
        RsConnectionOptions::new(
            self.host.clone(),
            self.port,
            self.tls,
            self.pat_token.clone(),
        )
        .with_insecure_tls(self.allow_insecure_tls)
    }
}

#[pymethods]
impl ConnectionOptions {
    #[new]
    #[pyo3(signature = (host, port, tls=false, pat_token=None, allow_insecure_tls=false))]
    fn new(
        host: String,
        port: i64,
        tls: bool,
        pat_token: Option<String>,
        allow_insecure_tls: bool,
    ) -> Self {
        Self {
            host,
            port,
            tls,
            allow_insecure_tls,
            pat_token,
        }
    }

    fn ws_url(&self) -> PyResult<String> {
        self.to_rust().ws_url().map_err(to_py_err)
    }

    fn __repr__(&self) -> String {
        format!(
            "ConnectionOptions(host={:?}, port={}, tls={}, allow_insecure_tls={}, pat_token={:?})",
            self.host, self.port, self.tls, self.allow_insecure_tls, self.pat_token
        )
    }
}

#[pyclass(module = "lirays._lirays")]
#[derive(Clone, Debug)]
pub struct IntegerVar {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub unit: Option<String>,
    #[pyo3(get, set)]
    pub min: Option<f64>,
    #[pyo3(get, set)]
    pub max: Option<f64>,
}

impl IntegerVar {
    fn to_rust(&self) -> RsIntegerVar {
        RsIntegerVar {
            name: self.name.clone(),
            unit: self.unit.clone(),
            min: self.min,
            max: self.max,
        }
    }
}

#[pymethods]
impl IntegerVar {
    #[new]
    #[pyo3(signature = (name, unit=None, min=None, max=None))]
    fn new(name: String, unit: Option<String>, min: Option<f64>, max: Option<f64>) -> Self {
        Self {
            name,
            unit,
            min,
            max,
        }
    }
}

#[pyclass(module = "lirays._lirays")]
#[derive(Clone, Debug)]
pub struct FloatVar {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub unit: Option<String>,
    #[pyo3(get, set)]
    pub min: Option<f64>,
    #[pyo3(get, set)]
    pub max: Option<f64>,
}

impl FloatVar {
    fn to_rust(&self) -> RsFloatVar {
        RsFloatVar {
            name: self.name.clone(),
            unit: self.unit.clone(),
            min: self.min,
            max: self.max,
        }
    }
}

#[pymethods]
impl FloatVar {
    #[new]
    #[pyo3(signature = (name, unit=None, min=None, max=None))]
    fn new(name: String, unit: Option<String>, min: Option<f64>, max: Option<f64>) -> Self {
        Self {
            name,
            unit,
            min,
            max,
        }
    }
}

#[pyclass(module = "lirays._lirays")]
#[derive(Clone, Debug)]
pub struct TextVar {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub unit: Option<String>,
    #[pyo3(get, set)]
    pub options: Vec<String>,
    #[pyo3(get, set)]
    pub max_len: Option<u64>,
}

impl TextVar {
    fn to_rust(&self) -> RsTextVar {
        RsTextVar {
            name: self.name.clone(),
            unit: self.unit.clone(),
            options: self.options.clone(),
            max_len: self.max_len,
        }
    }
}

#[pymethods]
impl TextVar {
    #[new]
    #[pyo3(signature = (name, unit=None, options=Vec::new(), max_len=None))]
    fn new(name: String, unit: Option<String>, options: Vec<String>, max_len: Option<u64>) -> Self {
        Self {
            name,
            unit,
            options,
            max_len,
        }
    }
}

#[pyclass(module = "lirays._lirays")]
#[derive(Clone, Debug)]
pub struct BooleanVar {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub unit: Option<String>,
}

impl BooleanVar {
    fn to_rust(&self) -> RsBooleanVar {
        RsBooleanVar {
            name: self.name.clone(),
            unit: self.unit.clone(),
        }
    }
}

#[pymethods]
impl BooleanVar {
    #[new]
    #[pyo3(signature = (name, unit=None))]
    fn new(name: String, unit: Option<String>) -> Self {
        Self { name, unit }
    }
}

#[pyclass(module = "lirays._lirays")]
#[derive(Clone, Debug, Default)]
pub struct VariableMetadataPatch {
    #[pyo3(get, set)]
    pub unit: Option<String>,
    #[pyo3(get, set)]
    pub min: Option<f64>,
    #[pyo3(get, set)]
    pub max: Option<f64>,
    #[pyo3(get, set)]
    pub options: Vec<String>,
    #[pyo3(get, set)]
    pub max_len: Option<u64>,
}

impl VariableMetadataPatch {
    fn to_rust(&self) -> RsVariableMetadataPatch {
        RsVariableMetadataPatch {
            unit: self.unit.clone(),
            min: self.min,
            max: self.max,
            options: self.options.clone(),
            max_len: self.max_len,
        }
    }
}

#[pymethods]
impl VariableMetadataPatch {
    #[new]
    #[pyo3(signature = (unit=None, min=None, max=None, options=Vec::new(), max_len=None))]
    fn new(
        unit: Option<String>,
        min: Option<f64>,
        max: Option<f64>,
        options: Vec<String>,
        max_len: Option<u64>,
    ) -> Self {
        Self {
            unit,
            min,
            max,
            options,
            max_len,
        }
    }
}

#[pyclass(module = "lirays._lirays")]
#[derive(Clone, Debug)]
pub struct FolderInfo {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub name: String,
}

impl FolderInfo {
    fn from_rust(folder: namespace::FolderInfo) -> Self {
        Self {
            id: folder.id,
            name: folder.name,
        }
    }
}

#[pymethods]
impl FolderInfo {
    #[new]
    fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}

#[pyclass(module = "lirays._lirays")]
#[derive(Clone, Debug)]
pub struct VarInfo {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub var_d_type: String,
    #[pyo3(get)]
    pub unit: Option<String>,
    #[pyo3(get)]
    pub min: Option<f64>,
    #[pyo3(get)]
    pub max: Option<f64>,
    #[pyo3(get)]
    pub options: Vec<String>,
    #[pyo3(get)]
    pub max_len: Option<u64>,
}

impl VarInfo {
    fn from_rust(var: namespace::VarInfo) -> Self {
        Self {
            id: var.id,
            name: var.name,
            var_d_type: var_data_type_name(var.var_d_type).to_string(),
            unit: var.unit,
            min: var.min,
            max: var.max,
            options: var.options,
            max_len: var.max_len,
        }
    }
}

#[pymethods]
impl VarInfo {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (id, name, var_d_type, unit=None, min=None, max=None, options=Vec::new(), max_len=None))]
    fn new(
        id: String,
        name: String,
        var_d_type: String,
        unit: Option<String>,
        min: Option<f64>,
        max: Option<f64>,
        options: Vec<String>,
        max_len: Option<u64>,
    ) -> Self {
        Self {
            id,
            name,
            var_d_type,
            unit,
            min,
            max,
            options,
            max_len,
        }
    }
}

struct ClientState {
    runtime: SharedRuntime,
    client: RsClient,
}

#[pyclass(module = "lirays._lirays", unsendable)]
pub struct Client {
    state: Mutex<Option<ClientState>>,
    options: ConnectionOptions,
}

#[pyclass(module = "lirays._lirays", unsendable)]
pub struct Subscription {
    receiver: Mutex<Option<mpsc::Receiver<(String, Option<value::Typed>)>>>,
}

impl Client {
    fn from_options(options: ConnectionOptions) -> PyResult<Self> {
        let runtime = Arc::new(Mutex::new(build_runtime()?));

        let client = {
            let runtime_guard = runtime.lock().map_err(|_| lock_err("runtime"))?;
            runtime_guard
                .block_on(RsClient::connect_with_options(options.to_rust()))
                .map_err(to_py_err)?
        };

        Ok(Self {
            state: Mutex::new(Some(ClientState { runtime, client })),
            options,
        })
    }

    fn with_state<T>(&self, f: impl FnOnce(&ClientState) -> PyResult<T>) -> PyResult<T> {
        let guard = self.state.lock().map_err(|_| lock_err("client state"))?;
        let state = guard
            .as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("client is disconnected"))?;
        f(state)
    }
}

#[pymethods]
impl Client {
    #[staticmethod]
    #[pyo3(signature = (host, port, tls=false, allow_insecure_tls=false))]
    fn connect(host: String, port: i64, tls: bool, allow_insecure_tls: bool) -> PyResult<Self> {
        Self::from_options(ConnectionOptions {
            host,
            port,
            tls,
            allow_insecure_tls,
            pat_token: None,
        })
    }

    #[staticmethod]
    #[pyo3(signature = (host, port, tls, pat_token, allow_insecure_tls=false))]
    fn connect_with_pat(
        host: String,
        port: i64,
        tls: bool,
        pat_token: String,
        allow_insecure_tls: bool,
    ) -> PyResult<Self> {
        Self::from_options(ConnectionOptions {
            host,
            port,
            tls,
            allow_insecure_tls,
            pat_token: Some(pat_token),
        })
    }

    #[staticmethod]
    fn connect_with_options(options: PyRef<'_, ConnectionOptions>) -> PyResult<Self> {
        Self::from_options(options.clone())
    }

    fn connection_options(&self) -> ConnectionOptions {
        self.options.clone()
    }

    fn is_connected(&self) -> bool {
        match self.state.lock() {
            Ok(guard) => guard.is_some(),
            Err(_) => false,
        }
    }

    fn disconnect(&self) -> PyResult<()> {
        let mut state_guard = self.state.lock().map_err(|_| lock_err("client state"))?;
        let Some(state) = state_guard.as_ref() else {
            return Ok(());
        };

        {
            let runtime_guard = state.runtime.lock().map_err(|_| lock_err("runtime"))?;
            runtime_guard
                .block_on(state.client.disconnect())
                .map_err(to_py_err)?;
        }

        *state_guard = None;
        Ok(())
    }

    #[pyo3(signature = (folder_id=None, timeout_ms=5000))]
    fn list(
        &self,
        folder_id: Option<String>,
        timeout_ms: u64,
    ) -> PyResult<(Vec<FolderInfo>, Vec<VarInfo>)> {
        self.with_state(|state| {
            let runtime_guard = state.runtime.lock().map_err(|_| lock_err("runtime"))?;
            let (folders, variables) = runtime_guard
                .block_on(state.client.list(folder_id, timeout_ms))
                .map_err(to_py_err)?;
            Ok((
                folders.into_iter().map(FolderInfo::from_rust).collect(),
                variables.into_iter().map(VarInfo::from_rust).collect(),
            ))
        })
    }

    #[pyo3(signature = (names, parent_id=None, timeout_ms=5000))]
    fn create_folders(
        &self,
        names: Vec<String>,
        parent_id: Option<String>,
        timeout_ms: u64,
    ) -> PyResult<()> {
        self.with_state(|state| {
            let runtime_guard = state.runtime.lock().map_err(|_| lock_err("runtime"))?;
            runtime_guard
                .block_on(state.client.create_folders(names, parent_id, timeout_ms))
                .map_err(to_py_err)
        })
    }

    #[pyo3(signature = (vars, parent_id=None, timeout_ms=5000))]
    fn create_integer_variables(
        &self,
        vars: Vec<PyRef<'_, IntegerVar>>,
        parent_id: Option<String>,
        timeout_ms: u64,
    ) -> PyResult<()> {
        let vars: Vec<RsIntegerVar> = vars.into_iter().map(|v| v.to_rust()).collect();
        self.with_state(|state| {
            let runtime_guard = state.runtime.lock().map_err(|_| lock_err("runtime"))?;
            runtime_guard
                .block_on(
                    state
                        .client
                        .create_integer_variables(vars, parent_id, timeout_ms),
                )
                .map_err(to_py_err)
        })
    }

    #[pyo3(signature = (vars, parent_id=None, timeout_ms=5000))]
    fn create_float_variables(
        &self,
        vars: Vec<PyRef<'_, FloatVar>>,
        parent_id: Option<String>,
        timeout_ms: u64,
    ) -> PyResult<()> {
        let vars: Vec<RsFloatVar> = vars.into_iter().map(|v| v.to_rust()).collect();
        self.with_state(|state| {
            let runtime_guard = state.runtime.lock().map_err(|_| lock_err("runtime"))?;
            runtime_guard
                .block_on(
                    state
                        .client
                        .create_float_variables(vars, parent_id, timeout_ms),
                )
                .map_err(to_py_err)
        })
    }

    #[pyo3(signature = (vars, parent_id=None, timeout_ms=5000))]
    fn create_text_variables(
        &self,
        vars: Vec<PyRef<'_, TextVar>>,
        parent_id: Option<String>,
        timeout_ms: u64,
    ) -> PyResult<()> {
        let vars: Vec<RsTextVar> = vars.into_iter().map(|v| v.to_rust()).collect();
        self.with_state(|state| {
            let runtime_guard = state.runtime.lock().map_err(|_| lock_err("runtime"))?;
            runtime_guard
                .block_on(
                    state
                        .client
                        .create_text_variables(vars, parent_id, timeout_ms),
                )
                .map_err(to_py_err)
        })
    }

    #[pyo3(signature = (vars, parent_id=None, timeout_ms=5000))]
    fn create_boolean_variables(
        &self,
        vars: Vec<PyRef<'_, BooleanVar>>,
        parent_id: Option<String>,
        timeout_ms: u64,
    ) -> PyResult<()> {
        let vars: Vec<RsBooleanVar> = vars.into_iter().map(|v| v.to_rust()).collect();
        self.with_state(|state| {
            let runtime_guard = state.runtime.lock().map_err(|_| lock_err("runtime"))?;
            runtime_guard
                .block_on(
                    state
                        .client
                        .create_boolean_variables(vars, parent_id, timeout_ms),
                )
                .map_err(to_py_err)
        })
    }

    #[pyo3(signature = (item_ids, timeout_ms=5000))]
    fn delete_items(&self, item_ids: Vec<String>, timeout_ms: u64) -> PyResult<()> {
        self.with_state(|state| {
            let runtime_guard = state.runtime.lock().map_err(|_| lock_err("runtime"))?;
            runtime_guard
                .block_on(state.client.delete_items(item_ids, timeout_ms))
                .map_err(to_py_err)
        })
    }

    #[pyo3(signature = (var_ids, timeout_ms=5000))]
    fn get_values(
        &self,
        py: Python<'_>,
        var_ids: Vec<String>,
        timeout_ms: u64,
    ) -> PyResult<Vec<PyObject>> {
        self.with_state(|state| {
            let runtime_guard = state.runtime.lock().map_err(|_| lock_err("runtime"))?;
            let values = runtime_guard
                .block_on(state.client.get_values(var_ids, timeout_ms))
                .map_err(to_py_err)?;
            Ok(values
                .into_iter()
                .map(|typed| typed_to_object(py, typed))
                .collect())
        })
    }

    #[pyo3(signature = (var_ids, values, timeout_ms=5000))]
    fn set_integer_variables(
        &self,
        var_ids: Vec<String>,
        values: Vec<i64>,
        timeout_ms: u64,
    ) -> PyResult<()> {
        self.with_state(|state| {
            let runtime_guard = state.runtime.lock().map_err(|_| lock_err("runtime"))?;
            runtime_guard
                .block_on(
                    state
                        .client
                        .set_integer_variables(var_ids, values, timeout_ms),
                )
                .map_err(to_py_err)
        })
    }

    #[pyo3(signature = (var_ids, values, timeout_ms=5000))]
    fn set_float_variables(
        &self,
        var_ids: Vec<String>,
        values: Vec<f64>,
        timeout_ms: u64,
    ) -> PyResult<()> {
        self.with_state(|state| {
            let runtime_guard = state.runtime.lock().map_err(|_| lock_err("runtime"))?;
            runtime_guard
                .block_on(
                    state
                        .client
                        .set_float_variables(var_ids, values, timeout_ms),
                )
                .map_err(to_py_err)
        })
    }

    #[pyo3(signature = (var_ids, values, timeout_ms=5000))]
    fn set_text_variables(
        &self,
        var_ids: Vec<String>,
        values: Vec<String>,
        timeout_ms: u64,
    ) -> PyResult<()> {
        self.with_state(|state| {
            let runtime_guard = state.runtime.lock().map_err(|_| lock_err("runtime"))?;
            runtime_guard
                .block_on(state.client.set_text_variables(var_ids, values, timeout_ms))
                .map_err(to_py_err)
        })
    }

    #[pyo3(signature = (var_ids, values, timeout_ms=5000))]
    fn set_boolean_variables(
        &self,
        var_ids: Vec<String>,
        values: Vec<bool>,
        timeout_ms: u64,
    ) -> PyResult<()> {
        self.with_state(|state| {
            let runtime_guard = state.runtime.lock().map_err(|_| lock_err("runtime"))?;
            runtime_guard
                .block_on(
                    state
                        .client
                        .set_boolean_variables(var_ids, values, timeout_ms),
                )
                .map_err(to_py_err)
        })
    }

    #[pyo3(signature = (var_id, patch, timeout_ms=5000))]
    fn edit_variable_metadata(
        &self,
        var_id: String,
        patch: PyRef<'_, VariableMetadataPatch>,
        timeout_ms: u64,
    ) -> PyResult<()> {
        let patch = patch.to_rust();
        self.with_state(|state| {
            let runtime_guard = state.runtime.lock().map_err(|_| lock_err("runtime"))?;
            runtime_guard
                .block_on(
                    state
                        .client
                        .edit_variable_metadata(var_id, patch, timeout_ms),
                )
                .map_err(to_py_err)
        })
    }

    #[pyo3(signature = (json, parent_id=None, timeout_ms=5000))]
    fn create_bulk_from_json(
        &self,
        json: String,
        parent_id: Option<String>,
        timeout_ms: u64,
    ) -> PyResult<()> {
        self.with_state(|state| {
            let runtime_guard = state.runtime.lock().map_err(|_| lock_err("runtime"))?;
            runtime_guard
                .block_on(
                    state
                        .client
                        .create_bulk_from_json(&json, parent_id, timeout_ms),
                )
                .map_err(to_py_err)
        })
    }

    #[pyo3(signature = (var_ids, timeout_ms=5000))]
    fn subscribe_var_values(
        &self,
        var_ids: Vec<String>,
        timeout_ms: u64,
    ) -> PyResult<Subscription> {
        let options = self.options.clone().to_rust();
        let (tx, rx) = mpsc::channel::<(String, Option<value::Typed>)>();

        std::thread::spawn(move || {
            let runtime = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(_) => return,
            };

            let _ = runtime.block_on(async move {
                let client = RsClient::connect_with_options(options).await?;
                let mut stream = client.subscribe_var_values(var_ids, timeout_ms).await?;

                while let Some(item) = stream.next().await {
                    if tx.send(item).is_err() {
                        break;
                    }
                }

                Ok::<(), ClientError>(())
            });
        });

        Ok(Subscription {
            receiver: Mutex::new(Some(rx)),
        })
    }
}

#[pymethods]
impl Subscription {
    #[pyo3(signature = (timeout_ms=None))]
    fn next_event(
        &self,
        py: Python<'_>,
        timeout_ms: Option<u64>,
    ) -> PyResult<Option<(String, PyObject)>> {
        let mut receiver_guard = self
            .receiver
            .lock()
            .map_err(|_| lock_err("subscription receiver"))?;

        let Some(receiver) = receiver_guard.as_mut() else {
            return Ok(None);
        };

        let next = match timeout_ms {
            Some(ms) => match receiver.recv_timeout(Duration::from_millis(ms)) {
                Ok(item) => Some(item),
                Err(mpsc::RecvTimeoutError::Timeout) => return Ok(None),
                Err(mpsc::RecvTimeoutError::Disconnected) => None,
            },
            None => receiver.recv().ok(),
        };

        match next {
            Some((var_id, value)) => Ok(Some((var_id, typed_to_object(py, value)))),
            None => {
                *receiver_guard = None;
                Ok(None)
            }
        }
    }

    fn close(&self) -> PyResult<()> {
        let mut receiver_guard = self
            .receiver
            .lock()
            .map_err(|_| lock_err("subscription receiver"))?;
        *receiver_guard = None;
        Ok(())
    }
}

#[pymodule]
fn _lirays(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BooleanVar>()?;
    m.add_class::<Client>()?;
    m.add_class::<ConnectionOptions>()?;
    m.add_class::<FloatVar>()?;
    m.add_class::<FolderInfo>()?;
    m.add_class::<IntegerVar>()?;
    m.add_class::<Subscription>()?;
    m.add_class::<TextVar>()?;
    m.add_class::<VarInfo>()?;
    m.add_class::<VariableMetadataPatch>()?;
    Ok(())
}
