pub mod req_if;

#[cfg(test)]
mod tests {

    use crate::req_if::{Object, ReqIf, SpecHierarchy, SpecObjectRequirement};
    use chrono::{DateTime, Local, SecondsFormat};

    #[test]
    fn test_serialize() {
        let mut reqif = ReqIf::new();

        let local: DateTime<Local> = Local::now();

        let now = local.to_rfc3339_opts(SecondsFormat::Millis, false);

        reqif.add_requirement(SpecObjectRequirement::new(
            "CAUR-1".to_string(),
            now.clone(),
            "Titulo del requerimiento 1".to_string(),
            "Texto del requerimiento 1.".to_string(),
            &reqif.core_content.req_if_content.spec_types,
        ));

        reqif.add_requirement(SpecObjectRequirement::new(
            "CAUR-2".to_string(),
            now.clone(),
            "Titulo del requerimiento 2".to_string(),
            "Texto del requerimiento 2sdlfkj ".to_string(),
            &reqif.core_content.req_if_content.spec_types,
        ));

        let mut specification = reqif.build_module_specification(
            "CAUR".to_string(),
            now.to_string(),
            "Crypto Ar User Requirements".to_string(),
        );

        specification.children.spec_hierarchy.push(SpecHierarchy {
            identifier: "h1".to_string(),
            last_change: now.clone(),
            object: Object {
                object_ref: "CAUR-1".to_string(),
            },
        });

        specification.children.spec_hierarchy.push(SpecHierarchy {
            identifier: "h2".to_string(),
            last_change: now.clone(),
            object: Object {
                object_ref: "CAUR-2".to_string(),
            },
        });

        reqif.add_specification(specification);
        reqif.write_to("libtest.reqif").unwrap();
    }
}
