use super::super::interface::GDExtension;

pub fn _gdext_print_error(s: &mut String, file: &str, line: u32, notify_ed: u8) {
    unsafe {
        GDExtension.gdextension_interface_print_error.unwrap()(s.as_mut_str().as_ptr().cast(),("<unknown>").as_ptr().cast(),file.as_ptr().cast(),line as i32,notify_ed);
    }
}
pub fn _gdext_print_error_msg(s: &mut String, msg: &mut String, file: &str, line: u32, notify_ed: u8) {
    unsafe {
        GDExtension.gdextension_interface_print_error_with_message.unwrap()(s.as_mut_str().as_ptr().cast(),msg.as_mut_str().as_ptr().cast(), ("<unknown>").as_ptr().cast(),file.as_ptr().cast(),line as i32,notify_ed);
    }
}

/**
 * Error macros.
 * WARNING: These macros work in the opposite way to assert().
 *
 * Unlike exceptions and asserts, these macros try to maintain consistency and stability.
 * In most cases, bugs and/or invalid data are not fatal. They should never allow a perfectly
 * running application to fail or crash.
 * Always try to return processable data, so the engine can keep running well.
 * Use the _MSG versions to print a meaningful message to help with debugging.
 *
 * The `((void)0)` no-op statement is used as a trick to force us to put a semicolon after
 * those macros, making them look like proper statements.
 * The if wrappers are used to ensure that the macro replacement does not trigger unexpected
 * issues when expanded e.g. after an `if (cond) ERR_FAIL();` without braces.
 */
 
// Index out of bounds error macros.
// These macros should be used instead of `ERR_FAIL_COND` for bounds checking.

// Integer index out of bounds error macros.

/**
 * Ensures an integer index `m_index` is less than `m_size` and greater than or equal to 0.
 * If not, the current function returns.
 */
#[macro_export]
macro_rules! err_fail_index {
    ($m_index: expr, $m_size: expr) => {
        if ($m_index < 0 || $m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 0);
            return;
        }
    };
    ($m_index: expr, $m_size: expr, $m_msg: expr) => {
        if ($m_index < 0 || $m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 0);
            return;
        }
    };
}
/**
 * Ensures an integer index `m_index` is less than `m_size` and greater than or equal to 0.
 * If not, prints `m_msg` and the current function returns `m_retval`.
 */
#[macro_export]
macro_rules! err_fail_index_v {
    ($m_index: expr, $m_size: expr, $m_retval: expr) => {
        if ($m_index < 0 || $m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 0);
            return $m_retval;
        }
    };
    ($m_index: expr, $m_size: expr, $m_msg: expr, $m_retval: expr) => {
        if ($m_index < 0 || $m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 0);
            return $m_retval;
        }
    };
}
/**
 * Same as `err_fail_index` but also notifies the editor.
 */
#[macro_export]
macro_rules! err_fail_index_ed {
    ($m_index: expr, $m_size: expr) => {
        if ($m_index < 0 || $m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 1);
            return;
        }
    };
    ($m_index: expr, $m_size: expr, $m_msg: expr) => {
        if ($m_index < 0 || $m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 1);
            return;
        }
    };
}
/**
 * Same as `err_fail_index_v` but also notifies the editor.
 */
#[macro_export]
macro_rules! err_fail_index_v_ed {
    ($m_index: expr, $m_size: expr, $m_retval: expr) => {
        if ($m_index < 0 || $m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 1);
            return $m_retval;
        }
    };
    ($m_index: expr, $m_size: expr, $m_msg: expr, $m_retval: expr) => {
        if ($m_index < 0 || $m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 1);
            return $m_retval;
        }
    };
}

/**
 * Try using `err_fail_index` or `err_fail_index_v`.
 * Only use this macro if there is no sensible fallback i.e. the error is unrecoverable.
 *
 * Ensures an integer index `m_index` is less than `m_size` and greater than or equal to 0.
 * If not, the application crashes.
 */
#[macro_export]
macro_rules! crash_bad_index {
    ($m_index: expr, $m_size: expr) => {
        if ($m_index < 0 || $m_index >= $m_size) {
            let s = format!("FATAL: Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 1);
            panic!(s);
        }
    };
    ($m_index: expr, $m_size: expr, $m_msg: expr) => {
        if ($m_index < 0 || $m_index >= $m_size) {
            let s = format!("FATAL: Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 1);
            panic!(s);
        }
    };
}

// Unsigned integer index out of bounds error macros.


/**
 * Ensures an unsigned integer index `m_index` is less than `m_size`.
 * If not, prints `m_msg` and the current function returns.
 */
#[macro_export]
macro_rules! err_fail_unsigned_index {
    ($m_index: expr, $m_size: expr) => {
        if ($m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 0);
            return;
        }
    };
    ($m_index: expr, $m_size: expr, $m_msg: expr) => {
        if ($m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 0);
            return;
        }
    };
}
/**
 * Ensures an unsigned integer index `m_index` is less than `m_size`.
 * If not, prints `m_msg` and the current function returns `m_retval`.
 */
#[macro_export]
macro_rules! err_fail_unsigned_index_v {
    ($m_index: expr, $m_size: expr, $m_retval: expr) => {
        if ($m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 0);
            return $m_retval;
        }
    };
    ($m_index: expr, $m_size: expr, $m_msg: expr, $m_retval: expr) => {
        if ($m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 0);
            return $m_retval;
        }
    };
}
/**
 * Same as `err_fail_unsigned_index` but also notifies the editor.
 */
#[macro_export]
macro_rules! err_fail_unsigned_index_ed {
    ($m_index: expr, $m_size: expr) => {
        if ($m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 1);
            return;
        }
    };
    ($m_index: expr, $m_size: expr, $m_msg: expr) => {
        if ($m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 1);
            return;
        }
    };
}
/**
 * Same as `err_fail_unsigned_index_v` but also notifies the editor.
 */
#[macro_export]
macro_rules! err_fail_unsigned_index_v_ed {
    ($m_index: expr, $m_size: expr, $m_retval: expr) => {
        if ($m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 1);
            return $m_retval;
        }
    };
    ($m_index: expr, $m_size: expr, $m_msg: expr, $m_retval: expr) => {
        if ($m_index >= $m_size) {
            let m_s = format!("Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 1);
            return $m_retval;
        }
    };
}

/**
 * Try using `err_fail_unsigned_index` or `err_fail_unsigned_index_v`.
 * Only use this macro if there is no sensible fallback i.e. the error is unrecoverable.
 *
 * Ensures an unsigned integer index `m_index` is less than `m_size`.
 * If not, prints `m_msg` and the application crashes.
 */
#[macro_export]
macro_rules! crash_bad_unsigned_index {
    ($m_index: expr, $m_size: expr) => {
        if ($m_index >= $m_size) {
            let s = format!("FATAL: Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 1);
            panic!(s);
        }
    };
    ($m_index: expr, $m_size: expr, $m_msg: expr) => {
        if ($m_index >= $m_size) {
            let s = format!("FATAL: Index {} = {} is out of bounds ({} = {}).", stringify!($m_index), $m_index, stringify!($m_size), $m_size);
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 1);
            panic!(s);
        }
    };
}

/**
 * Ensures `m_cond` is false.
 * If `m_cond` is true, prints `m_msg` and the current function returns.
 */
#[macro_export]
macro_rules! err_fail_cond {
    ($m_cond: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true.", stringify!($m_cond));
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 0);
            return;
        }
    };
    ($m_cond: expr, $m_msg: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true.", stringify!($m_cond));
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 0);
            return;
        }
    };
}
/**
 * Ensures `m_cond` is false.
 * If `m_cond` is true, the current function returns `m_retval`.
  */
#[macro_export]
macro_rules! err_fail_cond_v {
    ($m_cond: expr, $m_retval: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true. Returning: {}", stringify!($m_cond), stringify!($m_retval));
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 0);
            return $m_retval;
        }
    };
    ($m_cond: expr, $m_msg: expr, $m_retval: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true. Returning: {}", stringify!($m_cond), stringify!($m_retval));
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 0);
            return $m_retval;
        }
    };
}
/**
 * Same as `err_fail_cond` but also notifies the editor.
 */
#[macro_export]
macro_rules! err_fail_cond_ed {
    ($m_cond: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true.", stringify!($m_cond));
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 1);
            return;
        }
    };
    ($m_cond: expr, $m_msg: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true.", stringify!($m_cond));
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 1);
            return;
        }
    };
}
/**
 * Same as `err_fail_cond_v` but also notifies the editor.
 */
#[macro_export]
macro_rules! err_fail_cond_v_ed {
    ($m_cond: expr, $m_retval: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true. Returning: {}", stringify!($m_cond), stringify!($m_retval));
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 1);
            return $m_retval;
        }
    };
    ($m_cond: expr, $m_msg: expr, $m_retval: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true. Returning: {}", stringify!($m_cond), stringify!($m_retval));
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 1);
            return $m_retval;
        }
    };
}

/**
 * Ensures `m_cond` is false.
 * If not, prints `m_msg` and continues.
 */
#[macro_export]
macro_rules! err_continue {
    ($m_cond: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true. Continuing.", stringify!($m_cond));
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 0);
            continue;
        }
    };
    ($m_cond: expr, $m_msg: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true. Continuing.", stringify!($m_cond));
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 0);
            continue;
        }
    };
}

/**
 * Same as `err_continue` but also notifies the editor.
 */
#[macro_export]
macro_rules! err_continue_ed {
    ($m_cond: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true. Continuing.", stringify!($m_cond));
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 1);
            continue;
        }
    };
    ($m_cond: expr, $m_msg: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true. Continuing.", stringify!($m_cond));
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 1);
            continue;
        }
    };
}

/**
 * Ensures `m_cond` is false.
 * If not, prints `m_msg` and breaks.
 */
#[macro_export]
macro_rules! err_break {
    ($m_cond: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true. Breaking.", stringify!($m_cond));
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 0);
            break;
        }
    };
    ($m_cond: expr, $m_msg: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true. Breaking.", stringify!($m_cond));
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 0);
            break;
        }
    };
}

/**
 * Same as `err_break` but also notifies the editor.
 */
#[macro_export]
macro_rules! err_break_ed {
    ($m_cond: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true. Breaking.", stringify!($m_cond));
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 1);
            break;
        }
    };
    ($m_cond: expr, $m_msg: expr) => {
        if ($m_cond) {
            let m_s = format!("Condition {} is true. Breaking.", stringify!($m_cond));
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 1);
            break;
        }
    };
}

/**
 * Try using `err_fail_cond` or `err_fail_cond_v`.
 * Only use this macro if there is no sensible fallback i.e. the error is unrecoverable.
 *
 * Ensures `m_cond` is false.
 * If `m_cond` is true, prints `m_msg` and the application crashes.
 */
#[macro_export]
macro_rules! crash_cond {
    ($m_cond: expr) => {
        if ($m_cond) {
            let m_s = format!("FATAL: Condition {} is true.", stringify!($m_cond));
            crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 0);
            panic!(m_s);
        }
    };
    ($m_cond: expr, $m_msg: expr) => {
        if ($m_cond) {
            let m_s = format!("FATAL: Condition {} is true.", stringify!($m_cond));
            crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 0);
            panic!(m_s);
        }
    };
}

#[macro_export]
macro_rules! err_fail {
    () => {
        let m_s = format!("Method/function failed.");
        crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 0);
        return;
    };
    ($m_msg: expr) => {
        let m_s = format!("Method/function failed.");
        crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 0);
        return;
    };
}

#[macro_export]
macro_rules! err_fail_ed {
    () => {
        let m_s = format!("Method/function failed.");
        crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 1);
        return;
    };
    ($m_msg: expr) => {
        let m_s = format!("Method/function failed.");
        crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 1);
        return;
    };
}

#[macro_export]
macro_rules! err_fail_v {
    ($m_retval: expr, ) => {
        let m_s = format!("Method/function failed.");
        crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 0);
        return $m_retval;
    };
    ($m_retval: expr, $m_msg: expr) => {
        let m_s = format!("Method/function failed.");
        crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 0);
        return $m_retval;
    };
}

#[macro_export]
macro_rules! err_fail_v_ed {
    ($m_retval: expr) => {
        let m_s = format!("Method/function failed.");
        crate::gdextension::core::error_macros::_gdext_print_error(m_s, file!(), line!(), 1);
        return $m_retval;
    };
    ($m_retval: expr, $m_msg: expr) => {
        let m_s = format!("Method/function failed.");
        crate::gdextension::core::error_macros::_gdext_print_error_msg(m_s, $m_msg, file!(), line!(), 1);
        return $m_retval;
    };
}

#[macro_export]
macro_rules! err_print {
    ($m_msg: expr) => {
        crate::gdextension::core::error_macros::_gdext_print_error($m_msg, file!(), line!(), 0);
    }
}

#[macro_export]
macro_rules! err_print_ed {
    ($m_msg: expr) => {
        crate::gdextension::core::error_macros::_gdext_print_error($m_msg, file!(), line!(), 1);
    }
}

#[macro_export]
macro_rules! err_print_once {
    ($m_msg: expr) => {
        {
            static mut first_print: bool = true;
            if (first_print) {
                crate::gdextension::core::error_macros::_gdext_print_error($m_msg, file!(), line!(), 0);
                first_print = false;
            }
        }
    }
}

#[macro_export]
macro_rules! err_print_once_ed {
    ($m_msg: expr) => {
        {
            static mut first_print: bool = true;
            if (first_print) {
                crate::gdextension::core::error_macros::_gdext_print_error($m_msg, file!(), line!(), 1);
                first_print = false;
            }
        }
    }
}