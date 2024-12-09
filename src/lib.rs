use zed_extension_api as zed;

struct CythonExtension;

impl zed::Extension for CythonExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(CythonExtension);
