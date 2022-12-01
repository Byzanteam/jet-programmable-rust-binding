extern "C" {
    pub fn hostcall_set_outputs(outputs_ptr: *const u8, outputs_len: usize);

    /// Parameters and return values for network requests
    ///
    /// # Arguments
    ///
    /// * `inputs_ptr` - RequestData Pointer to data
    /// * `inputs_len` - RequestData Length of data
    /// * `new_ptr` - ResponseData Pointer to data
    ///
    /// # So the number returned is the length of the response data
    ///
    /// # Examples
    /// ## RequestData
    /// ```no_run
    /// #[derive(Serialize)]
    /// struct RequestData {
    ///     method: String,
    ///     url: String,
    ///     headers: Vec<(String, String)>,
    ///     body: Option<String>,
    /// }
    /// ```
    /// ## Code Example
    /// ```no_run    
    /// let json = RequestData {
    ///     method: "GET".to_string(),
    ///     url: "https://rust-lang.org/".to_string(),
    ///     headers: Vec::new(),
    ///     body: None,
    /// };
    ///
    /// let data = serde_json::to_string(&json).unwrap();
    /// // Create the address of the return value
    /// let mut response_data = "";
    /// response_data = unsafe {
    ///     // Get the data length of the response
    ///     let len = hostcall_networking_request(data.as_ptr(), data.len(), response_data.as_ptr());
    ///     // Read data
    ///     let slice = slice::from_raw_parts(response_data.as_ptr(), len);
    ///     // convert &str
    ///     str::from_utf8(slice).unwrap();
    /// };
    /// ```
    pub fn hostcall_networking_request(
        inputs_ptr: *const u8,
        inputs_len: usize,
        new_ptr: *const u8,
    ) -> usize;
}
