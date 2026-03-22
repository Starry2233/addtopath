mod utils;
use pyo3::{prelude::*, types::PyString};

#[pyfunction(signature = (key, value, check = true))]
pub fn set_process_env(key: &str, value: &str, check: bool) -> PyResult<Option<bool>> {
    if check { 
        unsafe { utils::set_process_env(key, value); }
        Ok(None)
    }
    else {
        Ok(Some(utils::set_process_env_safe(key, value)))
    }
}

#[pyfunction(signature = (key, value, check = true))]
pub fn set_user_env(key: &str, value: &str, check: bool) -> PyResult<Option<bool>> {
    if check { 
        unsafe { utils::set_user_env(key, value); }
        Ok(None)
    }
    else {
        Ok(Some(utils::set_user_env_safe(key, value)))
    }
}

#[pyfunction(signature = (key, value, check = true))]
pub fn set_system_env(key: &str, value: &str, check: bool) -> PyResult<Option<bool>> {
    if check { 
        unsafe { utils::set_system_env(key, value); }
        Ok(None)
    }
    else {
        Ok(Some(utils::set_system_env_safe(key, value)))
    }
}

#[pyfunction(signature = (key, default = None))]
pub fn get_process_env(
    key: &str,
    default: Option<PyObject>,
    py: Python,
) -> PyResult<PyObject> {
    match utils::get_process_env(key) {
        Ok(val) => Ok(PyString::new(py, &val).into()),
        Err(_) => match default {
            Some(obj) => Ok(obj),
            None => Ok(py.None()),
        },
    }
}

#[pyfunction(signature = (key, default = None))]
pub fn get_user_env(
    key: &str,
    default: Option<PyObject>,
    py: Python,
) -> PyResult<PyObject> {
    match utils::get_user_env(key) {
        Ok(val) => Ok(PyString::new(py, &val).into()),
        Err(_) => match default {
            Some(obj) => Ok(obj),
            None => Ok(py.None()),
        },
    }
}

#[pyfunction(signature = (key, default = None))]
pub fn get_system_env(
    key: &str,
    default: Option<PyObject>,
    py: Python,
) -> PyResult<PyObject> {
    match utils::get_system_env(key) {
        Ok(val) => Ok(PyString::new(py, &val).into()),
        Err(_) => match default {
            Some(obj) => Ok(obj),
            None => Ok(py.None()),
        },
    }
}

#[pymodule]
fn myenv(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_process_env, m)?)?;
    m.add_function(wrap_pyfunction!(set_user_env, m)?)?;
    m.add_function(wrap_pyfunction!(set_system_env, m)?)?;
    m.add_function(wrap_pyfunction!(get_process_env, m)?)?;
    m.add_function(wrap_pyfunction!(get_user_env, m)?)?;
    m.add_function(wrap_pyfunction!(get_system_env, m)?)?;

    Ok(())
}
