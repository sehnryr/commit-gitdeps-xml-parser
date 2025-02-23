#[derive(Debug, PartialEq)]
pub struct DependencyManifest<'a> {
    xml_schema_definition_namespace: &'a str,
    xml_schema_instance_namespace: &'a str,
    base_url: &'a str,
}

impl<'a> DependencyManifest<'a> {
    pub fn new(
        xml_schema_definition_namespace: &'a str,
        xml_schema_instance_namespace: &'a str,
        base_url: &'a str,
    ) -> Self {
        Self {
            xml_schema_definition_namespace,
            xml_schema_instance_namespace,
            base_url,
        }
    }

    pub fn base_url(&self) -> &str {
        self.base_url
    }
}
