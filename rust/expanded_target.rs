use pyo3::{pyclass, pymethods, pymodule, types::PyModule, PyResult, Python};

#[pymodule]
#[pyo3(name = "adt_stuff")]
fn py_compnet(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<SimpleStruct>()?;
    m.add_class::<DayOfTheWeek>()?;
    m.add_class::<ComplexEnum>()?;
    Ok(())
}

#[pyclass]
pub struct SimpleStruct {
    pub i: i32,
    pub f: f64,
    pub s: String,
}

pub enum ComplexEnum {
    Int { i: i32 },
    Float { f: f64 },
    Str { s: String },
}

const _: () = {
    use pyo3 as _pyo3;
    unsafe impl _pyo3::type_object::PyTypeInfo for ComplexEnum {
        type AsRefTarget = _pyo3::PyCell<Self>;
        const NAME: &'static str = "ComplexEnum";
        const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
        #[inline]
        fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
            <ComplexEnum as _pyo3::impl_::pyclass::PyClassImpl>::lazy_type_object()
                .get_or_init(py)
                .as_type_ptr()
        }
    }
    impl _pyo3::PyClass for ComplexEnum {
        type Frozen = _pyo3::pyclass::boolean_struct::False;
    }
    impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a ComplexEnum {
        type Holder = ::std::option::Option<_pyo3::PyRef<'py, ComplexEnum>>;
        #[inline]
        fn extract(obj: &'py _pyo3::PyAny, holder: &'a mut Self::Holder) -> _pyo3::PyResult<Self> {
            _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
        }
    }
    impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a mut ComplexEnum {
        type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, ComplexEnum>>;
        #[inline]
        fn extract(obj: &'py _pyo3::PyAny, holder: &'a mut Self::Holder) -> _pyo3::PyResult<Self> {
            _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
        }
    }
    impl _pyo3::IntoPy<_pyo3::PyObject> for ComplexEnum {
        fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
            _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
        }
    }
    impl _pyo3::impl_::pyclass::PyClassImpl for ComplexEnum {
        const IS_BASETYPE: bool = false;
        const IS_SUBCLASS: bool = false;
        const IS_MAPPING: bool = false;
        const IS_SEQUENCE: bool = false;
        type BaseType = _pyo3::PyAny;
        type ThreadChecker = _pyo3::impl_::pyclass::SendablePyClass<ComplexEnum>;
        type PyClassMutability =
                <<_pyo3::PyAny as
                _pyo3::impl_::pyclass::PyClassBaseType>::PyClassMutability as
                _pyo3::impl_::pycell::PyClassMutability>::MutableChild;
        type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
        type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
        type BaseNativeType = _pyo3::PyAny;
        fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
            use _pyo3::impl_::pyclass::*;
            let collector = PyClassImplCollector::<Self>::new();
            static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                methods: &[],
                slots: &[],
            };
            PyClassItemsIter::new(&INTRINSIC_ITEMS, collector.py_methods())
        }
        fn doc(py: _pyo3::Python<'_>) -> _pyo3::PyResult<&'static ::std::ffi::CStr> {
            use _pyo3::impl_::pyclass::*;
            static DOC: _pyo3::once_cell::GILOnceCell<
                ::std::borrow::Cow<'static, ::std::ffi::CStr>,
            > = _pyo3::once_cell::GILOnceCell::new();
            DOC.get_or_try_init(py, || {
                let collector = PyClassImplCollector::<Self>::new();
                build_pyclass_doc(
                    <ComplexEnum as _pyo3::PyTypeInfo>::NAME,
                    "\0",
                    ::std::option::Option::None.or_else(|| collector.new_text_signature()),
                )
            })
            .map(::std::ops::Deref::deref)
        }
        fn lazy_type_object() -> &'static _pyo3::impl_::pyclass::LazyTypeObject<Self> {
            use _pyo3::impl_::pyclass::LazyTypeObject;
            static TYPE_OBJECT: LazyTypeObject<ComplexEnum> = LazyTypeObject::new();
            &TYPE_OBJECT
        }
    }
    impl _pyo3::impl_::pyclass::PyMethods<ComplexEnum>
        for _pyo3::impl_::pyclass::PyClassImplCollector<ComplexEnum>
    {
        fn py_methods(self) -> &'static _pyo3::impl_::pyclass::PyClassItems {
            static ITEMS: _pyo3::impl_::pyclass::PyClassItems =
                _pyo3::impl_::pyclass::PyClassItems {
                    methods: &[
                        // class attributes
                        _pyo3::class::PyMethodDefType::ClassAttribute({
                            _pyo3::class::PyClassAttributeDef::new(
                                "Int\0",
                                _pyo3::impl_::pymethods::PyClassAttributeFactory(
                                    ComplexEnum::__pymethod_attr_Int__,
                                ),
                            )
                        }),
                        _pyo3::class::PyMethodDefType::ClassAttribute({
                            _pyo3::class::PyClassAttributeDef::new(
                                "Float\0",
                                _pyo3::impl_::pymethods::PyClassAttributeFactory(
                                    ComplexEnum::__pymethod_attr_Float__,
                                ),
                            )
                        }),
                        _pyo3::class::PyMethodDefType::ClassAttribute({
                            _pyo3::class::PyClassAttributeDef::new(
                                "Str\0",
                                _pyo3::impl_::pymethods::PyClassAttributeFactory(
                                    ComplexEnum::__pymethod_attr_Str__,
                                ),
                            )
                        }),
                    ],
                    slots: &[],
                };
            &ITEMS
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    impl ComplexEnum {
        // class attributes

        fn __pymethod_attr_Int__(py: _pyo3::Python<'_>) -> _pyo3::PyResult<_pyo3::PyObject> {
            let class = py.get_type::<ComplexEnum_Int>();
            _pyo3::impl_::wrap::OkWrap::wrap(class, py)
                .map_err(::core::convert::Into::<_pyo3::PyErr>::into)
        }

        fn __pymethod_attr_Float__(py: _pyo3::Python<'_>) -> _pyo3::PyResult<_pyo3::PyObject> {
            let class = py.get_type::<ComplexEnum_Float>();
            _pyo3::impl_::wrap::OkWrap::wrap(class, py)
                .map_err(::core::convert::Into::<_pyo3::PyErr>::into)
        }

        fn __pymethod_attr_Str__(py: _pyo3::Python<'_>) -> _pyo3::PyResult<_pyo3::PyObject> {
            let class = py.get_type::<ComplexEnum_Str>();
            _pyo3::impl_::wrap::OkWrap::wrap(class, py)
                .map_err(::core::convert::Into::<_pyo3::PyErr>::into)
        }
    }
};

#[allow(non_camel_case_types)]
struct ComplexEnum_Int(ComplexEnum);

const _: () = {
    use pyo3 as _pyo3;
    unsafe impl _pyo3::type_object::PyTypeInfo for ComplexEnum_Int {
        type AsRefTarget = _pyo3::PyCell<Self>;
        const NAME: &'static str = "ComplexEnum_Int";
        const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
        #[inline]
        fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
            <ComplexEnum_Int as _pyo3::impl_::pyclass::PyClassImpl>::lazy_type_object()
                .get_or_init(py)
                .as_type_ptr()
        }
    }
    impl _pyo3::PyClass for ComplexEnum_Int {
        type Frozen = _pyo3::pyclass::boolean_struct::False;
    }
    impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a ComplexEnum_Int {
        type Holder = ::std::option::Option<_pyo3::PyRef<'py, ComplexEnum_Int>>;
        #[inline]
        fn extract(obj: &'py _pyo3::PyAny, holder: &'a mut Self::Holder) -> _pyo3::PyResult<Self> {
            _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
        }
    }
    impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py>
        for &'a mut ComplexEnum_Int
    {
        type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, ComplexEnum_Int>>;
        #[inline]
        fn extract(obj: &'py _pyo3::PyAny, holder: &'a mut Self::Holder) -> _pyo3::PyResult<Self> {
            _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
        }
    }
    impl _pyo3::IntoPy<_pyo3::PyObject> for ComplexEnum_Int {
        fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
            _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
        }
    }
    impl _pyo3::impl_::pyclass::PyClassImpl for ComplexEnum_Int {
        const IS_BASETYPE: bool = false;
        const IS_SUBCLASS: bool = false;
        const IS_MAPPING: bool = false;
        const IS_SEQUENCE: bool = false;
        type BaseType = _pyo3::PyAny;
        type ThreadChecker = _pyo3::impl_::pyclass::SendablePyClass<ComplexEnum_Int>;
        type PyClassMutability =
                <<_pyo3::PyAny as
                _pyo3::impl_::pyclass::PyClassBaseType>::PyClassMutability as
                _pyo3::impl_::pycell::PyClassMutability>::MutableChild;
        type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
        type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
        type BaseNativeType = _pyo3::PyAny;
        fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
            use _pyo3::impl_::pyclass::*;
            let collector = PyClassImplCollector::<Self>::new();
            static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                methods: &[
                    // getters
                    _pyo3::class::PyMethodDefType::Getter(_pyo3::class::PyGetterDef::new(
                        "i\0",
                        _pyo3::impl_::pymethods::PyGetter(ComplexEnum_Int::__pymethod_get_i__),
                        "\0",
                    )),
                ],
                slots: &[],
            };
            PyClassItemsIter::new(&INTRINSIC_ITEMS, collector.py_methods())
        }
        fn doc(py: _pyo3::Python<'_>) -> _pyo3::PyResult<&'static ::std::ffi::CStr> {
            use _pyo3::impl_::pyclass::*;
            static DOC: _pyo3::once_cell::GILOnceCell<
                ::std::borrow::Cow<'static, ::std::ffi::CStr>,
            > = _pyo3::once_cell::GILOnceCell::new();
            DOC.get_or_try_init(py, || {
                let collector = PyClassImplCollector::<Self>::new();
                build_pyclass_doc(
                    <ComplexEnum_Int as _pyo3::PyTypeInfo>::NAME,
                    "\0",
                    ::std::option::Option::None.or_else(|| collector.new_text_signature()),
                )
            })
            .map(::std::ops::Deref::deref)
        }
        fn lazy_type_object() -> &'static _pyo3::impl_::pyclass::LazyTypeObject<Self> {
            use _pyo3::impl_::pyclass::LazyTypeObject;
            static TYPE_OBJECT: LazyTypeObject<ComplexEnum_Int> = LazyTypeObject::new();
            &TYPE_OBJECT
        }
    }
    impl _pyo3::impl_::pyclass::PyMethods<ComplexEnum_Int>
        for _pyo3::impl_::pyclass::PyClassImplCollector<ComplexEnum_Int>
    {
        fn py_methods(self) -> &'static _pyo3::impl_::pyclass::PyClassItems {
            static ITEMS: _pyo3::impl_::pyclass::PyClassItems =
                _pyo3::impl_::pyclass::PyClassItems {
                    methods: &[],
                    slots: &[_pyo3::ffi::PyType_Slot {
                        slot: _pyo3::ffi::Py_tp_new,
                        pfunc: {
                            unsafe extern "C" fn trampoline(
                                subtype: *mut _pyo3::ffi::PyTypeObject,
                                args: *mut _pyo3::ffi::PyObject,
                                kwargs: *mut _pyo3::ffi::PyObject,
                            ) -> *mut _pyo3::ffi::PyObject {
                                use _pyo3::impl_::pyclass::*;
                                impl PyClassNewTextSignature<ComplexEnum_Int> for PyClassImplCollector<ComplexEnum_Int> {
                                    #[inline]
                                    fn new_text_signature(
                                        self,
                                    ) -> ::std::option::Option<&'static str>
                                    {
                                        ::std::option::Option::Some("(i)")
                                    }
                                }
                                _pyo3::impl_::trampoline::newfunc(
                                    subtype,
                                    args,
                                    kwargs,
                                    ComplexEnum_Int::__pymethod___new____,
                                )
                            }
                            trampoline
                        } as _pyo3::ffi::newfunc as _,
                    }],
                };
            &ITEMS
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    impl ComplexEnum_Int {
        // constructor

        unsafe fn __pymethod___new____(
            py: _pyo3::Python<'_>,
            subtype: *mut _pyo3::ffi::PyTypeObject,
            _args: *mut _pyo3::ffi::PyObject,
            _kwargs: *mut _pyo3::ffi::PyObject,
        ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
            use _pyo3::callback::IntoPyCallbackOutput;
            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                _pyo3::impl_::extract_argument::FunctionDescription {
                    cls_name: ::std::option::Option::Some(
                        <SimpleStruct as _pyo3::type_object::PyTypeInfo>::NAME,
                    ),
                    func_name: "__new__",
                    positional_parameter_names: &["i"],
                    positional_only_parameters: 0usize,
                    required_positional_parameters: 1usize,
                    keyword_only_parameters: &[],
                };
            let mut output = [::std::option::Option::None; 1usize];
            let (_args, _kwargs) =
                    DESCRIPTION.extract_arguments_tuple_dict::<_pyo3::impl_::extract_argument::NoVarargs,
                            _pyo3::impl_::extract_argument::NoVarkeywords>(py, _args,
                            _kwargs, &mut output)?;

            let i = _pyo3::impl_::extract_argument::extract_argument(
                _pyo3::impl_::extract_argument::unwrap_required_argument(output[0usize]),
                &mut { _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT },
                "i",
            )?;

            let result = ComplexEnum_Int(ComplexEnum::Int { i });

            let initializer: _pyo3::PyClassInitializer<ComplexEnum_Int> = result.convert(py)?;
            let cell = initializer.create_cell_from_subtype(py, subtype)?;
            ::std::result::Result::Ok(cell as *mut _pyo3::ffi::PyObject)
        }

        // getters

        unsafe fn __pymethod_get_i__(
            py: _pyo3::Python<'_>,
            _slf: *mut _pyo3::ffi::PyObject,
        ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
            let mut ffi_obj = _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT;

            let inner_self = _pyo3::impl_::extract_argument::extract_pyclass_ref::<ComplexEnum_Int>(
                py.from_borrowed_ptr::<_pyo3::PyAny>(_slf),
                &mut ffi_obj,
            )?;

            let py_result = match &inner_self.0 {
                ComplexEnum::Int { i } => {
                    _pyo3::impl_::wrap::OkWrap::wrap(::std::clone::Clone::clone(i), py)
                }
                ComplexEnum::Float { .. } => Err(_pyo3::exceptions::PyAttributeError::new_err(
                    "`ComplexEnum::Float` has no field `i`",
                )),
                ComplexEnum::Str { .. } => Err(_pyo3::exceptions::PyAttributeError::new_err(
                    "`ComplexEnum::Str` has no field `i`",
                )),
            };

            py_result
                .map_err(::core::convert::Into::<_pyo3::PyErr>::into)
                .map(_pyo3::PyObject::into_ptr)
        }
    }
};

#[allow(non_camel_case_types)]
struct ComplexEnum_Float(ComplexEnum);

const _: () = {
    use pyo3 as _pyo3;
    unsafe impl _pyo3::type_object::PyTypeInfo for ComplexEnum_Float {
        type AsRefTarget = _pyo3::PyCell<Self>;
        const NAME: &'static str = "ComplexEnum_Float";
        const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
        #[inline]
        fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
            <ComplexEnum_Float as _pyo3::impl_::pyclass::PyClassImpl>::lazy_type_object()
                .get_or_init(py)
                .as_type_ptr()
        }
    }
    impl _pyo3::PyClass for ComplexEnum_Float {
        type Frozen = _pyo3::pyclass::boolean_struct::False;
    }
    impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py>
        for &'a ComplexEnum_Float
    {
        type Holder = ::std::option::Option<_pyo3::PyRef<'py, ComplexEnum_Float>>;
        #[inline]
        fn extract(obj: &'py _pyo3::PyAny, holder: &'a mut Self::Holder) -> _pyo3::PyResult<Self> {
            _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
        }
    }
    impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py>
        for &'a mut ComplexEnum_Float
    {
        type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, ComplexEnum_Float>>;
        #[inline]
        fn extract(obj: &'py _pyo3::PyAny, holder: &'a mut Self::Holder) -> _pyo3::PyResult<Self> {
            _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
        }
    }
    impl _pyo3::IntoPy<_pyo3::PyObject> for ComplexEnum_Float {
        fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
            _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
        }
    }
    impl _pyo3::impl_::pyclass::PyClassImpl for ComplexEnum_Float {
        const IS_BASETYPE: bool = false;
        const IS_SUBCLASS: bool = false;
        const IS_MAPPING: bool = false;
        const IS_SEQUENCE: bool = false;
        type BaseType = _pyo3::PyAny;
        type ThreadChecker = _pyo3::impl_::pyclass::SendablePyClass<ComplexEnum_Float>;
        type PyClassMutability =
                <<_pyo3::PyAny as
                _pyo3::impl_::pyclass::PyClassBaseType>::PyClassMutability as
                _pyo3::impl_::pycell::PyClassMutability>::MutableChild;
        type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
        type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
        type BaseNativeType = _pyo3::PyAny;
        fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
            use _pyo3::impl_::pyclass::*;
            let collector = PyClassImplCollector::<Self>::new();
            static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                methods: &[
                    // getters
                    _pyo3::class::PyMethodDefType::Getter(_pyo3::class::PyGetterDef::new(
                        "f\0",
                        _pyo3::impl_::pymethods::PyGetter(ComplexEnum_Float::__pymethod_get_f__),
                        "\0",
                    )),
                ],
                slots: &[],
            };
            PyClassItemsIter::new(&INTRINSIC_ITEMS, collector.py_methods())
        }
        fn doc(py: _pyo3::Python<'_>) -> _pyo3::PyResult<&'static ::std::ffi::CStr> {
            use _pyo3::impl_::pyclass::*;
            static DOC: _pyo3::once_cell::GILOnceCell<
                ::std::borrow::Cow<'static, ::std::ffi::CStr>,
            > = _pyo3::once_cell::GILOnceCell::new();
            DOC.get_or_try_init(py, || {
                let collector = PyClassImplCollector::<Self>::new();
                build_pyclass_doc(
                    <ComplexEnum_Float as _pyo3::PyTypeInfo>::NAME,
                    "\0",
                    ::std::option::Option::None.or_else(|| collector.new_text_signature()),
                )
            })
            .map(::std::ops::Deref::deref)
        }
        fn lazy_type_object() -> &'static _pyo3::impl_::pyclass::LazyTypeObject<Self> {
            use _pyo3::impl_::pyclass::LazyTypeObject;
            static TYPE_OBJECT: LazyTypeObject<ComplexEnum_Float> = LazyTypeObject::new();
            &TYPE_OBJECT
        }
    }
    impl _pyo3::impl_::pyclass::PyMethods<ComplexEnum_Float>
        for _pyo3::impl_::pyclass::PyClassImplCollector<ComplexEnum_Float>
    {
        fn py_methods(self) -> &'static _pyo3::impl_::pyclass::PyClassItems {
            static ITEMS: _pyo3::impl_::pyclass::PyClassItems =
                _pyo3::impl_::pyclass::PyClassItems {
                    methods: &[],
                    slots: &[_pyo3::ffi::PyType_Slot {
                        slot: _pyo3::ffi::Py_tp_new,
                        pfunc: {
                            unsafe extern "C" fn trampoline(
                                subtype: *mut _pyo3::ffi::PyTypeObject,
                                args: *mut _pyo3::ffi::PyObject,
                                kwargs: *mut _pyo3::ffi::PyObject,
                            ) -> *mut _pyo3::ffi::PyObject {
                                use _pyo3::impl_::pyclass::*;
                                impl PyClassNewTextSignature<ComplexEnum_Float> for PyClassImplCollector<ComplexEnum_Float> {
                                    #[inline]
                                    fn new_text_signature(
                                        self,
                                    ) -> ::std::option::Option<&'static str>
                                    {
                                        ::std::option::Option::Some("(f)")
                                    }
                                }
                                _pyo3::impl_::trampoline::newfunc(
                                    subtype,
                                    args,
                                    kwargs,
                                    ComplexEnum_Float::__pymethod___new____,
                                )
                            }
                            trampoline
                        } as _pyo3::ffi::newfunc as _,
                    }],
                };
            &ITEMS
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    impl ComplexEnum_Float {
        // constructor

        unsafe fn __pymethod___new____(
            py: _pyo3::Python<'_>,
            subtype: *mut _pyo3::ffi::PyTypeObject,
            _args: *mut _pyo3::ffi::PyObject,
            _kwargs: *mut _pyo3::ffi::PyObject,
        ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
            use _pyo3::callback::IntoPyCallbackOutput;
            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                _pyo3::impl_::extract_argument::FunctionDescription {
                    cls_name: ::std::option::Option::Some(
                        <SimpleStruct as _pyo3::type_object::PyTypeInfo>::NAME,
                    ),
                    func_name: "__new__",
                    positional_parameter_names: &["f"],
                    positional_only_parameters: 0usize,
                    required_positional_parameters: 1usize,
                    keyword_only_parameters: &[],
                };
            let mut output = [::std::option::Option::None; 1usize];
            let (_args, _kwargs) =
                    DESCRIPTION.extract_arguments_tuple_dict::<_pyo3::impl_::extract_argument::NoVarargs,
                            _pyo3::impl_::extract_argument::NoVarkeywords>(py, _args,
                            _kwargs, &mut output)?;

            let f = _pyo3::impl_::extract_argument::extract_argument(
                _pyo3::impl_::extract_argument::unwrap_required_argument(output[0usize]),
                &mut { _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT },
                "f",
            )?;

            let result = ComplexEnum_Float(ComplexEnum::Float { f });

            let initializer: _pyo3::PyClassInitializer<ComplexEnum_Float> = result.convert(py)?;
            let cell = initializer.create_cell_from_subtype(py, subtype)?;
            ::std::result::Result::Ok(cell as *mut _pyo3::ffi::PyObject)
        }

        // getters

        unsafe fn __pymethod_get_f__(
            py: _pyo3::Python<'_>,
            _slf: *mut _pyo3::ffi::PyObject,
        ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
            let mut ffi_obj = _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT;

            let inner_self = _pyo3::impl_::extract_argument::extract_pyclass_ref::<
                ComplexEnum_Float,
            >(
                py.from_borrowed_ptr::<_pyo3::PyAny>(_slf), &mut ffi_obj
            )?;

            let py_result = match &inner_self.0 {
                ComplexEnum::Int { .. } => Err(_pyo3::exceptions::PyAttributeError::new_err(
                    "`ComplexEnum::Float` has no field `f`",
                )),
                ComplexEnum::Float { f } => {
                    _pyo3::impl_::wrap::OkWrap::wrap(::std::clone::Clone::clone(f), py)
                }
                ComplexEnum::Str { .. } => Err(_pyo3::exceptions::PyAttributeError::new_err(
                    "`ComplexEnum::Str` has no field `f`",
                )),
            };

            py_result
                .map_err(::core::convert::Into::<_pyo3::PyErr>::into)
                .map(_pyo3::PyObject::into_ptr)
        }
    }
};

#[allow(non_camel_case_types)]
struct ComplexEnum_Str(ComplexEnum);

const _: () = {
    use pyo3 as _pyo3;
    unsafe impl _pyo3::type_object::PyTypeInfo for ComplexEnum_Str {
        type AsRefTarget = _pyo3::PyCell<Self>;
        const NAME: &'static str = "ComplexEnum_Str";
        const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
        #[inline]
        fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
            <ComplexEnum_Str as _pyo3::impl_::pyclass::PyClassImpl>::lazy_type_object()
                .get_or_init(py)
                .as_type_ptr()
        }
    }
    impl _pyo3::PyClass for ComplexEnum_Str {
        type Frozen = _pyo3::pyclass::boolean_struct::False;
    }
    impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a ComplexEnum_Str {
        type Holder = ::std::option::Option<_pyo3::PyRef<'py, ComplexEnum_Str>>;
        #[inline]
        fn extract(obj: &'py _pyo3::PyAny, holder: &'a mut Self::Holder) -> _pyo3::PyResult<Self> {
            _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
        }
    }
    impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py>
        for &'a mut ComplexEnum_Str
    {
        type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, ComplexEnum_Str>>;
        #[inline]
        fn extract(obj: &'py _pyo3::PyAny, holder: &'a mut Self::Holder) -> _pyo3::PyResult<Self> {
            _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
        }
    }
    impl _pyo3::IntoPy<_pyo3::PyObject> for ComplexEnum_Str {
        fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
            _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
        }
    }
    impl _pyo3::impl_::pyclass::PyClassImpl for ComplexEnum_Str {
        const IS_BASETYPE: bool = false;
        const IS_SUBCLASS: bool = false;
        const IS_MAPPING: bool = false;
        const IS_SEQUENCE: bool = false;
        type BaseType = _pyo3::PyAny;
        type ThreadChecker = _pyo3::impl_::pyclass::SendablePyClass<ComplexEnum_Str>;
        type PyClassMutability =
                <<_pyo3::PyAny as
                _pyo3::impl_::pyclass::PyClassBaseType>::PyClassMutability as
                _pyo3::impl_::pycell::PyClassMutability>::MutableChild;
        type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
        type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
        type BaseNativeType = _pyo3::PyAny;
        fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
            use _pyo3::impl_::pyclass::*;
            let collector = PyClassImplCollector::<Self>::new();
            static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                methods: &[
                    // getters
                    _pyo3::class::PyMethodDefType::Getter(_pyo3::class::PyGetterDef::new(
                        "s\0",
                        _pyo3::impl_::pymethods::PyGetter(ComplexEnum_Str::__pymethod_get_s__),
                        "\0",
                    )),
                ],
                slots: &[],
            };
            PyClassItemsIter::new(&INTRINSIC_ITEMS, collector.py_methods())
        }
        fn doc(py: _pyo3::Python<'_>) -> _pyo3::PyResult<&'static ::std::ffi::CStr> {
            use _pyo3::impl_::pyclass::*;
            static DOC: _pyo3::once_cell::GILOnceCell<
                ::std::borrow::Cow<'static, ::std::ffi::CStr>,
            > = _pyo3::once_cell::GILOnceCell::new();
            DOC.get_or_try_init(py, || {
                let collector = PyClassImplCollector::<Self>::new();
                build_pyclass_doc(
                    <ComplexEnum_Str as _pyo3::PyTypeInfo>::NAME,
                    "\0",
                    ::std::option::Option::None.or_else(|| collector.new_text_signature()),
                )
            })
            .map(::std::ops::Deref::deref)
        }
        fn lazy_type_object() -> &'static _pyo3::impl_::pyclass::LazyTypeObject<Self> {
            use _pyo3::impl_::pyclass::LazyTypeObject;
            static TYPE_OBJECT: LazyTypeObject<ComplexEnum_Str> = LazyTypeObject::new();
            &TYPE_OBJECT
        }
    }
    impl _pyo3::impl_::pyclass::PyMethods<ComplexEnum_Str>
        for _pyo3::impl_::pyclass::PyClassImplCollector<ComplexEnum_Str>
    {
        fn py_methods(self) -> &'static _pyo3::impl_::pyclass::PyClassItems {
            static ITEMS: _pyo3::impl_::pyclass::PyClassItems =
                _pyo3::impl_::pyclass::PyClassItems {
                    methods: &[],
                    slots: &[_pyo3::ffi::PyType_Slot {
                        slot: _pyo3::ffi::Py_tp_new,
                        pfunc: {
                            unsafe extern "C" fn trampoline(
                                subtype: *mut _pyo3::ffi::PyTypeObject,
                                args: *mut _pyo3::ffi::PyObject,
                                kwargs: *mut _pyo3::ffi::PyObject,
                            ) -> *mut _pyo3::ffi::PyObject {
                                use _pyo3::impl_::pyclass::*;
                                impl PyClassNewTextSignature<ComplexEnum_Str> for PyClassImplCollector<ComplexEnum_Str> {
                                    #[inline]
                                    fn new_text_signature(
                                        self,
                                    ) -> ::std::option::Option<&'static str>
                                    {
                                        ::std::option::Option::Some("(s)")
                                    }
                                }
                                _pyo3::impl_::trampoline::newfunc(
                                    subtype,
                                    args,
                                    kwargs,
                                    ComplexEnum_Str::__pymethod___new____,
                                )
                            }
                            trampoline
                        } as _pyo3::ffi::newfunc as _,
                    }],
                };
            &ITEMS
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    impl ComplexEnum_Str {
        // constructor

        unsafe fn __pymethod___new____(
            py: _pyo3::Python<'_>,
            subtype: *mut _pyo3::ffi::PyTypeObject,
            _args: *mut _pyo3::ffi::PyObject,
            _kwargs: *mut _pyo3::ffi::PyObject,
        ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
            use _pyo3::callback::IntoPyCallbackOutput;
            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                _pyo3::impl_::extract_argument::FunctionDescription {
                    cls_name: ::std::option::Option::Some(
                        <SimpleStruct as _pyo3::type_object::PyTypeInfo>::NAME,
                    ),
                    func_name: "__new__",
                    positional_parameter_names: &["s"],
                    positional_only_parameters: 0usize,
                    required_positional_parameters: 1usize,
                    keyword_only_parameters: &[],
                };
            let mut output = [::std::option::Option::None; 1usize];
            let (_args, _kwargs) =
                    DESCRIPTION.extract_arguments_tuple_dict::<_pyo3::impl_::extract_argument::NoVarargs,
                            _pyo3::impl_::extract_argument::NoVarkeywords>(py, _args,
                            _kwargs, &mut output)?;

            let s = _pyo3::impl_::extract_argument::extract_argument(
                _pyo3::impl_::extract_argument::unwrap_required_argument(output[0usize]),
                &mut { _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT },
                "s",
            )?;

            let result = ComplexEnum_Str(ComplexEnum::Str { s });

            let initializer: _pyo3::PyClassInitializer<ComplexEnum_Str> = result.convert(py)?;
            let cell = initializer.create_cell_from_subtype(py, subtype)?;
            ::std::result::Result::Ok(cell as *mut _pyo3::ffi::PyObject)
        }

        // getters

        unsafe fn __pymethod_get_s__(
            py: _pyo3::Python<'_>,
            _slf: *mut _pyo3::ffi::PyObject,
        ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
            let mut ffi_obj = _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT;

            let inner_self = _pyo3::impl_::extract_argument::extract_pyclass_ref::<ComplexEnum_Str>(
                py.from_borrowed_ptr::<_pyo3::PyAny>(_slf),
                &mut ffi_obj,
            )?;

            let py_result = match &inner_self.0 {
                ComplexEnum::Int { .. } => Err(_pyo3::exceptions::PyAttributeError::new_err(
                    "`ComplexEnum::Str` has no field `s`",
                )),
                ComplexEnum::Float { .. } => Err(_pyo3::exceptions::PyAttributeError::new_err(
                    "`ComplexEnum::Str` has no field `s`",
                )),
                ComplexEnum::Str { s } => {
                    _pyo3::impl_::wrap::OkWrap::wrap(::std::clone::Clone::clone(s), py)
                }
            };

            py_result
                .map_err(::core::convert::Into::<_pyo3::PyErr>::into)
                .map(_pyo3::PyObject::into_ptr)
        }
    }
};

#[pyclass]
pub enum DayOfTheWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
