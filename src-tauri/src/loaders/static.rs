use dnd_protos::protos::*;

#[macro_export]
macro_rules! read_proto {
    // TODO cache? Technically, we read binary, so maybe not useful...
    ($file_name:expr, $message_type:ident) => {{
        use prost::Message;
        use $crate::GeneratedAsset;

        if let Some(file) = GeneratedAsset::get(&$file_name[..]) {
            let data = file.data;
            let data_ptr = data.as_ref();
            if let Ok(message) = $message_type::decode(data_ptr) {
                Some(message)
            } else {
                None
            }
        } else {
            None
        }
    }};
}

pub fn get_full_class_name(class: &Class) -> String {
    if !class.subclass.is_empty() {
        format!("{}_{}", class.subclass, class.name)
    } else {
        class.name.clone()
    }
}
