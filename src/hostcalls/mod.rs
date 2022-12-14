extern "C" {
    pub fn hostcall_set_outputs(outputs_ptr: *const u8, outputs_len: usize);

    /// Parameters and return values for network requests
    ///
    /// # Arguments
    ///
    /// * `inputs_ptr` - NetworkingRequest Pointer to data
    /// * `inputs_len` - NetworkingRequest Length of data
    /// * Return the length of `response`
    ///
    /// # Examples
    /// ## NetworkingRequest
    /// ```no_run
    /// #[derive(Serialize)]
    /// struct NetworkingRequest {
    ///     method: String,
    ///     url: String,
    ///     headers: Vec<(String, String)>,
    ///     body: Option<String>,
    /// }
    /// ```
    /// ## Code Example
    /// ```no_run
    /// let request = NetworkingRequest {
    ///     method: "GET".to_string(),
    ///     url: "https://rust-lang.org/".to_string(),
    ///     headers: Vec::new(),
    ///     body: None,
    /// };
    /// let request_binary = serde_json::to_string(&resquest).unwrap();
    /// let response_len =
    ///     unsafe { hostcall_networking_request(request_binary.as_ptr(), request_binary.len()) };
    /// ```
    pub fn hostcall_networking_request(inputs_ptr: *const u8, inputs_len: usize) -> usize;
    /// Memory request and outgoing memory address for network request response
    ///
    /// # Arguments
    ///
    /// * `inputs_ptr` - NetworkingResponse Pointer to data
    /// ## Code Example
    /// ```no_run
    /// let response_ptr = __wasm_malloc(response_len);
    /// let response_str = unsafe {
    ///    hostcall_networking_retrieve_response(response_ptr);
    ///    let slice = slice::from_raw_parts(response_ptr, response_len);
    ///    str::from_utf8(slice).unwrap()
    ///};
    /// ```
    pub fn hostcall_networking_retrieve_response(inputs_ptr: *const u8);
}