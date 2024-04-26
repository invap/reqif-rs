// reqif-rs: Help library to write reqif files implemented in Rust.
// Copyright (C) <2024>  INVAP S.E.
//
// This file is part of reqif-rs.
//
// reqif-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
pub mod req_if;

#[cfg(test)]
mod tests {

    use crate::req_if::{Object, ReqIf, SpecHierarchy, SpecObjectRequirement};
    use chrono::{DateTime, Local, SecondsFormat};

    #[test]
    fn test_serialize() {

        let local: DateTime<Local> = Local::now();
        let identifier = "1234567890".to_string();
        let repository_id = "123456789io0pxazsxdbghnjmk".to_string();
        let req_if_tool_id = "Doorstop".to_string();
        let source_tool_id = "Doorstop".to_string();
        let title = "Requirements examples".to_string();

        let mut reqif = ReqIf::new(
            identifier,
            local,
            repository_id,
            req_if_tool_id,
            source_tool_id,
            title,
        );

        let local: DateTime<Local> = Local::now();

        let now = local.to_rfc3339_opts(SecondsFormat::Millis, false);

        reqif.add_requirement(SpecObjectRequirement::new(
            "REQS-1".to_string(),
            now.clone(),
            "Titulo del requerimiento 1".to_string(),
            "Texto del requerimiento 1.".to_string(),
            &reqif.core_content.req_if_content.spec_types,
        ));

        reqif.add_requirement(SpecObjectRequirement::new(
            "REQS-2".to_string(),
            now.clone(),
            "Titulo del requerimiento 2".to_string(),
            "Texto del requerimiento 2.".to_string(),
            &reqif.core_content.req_if_content.spec_types,
        ));

        let mut specification = reqif.build_module_specification(
            "REQS".to_string(),
            now.to_string(),
            "Project User Requirements".to_string(),
        );

        specification
            .children
            .add_spec_hierarchy(
                SpecHierarchy {
                    identifier: "h1".to_string(),
                    last_change: now.clone(),
                    object: Object {
                        object_ref: "REQS-1".to_string(),
                    },
                    children: None,
                },
                0,
            )
            .expect("Unexpected error adding children");

        specification
            .children
            .add_spec_hierarchy(
                SpecHierarchy {
                    identifier: "h2".to_string(),
                    last_change: now.clone(),
                    object: Object {
                        object_ref: "REQS-2".to_string(),
                    },
                    children: None,
                },
                1,
            )
            .expect("Unexpected error adding children");

        reqif.add_specification(specification);
        reqif.write_to("libtest.reqif").unwrap();
    }
}
