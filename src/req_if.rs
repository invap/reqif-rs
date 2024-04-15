use yaserde_derive::YaSerialize;

const LAST_CHANGE_DATE: &str = "2017-11-14T15:44:26.000+02:00";

#[derive(Debug, PartialEq, YaSerialize)]
pub struct ReqIfHeader {
    #[yaserde(rename = "IDENTIFIER", attribute)]
    pub identifier: String,
    #[yaserde(rename = "CREATION-TIME")]
    pub creation_time: String,
    #[yaserde(rename = "REPOSITORY-ID")]
    pub repository_id: String,
    #[yaserde(rename = "REQ-IF-TOOL-ID")]
    pub req_if_tool_id: String,
    #[yaserde(rename = "REQ-IF-VERSION")]
    pub req_if_version: String,
    #[yaserde(rename = "SOURCE-TOOL-ID")]
    pub source_tool_id: String,
    #[yaserde(rename = "TITLE")]
    pub title: String,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct TheHeader {
    #[yaserde(rename = "REQ-IF-HEADER")]
    pub req_if_header: ReqIfHeader,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct DataType {
    #[yaserde(attribute, rename = "IDENTIFIER")]
    identifier: String,
    #[yaserde(attribute, rename = "LAST-CHANGE")]
    last_change: String,
    #[yaserde(attribute, rename = "LONG-NAME")]
    long_name: String,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct DataTypes {
    #[yaserde(rename = "DATATYPE-DEFINITION-STRING")]
    xhtml_definition: DataType,
}

impl DataTypes {
    pub fn new() -> Self {
        DataTypes {
            xhtml_definition: DataType {
                identifier: "DATATYPE-DEFINITION-XHTML-IDENTIFIER".to_string(),
                last_change: LAST_CHANGE_DATE.to_string(),
                long_name: "XHTMLString".to_string(),
            },
        }
    }
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct SpecificationTypeModuleAttributes {
    #[yaserde(rename = "ATTRIBUTE-DEFINITION-XHTML")]
    reqif_name_attribute: AttributeDefinitionXHtml,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct SpecificationTypeModule {
    #[yaserde(attribute, rename = "IDENTIFIER")]
    identifier: String,
    #[yaserde(attribute, rename = "LAST-CHANGE")]
    last_change: String,
    #[yaserde(attribute, rename = "LONG-NAME")]
    long_name: String,
    #[yaserde(rename = "SPEC-ATTRIBUTES")]
    attributes: SpecificationTypeModuleAttributes,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct SpecTypes {
    #[yaserde(rename = "SPEC-OBJECT-TYPE")]
    spec_object_type_requirement: SpecObjectTypeRequirement,
    #[yaserde(rename = "SPECIFICATION-TYPE")]
    specification_type_module: SpecificationTypeModule,
}

impl SpecTypes {
    fn new(data_types: &DataTypes) -> Self {
        SpecTypes {
            specification_type_module: SpecificationTypeModule {
                identifier: "MODULE-SPECIFICATION-TYPE-ID".to_string(),
                last_change: LAST_CHANGE_DATE.to_string(),
                long_name: "Module Type".to_string(),
                attributes: SpecificationTypeModuleAttributes {
                    reqif_name_attribute: AttributeDefinitionXHtml {
                        identifier: "ATTRIBUTE-DEFINITION-XHTML-REQIF.NAME-ID".to_string(),
                        last_change: LAST_CHANGE_DATE.to_string(),
                        long_name: "ReqIF.Name".to_string(),
                        type_ref: TypeDefinitionXHtmlRef {
                            reference: data_types.xhtml_definition.identifier.clone(),
                        },
                    },
                },
            },
            spec_object_type_requirement: SpecObjectTypeRequirement::new(data_types),
        }
    }
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct TypeDefinitionXHtmlRef {
    #[yaserde(rename = "DATATYPE-DEFINITION-XHTML-REF")]
    reference: String,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct AttributeDefinitionXHtml {
    #[yaserde(attribute, rename = "IDENTIFIER")]
    identifier: String,
    #[yaserde(attribute, rename = "LAST-CHANGE")]
    last_change: String,
    #[yaserde(attribute, rename = "LONG-NAME")]
    long_name: String,
    #[yaserde(rename = "TYPE")]
    type_ref: TypeDefinitionXHtmlRef,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct RequirementSpecObjectTypeAttributes {
    #[yaserde(rename = "ATTRIBUTE-DEFINITION-XHTML")]
    text_attribute: AttributeDefinitionXHtml,
    #[yaserde(rename = "ATTRIBUTE-DEFINITION-XHTML")]
    id_attribute: AttributeDefinitionXHtml,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct SpecObjectTypeRequirement {
    #[yaserde(attribute, rename = "IDENTIFIER")]
    identifier: String,
    #[yaserde(attribute, rename = "LAST-CHANGE")]
    last_change: String,
    #[yaserde(attribute, rename = "LONG-NAME")]
    long_name: String,
    #[yaserde(rename = "SPEC-ATTRIBUTES")]
    attributes: RequirementSpecObjectTypeAttributes,
}

impl SpecObjectTypeRequirement {
    fn new(data_types: &DataTypes) -> Self {
        SpecObjectTypeRequirement {
            identifier: "SPEC-OBJEC-TYPE-REQ-TYPE-IDENTIFIER".to_string(),
            long_name: "Requirement Type".to_string(),
            last_change: LAST_CHANGE_DATE.to_string(),
            attributes: RequirementSpecObjectTypeAttributes {
                text_attribute: AttributeDefinitionXHtml {
                    identifier: "ATTRIBUTE-DEFINITION-XHTML-REQIF.Text-ID".to_string(),
                    last_change: LAST_CHANGE_DATE.to_string(),
                    long_name: "ReqIF.Text".to_string(),
                    type_ref: TypeDefinitionXHtmlRef {
                        reference: data_types.xhtml_definition.identifier.clone(),
                    },
                },
                id_attribute: AttributeDefinitionXHtml {
                    identifier: "ATTRIBUTE-DEFINITION-XHTML-PUID-ID".to_string(),
                    last_change: LAST_CHANGE_DATE.to_string(),
                    long_name: "IE PUID".to_string(),
                    type_ref: TypeDefinitionXHtmlRef {
                        reference: data_types.xhtml_definition.identifier.clone(),
                    },
                },
            },
        }
    }
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct SpecObjectType {
    #[yaserde(rename = "SPEC-OBJECT-TYPE-REF")]
    reference: String,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct AttributeValueXHtmlDefinition {
    #[yaserde(rename = "ATTRIBUTE-DEFINITION-XHTML-REF")]
    reference: String,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct XHtmlValue {
    #[yaserde(rename = "xhtml:div")]
    value: String,
}
#[derive(Debug, PartialEq, YaSerialize)]
pub struct AttributeValueXHtml {
    #[yaserde(rename = "THE-VALUE")]
    the_value: XHtmlValue,
    #[yaserde(rename = "DEFINITION")]
    definition: AttributeValueXHtmlDefinition,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct SpecObjectRequirementValues {
    #[yaserde(rename = "ATTRIBUTE-VALUE-XHTML")]
    req_id: AttributeValueXHtml,
    #[yaserde(rename = "ATTRIBUTE-VALUE-XHTML")]
    req_text: AttributeValueXHtml,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct SpecObjectRequirement {
    #[yaserde(attribute, rename = "IDENTIFIER")]
    identifier: String,
    #[yaserde(attribute, rename = "LAST-CHANGE")]
    last_change: String,
    #[yaserde(attribute, rename = "LONG-NAME")]
    long_name: String,
    #[yaserde(rename = "TYPE")]
    spec_object_type: SpecObjectType,
    #[yaserde(rename = "VALUES")]
    values: SpecObjectRequirementValues,
}

impl SpecObjectRequirement {
    fn new(
        identifier: String,
        last_change: String,
        long_name: String,
        text: String,
        spec_types: &SpecTypes,
    ) -> Self {
        SpecObjectRequirement {
            identifier: identifier.clone(),
            last_change,
            long_name,
            spec_object_type: SpecObjectType {
                reference: spec_types.spec_object_type_requirement.identifier.clone(),
            },
            values: SpecObjectRequirementValues {
                req_id: AttributeValueXHtml {
                    definition: AttributeValueXHtmlDefinition {
                        reference: spec_types.spec_object_type_requirement.attributes.id_attribute.identifier.clone(),
                    },
                    the_value: XHtmlValue { value: identifier },
                },
                req_text: AttributeValueXHtml {
                    definition: AttributeValueXHtmlDefinition {
                        reference: spec_types.spec_object_type_requirement.attributes.text_attribute.identifier.clone(),
                    },
                    the_value: XHtmlValue{value: text },
                },
            },
        }
    }
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct SpecObjects {
    #[yaserde(rename = "SPEC-OBJECT")]
    requirements: Vec<SpecObjectRequirement>,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct SpecificationRef {
    #[yaserde(rename = "SPECIFICATION-TYPE-REF")]
    spec_ref: String,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct Object {
    #[yaserde(rename = "SPEC-OBJECT-REF")]
    object_ref: String,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct SpecHierarchy {
    #[yaserde(attribute, rename = "IDENTIFIER")]
    identifier: String,
    #[yaserde(attribute, rename = "LAST-CHANGE")]
    last_change: String,
    #[yaserde(rename = "OBJECT")]
    object: Object,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct Children {
    #[yaserde(rename = "SPEC-HIERARCHY")]
    spec_hierarchy: SpecHierarchy,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct Specification {
    #[yaserde(attribute, rename = "IDENTIFIER")]
    identifier: String,
    #[yaserde(attribute, rename = "LAST-CHANGE")]
    last_change: String,
    #[yaserde(attribute, rename = "LONG-NAME")]
    long_name: String,
    #[yaserde(rename = "TYPE")]
    type_ref: SpecificationRef,
    #[yaserde(rename = "CHILDREN")]
    children: Children,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct Specifications {
    #[yaserde(rename = "SPECIFICATION")]
    specifications: Vec<Specification>,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct ReqIfContent {
    #[yaserde(rename = "DATATYPES")]
    pub data_types: DataTypes,
    #[yaserde(rename = "SPEC-TYPES")]
    pub spec_types: SpecTypes,
    #[yaserde(rename = "SPEC-OBJECTS")]
    pub spec_object: SpecObjects,
    #[yaserde(rename = "SPECIFICATIONS")]
    pub specifications: Specifications,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct CoreContent {
    #[yaserde(rename = "REQ-IF-CONTENT")]
    pub req_if_content: ReqIfContent,
}

impl CoreContent {
    pub fn new() -> Self {
        let data_types = DataTypes::new();
        let spec_types = SpecTypes::new(&data_types);
        CoreContent {
            req_if_content: ReqIfContent {
                spec_object: SpecObjects {
                    requirements: vec![],
                },
                specifications: Specifications {
                    specifications: vec![Specification {
                        identifier: "CAUR".to_string(),
                        last_change: LAST_CHANGE_DATE.to_string(),
                        long_name: "Crypto Ar User Requirements".to_string(),
                        type_ref: SpecificationRef {
                            spec_ref: spec_types.specification_type_module.identifier.clone(),
                        },
                        children: Children {
                            spec_hierarchy: SpecHierarchy {
                                identifier: "h1".to_string(),
                                last_change: LAST_CHANGE_DATE.to_string(),
                                object: Object {
                                    object_ref: "CAUR-2".to_string(),
                                },
                            },
                        },
                    }],
                },
                spec_types,
                data_types,
            },
        }
    }
}

#[derive(Debug, PartialEq, YaSerialize)]
#[yaserde(rename = "REQ-IF")]
pub struct ReqIf {
    #[yaserde(attribute)]
    pub xmlns: String,
    #[yaserde(rename = "xmlns:xhtml", attribute)]
    pub xmlns_xhtml: String,
    #[yaserde(rename = "THE-HEADER")]
    pub the_header: TheHeader,
    #[yaserde(rename = "CORE-CONTENT")]
    pub core_content: CoreContent,
}

#[cfg(test)]
mod tests {

    use crate::req_if::TheHeader;
    use crate::req_if::LAST_CHANGE_DATE;
    use crate::req_if::{CoreContent, ReqIf, ReqIfHeader, SpecObjectRequirement};
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_serialize() {
        let req_if_header = ReqIfHeader {
            identifier: "1234567890".to_string(),
            creation_time: LAST_CHANGE_DATE.to_string(),
            repository_id: "123456789io0pxazsxdbghnjmk".to_string(),
            req_if_tool_id: "Doorstop".to_string(),
            req_if_version: "1.0".to_string(),
            source_tool_id: "Doorstop".to_string(),
            title: "Requirements examples".to_string(),
        };

        let the_header = TheHeader { req_if_header };
        let xmlns = "http://www.omg.org/spec/ReqIF/20110401/reqif.xsd".to_string();
        let xmlns_xhtml = "http://www.w3.org/1999/xhtml".to_string();

        let mut a = ReqIf {
            the_header,
            xmlns,
            xmlns_xhtml,
            core_content: CoreContent::new(),
        };

        a.core_content
            .req_if_content
            .spec_object
            .requirements
            .push(SpecObjectRequirement::new(
                "CAUR-2".to_string(),
                LAST_CHANGE_DATE.to_string(),
                "Titulo del requerimiento 2".to_string(),
                "Texto del requerimiento 2".to_string(),
                &a.core_content
                    .req_if_content
                    .spec_types,
            ));

        let yaserde_cfg = yaserde::ser::Config {
            perform_indent: true,
            ..Default::default()
        };

        let s = yaserde::ser::to_string_with_config(&a, &yaserde_cfg)
            .ok()
            .unwrap();
        // println!("{}", s);
        let mut file = File::create("out.reqif").ok().unwrap();
        let _ = file.write_all(s.as_bytes());
    }
}
