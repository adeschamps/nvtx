/// Marks an instantaneous event in the application.
///
/// # Arguments
///
/// * `message` - The message associated to this marker event.
///
/// # Examples
///
/// ```
/// use nvtx::{mark};
/// mark!("Operation A");
/// ```
#[macro_export]
macro_rules! mark {
    ($($tt:tt)*) => {
        $crate::__private::_mark(::core::format_args!($($tt)*))
    };
}

/// Starts a nested thread range.
///
/// # Arguments
///
/// * `message` - The event message associated to this range event.
///
/// # Returns
///
/// * returns the 0 based level of range being started. If an error occurs a
/// negative value is returned.
///
/// # Examples
///
/// ```
/// use nvtx::{range_pop, range_push};
/// range_push!("Hello World!");
/// range_pop!();
/// ```
#[macro_export]
macro_rules! range_push {
    ($($tt:tt)*) => {
        $crate::__private::_range_push(::core::format_args!($($tt)*))
    };
}

/// Ends a nested thread range.
///
/// # Returns
///
/// * returns the level of the range being ended. If an error occurs a negative
/// value is returned on the current thread.
///
/// # Examples
///
/// ```
/// use nvtx::{range_pop, range_push};
/// range_push!("Hello World!");
/// range_pop!();
/// ```
#[macro_export]
macro_rules! range_pop {
    () => {
        $crate::__private::_range_pop()
    };
}

/// Starts a range that can occur on a different thread than the end.
///
/// # Arguments
///
/// * `message` - The event message associated to this range event.
///
/// # Returns
///
/// * returns the `id` of the range.
///
/// # Examples
///
/// ```
/// use nvtx::{range_end, range_start};
/// let id = range_start!("Hello World!");
/// range_end!(id);
/// ```
#[macro_export]
macro_rules! range_start{
    ($($tt:tt)*) => {
        $crate::__private::_range_start(::core::format_args!($($tt)*))
    };
}

/// Ends a range that can occur on a different thread than the start.
///
/// # Arguments
///
/// * `id` - The event id associated to this range event.
///
/// # Examples
///
/// ```
/// use nvtx::{range_end, range_start};
/// let id = range_start!("Hello World!");
/// range_end!(id);
/// ```
#[macro_export]
macro_rules! range_end {
    ($tt:tt) => {
        $crate::__private::_range_end($tt)
    };
}

/// Annotate an OS thread with a name.
///
/// # Examples
///
/// ```
/// use std::thread;
/// use nvtx::{name_thread};
/// name_thread!("Thread 1");
/// let handler = thread::spawn(|| {
///    name_thread!("Thread 2");
/// });
/// handler.join().unwrap();
/// ```
#[macro_export]
macro_rules! name_thread {
    ($($tt:tt)*) => {
        $crate::__private::_name_thread(::core::format_args!($($tt)*))
    };
}
