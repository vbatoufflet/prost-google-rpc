use std::{env, fs::OpenOptions, io::Write, path::PathBuf};

use heck::ToUpperCamelCase;
use prost::Message;
use prost_types::FileDescriptorSet;
use quote::{format_ident, quote};

pub struct Builder {
    out_dir: Option<PathBuf>,
    bytes: Option<Vec<u8>>,
    file_descriptor_set_path: Option<PathBuf>,
}

impl Builder {
    pub fn bytes(mut self, bytes: Vec<u8>) -> Self {
        self.bytes = Some(bytes);
        self
    }

    pub fn file_descriptor_set_path(mut self, path: PathBuf) -> Self {
        self.file_descriptor_set_path = Some(path);
        self
    }

    pub fn out_dir(mut self, out_dir: PathBuf) -> Self {
        self.out_dir = Some(out_dir);
        self
    }

    pub fn generate(self) -> Result<(), Box<dyn std::error::Error>> {
        let out_dir = self.out_dir.unwrap_or(PathBuf::from(env::var("OUT_DIR")?));

        let bytes = if let Some(bytes) = self.bytes {
            bytes
        } else if let Some(path) = self.file_descriptor_set_path {
            std::fs::read(path)?
        } else {
            return Err("missing file descriptor set".into());
        };

        let set = FileDescriptorSet::decode(bytes.as_slice())?;

        for fd in set.file {
            let package = fd.package();

            if package.starts_with("google.") {
                continue;
            }

            let mut file = OpenOptions::new()
                .append(true)
                .open(out_dir.join(format!("{package}.rs")))?;

            for msg in &fd.message_type {
                let name = match msg.name {
                    Some(ref name) => name,
                    None => continue,
                };

                let type_name = format_ident!("{}", name.to_upper_camel_case());
                let type_url = format!("type.googleapis.com/{package}.{name}");

                let tokens = quote! {
                    impl prost_google_rpc::TypeURL for #type_name {
                        fn type_url() -> &'static str {
                            #type_url
                        }
                    }
                };

                writeln!(file, "\n{}", &tokens).unwrap();
            }
        }

        Ok(())
    }
}

pub fn configure() -> Builder {
    Builder {
        bytes: None,
        file_descriptor_set_path: None,
        out_dir: None,
    }
}
