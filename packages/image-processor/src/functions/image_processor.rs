pub struct BlobTrigger {
    pub path: String,
    pub blob: Vec<u8>,
}

pub fn image_processor(trigger: BlobTrigger) {
    println!("Processing blob at path {}", trigger.path);
    // placeholder for processing logic
}
