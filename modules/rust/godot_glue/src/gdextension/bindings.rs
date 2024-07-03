/**************************************************************************/
/*  gdextension.rs                                                        */
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

use std::ffi;

type c32 = u32;
type c16 = u16;
type void_p = *mut ffi::c_void;
type const_void_p = *const ffi::c_void;
type char_p = *mut ffi::c_char;
type const_char_p = *const ffi::c_char;
type wchar_p = *mut c16; // can't solve this using FFI
type const_wchar_p = *const c16;

#[repr(C)]
pub(super) enum GDExtensionVariantType {
	GDEXTENSION_VARIANT_TYPE_NIL,

	/*  atomic types */
	GDEXTENSION_VARIANT_TYPE_BOOL,
	GDEXTENSION_VARIANT_TYPE_INT,
	GDEXTENSION_VARIANT_TYPE_FLOAT,
	GDEXTENSION_VARIANT_TYPE_STRING,

	/* math types */
	GDEXTENSION_VARIANT_TYPE_VECTOR2,
	GDEXTENSION_VARIANT_TYPE_VECTOR2I,
	GDEXTENSION_VARIANT_TYPE_RECT2,
	GDEXTENSION_VARIANT_TYPE_RECT2I,
	GDEXTENSION_VARIANT_TYPE_VECTOR3,
	GDEXTENSION_VARIANT_TYPE_VECTOR3I,
	GDEXTENSION_VARIANT_TYPE_TRANSFORM2D,
	GDEXTENSION_VARIANT_TYPE_VECTOR4,
	GDEXTENSION_VARIANT_TYPE_VECTOR4I,
	GDEXTENSION_VARIANT_TYPE_PLANE,
	GDEXTENSION_VARIANT_TYPE_QUATERNION,
	GDEXTENSION_VARIANT_TYPE_AABB,
	GDEXTENSION_VARIANT_TYPE_BASIS,
	GDEXTENSION_VARIANT_TYPE_TRANSFORM3D,
	GDEXTENSION_VARIANT_TYPE_PROJECTION,

	/* misc types */
	GDEXTENSION_VARIANT_TYPE_COLOR,
	GDEXTENSION_VARIANT_TYPE_STRING_NAME,
	GDEXTENSION_VARIANT_TYPE_NODE_PATH,
	GDEXTENSION_VARIANT_TYPE_RID,
	GDEXTENSION_VARIANT_TYPE_OBJECT,
	GDEXTENSION_VARIANT_TYPE_CALLABLE,
	GDEXTENSION_VARIANT_TYPE_SIGNAL,
	GDEXTENSION_VARIANT_TYPE_DICTIONARY,
	GDEXTENSION_VARIANT_TYPE_ARRAY,

	/* typed arrays */
	GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY,
	GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY,
	GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY,
	GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY,
	GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY,
	GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY,
	GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY,
	GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY,
	GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY,
	GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY,

	GDEXTENSION_VARIANT_TYPE_VARIANT_MAX
}

#[repr(C)]
pub(super) enum GDExtensionVariantOperator {
	/* comparison */
	GDEXTENSION_VARIANT_OP_EQUAL,
	GDEXTENSION_VARIANT_OP_NOT_EQUAL,
	GDEXTENSION_VARIANT_OP_LESS,
	GDEXTENSION_VARIANT_OP_LESS_EQUAL,
	GDEXTENSION_VARIANT_OP_GREATER,
	GDEXTENSION_VARIANT_OP_GREATER_EQUAL,

	/* mathematic */
	GDEXTENSION_VARIANT_OP_ADD,
	GDEXTENSION_VARIANT_OP_SUBTRACT,
	GDEXTENSION_VARIANT_OP_MULTIPLY,
	GDEXTENSION_VARIANT_OP_DIVIDE,
	GDEXTENSION_VARIANT_OP_NEGATE,
	GDEXTENSION_VARIANT_OP_POSITIVE,
	GDEXTENSION_VARIANT_OP_MODULE,
	GDEXTENSION_VARIANT_OP_POWER,

	/* bitwise */
	GDEXTENSION_VARIANT_OP_SHIFT_LEFT,
	GDEXTENSION_VARIANT_OP_SHIFT_RIGHT,
	GDEXTENSION_VARIANT_OP_BIT_AND,
	GDEXTENSION_VARIANT_OP_BIT_OR,
	GDEXTENSION_VARIANT_OP_BIT_XOR,
	GDEXTENSION_VARIANT_OP_BIT_NEGATE,

	/* logic */
	GDEXTENSION_VARIANT_OP_AND,
	GDEXTENSION_VARIANT_OP_OR,
	GDEXTENSION_VARIANT_OP_XOR,
	GDEXTENSION_VARIANT_OP_NOT,

	/* containment */
	GDEXTENSION_VARIANT_OP_IN,
	GDEXTENSION_VARIANT_OP_MAX

}

pub(super) type GDExtensionVariantPtr = void_p;
pub(super) type GDExtensionConstVariantPtr = const_void_p;
pub(super) type GDExtensionUninitializedVariantPtr = void_p;
pub(super) type GDExtensionStringNamePtr = void_p;
pub(super) type GDExtensionConstStringNamePtr = const_void_p;
pub(super) type GDExtensionUninitializedStringNamePtr = void_p;
pub(super) type GDExtensionStringPtr = void_p;
pub(super) type GDExtensionConstStringPtr = const_void_p;
pub(super) type GDExtensionUninitializedStringPtr = void_p;
pub(super) type GDExtensionObjectPtr = void_p;
pub(super) type GDExtensionConstObjectPtr = const_void_p;
pub(super) type GDExtensionUninitializedObjectPtr = void_p;
pub(super) type GDExtensionTypePtr = void_p;
pub(super) type GDExtensionConstTypePtr = const_void_p;
pub(super) type GDExtensionUninitializedTypePtr = void_p;
pub(super) type GDExtensionMethodBindPtr = const_void_p;
pub(super) type GDExtensionInt = i64;
pub(super) type GDExtensionBool = u8;
pub(super) type GDObjectInstanceID = u64;
pub(super) type GDExtensionRefPtr = void_p;
pub(super) type GDExtensionConstRefPtr = const_void_p;

// In this API there are multiple functions which expect the caller to pass a pointer
// on return value as parameter.
// In order to make it clear if the caller should initialize the return value or not
// we have two flavor of types:
// - `GDExtensionXXXPtr` for pointer on an initialized value
// - `GDExtensionUninitializedXXXPtr` for pointer on uninitialized value
//
// Notes:
// - Not respecting those requirements can seems harmless, but will lead to unexpected
//   segfault or memory leak (for instance with a specific compiler/OS, or when two
//   native extensions start doing ptrcall on each other).
// - Initialization must be done with the function pointer returned by `variant_get_ptr_constructor`,
//   zero-initializing the variable should not be considered a valid initialization method here !
// - Some types have no destructor (see `extension_api.json`'s `has_destructor` field), for
//   them it is always safe to skip the constructor for the return value if you are in a hurry ;-)


#[repr(C)]
pub(super) enum GDExtensionCallErrorType {
	GDEXTENSION_CALL_OK,
	GDEXTENSION_CALL_ERROR_INVALID_METHOD,
	GDEXTENSION_CALL_ERROR_INVALID_ARGUMENT, // Expected a different variant type.
	GDEXTENSION_CALL_ERROR_TOO_MANY_ARGUMENTS, // Expected lower number of arguments.
	GDEXTENSION_CALL_ERROR_TOO_FEW_ARGUMENTS, // Expected higher number of arguments.
	GDEXTENSION_CALL_ERROR_INSTANCE_IS_NULL,
	GDEXTENSION_CALL_ERROR_METHOD_NOT_CONST, // Used for const call.
}

#[repr(C)]
pub(super) struct GDExtensionCallError {
    error: GDExtensionCallErrorType,
	argument: i32,
	expected: i32,
}

pub(super) type GDExtensionVariantFromTypeConstructorFunc = unsafe extern "C" fn (GDExtensionUninitializedVariantPtr, GDExtensionTypePtr);
pub(super) type GDExtensionTypeFromVariantConstructorFunc = unsafe extern "C" fn (GDExtensionUninitializedTypePtr, GDExtensionVariantPtr);
pub(super) type GDExtensionPtrOperatorEvaluator = unsafe extern "C" fn (p_left: GDExtensionConstTypePtr, p_right: GDExtensionConstTypePtr, r_result: GDExtensionTypePtr);
pub(super) type GDExtensionPtrBuiltInMethod = unsafe extern "C" fn (p_base: GDExtensionTypePtr, p_args: *const GDExtensionConstTypePtr, r_return: GDExtensionTypePtr, p_argument_count: ffi::c_int);
pub(super) type GDExtensionPtrConstructor = unsafe extern "C" fn (p_base: GDExtensionUninitializedTypePtr, p_args: *const GDExtensionConstTypePtr);
pub(super) type GDExtensionPtrDestructor = unsafe extern "C" fn (p_base: GDExtensionTypePtr);
pub(super) type GDExtensionPtrSetter = unsafe extern "C" fn (p_base: GDExtensionTypePtr, p_value: GDExtensionConstTypePtr);
pub(super) type GDExtensionPtrGetter = unsafe extern "C" fn (p_base: GDExtensionConstTypePtr, r_value: GDExtensionTypePtr);
pub(super) type GDExtensionPtrIndexedSetter = unsafe extern "C" fn (p_base: GDExtensionTypePtr, p_index: GDExtensionInt, p_value: GDExtensionConstTypePtr);
pub(super) type GDExtensionPtrIndexedGetter = unsafe extern "C" fn (p_base: GDExtensionConstTypePtr, p_index: GDExtensionInt, r_value: GDExtensionTypePtr);
pub(super) type GDExtensionPtrKeyedSetter = unsafe extern "C" fn (p_base: GDExtensionTypePtr, p_key: GDExtensionConstTypePtr, p_value: GDExtensionConstTypePtr);
pub(super) type GDExtensionPtrKeyedGetter = unsafe extern "C" fn (p_base: GDExtensionConstTypePtr, p_key: GDExtensionConstTypePtr, r_value: GDExtensionTypePtr);
pub(super) type GDExtensionPtrKeyedChecker = unsafe extern "C" fn (p_base: GDExtensionConstVariantPtr, p_key: GDExtensionConstVariantPtr) -> u32;
pub(super) type GDExtensionPtrUtilityFunction = unsafe extern "C" fn (r_return: GDExtensionTypePtr, p_args: *const GDExtensionConstTypePtr, p_argument_count: ffi::c_int);

pub(super) type GDExtensionClassConstructor = unsafe extern "C" fn () -> GDExtensionObjectPtr;

pub(super) type GDExtensionInstanceBindingCreateCallback = unsafe extern "C" fn (p_token: void_p, p_instance: void_p) -> void_p;
pub(super) type GDExtensionInstanceBindingFreeCallback = unsafe extern "C" fn (p_token: void_p, p_instance: void_p, p_binding: void_p);
pub(super) type GDExtensionInstanceBindingReferenceCallback = unsafe extern "C" fn (p_token: void_p, p_binding: void_p, p_reference: GDExtensionBool) -> GDExtensionBool;

#[repr(C)]
pub(super) struct GDExtensionInstanceBindingCallbacks {
	create_callback: GDExtensionInstanceBindingCreateCallback,
	free_callback: GDExtensionInstanceBindingFreeCallback,
	reference_callback: GDExtensionInstanceBindingReferenceCallback
}

/* EXTENSION CLASSES */

pub(super) type GDExtensionClassInstancePtr = void_p;

pub(super) type GDExtensionClassSet = unsafe extern "C" fn (p_instance: GDExtensionClassInstancePtr, p_name: GDExtensionConstStringNamePtr, p_value: GDExtensionConstVariantPtr) -> GDExtensionBool;
pub(super) type GDExtensionClassGet = unsafe extern "C" fn (p_instance: GDExtensionClassInstancePtr, p_name: GDExtensionConstStringNamePtr, r_ret: GDExtensionVariantPtr) -> GDExtensionBool;
pub(super) type GDExtensionClassGetRID = unsafe extern "C" fn (p_instance: GDExtensionClassInstancePtr) -> u64;

#[repr(C)]
pub(super) struct GDExtensionPropertyInfo {
	f_type: GDExtensionVariantType,
	name: GDExtensionStringNamePtr,
	class_name: GDExtensionStringNamePtr,
	hint: u32, // Bitfield of `PropertyHint` (defined in `extension_api.json`).
	hint_string: GDExtensionStringPtr,
	usage: u32 // Bitfield of `PropertyUsageFlags` (defined in `extension_api.json`).
}

#[repr(C)]
pub(super) struct GDExtensionMethodInfo {
    name: GDExtensionStringNamePtr,
    return_value: GDExtensionPropertyInfo,
    flags: u32, // Bitfield of `GDExtensionClassMethodFlags`.
    id: i32,

    /* Arguments: `default_arguments` is an array of size `argument_count`. */
	argument_count: u32,
	arguments: *mut GDExtensionPropertyInfo,

	/* Default arguments: `default_arguments` is an array of size `default_argument_count`. */
	default_argument_count: u32,
	default_arguments: *mut GDExtensionVariantPtr
}

pub(super) type GDExtensionClassGetPropertyList = unsafe extern "C" fn (p_instance: GDExtensionClassInstancePtr, r_count: *mut u32) -> *const GDExtensionPropertyInfo;
pub(super) type GDExtensionClassFreePropertyList2 = unsafe extern "C" fn (p_instance: GDExtensionClassInstancePtr, p_list: *const GDExtensionPropertyInfo, p_count: u32);
pub(super) type GDExtensionClassPropertyCanRevert = unsafe extern "C" fn (p_instance: GDExtensionClassInstancePtr, p_name: GDExtensionConstStringNamePtr) -> GDExtensionBool;
pub(super) type GDExtensionClassPropertyGetRevert = unsafe extern "C" fn (p_instance: GDExtensionClassInstancePtr, p_name: GDExtensionConstStringNamePtr, r_ret: GDExtensionVariantPtr) -> GDExtensionBool;
pub(super) type GDExtensionClassValidateProperty = unsafe extern "C" fn (p_instance: GDExtensionClassInstancePtr, p_property: *mut GDExtensionPropertyInfo) -> GDExtensionBool;
pub(super) type GDExtensionClassNotification2 = unsafe extern "C" fn (p_instance: GDExtensionClassInstancePtr, p_what: i32, p_reversed: GDExtensionBool);
pub(super) type GDExtensionClassToString = unsafe extern "C" fn (p_instance: GDExtensionClassInstancePtr, r_is_valid: *mut GDExtensionBool, p_out: GDExtensionStringPtr);
pub(super) type GDExtensionClassReference = unsafe extern "C" fn (p_instance: GDExtensionClassInstancePtr);
pub(super) type GDExtensionClassUnreference = unsafe extern "C" fn (p_instance: GDExtensionClassInstancePtr);
pub(super) type GDExtensionClassCallVirtual = unsafe extern "C" fn (p_instance: GDExtensionClassInstancePtr, p_args: *const GDExtensionConstTypePtr, r_ret: GDExtensionTypePtr);
pub(super) type GDExtensionClassCreateInstance = unsafe extern "C" fn (p_class_userdata: void_p) -> GDExtensionObjectPtr;
pub(super) type GDExtensionClassFreeInstance = unsafe extern "C" fn (p_class_userdata: void_p, p_instance: GDExtensionClassInstancePtr);
pub(super) type GDExtensionClassRecreateInstance = unsafe extern "C" fn (p_class_userdata: void_p, p_object: GDExtensionObjectPtr) -> GDExtensionClassInstancePtr;
pub(super) type GDExtensionClassGetVirtual = unsafe extern "C" fn (p_class_userdata: void_p, p_name: GDExtensionConstStringNamePtr) -> GDExtensionClassCallVirtual;
pub(super) type GDExtensionClassGetVirtualCallData = unsafe extern "C" fn (p_class_userdata: void_p, p_name: GDExtensionConstStringNamePtr) -> void_p;
pub(super) type GDExtensionClassCallVirtualWithData = unsafe extern "C" fn (p_instance: GDExtensionClassInstancePtr, p_name: GDExtensionConstStringNamePtr, p_virtual_call_userdata: void_p, p_args: *const GDExtensionConstTypePtr, r_ret: GDExtensionTypePtr);

#[repr(C)]
pub(super) struct GDExtensionClassCreationInfo3 {
    is_virtual: GDExtensionBool,
    is_abstract: GDExtensionBool,
    is_exposed: GDExtensionBool,
    is_runtime: GDExtensionBool,
    set_func: GDExtensionClassSet,
    get_func: GDExtensionClassGet,
    get_property_list_func: GDExtensionClassGetPropertyList,
    free_property_list_func: GDExtensionClassFreePropertyList2,
    property_can_revert_func: GDExtensionClassPropertyCanRevert,
    property_get_revert_func: GDExtensionClassPropertyGetRevert,
    validate_property_func: GDExtensionClassValidateProperty,
    notification_func: GDExtensionClassNotification2,
    to_string_func: GDExtensionClassToString,
    reference_func: GDExtensionClassReference,
    unreference_func: GDExtensionClassUnreference,
    create_instance_func: GDExtensionClassCreateInstance, // (Default) constructor; mandatory. If the class is not instantiable, consider making it virtual or abstract.
    free_instance_func: GDExtensionClassFreeInstance,
    recreate_instance_func: GDExtensionClassRecreateInstance,
    // Queries a virtual function by name and returns a callback to invoke the requested virtual function.
	get_virtual_func: GDExtensionClassGetVirtual,
    // Paired with `call_virtual_with_data_func`, this is an alternative to `get_virtual_func` for extensions that
	// need or benefit from extra data when calling virtual functions.
	// Returns user data that will be passed to `call_virtual_with_data_func`.
	// Returning `NULL` from this function signals to Godot that the virtual function is not overridden.
	// Data returned from this function should be managed by the extension and must be valid until the extension is deinitialized.
	// You should supply either `get_virtual_func`, or `get_virtual_call_data_func` with `call_virtual_with_data_func`.
	get_virtual_call_data_func: GDExtensionClassGetVirtualCallData,
    // Used to call virtual functions when `get_virtual_call_data_func` is not null.
	call_virtual_with_data_func: GDExtensionClassCallVirtualWithData,
    get_rid_func: GDExtensionClassGetRID,
    class_userdata: void_p // Per-class user data, later accessible in instance bindings.
}

pub(super) type GDExtensionClassLibraryPtr = void_p;

/* Method */

#[repr(C)]
pub(super) enum GDExtensionClassMethodFlags {
	GDEXTENSION_METHOD_FLAG_NORMAL = 1,
	GDEXTENSION_METHOD_FLAG_EDITOR = 2,
	GDEXTENSION_METHOD_FLAG_CONST = 4,
	GDEXTENSION_METHOD_FLAG_VIRTUAL = 8,
	GDEXTENSION_METHOD_FLAG_VARARG = 16,
	GDEXTENSION_METHOD_FLAG_STATIC = 32,
}
impl GDExtensionClassMethodFlags {
    pub(super) const GDEXTENSION_METHOD_FLAGS_DEFAULT: Self = Self::GDEXTENSION_METHOD_FLAG_NORMAL;
}

#[repr(C)]
pub(super) enum GDExtensionClassMethodArgumentMetadata {
	GDEXTENSION_METHOD_ARGUMENT_METADATA_NONE,
	GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_INT8,
	GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_INT16,
	GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_INT32,
	GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_INT64,
	GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_UINT8,
	GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_UINT16,
	GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_UINT32,
	GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_UINT64,
	GDEXTENSION_METHOD_ARGUMENT_METADATA_REAL_IS_FLOAT,
	GDEXTENSION_METHOD_ARGUMENT_METADATA_REAL_IS_DOUBLE
}

pub(super) type GDExtensionClassMethodCall = unsafe extern "C" fn (method_userdata: void_p, p_instance: GDExtensionClassInstancePtr, p_args: *const GDExtensionConstVariantPtr, p_argument_count: GDExtensionInt, r_return: GDExtensionVariantPtr, r_error: *mut GDExtensionCallError);
pub(super) type GDExtensionClassMethodValidatedCall = unsafe extern "C" fn (method_userdata: void_p, p_instance: GDExtensionClassInstancePtr, p_args: *const GDExtensionConstVariantPtr, r_return: GDExtensionVariantPtr);
pub(super) type GDExtensionClassMethodPtrCall = unsafe extern "C" fn (method_userdata: void_p, p_instance: GDExtensionClassInstancePtr,  p_args: *const GDExtensionConstTypePtr, r_ret: GDExtensionTypePtr);

#[repr(C)]
pub(super) struct GDExtensionClassMethodInfo {
	name: GDExtensionStringNamePtr,
	method_userdata: void_p,
	call_func: GDExtensionClassMethodCall,
	ptrcall_func: GDExtensionClassMethodPtrCall,
	method_flags: u32, // Bitfield of `GDExtensionClassMethodFlags`.

	/* If `has_return_value` is false, `return_value_info` and `return_value_metadata` are ignored.
	 *
	 * @todo Consider dropping `has_return_value` and making the other two properties match `GDExtensionMethodInfo` and `GDExtensionClassVirtualMethod` for consistency in future version of this struct.
	 */
	has_return_value: GDExtensionBool,
	return_value_info: *mut GDExtensionPropertyInfo,
	return_value_metadata: GDExtensionClassMethodArgumentMetadata,

	/* Arguments: `arguments_info` and `arguments_metadata` are array of size `argument_count`.
	 * Name and hint information for the argument can be omitted in release builds. Class name should always be present if it applies.
	 *
	 * @todo Consider renaming `arguments_info` to `arguments` for consistency in future version of this struct.
	 */
	argument_count: u32,
	arguments_info: *mut GDExtensionPropertyInfo,
	arguments_metadata: *mut GDExtensionClassMethodArgumentMetadata,

	/* Default arguments: `default_arguments` is an array of size `default_argument_count`. */
	default_argument_count: u32,
	default_arguments: *mut GDExtensionVariantPtr
}

#[repr(C)]
pub(super) struct GDExtensionClassVirtualMethodInfo {
	name: GDExtensionStringNamePtr,
	method_flags: u32, // Bitfield of `GDExtensionClassMethodFlags`.

	return_value: GDExtensionPropertyInfo,
	return_value_metadata: GDExtensionClassMethodArgumentMetadata,

	argument_count: u32,
	arguments: *mut GDExtensionPropertyInfo,
	arguments_metadata: *mut GDExtensionClassMethodArgumentMetadata
}

pub(super) type GDExtensionCallableCustomCall = unsafe extern "C" fn (callable_userdata: void_p, p_args: *const GDExtensionConstVariantPtr, p_argument_count: GDExtensionInt, r_return: GDExtensionVariantPtr, r_error: *mut GDExtensionCallError);
pub(super) type GDExtensionCallableCustomIsValid = unsafe extern "C" fn (callable_userdata: void_p) -> GDExtensionBool;
pub(super) type GDExtensionCallableCustomFree = unsafe extern "C" fn (callable_userdata: void_p);

pub(super) type GDExtensionCallableCustomHash = unsafe extern "C" fn (callable_userdata: void_p) -> u32;
pub(super) type GDExtensionCallableCustomEqual = unsafe extern "C" fn (callable_userdata_a: void_p, callable_userdata_b: void_p) -> GDExtensionBool;
pub(super) type GDExtensionCallableCustomLessThan = unsafe extern "C" fn (callable_userdata_a: void_p, callable_userdata_b: void_p) -> GDExtensionBool;

pub(super) type GDExtensionCallableCustomToString = unsafe extern "C" fn (callable_userdata: void_p, r_is_valid: *mut GDExtensionBool, r_out: GDExtensionStringPtr);

pub(super) type GDExtensionCallableCustomGetArgumentCount = unsafe extern "C" fn (callable_userdata: void_p, r_is_valid: GDExtensionBool) -> GDExtensionInt;

#[repr(C)]
pub(super) struct GDExtensionCallableCustomInfo2 {
	/* Only `call_func` and `token` are strictly required, however, `object_id` should be passed if its not a static method.
	 *
	 * `token` should point to an address that uniquely identifies the GDExtension (for example, the
	 * `GDExtensionClassLibraryPtr` passed to the entry symbol function.
	 *
	 * `hash_func`, `equal_func`, and `less_than_func` are optional. If not provided both `call_func` and
	 * `callable_userdata` together are used as the identity of the callable for hashing and comparison purposes.
	 *
	 * The hash returned by `hash_func` is cached, `hash_func` will not be called more than once per callable.
	 *
	 * `is_valid_func` is necessary if the validity of the callable can change before destruction.
	 *
	 * `free_func` is necessary if `callable_userdata` needs to be cleaned up when the callable is freed.
	 */
	callable_userdata : void_p,
	token : void_p,

	object_id: GDObjectInstanceID,

	call_func: GDExtensionCallableCustomCall,
	is_valid_func: GDExtensionCallableCustomIsValid,
	free_func: GDExtensionCallableCustomFree,

	hash_func: GDExtensionCallableCustomHash,
	equal_func: GDExtensionCallableCustomEqual,
	less_than_func: GDExtensionCallableCustomLessThan,

	to_string_func: GDExtensionCallableCustomToString,

	get_argument_count_func: GDExtensionCallableCustomGetArgumentCount
}

/* SCRIPT INSTANCE EXTENSION */

pub(super) type GDExtensionScriptInstanceDataPtr = void_p;

pub(super) type GDExtensionScriptInstanceSet = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, p_name: GDExtensionConstStringNamePtr, p_value: GDExtensionConstVariantPtr) -> GDExtensionBool;
pub(super) type GDExtensionScriptInstanceGet = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, p_name: GDExtensionConstStringNamePtr, r_ret: GDExtensionVariantPtr) -> GDExtensionBool;
pub(super) type GDExtensionScriptInstanceGetPropertyList = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, r_count: *mut u32) -> *const GDExtensionPropertyInfo;
pub(super) type GDExtensionScriptInstanceFreePropertyList2 = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, p_list: *const GDExtensionPropertyInfo, p_count: u32);
pub(super) type GDExtensionScriptInstanceGetClassCategory = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, p_class_category: *mut GDExtensionPropertyInfo) -> GDExtensionBool;

pub(super) type GDExtensionScriptInstanceGetPropertyType = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, p_name: GDExtensionConstStringNamePtr, r_is_valid: *mut GDExtensionBool) -> GDExtensionVariantType;
pub(super) type GDExtensionScriptInstanceValidateProperty = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, p_property: *mut GDExtensionPropertyInfo) -> GDExtensionBool;

pub(super) type GDExtensionScriptInstancePropertyCanRevert = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, p_name: GDExtensionConstStringNamePtr) -> GDExtensionBool;
pub(super) type GDExtensionScriptInstancePropertyGetRevert = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, p_name: GDExtensionConstStringNamePtr, r_ret: GDExtensionVariantPtr) -> GDExtensionBool;

pub(super) type GDExtensionScriptInstanceGetOwner = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr) -> GDExtensionObjectPtr;
pub(super) type GDExtensionScriptInstancePropertyStateAdd = unsafe extern "C" fn (p_name: GDExtensionConstStringNamePtr, p_value: GDExtensionConstVariantPtr, p_userdata: void_p);
pub(super) type GDExtensionScriptInstanceGetPropertyState = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, p_add_func: GDExtensionScriptInstancePropertyStateAdd, p_userdata: void_p);

pub(super) type GDExtensionScriptInstanceGetMethodList = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, r_count: *mut u32) -> *const GDExtensionMethodInfo;
pub(super) type GDExtensionScriptInstanceFreeMethodList2 = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, p_list: *const GDExtensionMethodInfo, p_count: u32);

pub(super) type GDExtensionScriptInstanceHasMethod = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, p_name: GDExtensionConstStringNamePtr) -> GDExtensionBool;

pub(super) type GDExtensionScriptInstanceGetMethodArgumentCount = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, p_name: GDExtensionConstStringNamePtr, r_is_valid: *mut GDExtensionBool) -> GDExtensionInt;

pub(super) type GDExtensionScriptInstanceCall = unsafe extern "C" fn (p_self: GDExtensionScriptInstanceDataPtr, p_method: GDExtensionConstStringNamePtr, p_args: *const GDExtensionConstVariantPtr, p_argument_count: GDExtensionInt, r_return: GDExtensionVariantPtr, r_error: *mut GDExtensionCallError);
pub(super) type GDExtensionScriptInstanceNotification2 = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, p_what: i32, p_reversed: GDExtensionBool);
pub(super) type GDExtensionScriptInstanceToString = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr, r_is_valid: *mut GDExtensionBool, r_out: GDExtensionStringPtr );

pub(super) type GDExtensionScriptInstanceRefCountIncremented = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr);
pub(super) type GDExtensionScriptInstanceRefCountDecremented = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr) -> GDExtensionBool;

pub(super) type GDExtensionScriptInstanceGetScript = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr) -> GDExtensionObjectPtr;
pub(super) type GDExtensionScriptInstanceIsPlaceholder = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr) -> GDExtensionBool;

pub(super) type GDExtensionScriptLanguagePtr = void_p;

pub(super) type GDExtensionScriptInstanceGetLanguage = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr) -> GDExtensionScriptLanguagePtr;

pub(super) type GDExtensionScriptInstanceFree = unsafe extern "C" fn (p_instance: GDExtensionScriptInstanceDataPtr);

pub(super) type GDExtensionScriptInstancePtr = void_p; // Pointer to ScriptInstance.

#[repr(C)]
pub(super) struct GDExtensionScriptInstanceInfo3 {
	set_func: GDExtensionScriptInstanceSet,
	get_func: GDExtensionScriptInstanceGet,
	get_property_list_func: GDExtensionScriptInstanceGetPropertyList,
	free_property_list_func: GDExtensionScriptInstanceFreePropertyList2,
	get_class_category_func: GDExtensionScriptInstanceGetClassCategory, // Optional. Set to NULL for the default behavior.

	property_can_revert_func: GDExtensionScriptInstancePropertyCanRevert,
	property_get_revert_func: GDExtensionScriptInstancePropertyGetRevert,

	get_owner_func: GDExtensionScriptInstanceGetOwner,
	get_property_state_func: GDExtensionScriptInstanceGetPropertyState,

	get_method_list_func: GDExtensionScriptInstanceGetMethodList,
	free_method_list_func: GDExtensionScriptInstanceFreeMethodList2,
	get_property_type_func: GDExtensionScriptInstanceGetPropertyType,
	validate_property_func: GDExtensionScriptInstanceValidateProperty,

	has_method_func: GDExtensionScriptInstanceHasMethod,

	get_method_argument_count_func: GDExtensionScriptInstanceGetMethodArgumentCount,

	call_func: GDExtensionScriptInstanceCall,
	notification_func: GDExtensionScriptInstanceNotification2,

	to_string_func: GDExtensionScriptInstanceToString,

	refcount_incremented_func: GDExtensionScriptInstanceRefCountIncremented,
	refcount_decremented_func: GDExtensionScriptInstanceRefCountDecremented,

	get_script_func: GDExtensionScriptInstanceGetScript,

	is_placeholder_func: GDExtensionScriptInstanceIsPlaceholder,

	set_fallback_func: GDExtensionScriptInstanceSet,
	get_fallback_func: GDExtensionScriptInstanceGet,

	get_language_func: GDExtensionScriptInstanceGetLanguage,

	free_func: GDExtensionScriptInstanceFree

}

/* INITIALIZATION */

#[repr(C)]
pub(super) enum GDExtensionInitializationLevel {
	GDEXTENSION_INITIALIZATION_CORE,
	GDEXTENSION_INITIALIZATION_SERVERS,
	GDEXTENSION_INITIALIZATION_SCENE,
	GDEXTENSION_INITIALIZATION_EDITOR,
	GDEXTENSION_MAX_INITIALIZATION_LEVEL,
}

#[repr(C)]
pub(super) struct GDExtensionInitialization {
	/* Minimum initialization level required.
	 * If Core or Servers, the extension needs editor or game restart to take effect */
	minimum_initialization_level: GDExtensionInitializationLevel,
	/* Up to the user to supply when initializing */
	userdata: void_p, 
	/* This function will be called multiple times for each initialization level. */
	initialize: unsafe extern "C" fn (userdata: void_p, p_level: GDExtensionInitializationLevel),
	deinitialize: unsafe extern "C" fn (userdata: void_p, p_level: GDExtensionInitializationLevel)
}

pub(super) type GDExtensionInterfaceFunctionPtr = unsafe extern "C" fn ();
pub(super) type GDExtensionInterfaceGetProcAddress = unsafe extern "C" fn (p_function_name: const_char_p) -> GDExtensionInterfaceFunctionPtr;

pub(super) type GDExtensionInitializationFunction = unsafe extern "C" fn (p_get_proc_address: GDExtensionInterfaceGetProcAddress, p_library: GDExtensionClassLibraryPtr, r_initialization: *mut GDExtensionInitialization) -> GDExtensionBool;

/* INTERFACE */

#[repr(C)]
pub(super) struct GDExtensionGodotVersion {
	major: u32,
	minor: u32,
	patch: u32,
	string: const_char_p
}

pub(super) type GDExtensionInterfaceGetGodotVersion = unsafe extern "C" fn (r_godot_version: *mut GDExtensionGodotVersion);

/* INTERFACE: Memory */

/**
 * @name mem_alloc
 * @since 4.1
 *
 * Allocates memory.
 *
 * @param p_bytes The amount of memory to allocate in bytes.
 *
 * @return A pointer to the allocated memory, or NULL if unsuccessful.
 */
pub(super) type GDExtensionInterfaceMemAlloc = unsafe extern "C" fn (p_bytes: usize) -> void_p;

/**
 * @name mem_realloc
 * @since 4.1
 *
 * Reallocates memory.
 *
 * @param p_ptr A pointer to the previously allocated memory.
 * @param p_bytes The number of bytes to resize the memory block to.
 *
 * @return A pointer to the allocated memory, or NULL if unsuccessful.
 */
pub(super) type GDExtensionInterfaceMemRealloc = unsafe extern "C" fn (p_ptr: void_p, p_bytes: usize) -> void_p;

/**
 * @name mem_free
 * @since 4.1
 *
 * Frees memory.
 *
 * @param p_ptr A pointer to the previously allocated memory.
 */
pub(super) type GDExtensionInterfaceMemFree = unsafe extern "C" fn (p_ptr: void_p);

/* INTERFACE: Godot Core */

/**
 * @name print_error
 * @since 4.1
 *
 * Logs an error to Godot's built-in debugger and to the OS terminal.
 *
 * @param p_description The code trigging the error.
 * @param p_function The function name where the error occurred.
 * @param p_file The file where the error occurred.
 * @param p_line The line where the error occurred.
 * @param p_editor_notify Whether or not to notify the editor.
 */
pub(super) type GDExtensionInterfacePrintError = unsafe extern "C" fn (p_description: const_char_p, p_function: const_char_p, p_file: const_char_p, p_line: i32, p_editor_notify: GDExtensionBool);

/**
 * @name print_error_with_message
 * @since 4.1
 *
 * Logs an error with a message to Godot's built-in debugger and to the OS terminal.
 *
 * @param p_description The code trigging the error.
 * @param p_message The message to show along with the error.
 * @param p_function The function name where the error occurred.
 * @param p_file The file where the error occurred.
 * @param p_line The line where the error occurred.
 * @param p_editor_notify Whether or not to notify the editor.
 */
pub(super) type GDExtensionInterfacePrintErrorWithMessage = unsafe extern "C" fn (p_description: const_char_p, p_message: const_char_p, p_function: const_char_p, p_file: const_char_p, p_line: i32, p_editor_notify: GDExtensionBool);

/**
 * @name print_warning
 * @since 4.1
 *
 * Logs a warning to Godot's built-in debugger and to the OS terminal.
 *
 * @param p_description The code trigging the warning.
 * @param p_function The function name where the warning occurred.
 * @param p_file The file where the warning occurred.
 * @param p_line The line where the warning occurred.
 * @param p_editor_notify Whether or not to notify the editor.
 */
pub(super) type GDExtensionInterfacePrintWarning = unsafe extern "C" fn (p_description: const_char_p, p_function: const_char_p, p_file: const_char_p, p_line: i32, p_editor_notify: GDExtensionBool);

/**
 * @name print_warning_with_message
 * @since 4.1
 *
 * Logs a warning with a message to Godot's built-in debugger and to the OS terminal.
 *
 * @param p_description The code trigging the warning.
 * @param p_message The message to show along with the warning.
 * @param p_function The function name where the warning occurred.
 * @param p_file The file where the warning occurred.
 * @param p_line The line where the warning occurred.
 * @param p_editor_notify Whether or not to notify the editor.
 */
pub(super) type GDExtensionInterfacePrintWarningWithMessage = unsafe extern "C" fn (p_description: const_char_p, p_message: const_char_p, p_function: const_char_p, p_file: const_char_p, p_line: i32, p_editor_notify: GDExtensionBool);

/**
 * @name print_script_error
 * @since 4.1
 *
 * Logs a script error to Godot's built-in debugger and to the OS terminal.
 *
 * @param p_description The code trigging the error.
 * @param p_function The function name where the error occurred.
 * @param p_file The file where the error occurred.
 * @param p_line The line where the error occurred.
 * @param p_editor_notify Whether or not to notify the editor.
 */
pub(super) type GDExtensionInterfacePrintScriptError = unsafe extern "C" fn (p_description: const_char_p, p_function: const_char_p, p_file: const_char_p, p_line: i32, p_editor_notify: GDExtensionBool);

/**
 * @name print_script_error_with_message
 * @since 4.1
 *
 * Logs a script error with a message to Godot's built-in debugger and to the OS terminal.
 *
 * @param p_description The code trigging the error.
 * @param p_message The message to show along with the error.
 * @param p_function The function name where the error occurred.
 * @param p_file The file where the error occurred.
 * @param p_line The line where the error occurred.
 * @param p_editor_notify Whether or not to notify the editor.
 */
pub(super) type GDExtensionInterfacePrintScriptErrorWithMessage = unsafe extern "C" fn (p_description: const_char_p, p_message: const_char_p, p_function: const_char_p, p_file: const_char_p, p_line: i32, p_editor_notify: GDExtensionBool);

/**
 * @name get_native_struct_size
 * @since 4.1
 *
 * Gets the size of a native struct (ex. ObjectID) in bytes.
 *
 * @param p_name A pointer to a StringName identifying the struct name.
 *
 * @return The size in bytes.
 */
pub(super) type GDExtensionInterfaceGetNativeStructSize = unsafe extern "C" fn (p_name: GDExtensionConstStringNamePtr) -> u64;

/* INTERFACE: Variant */

/**
 * @name variant_new_copy
 * @since 4.1
 *
 * Copies one Variant into a another.
 *
 * @param r_dest A pointer to the destination Variant.
 * @param p_src A pointer to the source Variant.
 */
pub(super) type GDExtensionInterfaceVariantNewCopy = unsafe extern "C" fn (r_dest: GDExtensionUninitializedVariantPtr, p_src: GDExtensionConstVariantPtr);

/**
 * @name variant_new_nil
 * @since 4.1
 *
 * Creates a new Variant containing nil.
 *
 * @param r_dest A pointer to the destination Variant.
 */
pub(super) type GDExtensionInterfaceVariantNewNil = unsafe extern "C" fn (r_dest: GDExtensionUninitializedVariantPtr);

/**
 * @name variant_destroy
 * @since 4.1
 *
 * Destroys a Variant.
 *
 * @param p_self A pointer to the Variant to destroy.
 */
pub(super) type GDExtensionInterfaceVariantDestroy = unsafe extern "C" fn (p_self: GDExtensionVariantPtr);

/**
 * @name variant_call
 * @since 4.1
 *
 * Calls a method on a Variant.
 *
 * @param p_self A pointer to the Variant.
 * @param p_method A pointer to a StringName identifying the method.
 * @param p_args A pointer to a C array of Variant.
 * @param p_argument_count The number of arguments.
 * @param r_return A pointer a Variant which will be assigned the return value.
 * @param r_error A pointer the structure which will hold error information.
 *
 * @see Variant::callp()
 */
pub(super) type GDExtensionInterfaceVariantCall = unsafe extern "C" fn (p_self: GDExtensionVariantPtr, p_method: GDExtensionConstStringNamePtr, p_args: *const GDExtensionConstVariantPtr, p_argument_count: GDExtensionInt, r_return: GDExtensionUninitializedVariantPtr, r_error: GDExtensionCallError);

/**
 * @name variant_call_static
 * @since 4.1
 *
 * Calls a static method on a Variant.
 *
 * @param p_self A pointer to the Variant.
 * @param p_method A pointer to a StringName identifying the method.
 * @param p_args A pointer to a C array of Variant.
 * @param p_argument_count The number of arguments.
 * @param r_return A pointer a Variant which will be assigned the return value.
 * @param r_error A pointer the structure which will be updated with error information.
 *
 * @see Variant::call_static()
 */
pub(super) type GDExtensionInterfaceVariantCallStatic = unsafe extern "C" fn (p_type: GDExtensionVariantType, p_method: GDExtensionConstStringNamePtr, p_args: *const GDExtensionConstVariantPtr, p_argument_count: GDExtensionInt, r_return: GDExtensionUninitializedVariantPtr, r_error: GDExtensionCallError);

/**
 * @name variant_evaluate
 * @since 4.1
 *
 * Evaluate an operator on two Variants.
 *
 * @param p_op The operator to evaluate.
 * @param p_a The first Variant.
 * @param p_b The second Variant.
 * @param r_return A pointer a Variant which will be assigned the return value.
 * @param r_valid A pointer to a boolean which will be set to false if the operation is invalid.
 *
 * @see Variant::evaluate()
 */
pub(super) type GDExtensionInterfaceVariantEvaluate = unsafe extern "C" fn (p_op: GDExtensionVariantOperator, p_a: GDExtensionConstVariantPtr, p_b: GDExtensionConstVariantPtr, r_return: GDExtensionUninitializedVariantPtr, r_valid: GDExtensionBool);

/**
 * @name variant_set
 * @since 4.1
 *
 * Sets a key on a Variant to a value.
 *
 * @param p_self A pointer to the Variant.
 * @param p_key A pointer to a Variant representing the key.
 * @param p_value A pointer to a Variant representing the value.
 * @param r_valid A pointer to a boolean which will be set to false if the operation is invalid.
 *
 * @see Variant::set()
 */
pub(super) type GDExtensionInterfaceVariantSet = unsafe extern "C" fn (p_self: GDExtensionVariantPtr, p_key: GDExtensionConstVariantPtr, p_value: GDExtensionConstVariantPtr, r_valid: GDExtensionBool);

/**
 * @name variant_set_named
 * @since 4.1
 *
 * Sets a named key on a Variant to a value.
 *
 * @param p_self A pointer to the Variant.
 * @param p_key A pointer to a StringName representing the key.
 * @param p_value A pointer to a Variant representing the value.
 * @param r_valid A pointer to a boolean which will be set to false if the operation is invalid.
 *
 * @see Variant::set_named()
 */
pub(super) type GDExtensionInterfaceVariantSetNamed = unsafe extern "C" fn (p_self: GDExtensionVariantPtr, p_key:GDExtensionConstStringNamePtr, p_value: GDExtensionConstVariantPtr, r_valid: GDExtensionBool);

/**
 * @name variant_set_keyed
 * @since 4.1
 *
 * Sets a keyed property on a Variant to a value.
 *
 * @param p_self A pointer to the Variant.
 * @param p_key A pointer to a Variant representing the key.
 * @param p_value A pointer to a Variant representing the value.
 * @param r_valid A pointer to a boolean which will be set to false if the operation is invalid.
 *
 * @see Variant::set_keyed()
 */
pub(super) type GDExtensionInterfaceVariantSetKeyed = unsafe extern "C" fn (p_self: GDExtensionVariantPtr, p_key: GDExtensionConstVariantPtr, p_value: GDExtensionConstVariantPtr, r_valid: GDExtensionBool);

/**
 * @name variant_set_indexed
 * @since 4.1
 *
 * Sets an index on a Variant to a value.
 *
 * @param p_self A pointer to the Variant.
 * @param p_index The index.
 * @param p_value A pointer to a Variant representing the value.
 * @param r_valid A pointer to a boolean which will be set to false if the operation is invalid.
 * @param r_oob A pointer to a boolean which will be set to true if the index is out of bounds.
 */
pub(super) type GDExtensionInterfaceVariantSetIndexed = unsafe extern "C" fn (p_self: GDExtensionVariantPtr, p_index:GDExtensionInt, p_value: GDExtensionConstVariantPtr, r_valid: GDExtensionBool, r_oob: *mut GDExtensionBool);

/**
 * @name variant_get
 * @since 4.1
 *
 * Gets the value of a key from a Variant.
 *
 * @param p_self A pointer to the Variant.
 * @param p_key A pointer to a Variant representing the key.
 * @param r_ret A pointer to a Variant which will be assigned the value.
 * @param r_valid A pointer to a boolean which will be set to false if the operation is invalid.
 */
pub(super) type GDExtensionInterfaceVariantGet = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr, p_key: GDExtensionConstVariantPtr, r_ret: GDExtensionUninitializedVariantPtr, r_valid: GDExtensionBool);

/**
 * @name variant_get_named
 * @since 4.1
 *
 * Gets the value of a named key from a Variant.
 *
 * @param p_self A pointer to the Variant.
 * @param p_key A pointer to a StringName representing the key.
 * @param r_ret A pointer to a Variant which will be assigned the value.
 * @param r_valid A pointer to a boolean which will be set to false if the operation is invalid.
 */
pub(super) type GDExtensionInterfaceVariantGetNamed = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr, p_key:GDExtensionConstStringNamePtr, r_ret: GDExtensionUninitializedVariantPtr, r_valid: GDExtensionBool);

/**
 * @name variant_get_keyed
 * @since 4.1
 *
 * Gets the value of a keyed property from a Variant.
 *
 * @param p_self A pointer to the Variant.
 * @param p_key A pointer to a Variant representing the key.
 * @param r_ret A pointer to a Variant which will be assigned the value.
 * @param r_valid A pointer to a boolean which will be set to false if the operation is invalid.
 */
pub(super) type GDExtensionInterfaceVariantGetKeyed = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr, p_key: GDExtensionConstVariantPtr, r_ret: GDExtensionUninitializedVariantPtr, r_valid: GDExtensionBool);

/**
 * @name variant_get_indexed
 * @since 4.1
 *
 * Gets the value of an index from a Variant.
 *
 * @param p_self A pointer to the Variant.
 * @param p_index The index.
 * @param r_ret A pointer to a Variant which will be assigned the value.
 * @param r_valid A pointer to a boolean which will be set to false if the operation is invalid.
 * @param r_oob A pointer to a boolean which will be set to true if the index is out of bounds.
 */
pub(super) type GDExtensionInterfaceVariantGetIndexed = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr, p_index:GDExtensionInt, r_ret: GDExtensionUninitializedVariantPtr, r_valid: GDExtensionBool, r_oob: *mut GDExtensionBool);

/**
 * @name variant_iter_init
 * @since 4.1
 *
 * Initializes an iterator over a Variant.
 *
 * @param p_self A pointer to the Variant.
 * @param r_iter A pointer to a Variant which will be assigned the iterator.
 * @param r_valid A pointer to a boolean which will be set to false if the operation is invalid.
 *
 * @return true if the operation is valid; otherwise false.
 *
 * @see Variant::iter_init()
 */
pub(super) type GDExtensionInterfaceVariantIterInit = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr, r_iter: GDExtensionUninitializedVariantPtr, r_valid: GDExtensionBool) -> GDExtensionBool;

/**
 * @name variant_iter_next
 * @since 4.1
 *
 * Gets the next value for an iterator over a Variant.
 *
 * @param p_self A pointer to the Variant.
 * @param r_iter A pointer to a Variant which will be assigned the iterator.
 * @param r_valid A pointer to a boolean which will be set to false if the operation is invalid.
 *
 * @return true if the operation is valid; otherwise false.
 *
 * @see Variant::iter_next()
 */
pub(super) type GDExtensionInterfaceVariantIterNext = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr, r_iter: GDExtensionVariantPtr, r_valid: GDExtensionBool) -> GDExtensionBool;

/**
 * @name variant_iter_get
 * @since 4.1
 *
 * Gets the next value for an iterator over a Variant.
 *
 * @param p_self A pointer to the Variant.
 * @param r_iter A pointer to a Variant which will be assigned the iterator.
 * @param r_ret A pointer to a Variant which will be assigned false if the operation is invalid.
 * @param r_valid A pointer to a boolean which will be set to false if the operation is invalid.
 *
 * @see Variant::iter_get()
 */
pub(super) type GDExtensionInterfaceVariantIterGet = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr, r_iter: GDExtensionVariantPtr, r_ret: GDExtensionUninitializedVariantPtr, r_valid: GDExtensionBool);

/**
 * @name variant_hash
 * @since 4.1
 *
 * Gets the hash of a Variant.
 *
 * @param p_self A pointer to the Variant.
 *
 * @return The hash value.
 *
 * @see Variant::hash()
 */
pub(super) type GDExtensionInterfaceVariantHash = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr) -> GDExtensionInt;

/**
 * @name variant_recursive_hash
 * @since 4.1
 *
 * Gets the recursive hash of a Variant.
 *
 * @param p_self A pointer to the Variant.
 * @param p_recursion_count The number of recursive loops so far.
 *
 * @return The hash value.
 *
 * @see Variant::recursive_hash()
 */
pub(super) type GDExtensionInterfaceVariantRecursiveHash = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr, p_recursion_count: GDExtensionInt) -> GDExtensionInt;

/**
 * @name variant_hash_compare
 * @since 4.1
 *
 * Compares two Variants by their hash.
 *
 * @param p_self A pointer to the Variant.
 * @param p_other A pointer to the other Variant to compare it to.
 *
 * @return The hash value.
 *
 * @see Variant::hash_compare()
 */
pub(super) type GDExtensionInterfaceVariantHashCompare = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr, p_other: GDExtensionConstVariantPtr) -> GDExtensionBool;

/**
 * @name variant_booleanize
 * @since 4.1
 *
 * Converts a Variant to a boolean.
 *
 * @param p_self A pointer to the Variant.
 *
 * @return The boolean value of the Variant.
 */
pub(super) type GDExtensionInterfaceVariantBooleanize = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr) -> GDExtensionBool;

/**
 * @name variant_duplicate
 * @since 4.1
 *
 * Duplicates a Variant.
 *
 * @param p_self A pointer to the Variant.
 * @param r_ret A pointer to a Variant to store the duplicated value.
 * @param p_deep Whether or not to duplicate deeply (when supported by the Variant type).
 */
pub(super) type GDExtensionInterfaceVariantDuplicate = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr, r_ret: GDExtensionVariantPtr, p_deep: GDExtensionBool);

/**
 * @name variant_stringify
 * @since 4.1
 *
 * Converts a Variant to a string.
 *
 * @param p_self A pointer to the Variant.
 * @param r_ret A pointer to a String to store the resulting value.
 */
pub(super) type GDExtensionInterfaceVariantStringify = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr, r_ret: GDExtensionStringPtr);

/**
 * @name variant_get_type
 * @since 4.1
 *
 * Gets the type of a Variant.
 *
 * @param p_self A pointer to the Variant.
 *
 * @return The variant type.
 */
pub(super) type GDExtensionInterfaceVariantGetType = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr) -> GDExtensionVariantType;

/**
 * @name variant_has_method
 * @since 4.1
 *
 * Checks if a Variant has the given method.
 *
 * @param p_self A pointer to the Variant.
 * @param p_method A pointer to a StringName with the method name.
 *
 * @return
 */
pub(super) type GDExtensionInterfaceVariantHasMethod = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr, p_method: GDExtensionConstStringNamePtr) -> GDExtensionBool;

/**
 * @name variant_has_member
 * @since 4.1
 *
 * Checks if a type of Variant has the given member.
 *
 * @param p_type The Variant type.
 * @param p_member A pointer to a StringName with the member name.
 *
 * @return
 */
pub(super) type GDExtensionInterfaceVariantHasMember = unsafe extern "C" fn (p_type: GDExtensionVariantType, p_member: GDExtensionConstStringNamePtr) -> GDExtensionBool;

/**
 * @name variant_has_key
 * @since 4.1
 *
 * Checks if a Variant has a key.
 *
 * @param p_self A pointer to the Variant.
 * @param p_key A pointer to a Variant representing the key.
 * @param r_valid A pointer to a boolean which will be set to false if the key doesn't exist.
 *
 * @return true if the key exists; otherwise false.
 */
pub(super) type GDExtensionInterfaceVariantHasKey = unsafe extern "C" fn (p_self: GDExtensionConstVariantPtr, p_key: GDExtensionConstVariantPtr, r_valid: GDExtensionBool) -> GDExtensionBool;

/**
 * @name variant_get_type_name
 * @since 4.1
 *
 * Gets the name of a Variant type.
 *
 * @param p_type The Variant type.
 * @param r_name A pointer to a String to store the Variant type name.
 */
pub(super) type GDExtensionInterfaceVariantGetTypeName = unsafe extern "C" fn (p_type: GDExtensionVariantType, r_name: GDExtensionUninitializedStringPtr);

/**
 * @name variant_can_convert
 * @since 4.1
 *
 * Checks if Variants can be converted from one type to another.
 *
 * @param p_from The Variant type to convert from.
 * @param p_to The Variant type to convert to.
 *
 * @return true if the conversion is possible; otherwise false.
 */
pub(super) type GDExtensionInterfaceVariantCanConvert = unsafe extern "C" fn (p_from: GDExtensionVariantType, p_to: GDExtensionVariantType) -> GDExtensionBool;

/**
 * @name variant_can_convert_strict
 * @since 4.1
 *
 * Checks if Variant can be converted from one type to another using stricter rules.
 *
 * @param p_from The Variant type to convert from.
 * @param p_to The Variant type to convert to.
 *
 * @return true if the conversion is possible; otherwise false.
 */
pub(super) type GDExtensionInterfaceVariantCanConvertStrict = unsafe extern "C" fn (p_from: GDExtensionVariantType, p_to: GDExtensionVariantType) -> GDExtensionBool;

/**
 * @name get_variant_from_type_constructor
 * @since 4.1
 *
 * Gets a pointer to a function that can create a Variant of the given type from a raw value.
 *
 * @param p_type The Variant type.
 *
 * @return A pointer to a function that can create a Variant of the given type from a raw value.
 */
pub(super) type GDExtensionInterfaceGetVariantFromTypeConstructor = unsafe extern "C" fn (p_type: GDExtensionVariantType) -> GDExtensionVariantFromTypeConstructorFunc;

/**
 * @name get_variant_to_type_constructor
 * @since 4.1
 *
 * Gets a pointer to a function that can get the raw value from a Variant of the given type.
 *
 * @param p_type The Variant type.
 *
 * @return A pointer to a function that can get the raw value from a Variant of the given type.
 */
pub(super) type GDExtensionInterfaceGetVariantToTypeConstructor = unsafe extern "C" fn (p_type: GDExtensionVariantType) -> GDExtensionTypeFromVariantConstructorFunc;

/**
 * @name variant_get_ptr_operator_evaluator
 * @since 4.1
 *
 * Gets a pointer to a function that can evaluate the given Variant operator on the given Variant types.
 *
 * @param p_operator The variant operator.
 * @param p_type_a The type of the first Variant.
 * @param p_type_b The type of the second Variant.
 *
 * @return A pointer to a function that can evaluate the given Variant operator on the given Variant types.
 */
pub(super) type GDExtensionInterfaceVariantGetPtrOperatorEvaluator = unsafe extern "C" fn (p_operator: GDExtensionVariantOperator, p_type_a: GDExtensionVariantType, p_type_b: GDExtensionVariantType) -> GDExtensionPtrOperatorEvaluator;

/**
 * @name variant_get_ptr_builtin_method
 * @since 4.1
 *
 * Gets a pointer to a function that can call a builtin method on a type of Variant.
 *
 * @param p_type The Variant type.
 * @param p_method A pointer to a StringName with the method name.
 * @param p_hash A hash representing the method signature.
 *
 * @return A pointer to a function that can call a builtin method on a type of Variant.
 */
pub(super) type GDExtensionInterfaceVariantGetPtrBuiltinMethod = unsafe extern "C" fn (p_type: GDExtensionVariantType, p_method: GDExtensionConstStringNamePtr, p_hash: GDExtensionInt) -> GDExtensionPtrBuiltInMethod;

/**
 * @name variant_get_ptr_constructor
 * @since 4.1
 *
 * Gets a pointer to a function that can call one of the constructors for a type of Variant.
 *
 * @param p_type The Variant type.
 * @param p_constructor The index of the constructor.
 *
 * @return A pointer to a function that can call one of the constructors for a type of Variant.
 */
pub(super) type GDExtensionInterfaceVariantGetPtrConstructor = unsafe extern "C" fn (p_type: GDExtensionVariantType, p_constructor: i32) -> GDExtensionPtrConstructor;

/**
 * @name variant_get_ptr_destructor
 * @since 4.1
 *
 * Gets a pointer to a function than can call the destructor for a type of Variant.
 *
 * @param p_type The Variant type.
 *
 * @return A pointer to a function than can call the destructor for a type of Variant.
 */
pub(super) type GDExtensionInterfaceVariantGetPtrDestructor = unsafe extern "C" fn (p_type: GDExtensionVariantType) -> GDExtensionPtrDestructor;

/**
 * @name variant_construct
 * @since 4.1
 *
 * Constructs a Variant of the given type, using the first constructor that matches the given arguments.
 *
 * @param p_type The Variant type.
 * @param p_base A pointer to a Variant to store the constructed value.
 * @param p_args A pointer to a C array of Variant pointers representing the arguments for the constructor.
 * @param p_argument_count The number of arguments to pass to the constructor.
 * @param r_error A pointer the structure which will be updated with error information.
 */
pub(super) type GDExtensionInterfaceVariantConstruct = unsafe extern "C" fn (p_type: GDExtensionVariantType, r_base: GDExtensionUninitializedVariantPtr, p_args: *const GDExtensionConstVariantPtr, p_argument_count: i32, r_error: GDExtensionCallError);

/**
 * @name variant_get_ptr_setter
 * @since 4.1
 *
 * Gets a pointer to a function that can call a member's setter on the given Variant type.
 *
 * @param p_type The Variant type.
 * @param p_member A pointer to a StringName with the member name.
 *
 * @return A pointer to a function that can call a member's setter on the given Variant type.
 */
pub(super) type GDExtensionInterfaceVariantGetPtrSetter = unsafe extern "C" fn (p_type: GDExtensionVariantType, p_member: GDExtensionConstStringNamePtr) -> GDExtensionPtrSetter;

/**
 * @name variant_get_ptr_getter
 * @since 4.1
 *
 * Gets a pointer to a function that can call a member's getter on the given Variant type.
 *
 * @param p_type The Variant type.
 * @param p_member A pointer to a StringName with the member name.
 *
 * @return A pointer to a function that can call a member's getter on the given Variant type.
 */
pub(super) type GDExtensionInterfaceVariantGetPtrGetter = unsafe extern "C" fn (p_type: GDExtensionVariantType, p_member: GDExtensionConstStringNamePtr) -> GDExtensionPtrGetter;

/**
 * @name variant_get_ptr_indexed_setter
 * @since 4.1
 *
 * Gets a pointer to a function that can set an index on the given Variant type.
 *
 * @param p_type The Variant type.
 *
 * @return A pointer to a function that can set an index on the given Variant type.
 */
pub(super) type GDExtensionInterfaceVariantGetPtrIndexedSetter = unsafe extern "C" fn (p_type: GDExtensionVariantType) -> GDExtensionPtrIndexedSetter;

/**
 * @name variant_get_ptr_indexed_getter
 * @since 4.1
 *
 * Gets a pointer to a function that can get an index on the given Variant type.
 *
 * @param p_type The Variant type.
 *
 * @return A pointer to a function that can get an index on the given Variant type.
 */
pub(super) type GDExtensionInterfaceVariantGetPtrIndexedGetter = unsafe extern "C" fn (p_type: GDExtensionVariantType) -> GDExtensionPtrIndexedGetter;

/**
 * @name variant_get_ptr_keyed_setter
 * @since 4.1
 *
 * Gets a pointer to a function that can set a key on the given Variant type.
 *
 * @param p_type The Variant type.
 *
 * @return A pointer to a function that can set a key on the given Variant type.
 */
pub(super) type GDExtensionInterfaceVariantGetPtrKeyedSetter = unsafe extern "C" fn (p_type: GDExtensionVariantType) -> GDExtensionPtrKeyedSetter;

/**
 * @name variant_get_ptr_keyed_getter
 * @since 4.1
 *
 * Gets a pointer to a function that can get a key on the given Variant type.
 *
 * @param p_type The Variant type.
 *
 * @return A pointer to a function that can get a key on the given Variant type.
 */
pub(super) type GDExtensionInterfaceVariantGetPtrKeyedGetter = unsafe extern "C" fn (p_type: GDExtensionVariantType) -> GDExtensionPtrKeyedGetter;

/**
 * @name variant_get_ptr_keyed_checker
 * @since 4.1
 *
 * Gets a pointer to a function that can check a key on the given Variant type.
 *
 * @param p_type The Variant type.
 *
 * @return A pointer to a function that can check a key on the given Variant type.
 */
pub(super) type GDExtensionInterfaceVariantGetPtrKeyedChecker = unsafe extern "C" fn (p_type: GDExtensionVariantType) -> GDExtensionPtrKeyedChecker;

/**
 * @name variant_get_constant_value
 * @since 4.1
 *
 * Gets the value of a constant from the given Variant type.
 *
 * @param p_type The Variant type.
 * @param p_constant A pointer to a StringName with the constant name.
 * @param r_ret A pointer to a Variant to store the value.
 */
pub(super) type GDExtensionInterfaceVariantGetConstantValue = unsafe extern "C" fn (p_type: GDExtensionVariantType, p_constant: GDExtensionConstStringNamePtr, r_ret: GDExtensionUninitializedVariantPtr);

/**
 * @name variant_get_ptr_utility_function
 * @since 4.1
 *
 * Gets a pointer to a function that can call a Variant utility function.
 *
 * @param p_function A pointer to a StringName with the function name.
 * @param p_hash A hash representing the function signature.
 *
 * @return A pointer to a function that can call a Variant utility function.
 */
pub(super) type GDExtensionInterfaceVariantGetPtrUtilityFunction = unsafe extern "C" fn (p_function: GDExtensionConstStringNamePtr, p_hash: GDExtensionInt) -> GDExtensionPtrUtilityFunction;

/* INTERFACE: String Utilities */

/**
 * @name string_new_with_latin1_chars
 * @since 4.1
 *
 * Creates a String from a Latin-1 encoded C string.
 *
 * @param r_dest A pointer to a Variant to hold the newly created String.
 * @param p_contents A pointer to a Latin-1 encoded C string (null terminated).
 */
pub(super) type GDExtensionInterfaceStringNewWithLatin1Chars = unsafe extern "C" fn (r_dest: GDExtensionUninitializedStringPtr, p_contents: const_char_p);

/**
 * @name string_new_with_utf8_chars
 * @since 4.1
 *
 * Creates a String from a UTF-8 encoded C string.
 *
 * @param r_dest A pointer to a Variant to hold the newly created String.
 * @param p_contents A pointer to a UTF-8 encoded C string (null terminated).
 */
pub(super) type GDExtensionInterfaceStringNewWithUtf8Chars = unsafe extern "C" fn (r_dest: GDExtensionUninitializedStringPtr, p_contents: const_char_p);

/**
 * @name string_new_with_utf16_chars
 * @since 4.1
 *
 * Creates a String from a UTF-16 encoded C string.
 *
 * @param r_dest A pointer to a Variant to hold the newly created String.
 * @param p_contents A pointer to a UTF-16 encoded C string (null terminated).
 */
pub(super) type GDExtensionInterfaceStringNewWithUtf16Chars = unsafe extern "C" fn (r_dest: GDExtensionUninitializedStringPtr, p_contents: *const c16);

/**
 * @name string_new_with_utf32_chars
 * @since 4.1
 *
 * Creates a String from a UTF-32 encoded C string.
 *
 * @param r_dest A pointer to a Variant to hold the newly created String.
 * @param p_contents A pointer to a UTF-32 encoded C string (null terminated).
 */
pub(super) type GDExtensionInterfaceStringNewWithUtf32Chars = unsafe extern "C" fn (r_dest: GDExtensionUninitializedStringPtr, p_contents: *const c32);

/**
 * @name string_new_with_wide_chars
 * @since 4.1
 *
 * Creates a String from a wide C string.
 *
 * @param r_dest A pointer to a Variant to hold the newly created String.
 * @param p_contents A pointer to a wide C string (null terminated).
 */
pub(super) type GDExtensionInterfaceStringNewWithWideChars = unsafe extern "C" fn (r_dest: GDExtensionUninitializedStringPtr, p_contents: const_wchar_p);

/**
 * @name string_new_with_latin1_chars_and_len
 * @since 4.1
 *
 * Creates a String from a Latin-1 encoded C string with the given length.
 *
 * @param r_dest A pointer to a Variant to hold the newly created String.
 * @param p_contents A pointer to a Latin-1 encoded C string.
 * @param p_size The number of characters (= number of bytes).
 */
pub(super) type GDExtensionInterfaceStringNewWithLatin1CharsAndLen = unsafe extern "C" fn (r_dest: GDExtensionUninitializedStringPtr, p_contents: const_char_p, p_size: GDExtensionInt);

/**
 * @name string_new_with_utf8_chars_and_len
 * @since 4.1
 * @deprecated in Godot 4.3. Use `string_new_with_utf8_chars_and_len2` instead.
 *
 * Creates a String from a UTF-8 encoded C string with the given length.
 *
 * @param r_dest A pointer to a Variant to hold the newly created String.
 * @param p_contents A pointer to a UTF-8 encoded C string.
 * @param p_size The number of bytes (not code units).
 */
pub(super) type GDExtensionInterfaceStringNewWithUtf8CharsAndLen = unsafe extern "C" fn (r_dest: GDExtensionUninitializedStringPtr, p_contents: const_char_p, p_size: GDExtensionInt);

/**
 * @name string_new_with_utf8_chars_and_len2
 * @since 4.3
 *
 * Creates a String from a UTF-8 encoded C string with the given length.
 *
 * @param r_dest A pointer to a Variant to hold the newly created String.
 * @param p_contents A pointer to a UTF-8 encoded C string.
 * @param p_size The number of bytes (not code units).
 *
 * @return Error code signifying if the operation successful.
 */
pub(super) type GDExtensionInterfaceStringNewWithUtf8CharsAndLen2 = unsafe extern "C" fn (r_dest: GDExtensionUninitializedStringPtr, p_contents: const_char_p, p_size: GDExtensionInt) -> GDExtensionInt;

/**
 * @name string_new_with_utf16_chars_and_len
 * @since 4.1
 * @deprecated in Godot 4.3. Use `string_new_with_utf16_chars_and_len2` instead.
 *
 * Creates a String from a UTF-16 encoded C string with the given length.
 *
 * @param r_dest A pointer to a Variant to hold the newly created String.
 * @param p_contents A pointer to a UTF-16 encoded C string.
 * @param p_size The number of characters (not bytes).
 */
pub(super) type GDExtensionInterfaceStringNewWithUtf16CharsAndLen = unsafe extern "C" fn (r_dest: GDExtensionUninitializedStringPtr, p_contents: *const c16, p_char_count: GDExtensionInt);

/**
 * @name string_new_with_utf16_chars_and_len2
 * @since 4.3
 *
 * Creates a String from a UTF-16 encoded C string with the given length.
 *
 * @param r_dest A pointer to a Variant to hold the newly created String.
 * @param p_contents A pointer to a UTF-16 encoded C string.
 * @param p_size The number of characters (not bytes).
 * @param p_default_little_endian If true, UTF-16 use little endian.
 *
 * @return Error code signifying if the operation successful.
 */
pub(super) type GDExtensionInterfaceStringNewWithUtf16CharsAndLen2 = unsafe extern "C" fn (r_dest: GDExtensionUninitializedStringPtr, p_contents: *const c16, p_char_count: GDExtensionInt, p_default_little_endian: GDExtensionBool) -> GDExtensionInt;

/**
 * @name string_new_with_utf32_chars_and_len
 * @since 4.1
 *
 * Creates a String from a UTF-32 encoded C string with the given length.
 *
 * @param r_dest A pointer to a Variant to hold the newly created String.
 * @param p_contents A pointer to a UTF-32 encoded C string.
 * @param p_size The number of characters (not bytes).
 */
pub(super) type GDExtensionInterfaceStringNewWithUtf32CharsAndLen = unsafe extern "C" fn (r_dest: GDExtensionUninitializedStringPtr, p_contents: *const c32, p_char_count: GDExtensionInt);

/**
 * @name string_new_with_wide_chars_and_len
 * @since 4.1
 *
 * Creates a String from a wide C string with the given length.
 *
 * @param r_dest A pointer to a Variant to hold the newly created String.
 * @param p_contents A pointer to a wide C string.
 * @param p_size The number of characters (not bytes).
 */
pub(super) type GDExtensionInterfaceStringNewWithWideCharsAndLen = unsafe extern "C" fn (r_dest: GDExtensionUninitializedStringPtr, p_contents: const_wchar_p, p_char_count: GDExtensionInt);

/**
 * @name string_to_latin1_chars
 * @since 4.1
 *
 * Converts a String to a Latin-1 encoded C string.
 *
 * It doesn't write a null terminator.
 *
 * @param p_self A pointer to the String.
 * @param r_text A pointer to the buffer to hold the resulting data. If NULL is passed in, only the length will be computed.
 * @param p_max_write_length The maximum number of characters that can be written to r_text. It has no affect on the return value.
 *
 * @return The resulting encoded string length in characters (not bytes), not including a null terminator.
 */
pub(super) type GDExtensionInterfaceStringToLatin1Chars = unsafe extern "C" fn (p_self: GDExtensionConstStringPtr, r_text: char_p, p_max_write_length: GDExtensionInt) -> GDExtensionInt;

/**
 * @name string_to_utf8_chars
 * @since 4.1
 *
 * Converts a String to a UTF-8 encoded C string.
 *
 * It doesn't write a null terminator.
 *
 * @param p_self A pointer to the String.
 * @param r_text A pointer to the buffer to hold the resulting data. If NULL is passed in, only the length will be computed.
 * @param p_max_write_length The maximum number of characters that can be written to r_text. It has no affect on the return value.
 *
 * @return The resulting encoded string length in characters (not bytes), not including a null terminator.
 */
pub(super) type GDExtensionInterfaceStringToUtf8Chars = unsafe extern "C" fn (p_self: GDExtensionConstStringPtr, r_text: char_p, p_max_write_length: GDExtensionInt) -> GDExtensionInt;

/**
 * @name string_to_utf16_chars
 * @since 4.1
 *
 * Converts a String to a UTF-16 encoded C string.
 *
 * It doesn't write a null terminator.
 *
 * @param p_self A pointer to the String.
 * @param r_text A pointer to the buffer to hold the resulting data. If NULL is passed in, only the length will be computed.
 * @param p_max_write_length The maximum number of characters that can be written to r_text. It has no affect on the return value.
 *
 * @return The resulting encoded string length in characters (not bytes), not including a null terminator.
 */
pub(super) type GDExtensionInterfaceStringToUtf16Chars = unsafe extern "C" fn (p_self: GDExtensionConstStringPtr, r_text: *mut c16, p_max_write_length: GDExtensionInt) -> GDExtensionInt;

/**
 * @name string_to_utf32_chars
 * @since 4.1
 *
 * Converts a String to a UTF-32 encoded C string.
 *
 * It doesn't write a null terminator.
 *
 * @param p_self A pointer to the String.
 * @param r_text A pointer to the buffer to hold the resulting data. If NULL is passed in, only the length will be computed.
 * @param p_max_write_length The maximum number of characters that can be written to r_text. It has no affect on the return value.
 *
 * @return The resulting encoded string length in characters (not bytes), not including a null terminator.
 */
pub(super) type GDExtensionInterfaceStringToUtf32Chars = unsafe extern "C" fn (p_self: GDExtensionConstStringPtr, r_text: *mut c32, p_max_write_length: GDExtensionInt) -> GDExtensionInt;

/**
 * @name string_to_wide_chars
 * @since 4.1
 *
 * Converts a String to a wide C string.
 *
 * It doesn't write a null terminator.
 *
 * @param p_self A pointer to the String.
 * @param r_text A pointer to the buffer to hold the resulting data. If NULL is passed in, only the length will be computed.
 * @param p_max_write_length The maximum number of characters that can be written to r_text. It has no affect on the return value.
 *
 * @return The resulting encoded string length in characters (not bytes), not including a null terminator.
 */
pub(super) type GDExtensionInterfaceStringToWideChars = unsafe extern "C" fn (p_self: GDExtensionConstStringPtr, r_text: wchar_p, p_max_write_length: GDExtensionInt) -> GDExtensionInt;

/**
 * @name string_operator_index
 * @since 4.1
 *
 * Gets a pointer to the character at the given index from a String.
 *
 * @param p_self A pointer to the String.
 * @param p_index The index.
 *
 * @return A pointer to the requested character.
 */
pub(super) type GDExtensionInterfaceStringOperatorIndex = unsafe extern "C" fn (p_self:GDExtensionStringPtr, p_index: GDExtensionInt) -> *mut c32;

/**
 * @name string_operator_index_const
 * @since 4.1
 *
 * Gets a const pointer to the character at the given index from a String.
 *
 * @param p_self A pointer to the String.
 * @param p_index The index.
 *
 * @return A const pointer to the requested character.
 */
pub(super) type GDExtensionInterfaceStringOperatorIndexConst = unsafe extern "C" fn (p_self: GDExtensionConstStringPtr, p_index: GDExtensionInt) -> *const c32;

/**
 * @name string_operator_plus_eq_string
 * @since 4.1
 *
 * Appends another String to a String.
 *
 * @param p_self A pointer to the String.
 * @param p_b A pointer to the other String to append.
 */
pub(super) type GDExtensionInterfaceStringOperatorPlusEqString = unsafe extern "C" fn (p_self: GDExtensionStringPtr, p_b: GDExtensionConstStringPtr);

/**
 * @name string_operator_plus_eq_char
 * @since 4.1
 *
 * Appends a character to a String.
 *
 * @param p_self A pointer to the String.
 * @param p_b A pointer to the character to append.
 */
pub(super) type GDExtensionInterfaceStringOperatorPlusEqChar = unsafe extern "C" fn (p_self: GDExtensionStringPtr, p_b: c32);

/**
 * @name string_operator_plus_eq_cstr
 * @since 4.1
 *
 * Appends a Latin-1 encoded C string to a String.
 *
 * @param p_self A pointer to the String.
 * @param p_b A pointer to a Latin-1 encoded C string (null terminated).
 */
pub(super) type GDExtensionInterfaceStringOperatorPlusEqCstr = unsafe extern "C" fn (p_self: GDExtensionStringPtr, p_b: const_char_p);

/**
 * @name string_operator_plus_eq_wcstr
 * @since 4.1
 *
 * Appends a wide C string to a String.
 *
 * @param p_self A pointer to the String.
 * @param p_b A pointer to a wide C string (null terminated).
 */
pub(super) type GDExtensionInterfaceStringOperatorPlusEqWcstr = unsafe extern "C" fn (p_self: GDExtensionStringPtr, p_b: const_wchar_p);

/**
 * @name string_operator_plus_eq_c32str
 * @since 4.1
 *
 * Appends a UTF-32 encoded C string to a String.
 *
 * @param p_self A pointer to the String.
 * @param p_b A pointer to a UTF-32 encoded C string (null terminated).
 */
pub(super) type GDExtensionInterfaceStringOperatorPlusEqC32str = unsafe extern "C" fn (p_self: GDExtensionStringPtr, p_b: *const c32);

/**
 * @name string_resize
 * @since 4.2
 *
 * Resizes the underlying string data to the given number of characters.
 *
 * Space needs to be allocated for the null terminating character ('\0') which
 * also must be added manually, in order for all string functions to work correctly.
 *
 * Warning: This is an error-prone operation - only use it if there's no other
 * efficient way to accomplish your goal.
 *
 * @param p_self A pointer to the String.
 * @param p_resize The new length for the String.
 *
 * @return Error code signifying if the operation successful.
 */
pub(super) type GDExtensionInterfaceStringResize = unsafe extern "C" fn (p_self:GDExtensionStringPtr, p_resize: GDExtensionInt) -> GDExtensionInt;

/* INTERFACE: StringName Utilities */

/**
 * @name string_name_new_with_latin1_chars
 * @since 4.2
 *
 * Creates a StringName from a Latin-1 encoded C string.
 *
 * If `p_is_static` is true, then:
 * - The StringName will reuse the `p_contents` buffer instead of copying it.
 *   You must guarantee that the buffer remains valid for the duration of the application (e.g. string literal).
 * - You must not call a destructor for this StringName. Incrementing the initial reference once should achieve this.
 *
 * `p_is_static` is purely an optimization and can easily introduce undefined behavior if used wrong. In case of doubt, set it to false.
 *
 * @param r_dest A pointer to uninitialized storage, into which the newly created StringName is constructed.
 * @param p_contents A pointer to a C string (null terminated and Latin-1 or ASCII encoded).
 * @param p_is_static Whether the StringName reuses the buffer directly (see above).
 */
pub(super) type GDExtensionInterfaceStringNameNewWithLatin1Chars = unsafe extern "C" fn (r_dest: GDExtensionUninitializedStringNamePtr, p_contents: const_char_p, p_is_static: GDExtensionBool);

/**
 * @name string_name_new_with_utf8_chars
 * @since 4.2
 *
 * Creates a StringName from a UTF-8 encoded C string.
 *
 * @param r_dest A pointer to uninitialized storage, into which the newly created StringName is constructed.
 * @param p_contents A pointer to a C string (null terminated and UTF-8 encoded).
 */
pub(super) type GDExtensionInterfaceStringNameNewWithUtf8Chars = unsafe extern "C" fn (r_dest: GDExtensionUninitializedStringNamePtr, p_contents: const_char_p);

/**
 * @name string_name_new_with_utf8_chars_and_len
 * @since 4.2
 *
 * Creates a StringName from a UTF-8 encoded string with a given number of characters.
 *
 * @param r_dest A pointer to uninitialized storage, into which the newly created StringName is constructed.
 * @param p_contents A pointer to a C string (null terminated and UTF-8 encoded).
 * @param p_size The number of bytes (not UTF-8 code points).
 */
pub(super) type GDExtensionInterfaceStringNameNewWithUtf8CharsAndLen = unsafe extern "C" fn (r_dest: GDExtensionUninitializedStringNamePtr, p_contents: const_char_p, p_size: GDExtensionInt);

/* INTERFACE: XMLParser Utilities */

/**
 * @name xml_parser_open_buffer
 * @since 4.1
 *
 * Opens a raw XML buffer on an XMLParser instance.
 *
 * @param p_instance A pointer to an XMLParser object.
 * @param p_buffer A pointer to the buffer.
 * @param p_size The size of the buffer.
 *
 * @return A Godot error code (ex. OK, ERR_INVALID_DATA, etc).
 *
 * @see XMLParser::open_buffer()
 */
pub(super) type GDExtensionInterfaceXmlParserOpenBuffer = unsafe extern "C" fn (p_instance: GDExtensionObjectPtr, p_buffer: *const u8, p_size: usize) -> GDExtensionInt;

/* INTERFACE: FileAccess Utilities */

/**
 * @name file_access_store_buffer
 * @since 4.1
 *
 * Stores the given buffer using an instance of FileAccess.
 *
 * @param p_instance A pointer to a FileAccess object.
 * @param p_src A pointer to the buffer.
 * @param p_length The size of the buffer.
 *
 * @see FileAccess::store_buffer()
 */
pub(super) type GDExtensionInterfaceFileAccessStoreBuffer = unsafe extern "C" fn (p_instance: GDExtensionObjectPtr, p_src: *const u8, p_length: u64);

/**
 * @name file_access_get_buffer
 * @since 4.1
 *
 * Reads the next p_length bytes into the given buffer using an instance of FileAccess.
 *
 * @param p_instance A pointer to a FileAccess object.
 * @param p_dst A pointer to the buffer to store the data.
 * @param p_length The requested number of bytes to read.
 *
 * @return The actual number of bytes read (may be less than requested).
 */
pub(super) type GDExtensionInterfaceFileAccessGetBuffer = unsafe extern "C" fn (p_instance: GDExtensionConstObjectPtr, p_dst: u8, p_length: u64) -> u64;

/* INTERFACE: Image Utilities */

/**
 * @name image_ptrw
 * @since 4.3
 *
 * Returns writable pointer to internal Image buffer.
 *
 * @param p_instance A pointer to a Image object.
 *
 * @return Pointer to internal Image buffer.
 *
 * @see Image::ptrw()
 */
pub(super) type GDExtensionInterfaceImagePtrw = unsafe extern "C" fn (p_instance: GDExtensionObjectPtr) -> *mut u8;

/**
 * @name image_ptr
 * @since 4.3
 *
 * Returns read only pointer to internal Image buffer.
 *
 * @param p_instance A pointer to a Image object.
 *
 * @return Pointer to internal Image buffer.
 *
 * @see Image::ptr()
 */
pub(super) type GDExtensionInterfaceImagePtr = unsafe extern "C" fn (p_instance: GDExtensionObjectPtr) -> *const u8;

/* INTERFACE: WorkerThreadPool Utilities */


/**
 * @name worker_thread_pool_add_native_group_task
 * @since 4.1
 *
 * Adds a group task to an instance of WorkerThreadPool.
 *
 * @param p_instance A pointer to a WorkerThreadPool object.
 * @param p_func A pointer to a function to run in the thread pool.
 * @param p_userdata A pointer to arbitrary data which will be passed to p_func.
 * @param p_tasks The number of tasks needed in the group.
 * @param p_high_priority Whether or not this is a high priority task.
 * @param p_description A pointer to a String with the task description.
 *
 * @return The task group ID.
 *
 * @see WorkerThreadPool::add_group_task()
 */
pub(super) type GDExtensionInterfaceWorkerThreadPoolAddNativeGroupTask = unsafe extern "C" fn (p_instance: GDExtensionObjectPtr, p_func: unsafe extern "C" fn (void_p, u32), p_userdata: void_p, p_elements: ffi::c_int, p_tasks: ffi::c_int, p_high_priority: GDExtensionBool, p_description: GDExtensionConstStringPtr) -> i64;

/**
 * @name worker_thread_pool_add_native_task
 * @since 4.1
 *
 * Adds a task to an instance of WorkerThreadPool.
 *
 * @param p_instance A pointer to a WorkerThreadPool object.
 * @param p_func A pointer to a function to run in the thread pool.
 * @param p_userdata A pointer to arbitrary data which will be passed to p_func.
 * @param p_high_priority Whether or not this is a high priority task.
 * @param p_description A pointer to a String with the task description.
 *
 * @return The task ID.
 */
pub(super) type GDExtensionInterfaceWorkerThreadPoolAddNativeTask = unsafe extern "C" fn (p_instance: GDExtensionObjectPtr, p_func: unsafe extern "C" fn (void_p), p_userdata: void_p, p_high_priority: GDExtensionBool, p_description: GDExtensionConstStringPtr) -> i64;

/* INTERFACE: Packed Array */

/**
 * @name packed_byte_array_operator_index
 * @since 4.1
 *
 * Gets a pointer to a byte in a PackedByteArray.
 *
 * @param p_self A pointer to a PackedByteArray object.
 * @param p_index The index of the byte to get.
 *
 * @return A pointer to the requested byte.
 */
pub(super) type GDExtensionInterfacePackedByteArrayOperatorIndex = unsafe extern "C" fn (p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> *mut u8;

/**
 * @name packed_byte_array_operator_index_const
 * @since 4.1
 *
 * Gets a const pointer to a byte in a PackedByteArray.
 *
 * @param p_self A const pointer to a PackedByteArray object.
 * @param p_index The index of the byte to get.
 *
 * @return A const pointer to the requested byte.
 */
pub(super) type GDExtensionInterfacePackedByteArrayOperatorIndexConst = unsafe extern "C" fn (p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> *const u8;

/**
 * @name packed_float32_array_operator_index
 * @since 4.1
 *
 * Gets a pointer to a 32-bit float in a PackedFloat32Array.
 *
 * @param p_self A pointer to a PackedFloat32Array object.
 * @param p_index The index of the float to get.
 *
 * @return A pointer to the requested 32-bit float.
 */
pub(super) type GDExtensionInterfacePackedFloat32ArrayOperatorIndex = unsafe extern "C" fn (p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> *mut ffi::c_float;

/**
 * @name packed_float32_array_operator_index_const
 * @since 4.1
 *
 * Gets a const pointer to a 32-bit float in a PackedFloat32Array.
 *
 * @param p_self A const pointer to a PackedFloat32Array object.
 * @param p_index The index of the float to get.
 *
 * @return A const pointer to the requested 32-bit float.
 */
pub(super) type GDExtensionInterfacePackedFloat32ArrayOperatorIndexConst = unsafe extern "C" fn (p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> *const ffi::c_float;

/**
 * @name packed_float64_array_operator_index
 * @since 4.1
 *
 * Gets a pointer to a 64-bit float in a PackedFloat64Array.
 *
 * @param p_self A pointer to a PackedFloat64Array object.
 * @param p_index The index of the float to get.
 *
 * @return A pointer to the requested 64-bit float.
 */
pub(super) type GDExtensionInterfacePackedFloat64ArrayOperatorIndex = unsafe extern "C" fn (p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> *mut ffi::c_double;

/**
 * @name packed_float64_array_operator_index_const
 * @since 4.1
 *
 * Gets a const pointer to a 64-bit float in a PackedFloat64Array.
 *
 * @param p_self A const pointer to a PackedFloat64Array object.
 * @param p_index The index of the float to get.
 *
 * @return A const pointer to the requested 64-bit float.
 */
pub(super) type GDExtensionInterfacePackedFloat64ArrayOperatorIndexConst = unsafe extern "C" fn (p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> *const ffi::c_double;

/**
 * @name packed_int32_array_operator_index
 * @since 4.1
 *
 * Gets a pointer to a 32-bit integer in a PackedInt32Array.
 *
 * @param p_self A pointer to a PackedInt32Array object.
 * @param p_index The index of the integer to get.
 *
 * @return A pointer to the requested 32-bit integer.
 */
pub(super) type GDExtensionInterfacePackedInt32ArrayOperatorIndex = unsafe extern "C" fn (p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> *mut i32;

/**
 * @name packed_int32_array_operator_index_const
 * @since 4.1
 *
 * Gets a const pointer to a 32-bit integer in a PackedInt32Array.
 *
 * @param p_self A const pointer to a PackedInt32Array object.
 * @param p_index The index of the integer to get.
 *
 * @return A const pointer to the requested 32-bit integer.
 */
pub(super) type GDExtensionInterfacePackedInt32ArrayOperatorIndexConst = unsafe extern "C" fn (p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> *const i32;

/**
 * @name packed_int64_array_operator_index
 * @since 4.1
 *
 * Gets a pointer to a 64-bit integer in a PackedInt64Array.
 *
 * @param p_self A pointer to a PackedInt64Array object.
 * @param p_index The index of the integer to get.
 *
 * @return A pointer to the requested 64-bit integer.
 */
pub(super) type GDExtensionInterfacePackedInt64ArrayOperatorIndex = unsafe extern "C" fn (p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> *mut i64;

/**
 * @name packed_int64_array_operator_index_const
 * @since 4.1
 *
 * Gets a const pointer to a 64-bit integer in a PackedInt64Array.
 *
 * @param p_self A const pointer to a PackedInt64Array object.
 * @param p_index The index of the integer to get.
 *
 * @return A const pointer to the requested 64-bit integer.
 */
pub(super) type GDExtensionInterfacePackedInt64ArrayOperatorIndexConst = unsafe extern "C" fn (p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> *const i64;

/**
 * @name packed_string_array_operator_index
 * @since 4.1
 *
 * Gets a pointer to a string in a PackedStringArray.
 *
 * @param p_self A pointer to a PackedStringArray object.
 * @param p_index The index of the String to get.
 *
 * @return A pointer to the requested String.
 */
pub(super) type GDExtensionInterfacePackedStringArrayOperatorIndex = unsafe extern "C" fn (p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> GDExtensionStringPtr;

/**
 * @name packed_string_array_operator_index_const
 * @since 4.1
 *
 * Gets a const pointer to a string in a PackedStringArray.
 *
 * @param p_self A const pointer to a PackedStringArray object.
 * @param p_index The index of the String to get.
 *
 * @return A const pointer to the requested String.
 */
pub(super) type GDExtensionInterfacePackedStringArrayOperatorIndexConst = unsafe extern "C" fn (p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> GDExtensionStringPtr;

/**
 * @name packed_vector2_array_operator_index
 * @since 4.1
 *
 * Gets a pointer to a Vector2 in a PackedVector2Array.
 *
 * @param p_self A pointer to a PackedVector2Array object.
 * @param p_index The index of the Vector2 to get.
 *
 * @return A pointer to the requested Vector2.
 */
pub(super) type GDExtensionInterfacePackedVector2ArrayOperatorIndex = unsafe extern "C" fn (p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;

/**
 * @name packed_vector2_array_operator_index_const
 * @since 4.1
 *
 * Gets a const pointer to a Vector2 in a PackedVector2Array.
 *
 * @param p_self A const pointer to a PackedVector2Array object.
 * @param p_index The index of the Vector2 to get.
 *
 * @return A const pointer to the requested Vector2.
 */
pub(super) type GDExtensionInterfacePackedVector2ArrayOperatorIndexConst = unsafe extern "C" fn (p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;

/**
 * @name packed_vector3_array_operator_index
 * @since 4.1
 *
 * Gets a pointer to a Vector3 in a PackedVector3Array.
 *
 * @param p_self A pointer to a PackedVector3Array object.
 * @param p_index The index of the Vector3 to get.
 *
 * @return A pointer to the requested Vector3.
 */
pub(super) type GDExtensionInterfacePackedVector3ArrayOperatorIndex = unsafe extern "C" fn (p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;

/**
 * @name packed_vector3_array_operator_index_const
 * @since 4.1
 *
 * Gets a const pointer to a Vector3 in a PackedVector3Array.
 *
 * @param p_self A const pointer to a PackedVector3Array object.
 * @param p_index The index of the Vector3 to get.
 *
 * @return A const pointer to the requested Vector3.
 */
pub(super) type GDExtensionInterfacePackedVector3ArrayOperatorIndexConst = unsafe extern "C" fn (p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;

/**
 * @name packed_vector4_array_operator_index
 * @since 4.3
 *
 * Gets a pointer to a Vector4 in a PackedVector4Array.
 *
 * @param p_self A pointer to a PackedVector4Array object.
 * @param p_index The index of the Vector4 to get.
 *
 * @return A pointer to the requested Vector4.
 */
pub(super) type GDExtensionInterfacePackedVector4ArrayOperatorIndex = unsafe extern "C" fn (p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;

/**
 * @name packed_vector4_array_operator_index_const
 * @since 4.3
 *
 * Gets a const pointer to a Vector4 in a PackedVector4Array.
 *
 * @param p_self A const pointer to a PackedVector4Array object.
 * @param p_index The index of the Vector4 to get.
 *
 * @return A const pointer to the requested Vector4.
 */
pub(super) type GDExtensionInterfacePackedVector4ArrayOperatorIndexConst = unsafe extern "C" fn (p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;

/**
 * @name packed_color_array_operator_index
 * @since 4.1
 *
 * Gets a pointer to a color in a PackedColorArray.
 *
 * @param p_self A pointer to a PackedColorArray object.
 * @param p_index The index of the Color to get.
 *
 * @return A pointer to the requested Color.
 */
pub(super) type GDExtensionInterfacePackedColorArrayOperatorIndex = unsafe extern "C" fn (p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;

/**
 * @name packed_color_array_operator_index_const
 * @since 4.1
 *
 * Gets a const pointer to a color in a PackedColorArray.
 *
 * @param p_self A const pointer to a PackedColorArray object.
 * @param p_index The index of the Color to get.
 *
 * @return A const pointer to the requested Color.
 */
pub(super) type GDExtensionInterfacePackedColorArrayOperatorIndexConst = unsafe extern "C" fn (p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;

/**
 * @name array_operator_index
 * @since 4.1
 *
 * Gets a pointer to a Variant in an Array.
 *
 * @param p_self A pointer to an Array object.
 * @param p_index The index of the Variant to get.
 *
 * @return A pointer to the requested Variant.
 */
pub(super) type GDExtensionInterfaceArrayOperatorIndex = unsafe extern "C" fn (p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> GDExtensionVariantPtr;

/**
 * @name array_operator_index_const
 * @since 4.1
 *
 * Gets a const pointer to a Variant in an Array.
 *
 * @param p_self A const pointer to an Array object.
 * @param p_index The index of the Variant to get.
 *
 * @return A const pointer to the requested Variant.
 */
pub(super) type GDExtensionInterfaceArrayOperatorIndexConst = unsafe extern "C" fn (p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> GDExtensionVariantPtr;

/**
 * @name array_ref
 * @since 4.1
 *
 * Sets an Array to be a reference to another Array object.
 *
 * @param p_self A pointer to the Array object to update.
 * @param p_from A pointer to the Array object to reference.
 */
pub(super) type GDExtensionInterfaceArrayRef = unsafe extern "C" fn (p_self: GDExtensionTypePtr, p_from: GDExtensionConstTypePtr);

/**
 * @name array_set_typed
 * @since 4.1
 *
 * Makes an Array into a typed Array.
 *
 * @param p_self A pointer to the Array.
 * @param p_type The type of Variant the Array will store.
 * @param p_class_name A pointer to a StringName with the name of the object (if p_type is GDEXTENSION_VARIANT_TYPE_OBJECT).
 * @param p_script A pointer to a Script object (if p_type is GDEXTENSION_VARIANT_TYPE_OBJECT and the base class is extended by a script).
 */
pub(super) type GDExtensionInterfaceArraySetTyped = unsafe extern "C" fn (p_self: GDExtensionTypePtr, p_type: GDExtensionVariantType, p_class_name: GDExtensionConstStringNamePtr, p_script: GDExtensionConstVariantPtr);

/* INTERFACE: Dictionary */

/**
 * @name dictionary_operator_index
 * @since 4.1
 *
 * Gets a pointer to a Variant in a Dictionary with the given key.
 *
 * @param p_self A pointer to a Dictionary object.
 * @param p_key A pointer to a Variant representing the key.
 *
 * @return A pointer to a Variant representing the value at the given key.
 */
pub(super) type GDExtensionInterfaceDictionaryOperatorIndex = unsafe extern "C" fn (p_self: GDExtensionTypePtr, p_key: GDExtensionConstVariantPtr) -> GDExtensionVariantPtr;

/**
 * @name dictionary_operator_index_const
 * @since 4.1
 *
 * Gets a const pointer to a Variant in a Dictionary with the given key.
 *
 * @param p_self A const pointer to a Dictionary object.
 * @param p_key A pointer to a Variant representing the key.
 *
 * @return A const pointer to a Variant representing the value at the given key.
 */
pub(super) type GDExtensionInterfaceDictionaryOperatorIndexConst = unsafe extern "C" fn (p_self: GDExtensionConstTypePtr, p_key: GDExtensionConstVariantPtr) -> GDExtensionVariantPtr;

/* INTERFACE: Object */

/**
 * @name object_method_bind_call
 * @since 4.1
 *
 * Calls a method on an Object.
 *
 * @param p_method_bind A pointer to the MethodBind representing the method on the Object's class.
 * @param p_instance A pointer to the Object.
 * @param p_args A pointer to a C array of Variants representing the arguments.
 * @param p_arg_count The number of arguments.
 * @param r_ret A pointer to Variant which will receive the return value.
 * @param r_error A pointer to a GDExtensionCallError struct that will receive error information.
 */
pub(super) type GDExtensionInterfaceObjectMethodBindCall = unsafe extern "C" fn (p_method_bind: GDExtensionMethodBindPtr, p_instance: GDExtensionObjectPtr, p_args: *const GDExtensionConstVariantPtr, p_arg_count: GDExtensionInt, r_ret: GDExtensionUninitializedVariantPtr, r_error: *mut GDExtensionCallError);

/**
 * @name object_method_bind_ptrcall
 * @since 4.1
 *
 * Calls a method on an Object (using a "ptrcall").
 *
 * @param p_method_bind A pointer to the MethodBind representing the method on the Object's class.
 * @param p_instance A pointer to the Object.
 * @param p_args A pointer to a C array representing the arguments.
 * @param r_ret A pointer to the Object that will receive the return value.
 */
pub(super) type GDExtensionInterfaceObjectMethodBindPtrcall = unsafe extern "C" fn (p_method_bind: GDExtensionMethodBindPtr, p_instance: GDExtensionObjectPtr, p_args: *const GDExtensionConstTypePtr, r_ret: GDExtensionTypePtr);

/**
 * @name object_destroy
 * @since 4.1
 *
 * Destroys an Object.
 *
 * @param p_o A pointer to the Object.
 */
pub(super) type GDExtensionInterfaceObjectDestroy = unsafe extern "C" fn (p_o: GDExtensionObjectPtr);

/**
 * @name global_get_singleton
 * @since 4.1
 *
 * Gets a global singleton by name.
 *
 * @param p_name A pointer to a StringName with the singleton name.
 *
 * @return A pointer to the singleton Object.
 */
pub(super) type GDExtensionInterfaceGlobalGetSingleton = unsafe extern "C" fn (p_name: GDExtensionConstStringNamePtr) -> GDExtensionObjectPtr;

/**
 * @name object_get_instance_binding
 * @since 4.1
 *
 * Gets a pointer representing an Object's instance binding.
 *
 * @param p_o A pointer to the Object.
 * @param p_library A token the library received by the GDExtension's entry point function.
 * @param p_callbacks A pointer to a GDExtensionInstanceBindingCallbacks struct.
 *
 * @return
 */
pub(super) type GDExtensionInterfaceObjectGetInstanceBinding = unsafe extern "C" fn (p_o: GDExtensionObjectPtr, p_token: void_p, p_callbacks: *const GDExtensionInstanceBindingCallbacks) -> void_p;

/**
 * @name object_set_instance_binding
 * @since 4.1
 *
 * Sets an Object's instance binding.
 *
 * @param p_o A pointer to the Object.
 * @param p_library A token the library received by the GDExtension's entry point function.
 * @param p_binding A pointer to the instance binding.
 * @param p_callbacks A pointer to a GDExtensionInstanceBindingCallbacks struct.
 */
pub(super) type GDExtensionInterfaceObjectSetInstanceBinding = unsafe extern "C" fn (p_o: GDExtensionObjectPtr, p_token: void_p, p_binding: void_p, p_callbacks: *const GDExtensionInstanceBindingCallbacks);

/**
 * @name object_free_instance_binding
 * @since 4.2
 *
 * Free an Object's instance binding.
 *
 * @param p_o A pointer to the Object.
 * @param p_library A token the library received by the GDExtension's entry point function.
 */
pub(super) type GDExtensionInterfaceObjectFreeInstanceBinding = unsafe extern "C" fn (p_o: GDExtensionObjectPtr, p_token: void_p);

/**
 * @name object_set_instance
 * @since 4.1
 *
 * Sets an extension class instance on a Object.
 *
 * @param p_o A pointer to the Object.
 * @param p_classname A pointer to a StringName with the registered extension class's name.
 * @param p_instance A pointer to the extension class instance.
 */
pub(super) type GDExtensionInterfaceObjectSetInstance = unsafe extern "C" fn (p_o: GDExtensionObjectPtr, p_classname: GDExtensionConstStringNamePtr, p_instance: GDExtensionClassInstancePtr); /* p_classname should be a registered extension class and should extend the p_o object's class. */

/**
 * @name object_get_class_name
 * @since 4.1
 *
 * Gets the class name of an Object.
 *
 * If the GDExtension wraps the Godot object in an abstraction specific to its class, this is the
 * function that should be used to determine which wrapper to use.
 *
 * @param p_object A pointer to the Object.
 * @param p_library A pointer the library received by the GDExtension's entry point function.
 * @param r_class_name A pointer to a String to receive the class name.
 *
 * @return true if successful in getting the class name; otherwise false.
 */
pub(super) type GDExtensionInterfaceObjectGetClassName = unsafe extern "C" fn (p_object: GDExtensionConstObjectPtr, p_library: GDExtensionClassLibraryPtr, r_class_name: GDExtensionUninitializedStringNamePtr) -> GDExtensionBool;

/**
 * @name object_cast_to
 * @since 4.1
 *
 * Casts an Object to a different type.
 *
 * @param p_object A pointer to the Object.
 * @param p_class_tag A pointer uniquely identifying a built-in class in the ClassDB.
 *
 * @return Returns a pointer to the Object, or NULL if it can't be cast to the requested type.
 */
pub(super) type GDExtensionInterfaceObjectCastTo = unsafe extern "C" fn (p_object: GDExtensionConstObjectPtr, p_class_tag: void_p) -> GDExtensionObjectPtr;

/**
 * @name object_get_instance_from_id
 * @since 4.1
 *
 * Gets an Object by its instance ID.
 *
 * @param p_instance_id The instance ID.
 *
 * @return A pointer to the Object.
 */
pub(super) type GDExtensionInterfaceObjectGetInstanceFromId = unsafe extern "C" fn (p_instance_id: GDObjectInstanceID) -> GDExtensionObjectPtr;

/**
 * @name object_get_instance_id
 * @since 4.1
 *
 * Gets the instance ID from an Object.
 *
 * @param p_object A pointer to the Object.
 *
 * @return The instance ID.
 */
pub(super) type GDExtensionInterfaceObjectGetInstanceId = unsafe extern "C" fn (p_object: GDExtensionConstObjectPtr) -> GDObjectInstanceID;

/**
 * @name object_has_script_method
 * @since 4.3
 *
 * Checks if this object has a script with the given method.
 *
 * @param p_object A pointer to the Object.
 * @param p_method A pointer to a StringName identifying the method.
 *
 * @returns true if the object has a script and that script has a method with the given name. Returns false if the object has no script.
 */
pub(super) type GDExtensionInterfaceObjectHasScriptMethod = unsafe extern "C" fn (p_object: GDExtensionConstObjectPtr, p_method: GDExtensionConstStringNamePtr) -> GDExtensionBool;

/**
 * @name object_call_script_method
 * @since 4.3
 *
 * Call the given script method on this object.
 *
 * @param p_object A pointer to the Object.
 * @param p_method A pointer to a StringName identifying the method.
 * @param p_args A pointer to a C array of Variant.
 * @param p_argument_count The number of arguments.
 * @param r_return A pointer a Variant which will be assigned the return value.
 * @param r_error A pointer the structure which will hold error information.
 */
pub(super) type GDExtensionInterfaceObjectCallScriptMethod = unsafe extern "C" fn (p_object: GDExtensionObjectPtr, p_method: GDExtensionConstStringNamePtr, p_args: *const GDExtensionConstVariantPtr, p_argument_count: GDExtensionInt, r_return: GDExtensionUninitializedVariantPtr, r_error: *mut GDExtensionCallError);

/* INTERFACE: Reference */

/**
 * @name ref_get_object
 * @since 4.1
 *
 * Gets the Object from a reference.
 *
 * @param p_ref A pointer to the reference.
 *
 * @return A pointer to the Object from the reference or NULL.
 */
pub(super) type GDExtensionInterfaceRefGetObject = unsafe extern "C" fn (p_ref: GDExtensionConstRefPtr) -> GDExtensionObjectPtr;

/**
 * @name ref_set_object
 * @since 4.1
 *
 * Sets the Object referred to by a reference.
 *
 * @param p_ref A pointer to the reference.
 * @param p_object A pointer to the Object to refer to.
 */
pub(super) type GDExtensionInterfaceRefSetObject = unsafe extern "C" fn (p_ref: GDExtensionRefPtr, p_object: GDExtensionObjectPtr);

/* INTERFACE: Script Instance */

/**
 * @name script_instance_create3
 * @since 4.3
 *
 * Creates a script instance that contains the given info and instance data.
 *
 * @param p_info A pointer to a GDExtensionScriptInstanceInfo3 struct.
 * @param p_instance_data A pointer to a data representing the script instance in the GDExtension. This will be passed to all the function pointers on p_info.
 *
 * @return A pointer to a ScriptInstanceExtension object.
 */
pub(super) type GDExtensionInterfaceScriptInstanceCreate3 = unsafe extern "C" fn (p_info: *const GDExtensionScriptInstanceInfo3, p_instance_data: GDExtensionScriptInstanceDataPtr) -> GDExtensionScriptInstancePtr;

/**
 * @name placeholder_script_instance_create
 * @since 4.2
 *
 * Creates a placeholder script instance for a given script and instance.
 *
 * This interface is optional as a custom placeholder could also be created with script_instance_create().
 *
 * @param p_language A pointer to a ScriptLanguage.
 * @param p_script A pointer to a Script.
 * @param p_owner A pointer to an Object.
 *
 * @return A pointer to a PlaceHolderScriptInstance object.
 */
pub(super) type GDExtensionInterfacePlaceHolderScriptInstanceCreate = unsafe extern "C" fn (p_language: GDExtensionObjectPtr, p_script: GDExtensionObjectPtr, p_owner: GDExtensionObjectPtr) -> GDExtensionScriptInstancePtr;

/**
 * @name placeholder_script_instance_update
 * @since 4.2
 *
 * Updates a placeholder script instance with the given properties and values.
 *
 * The passed in placeholder must be an instance of PlaceHolderScriptInstance
 * such as the one returned by placeholder_script_instance_create().
 *
 * @param p_placeholder A pointer to a PlaceHolderScriptInstance.
 * @param p_properties A pointer to an Array of Dictionary representing PropertyInfo.
 * @param p_values A pointer to a Dictionary mapping StringName to Variant values.
 */
pub(super) type GDExtensionInterfacePlaceHolderScriptInstanceUpdate = unsafe extern "C" fn (p_placeholder: GDExtensionScriptInstancePtr, p_properties: GDExtensionConstTypePtr, p_values: GDExtensionConstTypePtr);

/**
 * @name object_get_script_instance
 * @since 4.2
 *
 * Get the script instance data attached to this object.
 *
 * @param p_object A pointer to the Object.
 * @param p_language A pointer to the language expected for this script instance.
 *
 * @return A GDExtensionScriptInstanceDataPtr that was attached to this object as part of script_instance_create.
 */
pub(super) type GDExtensionInterfaceObjectGetScriptInstance = unsafe extern "C" fn (p_object: GDExtensionConstObjectPtr, p_language: GDExtensionObjectPtr) -> GDExtensionScriptInstanceDataPtr;

/* INTERFACE: Callable */

/**
 * @name callable_custom_create2
 * @since 4.3
 *
 * Creates a custom Callable object from a function pointer.
 *
 * Provided struct can be safely freed once the function returns.
 *
 * @param r_callable A pointer that will receive the new Callable.
 * @param p_callable_custom_info The info required to construct a Callable.
 */
pub(super) type GDExtensionInterfaceCallableCustomCreate2 = unsafe extern "C" fn (r_callable: GDExtensionUninitializedTypePtr, p_callable_custom_info: *mut GDExtensionCallableCustomInfo2);

/**
 * @name callable_custom_get_userdata
 * @since 4.2
 *
 * Retrieves the userdata pointer from a custom Callable.
 *
 * If the Callable is not a custom Callable or the token does not match the one provided to callable_custom_create() via GDExtensionCallableCustomInfo then NULL will be returned.
 *
 * @param p_callable A pointer to a Callable.
 * @param p_token A pointer to an address that uniquely identifies the GDExtension.
 */
pub(super) type GDExtensionInterfaceCallableCustomGetUserData = unsafe extern "C" fn (p_callable: GDExtensionConstTypePtr, p_token: void_p) -> void_p;

/* INTERFACE: ClassDB */

/**
 * @name classdb_construct_object
 * @since 4.1
 *
 * Constructs an Object of the requested class.
 *
 * The passed class must be a built-in godot class, or an already-registered extension class. In both cases, object_set_instance() should be called to fully initialize the object.
 *
 * @param p_classname A pointer to a StringName with the class name.
 *
 * @return A pointer to the newly created Object.
 */
pub(super) type GDExtensionInterfaceClassdbConstructObject = unsafe extern "C" fn (p_classname: GDExtensionConstStringNamePtr) -> GDExtensionObjectPtr;

/**
 * @name classdb_get_method_bind
 * @since 4.1
 *
 * Gets a pointer to the MethodBind in ClassDB for the given class, method and hash.
 *
 * @param p_classname A pointer to a StringName with the class name.
 * @param p_methodname A pointer to a StringName with the method name.
 * @param p_hash A hash representing the function signature.
 *
 * @return A pointer to the MethodBind from ClassDB.
 */
pub(super) type GDExtensionInterfaceClassdbGetMethodBind = unsafe extern "C" fn (p_classname: GDExtensionConstStringNamePtr, p_methodname: GDExtensionConstStringNamePtr, p_hash: GDExtensionInt) -> GDExtensionMethodBindPtr;

/**
 * @name classdb_get_class_tag
 * @since 4.1
 *
 * Gets a pointer uniquely identifying the given built-in class in the ClassDB.
 *
 * @param p_classname A pointer to a StringName with the class name.
 *
 * @return A pointer uniquely identifying the built-in class in the ClassDB.
 */
pub(super) type GDExtensionInterfaceClassdbGetClassTag = unsafe extern "C" fn (p_classname: GDExtensionConstStringNamePtr) -> void_p;

/* INTERFACE: ClassDB Extension */

/**
 * @name classdb_register_extension_class3
 * @since 4.3
 *
 * Registers an extension class in the ClassDB.
 *
 * Provided struct can be safely freed once the function returns.
 *
 * @param p_library A pointer the library received by the GDExtension's entry point function.
 * @param p_class_name A pointer to a StringName with the class name.
 * @param p_parent_class_name A pointer to a StringName with the parent class name.
 * @param p_extension_funcs A pointer to a GDExtensionClassCreationInfo2 struct.
 */
pub(super) type GDExtensionInterfaceClassdbRegisterExtensionClass3 = unsafe extern "C" fn (p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_parent_class_name: GDExtensionConstStringNamePtr, p_extension_funcs: *const GDExtensionClassCreationInfo3);

/**
 * @name classdb_register_extension_class_method
 * @since 4.1
 *
 * Registers a method on an extension class in the ClassDB.
 *
 * Provided struct can be safely freed once the function returns.
 *
 * @param p_library A pointer the library received by the GDExtension's entry point function.
 * @param p_class_name A pointer to a StringName with the class name.
 * @param p_method_info A pointer to a GDExtensionClassMethodInfo struct.
 */
pub(super) type GDExtensionInterfaceClassdbRegisterExtensionClassMethod = unsafe extern "C" fn (p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_method_info: *const GDExtensionClassMethodInfo);

/**
 * @name classdb_register_extension_class_virtual_method
 * @since 4.3
 *
 * Registers a virtual method on an extension class in ClassDB, that can be implemented by scripts or other extensions.
 *
 * Provided struct can be safely freed once the function returns.
 *
 * @param p_library A pointer the library received by the GDExtension's entry point function.
 * @param p_class_name A pointer to a StringName with the class name.
 * @param p_method_info A pointer to a GDExtensionClassMethodInfo struct.
 */
pub(super) type GDExtensionInterfaceClassdbRegisterExtensionClassVirtualMethod = unsafe extern "C" fn (p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_method_info: *const GDExtensionClassVirtualMethodInfo);

/**
 * @name classdb_register_extension_class_integer_constant
 * @since 4.1
 *
 * Registers an integer constant on an extension class in the ClassDB.
 *
 * @param p_library A pointer the library received by the GDExtension's entry point function.
 * @param p_class_name A pointer to a StringName with the class name.
 * @param p_enum_name A pointer to a StringName with the enum name.
 * @param p_constant_name A pointer to a StringName with the constant name.
 * @param p_constant_value The constant value.
 * @param p_is_bitfield Whether or not this is a bit field.
 */
pub(super) type GDExtensionInterfaceClassdbRegisterExtensionClassIntegerConstant = unsafe extern "C" fn (p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_enum_name: GDExtensionConstStringNamePtr, p_constant_name: GDExtensionConstStringNamePtr, p_constant_value: GDExtensionInt, p_is_bitfield: GDExtensionBool);

/**
 * @name classdb_register_extension_class_property
 * @since 4.1
 *
 * Registers a property on an extension class in the ClassDB.
 *
 * Provided struct can be safely freed once the function returns.
 *
 * @param p_library A pointer the library received by the GDExtension's entry point function.
 * @param p_class_name A pointer to a StringName with the class name.
 * @param p_info A pointer to a GDExtensionPropertyInfo struct.
 * @param p_setter A pointer to a StringName with the name of the setter method.
 * @param p_getter A pointer to a StringName with the name of the getter method.
 */
pub(super) type GDExtensionInterfaceClassdbRegisterExtensionClassProperty = unsafe extern "C" fn (p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_info: *const GDExtensionPropertyInfo, p_setter: GDExtensionConstStringNamePtr, p_getter: GDExtensionConstStringNamePtr);

/**
 * @name classdb_register_extension_class_property_indexed
 * @since 4.2
 *
 * Registers an indexed property on an extension class in the ClassDB.
 *
 * Provided struct can be safely freed once the function returns.
 *
 * @param p_library A pointer the library received by the GDExtension's entry point function.
 * @param p_class_name A pointer to a StringName with the class name.
 * @param p_info A pointer to a GDExtensionPropertyInfo struct.
 * @param p_setter A pointer to a StringName with the name of the setter method.
 * @param p_getter A pointer to a StringName with the name of the getter method.
 * @param p_index The index to pass as the first argument to the getter and setter methods.
 */
pub(super) type GDExtensionInterfaceClassdbRegisterExtensionClassPropertyIndexed = unsafe extern "C" fn (p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_info: *const GDExtensionPropertyInfo, p_setter: GDExtensionConstStringNamePtr, p_getter: GDExtensionConstStringNamePtr, p_index: GDExtensionInt);

/**
 * @name classdb_register_extension_class_property_group
 * @since 4.1
 *
 * Registers a property group on an extension class in the ClassDB.
 *
 * @param p_library A pointer the library received by the GDExtension's entry point function.
 * @param p_class_name A pointer to a StringName with the class name.
 * @param p_group_name A pointer to a String with the group name.
 * @param p_prefix A pointer to a String with the prefix used by properties in this group.
 */
pub(super) type GDExtensionInterfaceClassdbRegisterExtensionClassPropertyGroup = unsafe extern "C" fn (p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_group_name: GDExtensionConstStringPtr, p_prefix: GDExtensionConstStringPtr);

/**
 * @name classdb_register_extension_class_property_subgroup
 * @since 4.1
 *
 * Registers a property subgroup on an extension class in the ClassDB.
 *
 * @param p_library A pointer the library received by the GDExtension's entry point function.
 * @param p_class_name A pointer to a StringName with the class name.
 * @param p_subgroup_name A pointer to a String with the subgroup name.
 * @param p_prefix A pointer to a String with the prefix used by properties in this subgroup.
 */
pub(super) type GDExtensionInterfaceClassdbRegisterExtensionClassPropertySubgroup = unsafe extern "C" fn (p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_subgroup_name: GDExtensionConstStringPtr, p_prefix: GDExtensionConstStringPtr);

/**
 * @name classdb_register_extension_class_signal
 * @since 4.1
 *
 * Registers a signal on an extension class in the ClassDB.
 *
 * Provided structs can be safely freed once the function returns.
 *
 * @param p_library A pointer the library received by the GDExtension's entry point function.
 * @param p_class_name A pointer to a StringName with the class name.
 * @param p_signal_name A pointer to a StringName with the signal name.
 * @param p_argument_info A pointer to a GDExtensionPropertyInfo struct.
 * @param p_argument_count The number of arguments the signal receives.
 */
pub(super) type GDExtensionInterfaceClassdbRegisterExtensionClassSignal = unsafe extern "C" fn (p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_signal_name: GDExtensionConstStringNamePtr, p_argument_info: *const GDExtensionPropertyInfo, p_argument_count: GDExtensionInt);

/**
 * @name classdb_unregister_extension_class
 * @since 4.1
 *
 * Unregisters an extension class in the ClassDB.
 *
 * @param p_library A pointer the library received by the GDExtension's entry point function.
 * @param p_class_name A pointer to a StringName with the class name.
 */
pub(super) type GDExtensionInterfaceClassdbUnregisterExtensionClass = unsafe extern "C" fn (p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr); /* Unregistering a parent class before a class that inherits it will result in failure. Inheritors must be unregistered first. */

/**
 * @name get_library_path
 * @since 4.1
 *
 * Gets the path to the current GDExtension library.
 *
 * @param p_library A pointer the library received by the GDExtension's entry point function.
 * @param r_path A pointer to a String which will receive the path.
 */
pub(super) type GDExtensionInterfaceGetLibraryPath = unsafe extern "C" fn (p_library: GDExtensionClassLibraryPtr, r_path: GDExtensionUninitializedStringPtr);

/**
 * @name editor_add_plugin
 * @since 4.1
 *
 * Adds an editor plugin.
 *
 * It's safe to call during initialization.
 *
 * @param p_class_name A pointer to a StringName with the name of a class (descending from EditorPlugin) which is already registered with ClassDB.
 */
pub(super) type GDExtensionInterfaceEditorAddPlugin = unsafe extern "C" fn (p_class_name: GDExtensionConstStringNamePtr);

/**
 * @name editor_remove_plugin
 * @since 4.1
 *
 * Removes an editor plugin.
 *
 * @param p_class_name A pointer to a StringName with the name of a class that was previously added as an editor plugin.
 */
pub(super) type GDExtensionInterfaceEditorRemovePlugin = unsafe extern "C" fn (p_class_name: GDExtensionConstStringNamePtr);

/**
 * @name editor_help_load_xml_from_utf8_chars
 * @since 4.3
 *
 * Loads new XML-formatted documentation data in the editor.
 *
 * The provided pointer can be immediately freed once the function returns.
 *
 * @param p_data A pointer to a UTF-8 encoded C string (null terminated).
 */
pub(super) type GDExtensionsInterfaceEditorHelpLoadXmlFromUtf8Chars = unsafe extern "C" fn (p_data: const_char_p);

/**
 * @name editor_help_load_xml_from_utf8_chars_and_len
 * @since 4.3
 *
 * Loads new XML-formatted documentation data in the editor.
 *
 * The provided pointer can be immediately freed once the function returns.
 *
 * @param p_data A pointer to a UTF-8 encoded C string.
 * @param p_size The number of bytes (not code units).
 */
pub(super) type GDExtensionsInterfaceEditorHelpLoadXmlFromUtf8CharsAndLen = unsafe extern "C" fn (p_data: const_char_p, p_size: GDExtensionInt);

macro_rules! get_gdextension_func {
	($p_get_proc_address: ident, $name: ident) => {
		unsafe {
			let x: crate::gdextension::bindings::GDExtensionInterfaceFunctionPtr = $p_get_proc_address((stringify!($name)).as_ptr().cast());
			let y: $name = *(std::ptr::addr_of!(x).cast());
			y
		}
	};
}
pub(super) use get_gdextension_func;