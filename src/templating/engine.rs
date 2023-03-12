use std::{collections::HashMap, env, fs::File, io::Read};

pub struct TemplateRenderer<'a> {
    pub template: &'a mut Template,
    pub env_vars: &'a HashMap<String, String>,
}

impl<'a> TemplateRenderer<'a> {
    pub fn new(template: &'a mut Template, env_vars: &'a HashMap<String, String>) -> Self {
        TemplateRenderer { template, env_vars }
    }

    pub fn render_text_vars(&mut self) {
        let mut buffer = [0; 1024];
        if let Ok(_) = File::read(&mut self.template.file, &mut buffer) {
            let mut template = String::from_utf8(buffer.to_vec()).unwrap();

            for (key, value) in self.env_vars.iter() {
                template = template.replace(&format!("{{{{ {} }}}}", key), value);
            }

            self.template.rendered = Some(template.to_string());
        } else {
            self.template.rendered = None;
        }
    }

    pub fn render_js_fn(&mut self) {
        let mut buffer = [0; 1024];
        if let Ok(_) = File::read(&mut self.template.file, &mut buffer) {
            let mut template = String::from_utf8(buffer.to_vec()).unwrap();

            // Generate a script tag to store the compiled template js functions
            let script_tag = "<script type=\"text/javascript\">!!{{}}!!</script>";
            template = template.replace("</head>", &format!("{}{}", script_tag, "</head>"));

            for (key, value) in self.env_vars.iter() {
                template = template.replace(&format!("{{{{ {} }}}}", key), value);
            }

            self.template.rendered = Some(template.to_string());
        } else {
            self.template.rendered = None;
        }
    }
}

pub struct Template {
    pub name: String,
    pub path: String,
    pub file: File,
    pub rendered: Option<String>,
    pub env: HashMap<String, String>,
}

impl Template {
    pub fn build(name: String, rel_path: String) -> Result<Self, String> {
        let cwd = env::current_dir().unwrap();
        let path = cwd.join("templates").join(&rel_path);

        if let Ok(file) = File::open(&path) {
            Ok(Template {
                name,
                path: path.as_path().display().to_string(),
                file,
                rendered: None,
                env: HashMap::new(),
            })
        } else {
            Err(format!("Template file not found: {}", rel_path))
        }
    }

    pub fn render(&mut self, env: HashMap<String, String>) -> &mut Self {
        self.env = env.clone();

        let mut buffer = [0; 1024];
        if let Ok(_) = File::read(&mut self.file, &mut buffer) {
            let mut template = String::from_utf8(buffer.to_vec()).unwrap();

            for (key, value) in env.iter() {
                template = template.replace(&format!("{{{{ {} }}}}", key), value);
            }

            self.rendered = Some(template.to_string());
            self
        } else {
            self.rendered = None;
            self
        }
    }
}
