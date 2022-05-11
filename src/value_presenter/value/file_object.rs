use serde_json::{json, Value};

use super::{json_codec::JsonCodec, uuid::Uuid};

#[derive(Debug)]
pub enum ParseFileObjectError {
    InvalidJson,
    InvalidObjectUuid,
    InvalidFilename,
    InvalidFilesize,
    InvalidMimetype,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileObject {
    pub object_uuid: Uuid,
    pub filename: String,
    pub filesize: u64,
    pub mimetype: String,
}

impl FileObject {
    pub fn new(object_uuid: Uuid, filename: String, filesize: u64, mimetype: String) -> Self {
        Self {
            object_uuid,
            filename,
            filesize,
            mimetype,
        }
    }
}

impl JsonCodec for FileObject {
    type Err = ParseFileObjectError;

    fn from_json(value: &Value) -> Result<Self, Self::Err> {
        if !value.is_object() {
            return Err(ParseFileObjectError::InvalidJson);
        }

        let object_uuid = match value.get("object_uuid") {
            Some(object_uuid) => match Uuid::from_json(object_uuid) {
                Ok(uuid) => uuid,
                Err(_) => return Err(ParseFileObjectError::InvalidObjectUuid),
            },
            None => return Err(ParseFileObjectError::InvalidObjectUuid),
        };

        let filename = match value.get("filename") {
            Some(filename) => match filename.as_str() {
                Some(filename) => filename.to_string(),
                None => return Err(ParseFileObjectError::InvalidFilename),
            },
            None => return Err(ParseFileObjectError::InvalidFilename),
        };

        let filesize = match value.get("filesize") {
            Some(filesize) => match filesize.as_u64() {
                Some(filesize) => filesize,
                None => return Err(ParseFileObjectError::InvalidFilesize),
            },
            None => return Err(ParseFileObjectError::InvalidFilesize),
        };

        let mimetype = match value.get("mimetype") {
            Some(mimetype) => match mimetype.as_str() {
                Some(mimetype) => mimetype.to_string(),
                None => return Err(ParseFileObjectError::InvalidMimetype),
            },
            None => return Err(ParseFileObjectError::InvalidMimetype),
        };

        Ok(FileObject {
            object_uuid,
            filename,
            filesize,
            mimetype,
        })
    }

    fn to_json(&self) -> Value {
        json!({
            "object_uuid": self.object_uuid.to_json(),
            "filename": self.filename,
            "filesize": self.filesize,
            "mimetype": self.mimetype,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_from_json() {
        {
            let json = json!({
                "object_uuid": "00000000-0000-0000-0000-ffff00000000",
                "filename": "test.txt",
                "filesize": 123,
                "mimetype": "text/plain",
            });

            let file_object = FileObject::from_json(&json);
            assert!(matches!(
                file_object,
                Ok(FileObject {
                    object_uuid,
                    filename,
                    filesize,
                    mimetype,
                }) if object_uuid == Uuid("00000000-0000-0000-0000-ffff00000000".to_string()) && filename == "test.txt" && filesize == 123 && mimetype == "text/plain"
            ));
        }
    }

    #[test]
    fn test_from_invalid_json() {
        {
            let json = json!([]);

            let file_object = FileObject::from_json(&json);
            assert!(matches!(
                file_object,
                Err(ParseFileObjectError::InvalidJson)
            ));
        }

        // invalid object_uuid
        {
            let json = json!({"object_uuid": "invalid", "filename": "test.txt", "filesize": 123 as i64, "mimetype": "text/plain"});

            let file_object = FileObject::from_json(&json);
            assert!(matches!(
                file_object,
                Err(ParseFileObjectError::InvalidObjectUuid)
            ));
        }

        // invalid filename
        {
            let json = json!({"object_uuid": "00000000-0000-0000-0000-ffff00000000", "filename": null,  "filesize": 123 as i64, "mimetype": "text/plain"});

            let file_object = FileObject::from_json(&json);
            assert!(matches!(
                file_object,
                Err(ParseFileObjectError::InvalidFilename)
            ));
        }

        // invalid filesize
        {
            let json = json!({"object_uuid": "00000000-0000-0000-0000-ffff00000000", "filename": "test.txt", "filesize": null, "mimetype": "text/plain"});

            let file_object = FileObject::from_json(&json);
            assert!(matches!(
                file_object,
                Err(ParseFileObjectError::InvalidFilesize)
            ));
        }

        // invalid mimetype
        {
            let json = json!({"object_uuid": "00000000-0000-0000-0000-ffff00000000", "filename": "test.txt", "filesize": 123 as i64, "mimetype": null});

            let file_object = FileObject::from_json(&json);
            assert!(matches!(
                file_object,
                Err(ParseFileObjectError::InvalidMimetype)
            ));
        }
    }

    #[test]
    fn test_to_json() {
        {
            let file_object = FileObject {
                object_uuid: Uuid("00000000-0000-0000-0000-ffff00000000".to_string()),
                filename: "test.txt".to_string(),
                filesize: 123,
                mimetype: "text/plain".to_string(),
            };

            let json = json!({
                "object_uuid": "00000000-0000-0000-0000-ffff00000000",
                "filename": "test.txt",
                "filesize": 123,
                "mimetype": "text/plain",
            });

            assert!(file_object.to_json() == json);
        }
    }
}
