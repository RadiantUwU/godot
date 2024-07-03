use std::alloc::Layout;
use std::ffi;
use std::mem::MaybeUninit;
use std::ptr::{addr_of, addr_of_mut, null_mut};

use crate::gdextension::bindings::{GDExtensionBool, GDExtensionInt, GDExtensionVariantFromTypeConstructorFunc};

/**************************************************************************/
/*  variant.rs                                                            */
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
use super::super::bindings::{GDExtensionVariantPtr,GDExtensionConstVariantPtr,GDExtensionUninitializedVariantPtr};
use std::alloc::GlobalAlloc;
use super::super::interface::GDExtension;

pub struct Variant {
    ptr: GDExtensionVariantPtr,
}
#[repr(C)]
enum VariantType {
    NIL,

    // atomic types
    BOOL,
    INT,
    FLOAT,
    STRING,

    // math types
    VECTOR2,
    VECTOR2I,
    RECT2,
    RECT2I,
    VECTOR3,
    VECTOR3I,
    TRANSFORM2D,
    VECTOR4,
    VECTOR4I,
    PLANE,
    QUATERNION,
    AABB,
    BASIS,
    TRANSFORM3D,
    PROJECTION,

    // misc types
    COLOR,
    STRING_NAME,
    NODE_PATH,
    RID,
    OBJECT,
    CALLABLE,
    SIGNAL,
    DICTIONARY,
    ARRAY,

    // typed arrays
    PACKED_BYTE_ARRAY,
    PACKED_INT32_ARRAY,
    PACKED_INT64_ARRAY,
    PACKED_FLOAT32_ARRAY,
    PACKED_FLOAT64_ARRAY,
    PACKED_STRING_ARRAY,
    PACKED_VECTOR2_ARRAY,
    PACKED_VECTOR3_ARRAY,
    PACKED_COLOR_ARRAY,
    PACKED_VECTOR4_ARRAY,

    VARIANT_MAX
}

static mut variant_from_type_constructors: [Option<GDExtensionVariantFromTypeConstructorFunc>; VariantType::VARIANT_MAX as usize] = unsafe { MaybeUninit::zeroed().assume_init() };
static mut variant_to_type_constructors: [Option<GDExtensionVariantFromTypeConstructorFunc>; VariantType::VARIANT_MAX as usize] = unsafe { MaybeUninit::zeroed().assume_init() };

impl Variant {
    pub(in super::super) unsafe fn init_bindings() {
        for i in 1..(VariantType::VARIANT_MAX as i32) {
            variant_from_type_constructors[i as usize] = Some(GDExtension.gdextension_interface_get_variant_from_type_constructor.unwrap()(std::mem::transmute(i)));
            variant_to_type_constructors[i as usize] = Some(GDExtension.gdextension_interface_get_variant_to_type_constructor.unwrap()(std::mem::transmute(i)));
        }
    }
    unsafe fn uninit() -> Self {
        Self {ptr: GDExtension.alloc(Layout::from_size_align_unchecked(GDExtension.variant_size, 0)).cast()}
    }
}

impl Default for Variant {
    fn default() -> Self {
        unsafe {
            let v: Self = Self {ptr: GDExtension.alloc(Layout::from_size_align_unchecked(GDExtension.variant_size, 0)).cast()};
            GDExtension.gdextension_interface_variant_new_nil.unwrap()(v.ptr.cast());
            v
        }
    }
}

unsafe impl Send for Variant {}

impl Clone for Variant {
    fn clone(&self) -> Self {
        unsafe {
            let v: Self = Self { ptr: GDExtension.alloc(Layout::from_size_align_unchecked(GDExtension.variant_size, 0)).cast() };
            GDExtension.gdextension_interface_variant_new_copy.unwrap()(v.ptr.cast(),addr_of!(self.ptr).cast());
            v
        }
    }
}

impl Drop for Variant {
    fn drop(&mut self) {
        unsafe {
            GDExtension.gdextension_interface_variant_destroy.unwrap()(self.ptr);
            GDExtension.dealloc(self.ptr.cast(), Layout::from_size_align_unchecked(GDExtension.variant_size, 0));
        }
    }
}

#[repr(C)]
enum VariantOperator {
    // comparison
    OP_EQUAL,
    OP_NOT_EQUAL,
    OP_LESS,
    OP_LESS_EQUAL,
    OP_GREATER,
    OP_GREATER_EQUAL,
    // mathematic
    OP_ADD,
    OP_SUBTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_NEGATE,
    OP_POSITIVE,
    OP_MODULE,
    OP_POWER,
    // bitwise
    OP_SHIFT_LEFT,
    OP_SHIFT_RIGHT,
    OP_BIT_AND,
    OP_BIT_OR,
    OP_BIT_XOR,
    OP_BIT_NEGATE,
    // logic
    OP_AND,
    OP_OR,
    OP_XOR,
    OP_NOT,
    // containment
    OP_IN,
    OP_MAX
}

impl From<bool> for Variant {
    fn from(value: bool) -> Self {
        let mut encoded: GDExtensionBool = value as u8;
        let v;
        unsafe {
            v = Variant::uninit();
            variant_from_type_constructors[VariantType::BOOL as usize].unwrap()(v.ptr, addr_of_mut!(encoded).cast());
        }
        v
    }
}
impl From<i64> for Variant {
    fn from(value: i64) -> Self {
        let mut encoded: GDExtensionInt = value as i64;
        let v;
        unsafe {
            v = Variant::uninit();
            variant_from_type_constructors[VariantType::INT as usize].unwrap()(v.ptr, addr_of_mut!(encoded).cast());
        }
        v
    }
}
impl From<u64> for Variant {
    fn from(value: u64) -> Self {
        Self::from(value as i64)
    }
}
impl From<i32> for Variant {
    fn from(value: i32) -> Self {
        Self::from(value as i64)
    }
}
impl From<u32> for Variant {
    fn from(value: u32) -> Self {
        Self::from(value as i64)
    }
}
impl From<i16> for Variant {
    fn from(value: i16) -> Self {
        Self::from(value as i64)
    }
}
impl From<u16> for Variant {
    fn from(value: u16) -> Self {
        Self::from(value as i64)
    }
}
impl From<i8> for Variant {
    fn from(value: i8) -> Self {
        Self::from(value as i64)
    }
}
impl From<u8> for Variant {
    fn from(value: u8) -> Self {
        Self::from(value as i64)
    }
}
impl From<f64> for Variant {
    fn from(value: f64) -> Self {
        let mut encoded: ffi::c_double = value as ffi::c_double;
        let v;
        unsafe {
            v = Variant::uninit();
            variant_from_type_constructors[VariantType::FLOAT as usize].unwrap()(v.ptr, addr_of_mut!(encoded).cast());
        }
        v
    }
}
impl From<f32> for Variant {
    fn from(value: f32) -> Self {
        Self::from(value as f64)
    }
}

impl Variant {
    
}