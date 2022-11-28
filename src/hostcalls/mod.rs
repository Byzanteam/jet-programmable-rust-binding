extern "C" {
    pub fn hostcall_set_outputs(outputs_ptr: *const u8, outputs_len: usize);
    pub fn hostcall_request_inputs(inputs_ptr: *const u8, inputs_len: usize, new_ptr: usize) -> usize;
}
 