pub struct BlobTrigger {
    pub path: String,
    pub blob: Vec<u8>,
}

pub fn image_processor(trigger: BlobTrigger) {
    // trigger.path has the path to the blob that triggered the function
    // trigger.blob has the contents of the blob
}
