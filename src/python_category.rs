use pyo3::types::PyString;
use pyo3::types::PyList;
use pyo3::prelude::*;

use crate::category::Category;

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
    type Error = ();

    fn name(&self) -> Result<String, ()> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let module = self.module_object.cast_as::<PyModule>(py).unwrap();

        let name_any = module.call1("get_name", ()).unwrap();

        let name = name_any.downcast::<PyString>().unwrap();

        Ok(name.to_string().unwrap().to_string())
    }

    fn get_entries(&self) -> Result<Vec<String>, ()> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let module = self.module_object.cast_as::<PyModule>(py).unwrap();

        let entries_any = module.call1("get_entries", ()).unwrap();

        let entries_list = entries_any.downcast::<PyList>().unwrap();

        let mut entries = Vec::new();

        for entry_any in entries_list.iter() {
            let entry = entry_any.downcast::<PyString>().unwrap();

            entries.push(entry.to_string().unwrap().to_string());
        }

        Ok(entries)
    }

    fn launch(&self, entry: &String) -> Result<(), ()> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let module = self.module_object.cast_as::<PyModule>(py).unwrap();

        let entry_py = PyString::new(py, entry);

        module.call1("launch_entry", (entry_py,)).unwrap();

        Ok(())
    }
}