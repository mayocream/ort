use alloc::boxed::Box;

pub(crate) struct Environment {}

impl Environment {
	pub fn new_sys() -> *mut ort_sys::OrtEnv {
		(Box::leak(Box::new(Self {})) as *mut Environment).cast()
	}

	pub unsafe fn cast_from_sys<'e>(ptr: *const ort_sys::OrtEnv) -> &'e Environment {
		unsafe { &*ptr.cast::<Environment>() }
	}

	pub unsafe fn consume_sys(ptr: *mut ort_sys::OrtEnv) -> Box<Environment> {
		unsafe { Box::from_raw(ptr.cast::<Environment>()) }
	}
}
