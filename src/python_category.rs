use pyo3::types::PyString;
use pyo3::types::PyList;
use pyo3::prelude::*;

use crate::category::Category;

#[derive(Debug)]
pub enum PythonError {
    InternalError,
    ScriptError,
    IncorrectReturnType(&'static str),
    MiscError(&'static str)
}

impl From<PyErr> for PythonError {
    fn from(e: PyErr) -> PythonError {
        let gil = Python::acquire_gil();
        let py = gil.python();
        e.print(py);
        PythonError::ScriptError
    }
}

impl From<std::io::Error> for PythonError {
    fn from(_: std::io::Error) -> PythonError {
        PythonError::MiscError("Could not get the current directory!")
    }
}

pub struct PythonCategory {
    module_object: PyObject
}

impl PythonCategory {
    pub fn new(module_name: &str) -> Result<PythonCategory, PythonError> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let syspath: &PyList = py.import("sys")?
            .get("path")?
            .try_into()
            .map_err(|_| PythonError::IncorrectReturnType("sys.path should be a list!"))?;

        let cwd = std::env::current_dir()?;
        let cwd = cwd.to_str().ok_or(PythonError::MiscError("No working directory found"))?;

        syspath.insert(0, cwd)?;

        let module = py.import(module_name)?;

        Ok(PythonCategory{
            module_object: module.to_object(py)
        })
    }
}

impl Category for PythonCategory {
    type Error = PythonError;

    fn name(&self) -> Result<String, PythonError> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let module = self.module_object
            .cast_as::<PyModule>(py)
            .map_err(|_| PythonError::InternalError)?;

        let name_any = module.call1("get_name", ())?;

        let name = name_any
            .downcast::<PyString>()
            .map_err(|_| PythonError::IncorrectReturnType("The category name must be a string"))?;

        Ok(name.to_string()?.to_string())
    }

    fn get_entries(&self) -> Result<Vec<String>, PythonError> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let module = self.module_object
            .cast_as::<PyModule>(py)
            .map_err(|_| PythonError::InternalError)?;

        let entries_any = module.call1("get_entries", ())?;

        let entries_list = entries_any.downcast::<PyList>()
            .map_err(|_| PythonError::IncorrectReturnType("You must return a list of entries"))?;

        let mut entries = Vec::new();

        for entry_any in entries_list.iter() {
            let entry = entry_any.downcast::<PyString>()
                .map_err(|_| PythonError::IncorrectReturnType("Each entry must be a string"))?;

            entries.push(entry.to_string()?.to_string());
        }

        Ok(entries)
    }

    fn launch(&self, entry: &String) -> Result<(), PythonError> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let module = self.module_object
            .cast_as::<PyModule>(py)
            .map_err(|_| PythonError::InternalError)?;

        let entry_py = PyString::new(py, entry);

        module.call1("launch_entry", (entry_py,))?;

        Ok(())
    }
}