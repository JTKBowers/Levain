use pyo3::types::PyString;
use pyo3::types::PyList;
use pyo3::prelude::*;

use crate::category::Category;

#[derive(Debug)]
pub enum PythonError {
    InternalError,
    ScriptError,
    IncorrectReturnType
}

impl From<PyErr> for PythonError {
    fn from(e: PyErr) -> PythonError {
        let gil = Python::acquire_gil();
        let py = gil.python();
        e.print(py);
        PythonError::ScriptError
    }
}

pub struct PythonCategory {
    module_object: PyObject
}

impl PythonCategory {
    pub fn new(module_name: &str) -> PythonCategory {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let syspath: &PyList = py.import("sys")
            .unwrap()
            .get("path")
            .unwrap()
            .try_into()
            .unwrap();

        let cwd = std::env::current_dir().unwrap();

        syspath.insert(0, cwd.to_str().unwrap()).unwrap();

        let module = py.import(module_name).unwrap();

        PythonCategory{
            module_object: module.to_object(py)
        }
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
            .map_err(|_| PythonError::IncorrectReturnType)?;

        Ok(name.to_string().unwrap().to_string())
    }

    fn get_entries(&self) -> Result<Vec<String>, PythonError> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let module = self.module_object
            .cast_as::<PyModule>(py)
            .map_err(|_| PythonError::InternalError)?;

        let entries_any = module.call1("get_entries", ())?;

        let entries_list = entries_any.downcast::<PyList>()
            .map_err(|_| PythonError::IncorrectReturnType)?;

        let mut entries = Vec::new();

        for entry_any in entries_list.iter() {
            let entry = entry_any.downcast::<PyString>()
                .map_err(|_| PythonError::IncorrectReturnType)?;

            entries.push(entry.to_string().unwrap().to_string());
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