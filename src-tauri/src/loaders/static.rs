#[macro_export]
macro_rules! read_proto {
    ($file_name:expr, $message_type:ident) => {
        {
            use $crate::Asset;
            use prost::Message;

            let data = Asset::get(&$file_name[..]).unwrap().data;
            let data_ptr = data.as_ref();   
            if let Ok(message) = $message_type::decode(data_ptr) {
                Some(message)
            } else {
                None
            }
        }
    };
}
