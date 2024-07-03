/**************************************************************************/
/*  interface.rs                                                          */
/**************************************************************************/
/*                         This file is part of:                          */
/*                             GODOT ENGINE                               */
/*                        https://godotengine.org                         */
/**************************************************************************/
/* Copyright (c) 2014-present Godot Engine contributors (see AUTHORS.md). */
/* Copyright (c) 2007-2014 Juan Linietsky, Ariel Manzur.                  */
/*                                                                        */
/* Permission is hereby granted, free of charge, to any person obtaining  */
/* a copy of this software and associated documentation files (the        */
/* "Software"), to deal in the Software without restriction, including    */
/* without limitation the rights to use, copy, modify, merge, publish,    */
/* distribute, sublicense, and/or sell copies of the Software, and to     */
/* permit persons to whom the Software is furnished to do so, subject to  */
/* the following conditions:                                              */
/*                                                                        */
/* The above copyright notice and this permission notice shall be         */
/* included in all copies or substantial portions of the Software.        */
/*                                                                        */
/* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,        */
/* EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF     */
/* MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. */
/* IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY   */
/* CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,   */
/* TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE      */
/* SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.                 */
/**************************************************************************/

use std::{alloc::GlobalAlloc, mem::MaybeUninit};

use crate::gdextension::{bindings::{GDExtensionInterfaceGetNativeStructSize, GDExtensionInterfaceGetVariantFromTypeConstructor, GDExtensionInterfaceGetVariantToTypeConstructor, GDExtensionInterfaceVariantBooleanize, GDExtensionInterfaceVariantCall, GDExtensionInterfaceVariantCallStatic, GDExtensionInterfaceVariantCanConvert, GDExtensionInterfaceVariantCanConvertStrict, GDExtensionInterfaceVariantConstruct, GDExtensionInterfaceVariantDestroy, GDExtensionInterfaceVariantDuplicate, GDExtensionInterfaceVariantEvaluate, GDExtensionInterfaceVariantGet, GDExtensionInterfaceVariantGetConstantValue, GDExtensionInterfaceVariantGetIndexed, GDExtensionInterfaceVariantGetKeyed, GDExtensionInterfaceVariantGetNamed, GDExtensionInterfaceVariantGetPtrBuiltinMethod, GDExtensionInterfaceVariantGetPtrConstructor, GDExtensionInterfaceVariantGetPtrDestructor, GDExtensionInterfaceVariantGetPtrGetter, GDExtensionInterfaceVariantGetPtrIndexedGetter, GDExtensionInterfaceVariantGetPtrIndexedSetter, GDExtensionInterfaceVariantGetPtrKeyedChecker, GDExtensionInterfaceVariantGetPtrKeyedGetter, GDExtensionInterfaceVariantGetPtrOperatorEvaluator, GDExtensionInterfaceVariantGetPtrSetter, GDExtensionInterfaceVariantGetPtrUtilityFunction, GDExtensionInterfaceVariantGetType, GDExtensionInterfaceVariantGetTypeName, GDExtensionInterfaceVariantHasKey, GDExtensionInterfaceVariantHasMember, GDExtensionInterfaceVariantHasMethod, GDExtensionInterfaceVariantHash, GDExtensionInterfaceVariantHashCompare, GDExtensionInterfaceVariantIterGet, GDExtensionInterfaceVariantIterInit, GDExtensionInterfaceVariantIterNext, GDExtensionInterfaceVariantNewCopy, GDExtensionInterfaceVariantNewNil, GDExtensionInterfaceVariantRecursiveHash, GDExtensionInterfaceVariantSet, GDExtensionInterfaceVariantSetIndexed, GDExtensionInterfaceVariantSetKeyed, GDExtensionInterfaceVariantSetNamed, GDExtensionInterfaceVariantStringify}, Variant};

use super::bindings::{get_gdextension_func, GDExtensionBool, GDExtensionClassLibraryPtr, GDExtensionInitialization, GDExtensionInterfaceGetProcAddress, GDExtensionInterfaceMemAlloc, GDExtensionInterfaceMemFree, GDExtensionInterfacePrintError, GDExtensionInterfacePrintErrorWithMessage, GDExtensionInterfacePrintScriptError, GDExtensionInterfacePrintScriptErrorWithMessage, GDExtensionInterfacePrintWarning, GDExtensionInterfacePrintWarningWithMessage};

pub(super) struct GDExtensionInterface {
    pub(super) class_library: GDExtensionClassLibraryPtr,

    /* Struct sizes */
    pub(super) variant_size: usize,
    
    /* Mem alloc */
    pub(super) gdextension_interface_mem_alloc: Option<GDExtensionInterfaceMemAlloc>,
    pub(super) gdextension_interface_mem_free: Option<GDExtensionInterfaceMemFree>,

    /* Variant */
    pub(super) gdextension_interface_variant_new_copy: Option<GDExtensionInterfaceVariantNewCopy>,
    pub(super) gdextension_interface_variant_new_nil: Option<GDExtensionInterfaceVariantNewNil>,
    pub(super) gdextension_interface_variant_destroy: Option<GDExtensionInterfaceVariantDestroy>,
    pub(super) gdextension_interface_variant_call: Option<GDExtensionInterfaceVariantCall>,
    pub(super) gdextension_interface_variant_call_static: Option<GDExtensionInterfaceVariantCallStatic>,
    pub(super) gdextension_interface_variant_evaluate: Option<GDExtensionInterfaceVariantEvaluate>,
    pub(super) gdextension_interface_variant_set: Option<GDExtensionInterfaceVariantSet>,
    pub(super) gdextension_interface_variant_set_named: Option<GDExtensionInterfaceVariantSetNamed>,
    pub(super) gdextension_interface_variant_set_keyed: Option<GDExtensionInterfaceVariantSetKeyed>,
    pub(super) gdextension_interface_variant_set_indexed: Option<GDExtensionInterfaceVariantSetIndexed>,
    pub(super) gdextension_interface_variant_get: Option<GDExtensionInterfaceVariantGet>,
    pub(super) gdextension_interface_variant_get_named: Option<GDExtensionInterfaceVariantGetNamed>,
    pub(super) gdextension_interface_variant_get_keyed: Option<GDExtensionInterfaceVariantGetKeyed>,
    pub(super) gdextension_interface_variant_get_indexed: Option<GDExtensionInterfaceVariantGetIndexed>,
    pub(super) gdextension_interface_variant_iter_init: Option<GDExtensionInterfaceVariantIterInit>,
    pub(super) gdextension_interface_variant_iter_next: Option<GDExtensionInterfaceVariantIterNext>,
    pub(super) gdextension_interface_variant_iter_get: Option<GDExtensionInterfaceVariantIterGet>,
    pub(super) gdextension_interface_variant_hash: Option<GDExtensionInterfaceVariantHash>,
    pub(super) gdextension_interface_variant_recursive_hash: Option<GDExtensionInterfaceVariantRecursiveHash>,
    pub(super) gdextension_interface_variant_hash_compare: Option<GDExtensionInterfaceVariantHashCompare>,
    pub(super) gdextension_interface_variant_booleanize: Option<GDExtensionInterfaceVariantBooleanize>,
    pub(super) gdextension_interface_variant_duplicate: Option<GDExtensionInterfaceVariantDuplicate>,
    pub(super) gdextension_interface_variant_stringify: Option<GDExtensionInterfaceVariantStringify>,
    pub(super) gdextension_interface_variant_get_type: Option<GDExtensionInterfaceVariantGetType>,
    pub(super) gdextension_interface_variant_has_method: Option<GDExtensionInterfaceVariantHasMethod>,
    pub(super) gdextension_interface_variant_has_member: Option<GDExtensionInterfaceVariantHasMember>,
    pub(super) gdextension_interface_variant_has_key: Option<GDExtensionInterfaceVariantHasKey>,
    pub(super) gdextension_interface_variant_get_type_name: Option<GDExtensionInterfaceVariantGetTypeName>,
    pub(super) gdextension_interface_variant_can_convert: Option<GDExtensionInterfaceVariantCanConvert>,
    pub(super) gdextension_interface_variant_can_convert_strict: Option<GDExtensionInterfaceVariantCanConvertStrict>,
    pub(super) gdextension_interface_get_variant_from_type_constructor: Option<GDExtensionInterfaceGetVariantFromTypeConstructor>,
    pub(super) gdextension_interface_get_variant_to_type_constructor: Option<GDExtensionInterfaceGetVariantToTypeConstructor>,
    pub(super) gdextension_interface_variant_get_ptr_operator_evaluator: Option<GDExtensionInterfaceVariantGetPtrOperatorEvaluator>,
    pub(super) gdextension_interface_variant_get_ptr_builtin_method: Option<GDExtensionInterfaceVariantGetPtrBuiltinMethod>,
    pub(super) gdextension_interface_variant_get_ptr_constructor: Option<GDExtensionInterfaceVariantGetPtrConstructor>,
    pub(super) gdextension_interface_variant_get_ptr_destructor: Option<GDExtensionInterfaceVariantGetPtrDestructor>,
    pub(super) gdextension_interface_variant_construct: Option<GDExtensionInterfaceVariantConstruct>,
    pub(super) gdextension_interface_variant_get_ptr_setter: Option<GDExtensionInterfaceVariantGetPtrSetter>,
    pub(super) gdextension_interface_variant_get_ptr_getter: Option<GDExtensionInterfaceVariantGetPtrGetter>,
    pub(super) gdextension_interface_variant_get_ptr_indexed_setter: Option<GDExtensionInterfaceVariantGetPtrIndexedSetter>,
    pub(super) gdextension_interface_variant_get_ptr_indexed_getter: Option<GDExtensionInterfaceVariantGetPtrIndexedGetter>,
    pub(super) gdextension_interface_variant_get_ptr_keyed_getter: Option<GDExtensionInterfaceVariantGetPtrKeyedGetter>,
    pub(super) gdextension_interface_variant_get_ptr_keyed_checker: Option<GDExtensionInterfaceVariantGetPtrKeyedChecker>,
    pub(super) gdextension_interface_variant_get_constant_value: Option<GDExtensionInterfaceVariantGetConstantValue>,
    pub(super) gdextension_interface_variant_get_ptr_utility_function: Option<GDExtensionInterfaceVariantGetPtrUtilityFunction>,

    /* Godot core */
    pub(super) gdextension_interface_print_error: Option<GDExtensionInterfacePrintError>,
    pub(super) gdextension_interface_print_error_with_message: Option<GDExtensionInterfacePrintErrorWithMessage>,
    pub(super) gdextension_interface_print_warning: Option<GDExtensionInterfacePrintWarning>,
    pub(super) gdextension_interface_print_warning_with_message: Option<GDExtensionInterfacePrintWarningWithMessage>,
    pub(super) gdextension_interface_print_script_error: Option<GDExtensionInterfacePrintScriptError>,
    pub(super) gdextension_interface_print_script_error_with_message: Option<GDExtensionInterfacePrintScriptErrorWithMessage>,
}

unsafe impl GlobalAlloc for GDExtensionInterface {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        (self.gdextension_interface_mem_alloc).unwrap()(layout.size()).cast()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: std::alloc::Layout) {
        (self.gdextension_interface_mem_free).unwrap()(ptr.cast())
    }
}

unsafe impl Sync for GDExtensionInterface {}


#[global_allocator]
pub(super) static mut GDExtension: GDExtensionInterface = unsafe {MaybeUninit::zeroed().assume_init()};

macro_rules! fetch_gdextension_func {
    ($func: ident, $field: ident, $name: ident) => {
        GDExtension.$field = Some(get_gdextension_func!($func, $name))
    };
}

unsafe extern "C" fn init_gdextension(p_get_proc_address: GDExtensionInterfaceGetProcAddress, p_library: GDExtensionClassLibraryPtr, r_initialization: *mut GDExtensionInitialization) -> GDExtensionBool {
    let gdextension_interface_get_native_struct_size = get_gdextension_func!(p_get_proc_address,GDExtensionInterfaceGetNativeStructSize);

    GDExtension.variant_size = gdextension_interface_get_native_struct_size(("Variant").as_ptr().cast()) as usize;
    GDExtension.class_library = p_library;

    Variant::init_bindings();
    
    /* Mem alloc */
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_mem_alloc, GDExtensionInterfaceMemAlloc);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_mem_free, GDExtensionInterfaceMemFree);
    /* Variant */
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_new_copy,GDExtensionInterfaceVariantNewCopy);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_new_nil,GDExtensionInterfaceVariantNewNil);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_destroy,GDExtensionInterfaceVariantDestroy);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_call,GDExtensionInterfaceVariantCall);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_call_static,GDExtensionInterfaceVariantCallStatic);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_evaluate,GDExtensionInterfaceVariantEvaluate);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_set,GDExtensionInterfaceVariantSet);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_set_named,GDExtensionInterfaceVariantSetNamed);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_set_keyed,GDExtensionInterfaceVariantSetKeyed);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_set_indexed,GDExtensionInterfaceVariantSetIndexed);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get,GDExtensionInterfaceVariantGet);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_named,GDExtensionInterfaceVariantGetNamed);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_keyed,GDExtensionInterfaceVariantGetKeyed);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_indexed,GDExtensionInterfaceVariantGetIndexed);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_iter_init,GDExtensionInterfaceVariantIterInit);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_iter_next,GDExtensionInterfaceVariantIterNext);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_iter_get,GDExtensionInterfaceVariantIterGet);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_hash,GDExtensionInterfaceVariantHash);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_recursive_hash,GDExtensionInterfaceVariantRecursiveHash);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_hash_compare,GDExtensionInterfaceVariantHashCompare);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_booleanize,GDExtensionInterfaceVariantBooleanize);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_duplicate,GDExtensionInterfaceVariantDuplicate);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_stringify,GDExtensionInterfaceVariantStringify);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_type,GDExtensionInterfaceVariantGetType);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_has_method,GDExtensionInterfaceVariantHasMethod);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_has_member,GDExtensionInterfaceVariantHasMember);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_has_key,GDExtensionInterfaceVariantHasKey);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_type_name,GDExtensionInterfaceVariantGetTypeName);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_can_convert,GDExtensionInterfaceVariantCanConvert);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_can_convert_strict,GDExtensionInterfaceVariantCanConvertStrict);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_get_variant_from_type_constructor,GDExtensionInterfaceGetVariantFromTypeConstructor);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_get_variant_to_type_constructor,GDExtensionInterfaceGetVariantToTypeConstructor);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_ptr_operator_evaluator,GDExtensionInterfaceVariantGetPtrOperatorEvaluator);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_ptr_builtin_method,GDExtensionInterfaceVariantGetPtrBuiltinMethod);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_ptr_constructor,GDExtensionInterfaceVariantGetPtrConstructor);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_ptr_destructor,GDExtensionInterfaceVariantGetPtrDestructor);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_construct,GDExtensionInterfaceVariantConstruct);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_ptr_setter,GDExtensionInterfaceVariantGetPtrSetter);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_ptr_getter,GDExtensionInterfaceVariantGetPtrGetter);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_ptr_indexed_setter,GDExtensionInterfaceVariantGetPtrIndexedSetter);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_ptr_indexed_getter,GDExtensionInterfaceVariantGetPtrIndexedGetter);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_ptr_keyed_getter,GDExtensionInterfaceVariantGetPtrKeyedGetter);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_ptr_keyed_checker,GDExtensionInterfaceVariantGetPtrKeyedChecker);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_constant_value,GDExtensionInterfaceVariantGetConstantValue);
    fetch_gdextension_func!(p_get_proc_address,gdextension_interface_variant_get_ptr_utility_function,GDExtensionInterfaceVariantGetPtrUtilityFunction);
    /* Godot core */
    fetch_gdextension_func!(p_get_proc_address, gdextension_interface_print_error, GDExtensionInterfacePrintError);
    fetch_gdextension_func!(p_get_proc_address, gdextension_interface_print_error_with_message, GDExtensionInterfacePrintErrorWithMessage);
    fetch_gdextension_func!(p_get_proc_address, gdextension_interface_print_warning, GDExtensionInterfacePrintWarning);
    fetch_gdextension_func!(p_get_proc_address, gdextension_interface_print_warning_with_message, GDExtensionInterfacePrintWarningWithMessage);
    fetch_gdextension_func!(p_get_proc_address, gdextension_interface_print_script_error, GDExtensionInterfacePrintScriptError);
    fetch_gdextension_func!(p_get_proc_address, gdextension_interface_print_script_error_with_message, GDExtensionInterfacePrintScriptErrorWithMessage);
    0
}