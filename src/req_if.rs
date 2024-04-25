/// reqif-rs: Help library to write reqif files implemented in Rust.
/// Copyright (C) <2024>  INVAP S.E.
///
/// This file is part of reqif-rs.
///
/// reqif-rs is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Affero General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Affero General Public License for more details.
///
/// You should have received a copy of the GNU Affero General Public License
/// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use anyhow::{bail, Result};
use chrono::{DateTime, Local, SecondsFormat};
use std::fs::File;
use std::io::Write;
use yaserde_derive::YaSerialize;

fn get_default_last_change_date() -> String {
    "2017-11-14T15:44:26.000+02:00".to_string()
}

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
                last_change: get_default_last_change_date(),
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
                last_change: get_default_last_change_date(),
                long_name: "Module Type".to_string(),
                attributes: SpecificationTypeModuleAttributes {
                    reqif_name_attribute: AttributeDefinitionXHtml {
                        identifier: "ATTRIBUTE-DEFINITION-XHTML-REQIF.NAME-ID".to_string(),
                        last_change: get_default_last_change_date(),
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
            last_change: get_default_last_change_date(),
            attributes: RequirementSpecObjectTypeAttributes {
                text_attribute: AttributeDefinitionXHtml {
                    identifier: "ATTRIBUTE-DEFINITION-XHTML-REQIF.Text-ID".to_string(),
                    last_change: get_default_last_change_date(),
                    long_name: "ReqIF.Text".to_string(),
                    type_ref: TypeDefinitionXHtmlRef {
                        reference: data_types.xhtml_definition.identifier.clone(),
                    },
                },
                id_attribute: AttributeDefinitionXHtml {
                    identifier: "ATTRIBUTE-DEFINITION-XHTML-PUID-ID".to_string(),
                    last_change: get_default_last_change_date(),
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
    pub fn new(
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
                        reference: spec_types
                            .spec_object_type_requirement
                            .attributes
                            .id_attribute
                            .identifier
                            .clone(),
                    },
                    the_value: XHtmlValue { value: identifier },
                },
                req_text: AttributeValueXHtml {
                    definition: AttributeValueXHtmlDefinition {
                        reference: spec_types
                            .spec_object_type_requirement
                            .attributes
                            .text_attribute
                            .identifier
                            .clone(),
                    },
                    the_value: XHtmlValue { value: text },
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
    pub spec_ref: String,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct Object {
    #[yaserde(rename = "SPEC-OBJECT-REF")]
    pub object_ref: String,
}
impl Object {
    pub fn new(object_ref: String) -> Object {
        Object { object_ref }
    }
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct SpecHierarchy {
    #[yaserde(attribute, rename = "IDENTIFIER")]
    pub identifier: String,
    #[yaserde(attribute, rename = "LAST-CHANGE")]
    pub last_change: String,
    #[yaserde(rename = "OBJECT")]
    pub object: Object,
    #[yaserde(rename = "CHILDREN")]
    pub children: Option<Children>,
}

impl SpecHierarchy {
    pub fn new(identifier: String, last_change: String, object: Object) -> Self {
        SpecHierarchy {
            identifier,
            last_change,
            object,
            children: None,
        }
    }
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct Children {
    #[yaserde(rename = "SPEC-HIERARCHY")]
    spec_hierarchy: Vec<SpecHierarchy>,
}

impl Children {
    /// Adds the `spec_hierarchy` as the last children in the given `depth`.
    /// a `depth` of 0 means add the spec as direct children.
    /// # Panics:
    /// Panic will occur in case that any intermediate level is missing.
    pub fn add_spec_hierarchy(
        &mut self,
        spec_hierarchy: SpecHierarchy,
        mut depth: i32,
    ) -> Result<()> {
        if depth == 0 {
            self.spec_hierarchy.push(spec_hierarchy);
        } else {
            let spec = match self.spec_hierarchy.last_mut() {
                Some(s) => s,
                None => bail!("Missing spech hierarchy at level: {}", depth),
            };

            depth -= 1;
            if spec.children.is_none() {
                spec.children = Some(Children::new());
            }

            spec.children
                .as_mut()
                .unwrap()
                .add_spec_hierarchy(spec_hierarchy, depth)?;
        }
        Ok(())
    }

    pub fn new() -> Self {
        Children {
            spec_hierarchy: Vec::new(),
        }
    }

    pub fn get_spec_hierarchy(&self) -> &Vec<SpecHierarchy> {
        &self.spec_hierarchy
    }
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct Specification {
    #[yaserde(attribute, rename = "IDENTIFIER")]
    pub identifier: String,
    #[yaserde(attribute, rename = "LAST-CHANGE")]
    pub last_change: String,
    #[yaserde(attribute, rename = "LONG-NAME")]
    pub long_name: String,
    #[yaserde(rename = "TYPE")]
    pub type_ref: SpecificationRef,
    #[yaserde(rename = "CHILDREN")]
    pub children: Children,
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
                    specifications: vec![],
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

impl ReqIf {
    pub fn new(
        identifier: String,
        creation_time: DateTime<Local>,
        repository_id: String,
        req_if_tool_id: String,
        source_tool_id: String,
        title: String,
    ) -> Self {
        let req_if_header = ReqIfHeader {
            identifier,
            creation_time: creation_time.to_rfc3339_opts(SecondsFormat::Millis, false),
            repository_id,
            req_if_tool_id,
            req_if_version: "1.0".to_string(),
            source_tool_id,
            title,
        };

        let the_header = TheHeader { req_if_header };
        let xmlns = "http://www.omg.org/spec/ReqIF/20110401/reqif.xsd".to_string();
        let xmlns_xhtml = "http://www.w3.org/1999/xhtml".to_string();

        ReqIf {
            the_header,
            xmlns,
            xmlns_xhtml,
            core_content: CoreContent::new(),
        }
    }

    pub fn add_requirement(&mut self, requirement: SpecObjectRequirement) {
        self.core_content
            .req_if_content
            .spec_object
            .requirements
            .push(requirement);
    }

    pub fn build_module_specification(
        &mut self,
        identifier: String,
        last_change: String,
        long_name: String,
    ) -> Specification {
        Specification {
            identifier,
            last_change,
            long_name,
            type_ref: SpecificationRef {
                spec_ref: self.get_module_specification_type().clone(),
            },
            children: Children {
                spec_hierarchy: vec![],
            },
        }
    }

    pub fn add_specification(&mut self, specification: Specification) {
        self.core_content
            .req_if_content
            .specifications
            .specifications
            .push(specification);
    }

    pub fn get_module_specification_type(&self) -> &String {
        &self
            .core_content
            .req_if_content
            .spec_types
            .specification_type_module
            .identifier
    }

    pub fn write_to(&self, filename: &str) -> anyhow::Result<()> {
        let yaserde_cfg = yaserde::ser::Config {
            perform_indent: true,
            ..Default::default()
        };

        let s = match yaserde::ser::to_string_with_config(self, &yaserde_cfg) {
            Ok(s) => s,
            Err(s) => bail!(s),
        };

        let mut file = File::create(filename)?;
        let _ = file.write_all(s.as_bytes());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{get_default_last_change_date, Children, Object, SpecHierarchy};

    #[test]
    fn test_add_spec_hierarchy() {
        let mut children = Children::new();
        for _ in 0..4 {
            children
                .add_spec_hierarchy(
                    SpecHierarchy::new(
                        "2.1.2".to_string(),
                        get_default_last_change_date(),
                        Object::new("REQ001".to_string()),
                    ),
                    0,
                )
                .expect("Error");
        }
        assert_eq!(children.spec_hierarchy.len(), 4);
    }

    #[test]
    fn test_add_spec_panic() {
        let mut children = Children::new();
        children
            .add_spec_hierarchy(
                SpecHierarchy::new(
                    "2.1.2".to_string(),
                    get_default_last_change_date(),
                    Object::new("REQ001".to_string()),
                ),
                0,
            )
            .expect("error");
        let res = children.add_spec_hierarchy(
            SpecHierarchy::new(
                "2.1.2".to_string(),
                get_default_last_change_date(),
                Object::new("REQ001".to_string()),
            ),
            2,
        );
        assert!(res.is_err());
    }

    #[test]
    fn test_add_spec_hierarchy_2() {
        let mut children = Children::new();
        children
            .add_spec_hierarchy(
                SpecHierarchy::new(
                    "2.1.2".to_string(),
                    get_default_last_change_date(),
                    Object::new("REQ001".to_string()),
                ),
                0,
            )
            .expect("error");
        children
            .add_spec_hierarchy(
                SpecHierarchy::new(
                    "2.1.2".to_string(),
                    get_default_last_change_date(),
                    Object::new("REQ001".to_string()),
                ),
                1,
            )
            .expect("error");

        children
            .add_spec_hierarchy(
                SpecHierarchy::new(
                    "2.1.2".to_string(),
                    get_default_last_change_date(),
                    Object::new("REQ001".to_string()),
                ),
                1,
            )
            .expect("error");

        assert_eq!(children.spec_hierarchy.len(), 1);
        let spec = children.get_spec_hierarchy().last().unwrap();
        let len = spec.children.as_ref().unwrap().spec_hierarchy.len();
        assert_eq!(len, 2)
    }
}
