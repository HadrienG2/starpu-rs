//! Port of the first vector scaling example from the StarPU 2024 tutorial

use starpu_sys::*;
use std::{f32::consts::PI, ffi::c_void, mem, ptr};

#[test]
fn vector_scal() {
    unsafe {
        // Define performance model
        let mut perfmodel = starpu_perfmodel {
            type_: STARPU_NL_REGRESSION_BASED,
            symbol: c"vector_scal".as_ptr(),
            ..Default::default()
        };

        // Define codelet
        extern "C" fn vector_scal_cpu(buffers: *mut *mut c_void, cl_arg: *mut c_void) {
            unsafe {
                let vector = buffers.add(0).read() as *mut starpu_vector_interface;
                let val = (*vector).ptr as *mut f32;
                let n = (*vector).nx as usize;

                let mut factor = 0.0f32;
                starpu_codelet_unpack_args(cl_arg, &mut factor);

                for i in 0..n {
                    *val.add(i) *= factor;
                }
            }
        }
        let mut cl = starpu_codelet {
            cpu_funcs: [Some(vector_scal_cpu), None, None, None],
            nbuffers: 1,
            modes: [STARPU_RW, 0, 0, 0, 0, 0, 0, 0],
            model: &mut perfmodel,
            ..Default::default()
        };

        // Initialize StartPU with default parameters
        starpu_init(ptr::null_mut());

        // Initialize and fill vector
        const NX: u32 = 2048;
        let mut vector = vec![1.0f32; NX as usize];

        // Register vector to StarPU
        //
        // From now on, the application is not supposed to access vector
        // directly, since its content may be copied and modified by a task
        // on a GPU, the main-memory copy then being outdated.
        let mut vector_handle = ptr::null_mut();
        starpu_vector_data_register(
            &mut vector_handle,
            0,
            vector.as_mut_ptr() as usize,
            NX,
            mem::size_of_val(&vector[0]),
        );

        // Define factor
        let factor = PI;

        // Submit an asynchronous task to StarPU
        #[rustfmt::skip]
        starpu_task_insert(
            &mut cl,
            STARPU_VALUE, &factor, mem::size_of_val(&factor),
            STARPU_RW, vector_handle,
            0
        );

        // Wait for task completion
        starpu_task_wait_for_all();

        // Unregister vector from StarPU, which brings back the modified
        // version to main memory, so the result can be read
        starpu_data_unregister(vector_handle);

        // Check that the results are correct
        assert!(vector.iter().all(|value| *value == factor));

        // Shut down StarPU
        starpu_shutdown();
    }
}
