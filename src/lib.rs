pub mod req_if;

#[cfg(test)]
mod tests {

    use crate::req_if::{
        Children, Object, ReqIf, SpecHierarchy, SpecObjectRequirement, Specification,
        SpecificationRef,
    };
    use chrono::{DateTime, Local, SecondsFormat};
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_serialize() {
        let mut a = ReqIf::new();

        let local: DateTime<Local> = Local::now();

        let now = local.to_rfc3339_opts(SecondsFormat::Millis, false);

        println!("{}", now);

        a.add_requirement(SpecObjectRequirement::new(
            "CAUR-1".to_string(),
            now.clone(),
            "Titulo del requerimiento 1".to_string(),
            "Texto del requerimiento 1.".to_string(),
            &a.core_content.req_if_content.spec_types,
        ));
        a.add_requirement(SpecObjectRequirement::new(
            "CAUR-2".to_string(),
            now.clone(),
            "Titulo del requerimiento 2".to_string(),
            "Texto del requerimiento 2sdlfkj ".to_string(),
            &a.core_content.req_if_content.spec_types,
        ));

        a.add_specification(Specification {
            identifier: "CAUR".to_string(),
            last_change: now.clone(),
            long_name: "Crypto Ar User Requirements".to_string(),
            type_ref: SpecificationRef {
                spec_ref: a.get_module_specification_type().clone(),
            },
            children: Children {
                spec_hierarchy: vec![
                    SpecHierarchy {
                        identifier: "h1".to_string(),
                        last_change: now.clone(),
                        object: Object {
                            object_ref: "CAUR-1".to_string(),
                        },
                    },
                    SpecHierarchy {
                        identifier: "h1".to_string(),
                        last_change: now.clone(),
                        object: Object {
                            object_ref: "CAUR-2".to_string(),
                        },
                    },
                ],
            },
        });

        let yaserde_cfg = yaserde::ser::Config {
            perform_indent: true,
            ..Default::default()
        };

        let s = yaserde::ser::to_string_with_config(&a, &yaserde_cfg)
            .ok()
            .unwrap();
        let mut file = File::create("out.reqif").ok().unwrap();
        let _ = file.write_all(s.as_bytes());
    }
}
