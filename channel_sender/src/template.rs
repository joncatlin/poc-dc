// use handlebars::Path:: {Relative as HBRelative, Local as HBLocal};


    //************************************************************************
    fn main() {
        let mut hb = handlebars::Handlebars::new();

        // Get the template structure to access the fields within it
        let template1 = hb.get_template("template1").unwrap();
        debug!("Temaple1 has fields {:?}", template1);

        let mut fields = Vec::<String>::new();
        get_template_fields(&template1.elements, &mut fields);
        debug!("ALL FIELDS FOUND: {:?}", fields);
    }


    //************************************************************************
    // Parses a template and returns a list containing the names of all the fields used within it
    pub fn get_template_fields(elements: &Vec<handlebars::template::TemplateElement>, mut fields: &mut Vec<String>) {

        for e in elements {
            get_field_in_template_element(e, &mut fields);        
        }

        // Remove any duplicate element names
        fields.sort_unstable();
        fields.dedup();
    }


    //************************************************************************
    // Scan the TemplateElement struct for fields
    fn get_field_in_template_element(element: &handlebars::template::TemplateElement, mut fields: &mut Vec<String>) {
        match element  {
            handlebars::template::TemplateElement::Expression(exp) => {
                debug!("Expression: {:?}", exp);
                get_fields_in_helper(&exp, &mut fields)
            },
            handlebars::template::TemplateElement::RawString(_s) => (),
            handlebars::template::TemplateElement::HTMLExpression(_html) => (),
            handlebars::template::TemplateElement::HelperBlock(hb) => {
                debug!("HelperBlock: {:?}", hb);
                get_fields_in_helper(&hb, &mut fields)
            },
            handlebars::template::TemplateElement::DecoratorExpression(d) |
            handlebars::template::TemplateElement::DecoratorBlock(d) |
            handlebars::template::TemplateElement::PartialExpression(d) |
            handlebars::template::TemplateElement::PartialBlock(d) => {
                debug!("DecoratorExpression or DecoratorBlock or PartialBlock or PartialExpression: {:?}", d);
                get_fields_in_decorator(&d, &mut fields)
            },
            handlebars::template::TemplateElement::Comment(_c) => (),
        }
    }



    //************************************************************************
    // Scan the HelperTemplate struct for fields
    fn get_fields_in_helper(ht: &handlebars::template::HelperTemplate, mut fields: &mut Vec<String>) {

        get_fields_parameter(&ht.name, &mut fields);
        for param in &ht.params {
            get_fields_parameter(&param, &mut fields)
        }
    }


    //************************************************************************
    // Scan the DecoratorTemplate struct for fields
    fn get_fields_in_decorator(ht: &handlebars::template::DecoratorTemplate, mut fields: &mut Vec<String>) {

        get_fields_parameter(&ht.name, &mut fields);
        for param in &ht.params {
            get_fields_parameter(&param, &mut fields)
        }
    }


    //************************************************************************
    fn get_fields_parameter(p: &handlebars::template::Parameter, mut fields: &mut Vec<String>) {
        match p {
            handlebars::template::Parameter::Name(s) => debug!("Found Name: {}\n", s),
            handlebars::template::Parameter::Path(path) => {
                debug!("Found Path: {:?} - adding it to list of fields\n", path);
                match path {
                    handlebars::Path::Relative(tup1) => {
                        let (_, var_name) = tup1;
                        info!("RELATIVE with name={} tuple={:?}", var_name, tup1);
                        fields.push(var_name.clone());
    //                    get_fields_in_path_seg(path_seg);
                    },
                    handlebars::Path::Local(tup2) => debug!("Found Local: {:?}\n", tup2),                
                }
            },
            handlebars::template::Parameter::Literal(j) => debug!("Found Literal: {:?}", j),
            handlebars::template::Parameter::Subexpression(u) => {
                debug!("Found Subexpression: {:?}", u);
                get_field_in_template_element(&u.element, &mut fields); 
            },
        }
    }


    //************************************************************************
//    fn get_fields_in_path_seg(ps: Vec<()>) {
        // Not implemented yet

        // This is likely needed to remove fields that are nested within a structure
//    }
