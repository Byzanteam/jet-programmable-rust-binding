extern "C" {
    pub fn hostcall_set_outputs(outputs_ptr: *const u8, outputs_len: usize);
    pub fn hostcall_networking_request(input_ptr: *const u8, input_len: usize);
}
