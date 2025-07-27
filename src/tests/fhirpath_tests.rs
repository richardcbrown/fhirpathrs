#[cfg(test)]
mod test_official {
  use std::cell::LazyCell;
  use std::collections::HashMap;
  use std::sync::Arc;
  use assert_json_diff::assert_json_eq;
  use serde_json::{json, Value};
  use crate::error::FhirpathError;
  use crate::evaluate::{compile, EvaluateOptions};
  use crate::models::{get_model_details, ModelType};
  const INPUT_MAP: LazyCell<HashMap<&str, Value>> = LazyCell::new(|| {
    HashMap::from([
      (
        "patient-example.xml",
        serde_json::from_str(
          "{\"_birthDate\":{\"extension\":[{\"url\":\"http://hl7.org/fhir/StructureDefinition/patient-birthTime\",\"valueDateTime\":\"1974-12-25T14:35:45-05:00\"}]},\"active\":true,\"address\":[{\"city\":\"PleasantVille\",\"district\":\"Rainbow\",\"line\":[\"534 Erewhon St\"],\"period\":{\"start\":\"1974-12-25\"},\"postalCode\":\"3999\",\"state\":\"Vic\",\"text\":\"534 Erewhon St PeasantVille, Rainbow, Vic  3999\",\"type\":\"both\",\"use\":\"home\"}],\"birthDate\":\"1974-12-25\",\"contact\":[{\"address\":{\"city\":\"PleasantVille\",\"district\":\"Rainbow\",\"line\":[\"534 Erewhon St\"],\"period\":{\"start\":\"1974-12-25\"},\"postalCode\":\"3999\",\"state\":\"Vic\",\"type\":\"both\",\"use\":\"home\"},\"gender\":\"female\",\"name\":{\"_family\":{\"extension\":[{\"url\":\"http://hl7.org/fhir/StructureDefinition/humanname-own-prefix\",\"valueString\":\"VV\"}]},\"family\":\"du Marché\",\"given\":[\"Bénédicte\"]},\"period\":{\"start\":\"2012\"},\"relationship\":[{\"coding\":[{\"code\":\"N\",\"system\":\"http://terminology.hl7.org/CodeSystem/v2-0131\"}]}],\"telecom\":[{\"system\":\"phone\",\"value\":\"+33 (237) 998327\"}]}],\"deceasedBoolean\":false,\"gender\":\"male\",\"id\":\"example\",\"identifier\":[{\"assigner\":{\"display\":\"Acme Healthcare\"},\"period\":{\"start\":\"2001-05-06\"},\"system\":\"urn:oid:1.2.36.146.595.217.0.1\",\"type\":{\"coding\":[{\"code\":\"MR\",\"system\":\"http://terminology.hl7.org/CodeSystem/v2-0203\"}]},\"use\":\"usual\",\"value\":\"12345\"}],\"managingOrganization\":{\"reference\":\"Organization/1\"},\"name\":[{\"family\":\"Chalmers\",\"given\":[\"Peter\",\"James\"],\"use\":\"official\"},{\"given\":[\"Jim\"],\"use\":\"usual\"},{\"family\":\"Windsor\",\"given\":[\"Peter\",\"James\"],\"period\":{\"end\":\"2002\"},\"use\":\"maiden\"}],\"resourceType\":\"Patient\",\"telecom\":[{\"use\":\"home\"},{\"rank\":1,\"system\":\"phone\",\"use\":\"work\",\"value\":\"(03) 5555 6473\"},{\"rank\":2,\"system\":\"phone\",\"use\":\"mobile\",\"value\":\"(03) 3410 5613\"},{\"period\":{\"end\":\"2014\"},\"system\":\"phone\",\"use\":\"old\",\"value\":\"(03) 5555 8834\"}],\"text\":{\"div\":\"<div xmlns='http://www.w3.org/1999/xhtml'>\\n  <table>\\n    <tbody>\\n      <tr>\\n        <td>Name</td>\\n        <td>Peter James<b>Chalmers</b>(\\\"Jim\\\")</td>\\n      </tr>\\n      <tr>\\n        <td>Address</td>\\n        <td>534 Erewhon, Pleasantville, Vic, 3999</td>\\n      </tr>\\n      <tr>\\n        <td>Contacts</td>\\n        <td>Home: unknown. Work: (03) 5555 6473</td>\\n      </tr>\\n      <tr>\\n        <td>Id</td>\\n        <td>MRN: 12345 (Acme Healthcare)</td>\\n      </tr>\\n    </tbody>\\n  </table>\\n</div>\",\"status\":\"generated\"}}",
        )
            .unwrap(),
      ),
      (
        "observation-example.xml",
        serde_json::from_str(
          "{\"category\":[{\"coding\":[{\"code\":\"vital-signs\",\"display\":\"Vital Signs\",\"system\":\"http://terminology.hl7.org/CodeSystem/observation-category\"}]}],\"code\":{\"coding\":[{\"code\":\"29463-7\",\"display\":\"Body Weight\",\"system\":\"http://loinc.org\"},{\"code\":\"3141-9\",\"display\":\"Body weight Measured\",\"system\":\"http://loinc.org\"},{\"code\":\"27113001\",\"display\":\"Body weight\",\"system\":\"http://snomed.info/sct\"},{\"code\":\"body-weight\",\"display\":\"Body Weight\",\"system\":\"http://acme.org/devices/clinical-codes\"}]},\"effectiveDateTime\":\"2016-03-28\",\"encounter\":{\"reference\":\"Encounter/example\"},\"id\":\"example\",\"resourceType\":\"Observation\",\"status\":\"final\",\"subject\":{\"reference\":\"Patient/example\"},\"text\":{\"div\":\"<div xmlns='http://www.w3.org/1999/xhtml'>\\n  <p>\\n    <b>Generated Narrative with Details</b>\\n  </p>\\n  <p>\\n    <b>id</b>: example</p>\\n  <p>\\n    <b>status</b>: final</p>\\n  <p>\\n    <b>category</b>: Vital Signs<span>(Details : {http://terminology.hl7.org/CodeSystem/observation-category code &apos;vital-signs&apos; = &apos;Vital Signs&apos;, given as &apos;Vital Signs&apos;})</span>\\n  </p>\\n  <p>\\n    <b>code</b>: Body Weight<span>(Details : {LOINC code &apos;29463-7&apos; = &apos;Body weight&apos;, given as &apos;Body Weight&apos;}; {LOINC code &apos;3141-9&apos; = &apos;Body weight Measured&apos;, given as &apos;Body weight Measured&apos;}; {SNOMED CT code &apos;27113001&apos; = &apos;Body weight&apos;, given as &apos;Body weight&apos;}; {http://acme.org/devices/clinical-codes code &apos;body-weight&apos; = &apos;body-weight&apos;, given as &apos;Body Weight&apos;})</span>\\n  </p>\\n  <p>\\n    <b>subject</b>:<a>Patient/example</a>\\n  </p>\\n  <p>\\n    <b>encounter</b>:<a>Encounter/example</a>\\n  </p>\\n  <p>\\n    <b>effective</b>: 28/03/2016</p>\\n  <p>\\n    <b>value</b>: 185 lbs<span>(Details: UCUM code [lb_av] = &apos;lb_av&apos;)</span>\\n  </p>\\n</div>\",\"status\":\"generated\"},\"valueQuantity\":{\"code\":\"[lb_av]\",\"system\":\"http://unitsofmeasure.org\",\"unit\":\"lbs\",\"value\":185}}",
        )
            .unwrap(),
      ),
      (
        "questionnaire-example.xml",
        serde_json::from_str(
          "{\"date\":\"2012-01\",\"id\":\"3141\",\"item\":[{\"code\":[{\"code\":\"COMORBIDITY\",\"system\":\"http://example.org/system/code/sections\"}],\"item\":[{\"answerValueSet\":\"http://hl7.org/fhir/ValueSet/yesnodontknow\",\"code\":[{\"code\":\"COMORB\",\"system\":\"http://example.org/system/code/questions\"}],\"item\":[{\"code\":[{\"code\":\"CARDIAL\",\"system\":\"http://example.org/system/code/sections\"}],\"enableWhen\":{\"answerCoding\":{\"code\":\"Y\",\"system\":\"http://terminology.hl7.org/CodeSystem/v2-0136\"},\"operator\":\"=\",\"question\":\"1.1\"},\"item\":[{\"answerValueSet\":\"http://hl7.org/fhir/ValueSet/yesnodontknow\",\"code\":[{\"code\":\"COMORBCAR\",\"system\":\"http://example.org/system/code/questions\"}],\"item\":[{\"answerValueSet\":\"http://hl7.org/fhir/ValueSet/yesnodontknow\",\"code\":[{\"code\":\"COMCAR00\",\"display\":\"Angina Pectoris\",\"system\":\"http://example.org/system/code/questions\"},{\"code\":\"194828000\",\"display\":\"Angina (disorder)\",\"system\":\"http://snomed.info/sct\"}],\"linkId\":\"1.1.1.1.1\",\"prefix\":\"1.1.1\",\"type\":\"choice\"},{\"answerValueSet\":\"http://hl7.org/fhir/ValueSet/yesnodontknow\",\"code\":[{\"code\":\"22298006\",\"display\":\"Myocardial infarction (disorder)\",\"system\":\"http://snomed.info/sct\"}],\"linkId\":\"1.1.1.1.2\",\"prefix\":\"1.1.2\",\"type\":\"choice\"}],\"linkId\":\"1.1.1.1\",\"prefix\":\"1.1\",\"type\":\"choice\"},{\"answerValueSet\":\"http://hl7.org/fhir/ValueSet/yesnodontknow\",\"code\":[{\"code\":\"COMORBVAS\",\"system\":\"http://example.org/system/code/questions\"}],\"linkId\":\"1.1.1.2\",\"prefix\":\"1.2\",\"type\":\"choice\"}],\"linkId\":\"1.1.1\",\"type\":\"group\"}],\"linkId\":\"1.1\",\"prefix\":\"1\",\"type\":\"choice\"}],\"linkId\":\"1\",\"type\":\"group\"},{\"code\":[{\"code\":\"HISTOPATHOLOGY\",\"system\":\"http://example.org/system/code/sections\"}],\"item\":[{\"code\":[{\"code\":\"ABDOMINAL\",\"system\":\"http://example.org/system/code/sections\"}],\"item\":[{\"code\":[{\"code\":\"STADPT\",\"display\":\"pT category\",\"system\":\"http://example.org/system/code/questions\"}],\"linkId\":\"2.1.2\",\"type\":\"choice\"}],\"linkId\":\"2.1\",\"type\":\"group\"}],\"linkId\":\"2\",\"type\":\"group\"}],\"resourceType\":\"Questionnaire\",\"status\":\"draft\",\"subjectType\":[\"Patient\"],\"text\":{\"div\":\"\",\"status\":\"generated\"},\"title\":\"Cancer Quality Forum Questionnaire 2012\",\"url\":\"http://hl7.org/fhir/Questionnaire/3141\"}",
        )
            .unwrap(),
      ),
      (
        "valueset-example-expansion.xml",
        serde_json::from_str(
          "{\"compose\":{\"include\":[{\"filter\":[{\"op\":\"=\",\"property\":\"parent\",\"value\":\"LP43571-6\"}],\"system\":\"http://loinc.org\"}]},\"contact\":[{\"telecom\":[{\"system\":\"url\",\"value\":\"http://hl7.org/fhir\"}]}],\"copyright\":\"This content from LOINC® is copyright © 1995 Regenstrief Institute, Inc. and the LOINC Committee, and available at no cost under the license at http://loinc.org/terms-of-use.\",\"date\":\"2015-06-22\",\"description\":\"This is an example value set that includes all the LOINC codes for serum/plasma cholesterol from v2.36.\",\"expansion\":{\"contains\":[{\"code\":\"14647-2\",\"display\":\"Cholesterol [Moles/volume] in Serum or Plasma\",\"system\":\"http://loinc.org\",\"version\":\"2.50\"},{\"abstract\":true,\"contains\":[{\"code\":\"2093-3\",\"display\":\"Cholesterol [Mass/volume] in Serum or Plasma\",\"system\":\"http://loinc.org\",\"version\":\"2.50\"},{\"code\":\"48620-9\",\"display\":\"Cholesterol [Mass/volume] in Serum or Plasma ultracentrifugate\",\"system\":\"http://loinc.org\",\"version\":\"2.50\"},{\"code\":\"9342-7\",\"display\":\"Cholesterol [Percentile]\",\"system\":\"http://loinc.org\",\"version\":\"2.50\"}],\"display\":\"Cholesterol codes\"},{\"abstract\":true,\"contains\":[{\"code\":\"2096-6\",\"display\":\"Cholesterol/Triglyceride [Mass Ratio] in Serum or Plasma\",\"system\":\"http://loinc.org\",\"version\":\"2.50\"},{\"code\":\"35200-5\",\"display\":\"Cholesterol/Triglyceride [Mass Ratio] in Serum or Plasma\",\"system\":\"http://loinc.org\",\"version\":\"2.50\"},{\"code\":\"48089-7\",\"display\":\"Cholesterol/Apolipoprotein B [Molar ratio] in Serum or Plasma\",\"system\":\"http://loinc.org\",\"version\":\"2.50\"},{\"code\":\"55838-7\",\"display\":\"Cholesterol/Phospholipid [Molar ratio] in Serum or Plasma\",\"system\":\"http://loinc.org\",\"version\":\"2.50\"}],\"display\":\"Cholesterol Ratios\"}],\"extension\":[{\"url\":\"http://hl7.org/fhir/StructureDefinition/valueset-expansionSource\",\"valueUri\":\"http://hl7.org/fhir/ValueSet/example-extensional\"}],\"identifier\":\"urn:uuid:42316ff8-2714-4680-9980-f37a6d1a71bc\",\"offset\":0,\"parameter\":[{\"name\":\"version\",\"valueString\":\"2.50\"}],\"timestamp\":\"2015-06-22T13:56:07Z\",\"total\":8},\"experimental\":true,\"id\":\"example-expansion\",\"meta\":{\"profile\":[\"http://hl7.org/fhir/StructureDefinition/shareablevalueset\"]},\"name\":\"LOINC Codes for Cholesterol in Serum/Plasma\",\"publisher\":\"FHIR Project team\",\"resourceType\":\"ValueSet\",\"status\":\"draft\",\"text\":{\"div\":\"<div xmlns=\\\"http://www.w3.org/1999/xhtml\\\"><table class=\\\"grid\\\"><tr><td>http://loinc.org</td><td>14647-2</td><td>Cholesterol [Moles/volume] in Serum or Plasma</td></tr><tr><td colspan=\\\"3\\\"><b>Additional Cholesterol codes</b>\\t</td></tr><tr><td>http://loinc.org</td><td>2093-3</td><td>Cholesterol [Mass/volume] in Serum or Plasma</td></tr><tr><td>http://loinc.org</td><td>48620-9</td><td>Cholesterol [Mass/volume] in Serum or Plasma ultracentrifugate</td></tr><tr>\\t<td>http://loinc.org</td>\\t<td>9342-7</td>\\t<td>Cholesterol [Percentile]</td></tr><tr>\\t<td colspan=\\\"3\\\">\\t\\t<b>Cholesterol Ratios</b>\\t</td></tr><tr>\\t<td>http://loinc.org</td>\\t<td>2096-6</td>\\t<td>Cholesterol/Triglyceride [Mass Ratio] in Serum or Plasma</td></tr><tr>\\t<td>http://loinc.org</td>\\t<td>35200-5</td>\\t<td>Cholesterol/Triglyceride [Mass Ratio] in Serum or Plasma</td></tr><tr>\\t<td>http://loinc.org</td><td>48089-7</td><td>Cholesterol/Apolipoprotein B [Molar ratio] in Serum or Plasma</td></tr><tr><td>http://loinc.org</td><td>55838-7</td><td>Cholesterol/Phospholipid [Molar ratio] in Serum or Plasma</td></tr></table></div>\",\"status\":\"generated\"},\"url\":\"http://hl7.org/fhir/ValueSet/example-expansion\",\"version\":\"20150622\"}",
        )
            .unwrap(),
      ),
    ])
  });
  #[test]
  fn test_miscellaneous_accessor_tests_test_extract_birth_date_1() {
    let compiled = compile(&"birthDate".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[\"1974-12-25\"]")
            .unwrap()
        )
  }
  #[test]
  fn test_miscellaneous_accessor_tests_test_patient_has_birth_date_2() {
    let compiled = compile(&"birthDate".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_miscellaneous_accessor_tests_test_patient_telecom_types_3() {
    let compiled = compile(&"telecom.use".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value >
            ("[\"home\",\"work\",\"mobile\",\"old\"]").unwrap()
        )
  }
  #[test]
  fn test_basics_test_simple_4() {
    let compiled = compile(&"name.given".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value >
            ("[\"Peter\",\"James\",\"Jim\",\"Peter\",\"James\"]").unwrap()
        )
  }
  #[test]
  fn test_basics_test_simple_none_5() {
    let compiled = compile(&"name.suffix".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_basics_test_escaped_identifier_6() {
    let compiled = compile(&"name.`given`".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value >
            ("[\"Peter\",\"James\",\"Jim\",\"Peter\",\"James\"]").unwrap()
        )
  }
  #[test]
  fn test_basics_test_simple_back_tick_1_7() {
    let compiled = compile(&"`Patient`.name.`given`".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value >
            ("[\"Peter\",\"James\",\"Jim\",\"Peter\",\"James\"]").unwrap()
        )
  }
  #[test]
  fn test_basics_test_simple_fail_8() {
    let compiled = compile(&"name.given1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    let err = evaluate_result.err().unwrap();
    match err {
      FhirpathError::EvaluateError { msg } => panic!(),
      _ => {}
    }
  }
  #[test]
  fn test_basics_test_simple_with_context_9() {
    let compiled = compile(&"Patient.name.given".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value >
            ("[\"Peter\",\"James\",\"Jim\",\"Peter\",\"James\"]").unwrap()
        )
  }
  #[test]
  fn test_basics_test_simple_with_wrong_context_10() {
    let compiled = compile(&"Encounter.name.given".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    let err = evaluate_result.err().unwrap();
    match err {
      FhirpathError::EvaluateError { msg } => panic!(),
      _ => {}
    }
  }
  #[test]
  fn test_observations_test_polymorphism_a_11() {
    let compiled = compile(&"Observation.value.unit".to_string()).unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[\"lbs\"]").unwrap()
        )
  }
  #[test]
  fn test_observations_test_polymorphism_b_12() {
    let compiled = compile(&"Observation.valueQuantity.unit".to_string()).unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    let err = evaluate_result.err().unwrap();
    match err {
      FhirpathError::EvaluateError { msg } => panic!(),
      _ => {}
    }
  }
  #[test]
  fn test_observations_test_polymorphism_is_a_13() {
    let compiled = compile(&"Observation.value.is(Quantity)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_observations_test_polymorphism_is_a_14() {
    let compiled = compile(&"Observation.value is Quantity".to_string()).unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_observations_test_polymorphism_is_b_15() {
    let compiled = compile(&"Observation.value.is(Period).not()".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_observations_test_polymorphism_as_a_16() {
    let compiled = compile(&"Observation.value.as(Quantity).unit".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[\"lbs\"]").unwrap()
        )
  }
  #[test]
  fn test_observations_test_polymorphism_as_a_function_17() {
    let compiled = compile(&"(Observation.value as Quantity).unit".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[\"lbs\"]").unwrap()
        )
  }
  #[test]
  fn test_observations_test_polymorphism_as_b_18() {
    let compiled = compile(&"(Observation.value as Period).unit".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    let err = evaluate_result.err().unwrap();
    match err {
      FhirpathError::EvaluateError { msg } => panic!(),
      _ => {}
    }
  }
  #[test]
  fn test_observations_test_polymorphism_as_b_function_19() {
    let compiled = compile(&"Observation.value.as(Period).start".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_dollar_test_dollar_this_1_20() {
    let compiled = compile(
      &"Patient.name.given.where(substring($this.length()-3) = 'out')"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_dollar_test_dollar_this_2_21() {
    let compiled = compile(
      &"Patient.name.given.where(substring($this.length()-3) = 'ter')"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value >
            ("[\"Peter\",\"Peter\"]").unwrap()
        )
  }
  #[test]
  fn test_dollar_test_dollar_order_allowed_22() {
    let compiled = compile(&"Patient.name.skip(1).given".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value >
            ("[\"Jim\",\"Peter\",\"James\"]").unwrap()
        )
  }
  #[test]
  fn test_dollar_test_dollar_order_allowed_a_23() {
    let compiled = compile(&"Patient.name.skip(3).given".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_dollar_test_dollar_order_not_allowed_24() {
    let compiled = compile(&"Patient.children().skip(1)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    let err = evaluate_result.err().unwrap();
    match err {
      FhirpathError::EvaluateError { msg } => panic!(),
      _ => {}
    }
  }
  #[test]
  fn test_literals_test_literal_true_25() {
    let compiled = compile(&"Patient.name.exists() = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_false_26() {
    let compiled = compile(&"Patient.name.empty() = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_string_27() {
    let compiled = compile(&"Patient.name.given.first() = 'Peter'".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_integer_1_28() {
    let compiled = compile(&"1.convertsToInteger()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_integer_0_29() {
    let compiled = compile(&"0.convertsToInteger()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_integer_negative_1_30() {
    let compiled = compile(&"(-1).convertsToInteger()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_integer_negative_1_invalid_31() {
    let compiled = compile(&"-1.convertsToInteger()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    let err = evaluate_result.err().unwrap();
    match err {
      FhirpathError::CompileError { msg } => panic!(),
      _ => {}
    }
  }
  #[test]
  fn test_literals_test_literal_integer_max_32() {
    let compiled = compile(&"2147483647.convertsToInteger()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_string_33() {
    let compiled = compile(&"'test'.convertsToString()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_string_escapes_34() {
    let compiled = compile(
      &"'\\\\\\/\\f\\r\\n\\t\\\"\\`\\'\\u002a'.convertsToString()".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_boolean_true_35() {
    let compiled = compile(&"true.convertsToBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_boolean_false_36() {
    let compiled = compile(&"false.convertsToBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_decimal_10_37() {
    let compiled = compile(&"1.0.convertsToDecimal()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_decimal_01_38() {
    let compiled = compile(&"0.1.convertsToDecimal()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_decimal_00_39() {
    let compiled = compile(&"0.0.convertsToDecimal()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_decimal_negative_01_40() {
    let compiled = compile(&"(-0.1).convertsToDecimal()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_decimal_negative_01_invalid_41() {
    let compiled = compile(&"-0.1.convertsToDecimal()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    let err = evaluate_result.err().unwrap();
    match err {
      FhirpathError::CompileError { msg } => panic!(),
      _ => {}
    }
  }
  #[test]
  fn test_literals_test_literal_decimal_max_42() {
    let compiled = compile(&"1234567890987654321.0.convertsToDecimal()".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_decimal_step_43() {
    let compiled = compile(&"0.00000001.convertsToDecimal()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_year_44() {
    let compiled = compile(&"@2015.is(Date)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_month_45() {
    let compiled = compile(&"@2015-02.is(Date)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_day_46() {
    let compiled = compile(&"@2015-02-04.is(Date)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_time_year_47() {
    let compiled = compile(&"@2015T.is(DateTime)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_time_month_48() {
    let compiled = compile(&"@2015-02T.is(DateTime)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_time_day_49() {
    let compiled = compile(&"@2015-02-04T.is(DateTime)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_time_hour_50() {
    let compiled = compile(&"@2015-02-04T14.is(DateTime)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_time_minute_51() {
    let compiled = compile(&"@2015-02-04T14:34.is(DateTime)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_time_second_52() {
    let compiled = compile(&"@2015-02-04T14:34:28.is(DateTime)".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_time_millisecond_53() {
    let compiled = compile(&"@2015-02-04T14:34:28.123.is(DateTime)".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_time_utc_54() {
    let compiled = compile(&"@2015-02-04T14:34:28Z.is(DateTime)".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_time_timezone_offset_55() {
    let compiled = compile(&"@2015-02-04T14:34:28+10:00.is(DateTime)".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_time_hour_56() {
    let compiled = compile(&"@T14.is(Time)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_time_minute_57() {
    let compiled = compile(&"@T14:34.is(Time)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_time_second_58() {
    let compiled = compile(&"@T14:34:28.is(Time)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_time_millisecond_59() {
    let compiled = compile(&"@T14:34:28.123.is(Time)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_time_utc_60() {
    let compiled = compile(&"@T14:34:28Z.is(Time)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_time_timezone_offset_61() {
    let compiled = compile(&"@T14:34:28+10:00.is(Time)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_quantity_decimal_62() {
    let compiled = compile(&"10.1 'mg'.convertsToQuantity()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_quantity_integer_63() {
    let compiled = compile(&"10 'mg'.convertsToQuantity()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_quantity_day_64() {
    let compiled = compile(&"4 days.convertsToQuantity()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_integer_not_equal_65() {
    let compiled = compile(&"-3 != 3".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_integer_equal_66() {
    let compiled = compile(&"Patient.name.given.count() = 5".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_polarity_precedence_67() {
    let compiled = compile(&"-Patient.name.given.count() = -5".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_integer_greater_than_68() {
    let compiled = compile(&"Patient.name.given.count() > -3".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_integer_count_not_equal_69() {
    let compiled = compile(&"Patient.name.given.count() != 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_integer_less_than_true_70() {
    let compiled = compile(&"1 < 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_integer_less_than_false_71() {
    let compiled = compile(&"1 < -2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_integer_less_than_polarity_true_72() {
    let compiled = compile(&"+1 < +2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_integer_less_than_polarity_false_73() {
    let compiled = compile(&"-1 < 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_decimal_greater_than_non_zero_true_74() {
    let compiled = compile(&"Observation.value.value > 180.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_decimal_greater_than_zero_true_75() {
    let compiled = compile(&"Observation.value.value > 0.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_decimal_greater_than_integer_true_76() {
    let compiled = compile(&"Observation.value.value > 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_decimal_less_than_integer_77() {
    let compiled = compile(&"Observation.value.value < 190".to_string()).unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_decimal_less_than_invalid_78() {
    let compiled = compile(&"Observation.value.value < 'test'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    let err = evaluate_result.err().unwrap();
    match err {
      FhirpathError::EvaluateError { msg } => panic!(),
      _ => {}
    }
  }
  #[test]
  fn test_literals_test_date_equal_79() {
    let compiled = compile(&"Patient.birthDate = @1974-12-25".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_date_not_equal_80() {
    let compiled = compile(&"Patient.birthDate != @1974-12-25T12:34:00".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_date_not_equal_timezone_offset_before_81() {
    let compiled = compile(
      &"Patient.birthDate != @1974-12-25T12:34:00-10:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_date_not_equal_timezone_offset_after_82() {
    let compiled = compile(
      &"Patient.birthDate != @1974-12-25T12:34:00+10:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_date_not_equal_utc_83() {
    let compiled = compile(&"Patient.birthDate != @1974-12-25T12:34:00Z".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_date_not_equal_time_second_84() {
    let compiled = compile(&"Patient.birthDate != @T12:14:15".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_date_not_equal_time_minute_85() {
    let compiled = compile(&"Patient.birthDate != @T12:14".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_date_not_equal_today_86() {
    let compiled = compile(&"Patient.birthDate < today()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_date_time_greater_than_date_87() {
    let compiled = compile(&"now() > Patient.birthDate".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_time_tz_greater_88() {
    let compiled = compile(
      &"@2017-11-05T01:30:00.0-04:00 > @2017-11-05T01:15:00.0-05:00"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_time_tz_less_89() {
    let compiled = compile(
      &"@2017-11-05T01:30:00.0-04:00 < @2017-11-05T01:15:00.0-05:00"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_time_tz_equal_false_90() {
    let compiled = compile(
      &"@2017-11-05T01:30:00.0-04:00 = @2017-11-05T01:15:00.0-05:00"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_date_time_tz_equal_true_91() {
    let compiled = compile(
      &"@2017-11-05T01:30:00.0-04:00 = @2017-11-05T00:30:00.0-05:00"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_unicode_92() {
    let compiled = compile(&"Patient.name.given.first() = 'P\\u0065ter'".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_collection_not_empty_93() {
    let compiled = compile(&"Patient.name.given.empty().not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_collection_not_equal_empty_94() {
    let compiled = compile(&"Patient.name.given != {}".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_expressions_95() {
    let compiled = compile(
      &"Patient.name.select(given | family).distinct()".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value >
            ("[\"Peter\",\"James\",\"Chalmers\",\"Jim\",\"Windsor\"]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_expressions_equal_96() {
    let compiled = compile(&"Patient.name.given.count() = 1 + 4".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_not_empty_97() {
    let compiled = compile(&"Patient.name.empty().not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_empty_98() {
    let compiled = compile(&"Patient.link.empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_not_true_99() {
    let compiled = compile(&"true.not() = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_literal_not_false_100() {
    let compiled = compile(&"false.not() = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_integer_boolean_not_true_101() {
    let compiled = compile(&"(0).not() = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_integer_boolean_not_false_102() {
    let compiled = compile(&"(1).not() = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_literals_test_not_invalid_103() {
    let compiled = compile(&"(1|2).not() = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    let err = evaluate_result.err().unwrap();
    match err {
      FhirpathError::EvaluateError { msg } => panic!(),
      _ => {}
    }
  }
  #[test]
  fn test_types_test_string_year_converts_to_date_104() {
    let compiled = compile(&"'2015'.convertsToDate()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_month_converts_to_date_105() {
    let compiled = compile(&"'2015-02'.convertsToDate()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_day_converts_to_date_106() {
    let compiled = compile(&"'2015-02-04'.convertsToDate()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_year_converts_to_date_time_107() {
    let compiled = compile(&"'2015'.convertsToDateTime()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_month_converts_to_date_time_108() {
    let compiled = compile(&"'2015-02'.convertsToDateTime()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_day_converts_to_date_time_109() {
    let compiled = compile(&"'2015-02-04'.convertsToDateTime()".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_hour_converts_to_date_time_110() {
    let compiled = compile(&"'2015-02-04T14'.convertsToDateTime()".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_minute_converts_to_date_time_111() {
    let compiled = compile(&"'2015-02-04T14:34'.convertsToDateTime()".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_second_converts_to_date_time_112() {
    let compiled = compile(&"'2015-02-04T14:34:28'.convertsToDateTime()".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_millisecond_converts_to_date_time_113() {
    let compiled = compile(
      &"'2015-02-04T14:34:28.123'.convertsToDateTime()".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_utc_converts_to_date_time_114() {
    let compiled = compile(
      &"'2015-02-04T14:34:28Z'.convertsToDateTime()".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_tz_converts_to_date_time_115() {
    let compiled = compile(
      &"'2015-02-04T14:34:28+10:00'.convertsToDateTime()".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_hour_converts_to_time_116() {
    let compiled = compile(&"'14'.convertsToTime()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_minute_converts_to_time_117() {
    let compiled = compile(&"'14:34'.convertsToTime()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_second_converts_to_time_118() {
    let compiled = compile(&"'14:34:28'.convertsToTime()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_millisecond_converts_to_time_119() {
    let compiled = compile(&"'14:34:28.123'.convertsToTime()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_converts_to_integer_120() {
    let compiled = compile(&"1.convertsToInteger()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_is_integer_121() {
    let compiled = compile(&"1.is(Integer)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_is_system_integer_122() {
    let compiled = compile(&"1.is(System.Integer)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_literal_converts_to_integer_123() {
    let compiled = compile(&"'1'.convertsToInteger()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_literal_converts_to_integer_false_124() {
    let compiled = compile(&"'a'.convertsToInteger().not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_decimal_converts_to_integer_false_125() {
    let compiled = compile(&"'1.0'.convertsToInteger().not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_literal_is_not_integer_126() {
    let compiled = compile(&"'1'.is(Integer).not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_boolean_literal_converts_to_integer_127() {
    let compiled = compile(&"true.convertsToInteger()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_boolean_literal_is_not_integer_128() {
    let compiled = compile(&"true.is(Integer).not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_date_is_not_integer_129() {
    let compiled = compile(&"@2013-04-05.is(Integer).not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_to_integer_130() {
    let compiled = compile(&"1.toInteger() = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_integer_literal_to_integer_131() {
    let compiled = compile(&"'1'.toInteger() = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_decimal_literal_to_integer_132() {
    let compiled = compile(&"'1.1'.toInteger() = {}".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_types_test_decimal_literal_to_integer_is_empty_133() {
    let compiled = compile(&"'1.1'.toInteger().empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_boolean_literal_to_integer_134() {
    let compiled = compile(&"true.toInteger() = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_converts_to_decimal_135() {
    let compiled = compile(&"1.convertsToDecimal()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_is_not_decimal_136() {
    let compiled = compile(&"1.is(Decimal).not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_decimal_literal_converts_to_decimal_137() {
    let compiled = compile(&"1.0.convertsToDecimal()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_decimal_literal_is_decimal_138() {
    let compiled = compile(&"1.0.is(Decimal)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_integer_literal_converts_to_decimal_139() {
    let compiled = compile(&"'1'.convertsToDecimal()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_integer_literal_is_not_decimal_140() {
    let compiled = compile(&"'1'.is(Decimal).not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_literal_converts_to_decimal_false_141() {
    let compiled = compile(&"'1.a'.convertsToDecimal().not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_decimal_literal_converts_to_decimal_142() {
    let compiled = compile(&"'1.0'.convertsToDecimal()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_decimal_literal_is_not_decimal_143() {
    let compiled = compile(&"'1.0'.is(Decimal).not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_boolean_literal_converts_to_decimal_144() {
    let compiled = compile(&"true.convertsToDecimal()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_boolean_literal_is_not_decimal_145() {
    let compiled = compile(&"true.is(Decimal).not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_to_decimal_146() {
    let compiled = compile(&"1.toDecimal() = 1.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_to_deciaml_equivalent_147() {
    let compiled = compile(&"1.toDecimal() ~ 1.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_decimal_literal_to_decimal_148() {
    let compiled = compile(&"1.0.toDecimal() = 1.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_decimal_literal_to_decimal_equal_149() {
    let compiled = compile(&"'1.1'.toDecimal() = 1.1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_boolean_literal_to_decimal_150() {
    let compiled = compile(&"true.toDecimal() = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_converts_to_quantity_151() {
    let compiled = compile(&"1.convertsToQuantity()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_is_not_quantity_152() {
    let compiled = compile(&"1.is(Quantity).not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_decimal_literal_converts_to_quantity_153() {
    let compiled = compile(&"1.0.convertsToQuantity()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_decimal_literal_is_not_quantity_154() {
    let compiled = compile(&"1.0.is(System.Quantity).not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_integer_literal_converts_to_quantity_155() {
    let compiled = compile(&"'1'.convertsToQuantity()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_integer_literal_is_not_quantity_156() {
    let compiled = compile(&"'1'.is(System.Quantity).not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_quantity_literal_converts_to_quantity_157() {
    let compiled = compile(&"'1 day'.convertsToQuantity()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_quantity_week_converts_to_quantity_158() {
    let compiled = compile(&"'1 \\'wk\\''.convertsToQuantity()".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_quantity_week_converts_to_quantity_false_159() {
    let compiled = compile(&"'1 wk'.convertsToQuantity().not()".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_decimal_literal_converts_to_quantity_false_160() {
    let compiled = compile(&"'1.a'.convertsToQuantity().not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_decimal_literal_converts_to_quantity_161() {
    let compiled = compile(&"'1.0'.convertsToQuantity()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_decimal_literal_is_not_system_quantity_162() {
    let compiled = compile(&"'1.0'.is(System.Quantity).not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_boolean_literal_converts_to_quantity_163() {
    let compiled = compile(&"true.convertsToQuantity()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_boolean_literal_is_not_system_quantity_164() {
    let compiled = compile(&"true.is(System.Quantity).not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_to_quantity_165() {
    let compiled = compile(&"1.toQuantity() = 1 '1'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_decimal_literal_to_quantity_166() {
    let compiled = compile(&"1.0.toQuantity() = 1.0 '1'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_integer_literal_to_quantity_167() {
    let compiled = compile(&"'1'.toQuantity()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[\"1 '1'\"]")
            .unwrap()
        )
  }
  #[test]
  fn test_types_test_string_quantity_literal_to_quantity_168() {
    let compiled = compile(&"'1 day'.toQuantity() = 1 day".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_quantity_day_literal_to_quantity_169() {
    let compiled = compile(&"'1 day'.toQuantity() = 1 '{day}'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_quantity_week_literal_to_quantity_170() {
    let compiled = compile(&"'1 \\'wk\\''.toQuantity() = 1 'wk'".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_decimal_literal_to_quantity_171() {
    let compiled = compile(&"'1.0'.toQuantity() ~ 1 '1'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_converts_to_boolean_172() {
    let compiled = compile(&"1.convertsToBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_converts_to_boolean_false_173() {
    let compiled = compile(&"2.convertsToBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_types_test_negative_integer_literal_converts_to_boolean_false_174() {
    let compiled = compile(&"(-1).convertsToBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_false_converts_to_boolean_175() {
    let compiled = compile(&"0.convertsToBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_decimal_literal_converts_to_boolean_176() {
    let compiled = compile(&"1.0.convertsToBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_true_literal_converts_to_boolean_177() {
    let compiled = compile(&"'true'.convertsToBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_false_literal_converts_to_boolean_178() {
    let compiled = compile(&"'false'.convertsToBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_false_literal_also_converts_to_boolean_179() {
    let compiled = compile(&"'False'.convertsToBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_true_literal_converts_to_boolean_180() {
    let compiled = compile(&"true.convertsToBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_false_literal_converts_to_boolean_181() {
    let compiled = compile(&"false.convertsToBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_to_boolean_182() {
    let compiled = compile(&"1.toBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_to_boolean_empty_183() {
    let compiled = compile(&"2.toBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_to_boolean_false_184() {
    let compiled = compile(&"0.toBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_true_to_boolean_185() {
    let compiled = compile(&"'true'.toBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_false_to_boolean_186() {
    let compiled = compile(&"'false'.toBoolean()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_converts_to_string_187() {
    let compiled = compile(&"1.convertsToString()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_is_not_string_188() {
    let compiled = compile(&"1.is(String).not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_negative_integer_literal_converts_to_string_189() {
    let compiled = compile(&"(-1).convertsToString()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_decimal_literal_converts_to_string_190() {
    let compiled = compile(&"1.0.convertsToString()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_literal_converts_to_string_191() {
    let compiled = compile(&"'true'.convertsToString()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_boolean_literal_converts_to_string_192() {
    let compiled = compile(&"true.convertsToString()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_quantity_literal_converts_to_string_193() {
    let compiled = compile(&"1 'wk'.convertsToString()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_types_test_integer_literal_to_string_194() {
    let compiled = compile(&"1.toString()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[\"1\"]").unwrap()
        )
  }
  #[test]
  fn test_types_test_negative_integer_literal_to_string_195() {
    let compiled = compile(&"(-1).toString()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[\"-1\"]").unwrap()
        )
  }
  #[test]
  fn test_types_test_decimal_literal_to_string_196() {
    let compiled = compile(&"1.0.toString()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[\"1.0\"]").unwrap()
        )
  }
  #[test]
  fn test_types_test_string_literal_to_string_197() {
    let compiled = compile(&"'true'.toString()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[\"true\"]")
            .unwrap()
        )
  }
  #[test]
  fn test_types_test_boolean_literal_to_string_198() {
    let compiled = compile(&"true.toString()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[\"true\"]")
            .unwrap()
        )
  }
  #[test]
  fn test_types_test_quantity_literal_wk_to_string_199() {
    let compiled = compile(&"1 'wk'.toString()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[\"1 'wk'\"]")
            .unwrap()
        )
  }
  #[test]
  fn test_types_test_quantity_literal_week_to_string_200() {
    let compiled = compile(&"1 week.toString()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[\"1 '{week}'\"]")
            .unwrap()
        )
  }
  #[test]
  fn test_all_test_all_true_1_201() {
    let compiled = compile(
      &"Patient.name.select(given.exists()).allTrue()".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_all_test_all_true_2_202() {
    let compiled = compile(
      &"Patient.name.select(period.exists()).allTrue()".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_all_test_all_true_3_203() {
    let compiled = compile(&"Patient.name.all(given.exists())".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_all_test_all_true_4_204() {
    let compiled = compile(&"Patient.name.all(period.exists())".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_sub_set_of_test_sub_set_of_1_205() {
    let compiled = compile(&"Patient.name.first().subsetOf($this.name)".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_sub_set_of_test_sub_set_of_2_206() {
    let compiled = compile(
      &"Patient.name.subsetOf($this.name.first()).not()".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_super_set_of_test_super_set_of_1_207() {
    let compiled = compile(
      &"Patient.name.first().supersetOf($this.name).not()".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_super_set_of_test_super_set_of_2_208() {
    let compiled = compile(
      &"Patient.name.supersetOf($this.name.first())".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_quantity_test_quantity_1_209() {
    let compiled = compile(&"4.0000 'g' = 4000.0 'mg'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_quantity_test_quantity_2_210() {
    let compiled = compile(&"4 'g' ~ 4000 'mg'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_quantity_test_quantity_3_211() {
    let compiled = compile(&"4 'g' != 4040 'mg'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_quantity_test_quantity_4_212() {
    let compiled = compile(&"4 'g' ~ 4040 'mg'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_quantity_test_quantity_5_213() {
    let compiled = compile(&"7 days = 1 week".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_quantity_test_quantity_6_214() {
    let compiled = compile(&"7 days = 1 'wk'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_quantity_test_quantity_7_215() {
    let compiled = compile(&"6 days < 1 week".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_quantity_test_quantity_8_216() {
    let compiled = compile(&"8 days > 1 week".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_quantity_test_quantity_9_217() {
    let compiled = compile(&"2.0 'cm' * 2.0 'm' = 0.040 'm2'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_quantity_test_quantity_10_218() {
    let compiled = compile(&"4.0 'g' / 2.0 'm' = 2 'g/m'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_quantity_test_quantity_11_219() {
    let compiled = compile(&"1.0 'm' / 1.0 'm' = 1 '1'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_collection_boolean_test_collection_boolean_1_220() {
    let compiled = compile(&"iif(1 | 2 | 3, true, false)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    let err = evaluate_result.err().unwrap();
    match err {
      FhirpathError::EvaluateError { msg } => panic!(),
      _ => {}
    }
  }
  #[test]
  fn test_collection_boolean_test_collection_boolean_2_221() {
    let compiled = compile(&"iif({}, true, false)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_collection_boolean_test_collection_boolean_3_222() {
    let compiled = compile(&"iif(true, true, false)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_collection_boolean_test_collection_boolean_4_223() {
    let compiled = compile(&"iif({} | true, true, false)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_collection_boolean_test_collection_boolean_5_224() {
    let compiled = compile(&"iif(true, true, 1/0)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_collection_boolean_test_collection_boolean_6_225() {
    let compiled = compile(&"iif(false, 1/0, true)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_distinct_test_distinct_1_226() {
    let compiled = compile(&"(1 | 2 | 3).isDistinct()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_distinct_test_distinct_2_227() {
    let compiled = compile(
      &"Questionnaire.descendants().linkId.isDistinct()".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("questionnaire-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_distinct_test_distinct_3_228() {
    let compiled = compile(
      &"Questionnaire.descendants().linkId.select(substring(0,1)).isDistinct().not()"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("questionnaire-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_distinct_test_distinct_4_229() {
    let compiled = compile(&"(1 | 2 | 3).distinct()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[1,2,3]").unwrap()
        )
  }
  #[test]
  fn test_distinct_test_distinct_5_230() {
    let compiled = compile(
      &"Questionnaire.descendants().linkId.distinct().count()".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("questionnaire-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[10]").unwrap()
        )
  }
  #[test]
  fn test_distinct_test_distinct_6_231() {
    let compiled = compile(
      &"Questionnaire.descendants().linkId.select(substring(0,1)).distinct().count()"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("questionnaire-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[2]").unwrap()
        )
  }
  #[test]
  fn test_count_test_count_1_232() {
    let compiled = compile(&"Patient.name.count()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[3]").unwrap()
        )
  }
  #[test]
  fn test_count_test_count_2_233() {
    let compiled = compile(&"Patient.name.count() = 3".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_count_test_count_3_234() {
    let compiled = compile(&"Patient.name.first().count()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[1]").unwrap()
        )
  }
  #[test]
  fn test_count_test_count_4_235() {
    let compiled = compile(&"Patient.name.first().count() = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_where_test_where_1_236() {
    let compiled = compile(&"Patient.name.count() = 3".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_where_test_where_2_237() {
    let compiled = compile(
      &"Patient.name.where(given = 'Jim').count() = 1".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_where_test_where_3_238() {
    let compiled = compile(
      &"Patient.name.where(given = 'X').count() = 0".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_where_test_where_4_239() {
    let compiled = compile(
      &"Patient.name.where($this.given = 'Jim').count() = 1".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_select_test_select_1_240() {
    let compiled = compile(&"Patient.name.select(given).count() = 5".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_select_test_select_2_241() {
    let compiled = compile(
      &"Patient.name.select(given | family).count() = 7 ".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_repeat_test_repeat_1_242() {
    let compiled = compile(
      &"ValueSet.expansion.repeat(contains).count() = 10".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP
        .get("valueset-example-expansion.xml")
        .unwrap()
        .clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_repeat_test_repeat_2_243() {
    let compiled = compile(
      &"Questionnaire.repeat(item).code.count() = 11".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("questionnaire-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_repeat_test_repeat_3_244() {
    let compiled = compile(
      &"Questionnaire.descendants().code.count() = 23".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("questionnaire-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_repeat_test_repeat_4_245() {
    let compiled = compile(&"Questionnaire.children().code.count() = 2".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("questionnaire-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_aggregate_test_aggregate_1_246() {
    let compiled = compile(
      &"(1|2|3|4|5|6|7|8|9).aggregate($this+$total, 0) = 45".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_aggregate_test_aggregate_2_247() {
    let compiled = compile(
      &"(1|2|3|4|5|6|7|8|9).aggregate($this+$total, 2) = 47".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_aggregate_test_aggregate_3_248() {
    let compiled = compile(
      &"(1|2|3|4|5|6|7|8|9).aggregate(iif($total.empty(), $this, iif($this < $total, $this, $total))) = 1"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_aggregate_test_aggregate_4_249() {
    let compiled = compile(
      &"(1|2|3|4|5|6|7|8|9).aggregate(iif($total.empty(), $this, iif($this > $total, $this, $total))) = 9"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_indexer_test_indexer_1_250() {
    let compiled = compile(&"Patient.name[0].given = 'Peter' | 'James'".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_indexer_test_indexer_2_251() {
    let compiled = compile(&"Patient.name[1].given = 'Jim'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_single_test_single_1_252() {
    let compiled = compile(&"Patient.name.first().single().exists()".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_single_test_single_2_253() {
    let compiled = compile(&"Patient.name.single().exists()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    let err = evaluate_result.err().unwrap();
    match err {
      FhirpathError::EvaluateError { msg } => panic!(),
      _ => {}
    }
  }
  #[test]
  fn test_first_last_test_first_last_1_254() {
    let compiled = compile(
      &"Patient.name.first().given = 'Peter' | 'James'".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_first_last_test_first_last_2_255() {
    let compiled = compile(
      &"Patient.name.last().given = 'Peter' | 'James'".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_tail_test_tail_1_256() {
    let compiled = compile(&"(0 | 1 | 2).tail() = 1 | 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_tail_test_tail_2_257() {
    let compiled = compile(
      &"Patient.name.tail().given = 'Jim' | 'Peter' | 'James'".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_skip_test_skip_1_258() {
    let compiled = compile(&"(0 | 1 | 2).skip(1) = 1 | 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_skip_test_skip_2_259() {
    let compiled = compile(&"(0 | 1 | 2).skip(2) = 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_skip_test_skip_3_260() {
    let compiled = compile(
      &"Patient.name.skip(1).given.trace('test') = 'Jim' | 'Peter' | 'James'"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_skip_test_skip_4_261() {
    let compiled = compile(
      &"Patient.name.skip(3).given.exists() = false".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_take_test_take_1_262() {
    let compiled = compile(&"(0 | 1 | 2).take(1) = 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_take_test_take_2_263() {
    let compiled = compile(&"(0 | 1 | 2).take(2) = 0 | 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_take_test_take_3_264() {
    let compiled = compile(
      &"Patient.name.take(1).given = 'Peter' | 'James'".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_take_test_take_4_265() {
    let compiled = compile(
      &"Patient.name.take(2).given = 'Peter' | 'James' | 'Jim'".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_take_test_take_5_266() {
    let compiled = compile(&"Patient.name.take(3).given.count() = 5".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_take_test_take_6_267() {
    let compiled = compile(&"Patient.name.take(4).given.count() = 5".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_take_test_take_7_268() {
    let compiled = compile(
      &"Patient.name.take(0).given.exists() = false".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_iif_test_iif_1_269() {
    let compiled = compile(
      &"iif(Patient.name.exists(), 'named', 'unnamed') = 'named'".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_iif_test_iif_2_270() {
    let compiled = compile(
      &"iif(Patient.name.empty(), 'unnamed', 'named') = 'named'".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_iif_test_iif_3_271() {
    let compiled = compile(&"iif(true, true, (1 | 2).toString())".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_iif_test_iif_4_272() {
    let compiled = compile(&"iif(false, (1 | 2).toString(), true)".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_integer_test_to_integer_1_273() {
    let compiled = compile(&"'1'.toInteger() = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_integer_test_to_integer_2_274() {
    let compiled = compile(&"'-1'.toInteger() = -1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_integer_test_to_integer_3_275() {
    let compiled = compile(&"'0'.toInteger() = 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_integer_test_to_integer_4_276() {
    let compiled = compile(&"'0.0'.toInteger().empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_integer_test_to_integer_5_277() {
    let compiled = compile(&"'st'.toInteger().empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_decimal_test_to_decimal_1_278() {
    let compiled = compile(&"'1'.toDecimal() = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_decimal_test_to_decimal_2_279() {
    let compiled = compile(&"'-1'.toInteger() = -1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_decimal_test_to_decimal_3_280() {
    let compiled = compile(&"'0'.toDecimal() = 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_decimal_test_to_decimal_4_281() {
    let compiled = compile(&"'0.0'.toDecimal() = 0.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_decimal_test_to_decimal_5_282() {
    let compiled = compile(&"'st'.toDecimal().empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_string_test_to_string_1_283() {
    let compiled = compile(&"1.toString() = '1'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_string_test_to_string_2_284() {
    let compiled = compile(&"'-1'.toInteger() = -1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_string_test_to_string_3_285() {
    let compiled = compile(&"0.toString() = '0'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_string_test_to_string_4_286() {
    let compiled = compile(&"0.0.toString() = '0.0'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_string_test_to_string_5_287() {
    let compiled = compile(&"@2014-12-14.toString() = '2014-12-14'".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_case_test_case_1_288() {
    let compiled = compile(&"'t'.upper() = 'T'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_case_test_case_2_289() {
    let compiled = compile(&"'t'.lower() = 't'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_case_test_case_3_290() {
    let compiled = compile(&"'T'.upper() = 'T'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_case_test_case_4_291() {
    let compiled = compile(&"'T'.lower() = 't'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_to_chars_test_to_chars_1_292() {
    let compiled = compile(&"'t2'.toChars() = 't' | '2'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_substring_test_substring_1_293() {
    let compiled = compile(&"'12345'.substring(2) = '345'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_substring_test_substring_2_294() {
    let compiled = compile(&"'12345'.substring(2,1) = '3'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_substring_test_substring_3_295() {
    let compiled = compile(&"'12345'.substring(2,5) = '345'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_substring_test_substring_4_296() {
    let compiled = compile(&"'12345'.substring(25).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_substring_test_substring_5_297() {
    let compiled = compile(&"'12345'.substring(-1).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_starts_with_test_starts_with_1_298() {
    let compiled = compile(&"'12345'.startsWith('2') = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_starts_with_test_starts_with_2_299() {
    let compiled = compile(&"'12345'.startsWith('1') = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_starts_with_test_starts_with_3_300() {
    let compiled = compile(&"'12345'.startsWith('12') = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_starts_with_test_starts_with_4_301() {
    let compiled = compile(&"'12345'.startsWith('13') = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_starts_with_test_starts_with_5_302() {
    let compiled = compile(&"'12345'.startsWith('12345') = true".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_starts_with_test_starts_with_6_303() {
    let compiled = compile(&"'12345'.startsWith('123456') = false".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_starts_with_test_starts_with_7_304() {
    let compiled = compile(&"'12345'.startsWith('') = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_ends_with_test_ends_with_1_305() {
    let compiled = compile(&"'12345'.endsWith('2') = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_ends_with_test_ends_with_2_306() {
    let compiled = compile(&"'12345'.endsWith('5') = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_ends_with_test_ends_with_3_307() {
    let compiled = compile(&"'12345'.endsWith('45') = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_ends_with_test_ends_with_4_308() {
    let compiled = compile(&"'12345'.endsWith('35') = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_ends_with_test_ends_with_5_309() {
    let compiled = compile(&"'12345'.endsWith('12345') = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_ends_with_test_ends_with_6_310() {
    let compiled = compile(&"'12345'.endsWith('012345') = false".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_ends_with_test_ends_with_7_311() {
    let compiled = compile(&"'12345'.endsWith('') = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_contains_string_test_contains_string_1_312() {
    let compiled = compile(&"'12345'.contains('6') = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_contains_string_test_contains_string_2_313() {
    let compiled = compile(&"'12345'.contains('5') = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_contains_string_test_contains_string_3_314() {
    let compiled = compile(&"'12345'.contains('45') = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_contains_string_test_contains_string_4_315() {
    let compiled = compile(&"'12345'.contains('35') = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_contains_string_test_contains_string_5_316() {
    let compiled = compile(&"'12345'.contains('12345') = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_contains_string_test_contains_string_6_317() {
    let compiled = compile(&"'12345'.contains('012345') = false".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_contains_string_test_contains_string_7_318() {
    let compiled = compile(&"'12345'.contains('') = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_length_test_length_1_319() {
    let compiled = compile(&"'123456'.length() = 6".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_length_test_length_2_320() {
    let compiled = compile(&"'12345'.length() = 5".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_length_test_length_3_321() {
    let compiled = compile(&"'123'.length() = 3".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_length_test_length_4_322() {
    let compiled = compile(&"'1'.length() = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_length_test_length_5_323() {
    let compiled = compile(&"''.length() = 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_trace_test_trace_1_324() {
    let compiled = compile(&"name.given.trace('test').count() = 5".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_trace_test_trace_2_325() {
    let compiled = compile(&"name.trace('test', given).count() = 3".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_today_test_today_1_326() {
    let compiled = compile(&"Patient.birthDate < today()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_today_test_today_2_327() {
    let compiled = compile(&"today().toString().length() = 10".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_now_test_now_1_328() {
    let compiled = compile(&"Patient.birthDate < now()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_now_test_now_2_329() {
    let compiled = compile(&"now().toString().length() > 10".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_1_330() {
    let compiled = compile(&"1 = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_2_331() {
    let compiled = compile(&"{} = {}".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_3_332() {
    let compiled = compile(&"true = {}".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_4_333() {
    let compiled = compile(&"(1) = (1)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_5_334() {
    let compiled = compile(&"(1 | 2) = (1 | 2)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_6_335() {
    let compiled = compile(&"(1 | 2 | 3) = (1 | 2 | 3)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_7_336() {
    let compiled = compile(&"(1 | 1) = (1 | 2 | {})".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_8_337() {
    let compiled = compile(&"1 = 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_9_338() {
    let compiled = compile(&"'a' = 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_10_339() {
    let compiled = compile(&"'a' = 'A'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_11_340() {
    let compiled = compile(&"'a' = 'b'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_12_341() {
    let compiled = compile(&"1.1 = 1.1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_13_342() {
    let compiled = compile(&"1.1 = 1.2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_14_343() {
    let compiled = compile(&"1.10 = 1.1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_15_344() {
    let compiled = compile(&"0 = 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_16_345() {
    let compiled = compile(&"0.0 = 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_17_346() {
    let compiled = compile(&"@2012-04-15 = @2012-04-15".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_18_347() {
    let compiled = compile(&"@2012-04-15 = @2012-04-16".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_19_348() {
    let compiled = compile(&"@2012-04-15 = @2012-04-15T10:00:00".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_20_349() {
    let compiled = compile(
      &"@2012-04-15T15:00:00 = @2012-04-15T10:00:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_21_350() {
    let compiled = compile(
      &"@2012-04-15T15:30:31 = @2012-04-15T15:30:31.0".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_22_351() {
    let compiled = compile(
      &"@2012-04-15T15:30:31 = @2012-04-15T15:30:31.1".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_23_352() {
    let compiled = compile(
      &"@2012-04-15T15:00:00Z = @2012-04-15T10:00:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_24_353() {
    let compiled = compile(
      &"@2012-04-15T15:00:00+02:00 = @2012-04-15T16:00:00+03:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_25_354() {
    let compiled = compile(&"name = name".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_26_355() {
    let compiled = compile(
      &"name.take(2) = name.take(2).first() | name.take(2).last()".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_27_356() {
    let compiled = compile(
      &"name.take(2) = name.take(2).last() | name.take(2).first()".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_equality_test_equality_28_357() {
    let compiled = compile(&"Observation.value = 185 '[lb_av]'".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_1_358() {
    let compiled = compile(&"1 != 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_2_359() {
    let compiled = compile(&"{} != {}".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_3_360() {
    let compiled = compile(&"1 != 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_4_361() {
    let compiled = compile(&"'a' != 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_5_362() {
    let compiled = compile(&"'a' != 'b'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_6_363() {
    let compiled = compile(&"1.1 != 1.1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_7_364() {
    let compiled = compile(&"1.1 != 1.2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_8_365() {
    let compiled = compile(&"1.10 != 1.1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_9_366() {
    let compiled = compile(&"0 != 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_10_367() {
    let compiled = compile(&"0.0 != 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_11_368() {
    let compiled = compile(&"@2012-04-15 != @2012-04-15".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_12_369() {
    let compiled = compile(&"@2012-04-15 != @2012-04-16".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_13_370() {
    let compiled = compile(&"@2012-04-15 != @2012-04-15T10:00:00".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_14_371() {
    let compiled = compile(
      &"@2012-04-15T15:00:00 != @2012-04-15T10:00:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_15_372() {
    let compiled = compile(
      &"@2012-04-15T15:30:31 != @2012-04-15T15:30:31.0".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_16_373() {
    let compiled = compile(
      &"@2012-04-15T15:30:31 != @2012-04-15T15:30:31.1".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_17_374() {
    let compiled = compile(
      &"@2012-04-15T15:00:00Z != @2012-04-15T10:00:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_18_375() {
    let compiled = compile(
      &"@2012-04-15T15:00:00+02:00 != @2012-04-15T16:00:00+03:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_19_376() {
    let compiled = compile(&"name != name".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_20_377() {
    let compiled = compile(
      &"name.take(2) != name.take(2).first() | name.take(2).last()".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_21_378() {
    let compiled = compile(
      &"name.take(2) != name.take(2).last() | name.take(2).first()".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_22_379() {
    let compiled = compile(&"1.2 / 1.8 != 0.6666667".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_23_380() {
    let compiled = compile(&"1.2 / 1.8 != 0.67".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_n_equality_test_n_equality_24_381() {
    let compiled = compile(&"Observation.value != 185 'kg'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_1_382() {
    let compiled = compile(&"1 ~ 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_2_383() {
    let compiled = compile(&"{} ~ {}".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_3_384() {
    let compiled = compile(&"1 ~ {}".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_4_385() {
    let compiled = compile(&"1 ~ 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_5_386() {
    let compiled = compile(&"'a' ~ 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_6_387() {
    let compiled = compile(&"'a' ~ 'A'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_7_388() {
    let compiled = compile(&"'a' ~ 'b'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_8_389() {
    let compiled = compile(&"1.1 ~ 1.1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_9_390() {
    let compiled = compile(&"1.1 ~ 1.2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_10_391() {
    let compiled = compile(&"1.10 ~ 1.1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_11_392() {
    let compiled = compile(&"1.2 / 1.8 ~ 0.67".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_12_393() {
    let compiled = compile(&"0 ~ 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_13_394() {
    let compiled = compile(&"0.0 ~ 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_14_395() {
    let compiled = compile(&"@2012-04-15 ~ @2012-04-15".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_15_396() {
    let compiled = compile(&"@2012-04-15 ~ @2012-04-16".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_16_397() {
    let compiled = compile(&"@2012-04-15 ~ @2012-04-15T10:00:00".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_17_398() {
    let compiled = compile(
      &"@2012-04-15T15:30:31 ~ @2012-04-15T15:30:31.0".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_18_399() {
    let compiled = compile(
      &"@2012-04-15T15:30:31 ~ @2012-04-15T15:30:31.1".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_19_400() {
    let compiled = compile(&"name ~ name".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_20_401() {
    let compiled = compile(
      &"name.take(2).given ~ name.take(2).first().given | name.take(2).last().given"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_21_402() {
    let compiled = compile(
      &"name.take(2).given ~ name.take(2).last().given | name.take(2).first().given"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_equivalent_test_equivalent_22_403() {
    let compiled = compile(&"Observation.value ~ 185 '[lb_av]'".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_1_404() {
    let compiled = compile(&"1 !~ 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_2_405() {
    let compiled = compile(&"{} !~ {}".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_3_406() {
    let compiled = compile(&"{} !~ 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_4_407() {
    let compiled = compile(&"1 !~ 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_5_408() {
    let compiled = compile(&"'a' !~ 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_6_409() {
    let compiled = compile(&"'a' !~ 'A'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_7_410() {
    let compiled = compile(&"'a' !~ 'b'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_8_411() {
    let compiled = compile(&"1.1 !~ 1.1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_9_412() {
    let compiled = compile(&"1.1 !~ 1.2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_10_413() {
    let compiled = compile(&"1.10 !~ 1.1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_11_414() {
    let compiled = compile(&"0 !~ 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_12_415() {
    let compiled = compile(&"0.0 !~ 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_13_416() {
    let compiled = compile(&"1.2 / 1.8 !~ 0.6".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_14_417() {
    let compiled = compile(&"@2012-04-15 !~ @2012-04-15".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_15_418() {
    let compiled = compile(&"@2012-04-15 !~ @2012-04-16".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_16_419() {
    let compiled = compile(&"@2012-04-15 !~ @2012-04-15T10:00:00".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_17_420() {
    let compiled = compile(
      &"@2012-04-15T15:30:31 !~ @2012-04-15T15:30:31.0".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_18_421() {
    let compiled = compile(
      &"@2012-04-15T15:30:31 !~ @2012-04-15T15:30:31.1".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_19_422() {
    let compiled = compile(&"name !~ name".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_20_423() {
    let compiled = compile(
      &"name.take(2).given !~ name.take(2).first().given | name.take(2).last().given"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_21_424() {
    let compiled = compile(
      &"name.take(2).given !~ name.take(2).last().given | name.take(2).first().given"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_not_equivalent_test_not_equivalent_22_425() {
    let compiled = compile(&"Observation.value !~ 185 'kg'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_1_426() {
    let compiled = compile(&"1 < 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_2_427() {
    let compiled = compile(&"1.0 < 1.2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_3_428() {
    let compiled = compile(&"'a' < 'b'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_4_429() {
    let compiled = compile(&"'A' < 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_5_430() {
    let compiled = compile(&"@2014-12-12 < @2014-12-13".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_6_431() {
    let compiled = compile(
      &"@2014-12-13T12:00:00 < @2014-12-13T12:00:01".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_7_432() {
    let compiled = compile(&"@T12:00:00 < @T14:00:00".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_8_433() {
    let compiled = compile(&"1 < 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_9_434() {
    let compiled = compile(&"1.0 < 1.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_10_435() {
    let compiled = compile(&"'a' < 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_11_436() {
    let compiled = compile(&"'A' < 'A'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_12_437() {
    let compiled = compile(&"@2014-12-12 < @2014-12-12".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_13_438() {
    let compiled = compile(
      &"@2014-12-13T12:00:00 < @2014-12-13T12:00:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_14_439() {
    let compiled = compile(&"@T12:00:00 < @T12:00:00".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_15_440() {
    let compiled = compile(&"2 < 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_16_441() {
    let compiled = compile(&"1.1 < 1.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_17_442() {
    let compiled = compile(&"'b' < 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_18_443() {
    let compiled = compile(&"'B' < 'A'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_19_444() {
    let compiled = compile(&"@2014-12-13 < @2014-12-12".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_20_445() {
    let compiled = compile(
      &"@2014-12-13T12:00:01 < @2014-12-13T12:00:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_21_446() {
    let compiled = compile(&"@T12:00:01 < @T12:00:00".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_22_447() {
    let compiled = compile(&"Observation.value < 200 '[lb_av]'".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_23_448() {
    let compiled = compile(&"@2018-03 < @2018-03-01".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_24_449() {
    let compiled = compile(&"@2018-03-01T10 < @2018-03-01T10:30".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_25_450() {
    let compiled = compile(&"@T10 < @T10:30".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_26_451() {
    let compiled = compile(
      &"@2018-03-01T10:30:00 < @2018-03-01T10:30:00.0".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_than_test_less_than_27_452() {
    let compiled = compile(&"@T10:30:00 < @T10:30:00.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_1_453() {
    let compiled = compile(&"1 <= 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_2_454() {
    let compiled = compile(&"1.0 <= 1.2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_3_455() {
    let compiled = compile(&"'a' <= 'b'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_4_456() {
    let compiled = compile(&"'A' <= 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_5_457() {
    let compiled = compile(&"@2014-12-12 <= @2014-12-13".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_6_458() {
    let compiled = compile(
      &"@2014-12-13T12:00:00 <= @2014-12-13T12:00:01".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_7_459() {
    let compiled = compile(&"@T12:00:00 <= @T14:00:00".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_8_460() {
    let compiled = compile(&"1 <= 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_9_461() {
    let compiled = compile(&"1.0 <= 1.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_10_462() {
    let compiled = compile(&"'a' <= 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_11_463() {
    let compiled = compile(&"'A' <= 'A'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_12_464() {
    let compiled = compile(&"@2014-12-12 <= @2014-12-12".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_13_465() {
    let compiled = compile(
      &"@2014-12-13T12:00:00 <= @2014-12-13T12:00:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_14_466() {
    let compiled = compile(&"@T12:00:00 <= @T12:00:00".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_15_467() {
    let compiled = compile(&"2 <= 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_16_468() {
    let compiled = compile(&"1.1 <= 1.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_17_469() {
    let compiled = compile(&"'b' <= 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_18_470() {
    let compiled = compile(&"'B' <= 'A'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_19_471() {
    let compiled = compile(&"@2014-12-13 <= @2014-12-12".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_20_472() {
    let compiled = compile(
      &"@2014-12-13T12:00:01 <= @2014-12-13T12:00:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_21_473() {
    let compiled = compile(&"@T12:00:01 <= @T12:00:00".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_22_474() {
    let compiled = compile(&"Observation.value <= 200 '[lb_av]'".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_23_475() {
    let compiled = compile(&"@2018-03 <= @2018-03-01".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_24_476() {
    let compiled = compile(&"@2018-03-01T10 <= @2018-03-01T10:30".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_25_477() {
    let compiled = compile(&"@T10 <= @T10:30".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_26_478() {
    let compiled = compile(
      &"@2018-03-01T10:30:00 <= @2018-03-01T10:30:00.0".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_less_or_equal_test_less_or_equal_27_479() {
    let compiled = compile(&"@T10:30:00 <= @T10:30:00.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_1_480() {
    let compiled = compile(&"1 >= 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_2_481() {
    let compiled = compile(&"1.0 >= 1.2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_3_482() {
    let compiled = compile(&"'a' >= 'b'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_4_483() {
    let compiled = compile(&"'A' >= 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_5_484() {
    let compiled = compile(&"@2014-12-12 >= @2014-12-13".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_6_485() {
    let compiled = compile(
      &"@2014-12-13T12:00:00 >= @2014-12-13T12:00:01".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_7_486() {
    let compiled = compile(&"@T12:00:00 >= @T14:00:00".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_8_487() {
    let compiled = compile(&"1 >= 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_9_488() {
    let compiled = compile(&"1.0 >= 1.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_10_489() {
    let compiled = compile(&"'a' >= 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_11_490() {
    let compiled = compile(&"'A' >= 'A'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_12_491() {
    let compiled = compile(&"@2014-12-12 >= @2014-12-12".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_13_492() {
    let compiled = compile(
      &"@2014-12-13T12:00:00 >= @2014-12-13T12:00:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_14_493() {
    let compiled = compile(&"@T12:00:00 >= @T12:00:00".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_15_494() {
    let compiled = compile(&"2 >= 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_16_495() {
    let compiled = compile(&"1.1 >= 1.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_17_496() {
    let compiled = compile(&"'b' >= 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_18_497() {
    let compiled = compile(&"'B' >= 'A'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_19_498() {
    let compiled = compile(&"@2014-12-13 >= @2014-12-12".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_20_499() {
    let compiled = compile(
      &"@2014-12-13T12:00:01 >= @2014-12-13T12:00:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_21_500() {
    let compiled = compile(&"@T12:00:01 >= @T12:00:00".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_22_501() {
    let compiled = compile(&"Observation.value >= 100 '[lb_av]'".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_23_502() {
    let compiled = compile(&"@2018-03 >= @2018-03-01".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_24_503() {
    let compiled = compile(&"@2018-03-01T10 >= @2018-03-01T10:30".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_25_504() {
    let compiled = compile(&"@T10 >= @T10:30".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_26_505() {
    let compiled = compile(
      &"@2018-03-01T10:30:00 >= @2018-03-01T10:30:00.0".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greator_or_equal_test_greator_or_equal_27_506() {
    let compiled = compile(&"@T10:30:00 >= @T10:30:00.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_1_507() {
    let compiled = compile(&"1 > 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_2_508() {
    let compiled = compile(&"1.0 > 1.2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_3_509() {
    let compiled = compile(&"'a' > 'b'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_4_510() {
    let compiled = compile(&"'A' > 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_5_511() {
    let compiled = compile(&"@2014-12-12 > @2014-12-13".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_6_512() {
    let compiled = compile(
      &"@2014-12-13T12:00:00 > @2014-12-13T12:00:01".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_7_513() {
    let compiled = compile(&"@T12:00:00 > @T14:00:00".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_8_514() {
    let compiled = compile(&"1 > 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_9_515() {
    let compiled = compile(&"1.0 > 1.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_10_516() {
    let compiled = compile(&"'a' > 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_11_517() {
    let compiled = compile(&"'A' > 'A'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_12_518() {
    let compiled = compile(&"@2014-12-12 > @2014-12-12".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_13_519() {
    let compiled = compile(
      &"@2014-12-13T12:00:00 > @2014-12-13T12:00:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_14_520() {
    let compiled = compile(&"@T12:00:00 > @T12:00:00".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_15_521() {
    let compiled = compile(&"2 > 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_16_522() {
    let compiled = compile(&"1.1 > 1.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_17_523() {
    let compiled = compile(&"'b' > 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_18_524() {
    let compiled = compile(&"'B' > 'A'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_19_525() {
    let compiled = compile(&"@2014-12-13 > @2014-12-12".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_20_526() {
    let compiled = compile(
      &"@2014-12-13T12:00:01 > @2014-12-13T12:00:00".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_21_527() {
    let compiled = compile(&"@T12:00:01 > @T12:00:00".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_22_528() {
    let compiled = compile(&"Observation.value > 100 '[lb_av]'".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("observation-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_23_529() {
    let compiled = compile(&"@2018-03 > @2018-03-01".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_24_530() {
    let compiled = compile(&"@2018-03-01T10 > @2018-03-01T10:30".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_25_531() {
    let compiled = compile(&"@T10 > @T10:30".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_26_532() {
    let compiled = compile(
      &"@2018-03-01T10:30:00 > @2018-03-01T10:30:00.0".to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_greater_than_test_greater_than_27_533() {
    let compiled = compile(&"@T10:30:00 > @T10:30:00.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_union_test_union_1_534() {
    let compiled = compile(&"(1 | 2 | 3).count() = 3".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_union_test_union_2_535() {
    let compiled = compile(&"(1 | 2 | 2).count() = 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_union_test_union_3_536() {
    let compiled = compile(&"(1|1).count() = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_union_test_union_4_537() {
    let compiled = compile(&"1.union(2).union(3).count() = 3".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_union_test_union_5_538() {
    let compiled = compile(&"1.union(2.union(3)).count() = 3".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_union_test_union_6_539() {
    let compiled = compile(&"(1 | 2).combine(2).count() = 3".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_union_test_union_7_540() {
    let compiled = compile(&"1.combine(1).count() = 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_union_test_union_8_541() {
    let compiled = compile(&"1.combine(1).union(2).count() = 2".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_intersect_test_intersect_1_542() {
    let compiled = compile(&"(1 | 2 | 3).intersect(2 | 4) = 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_intersect_test_intersect_2_543() {
    let compiled = compile(&"(1 | 2).intersect(4).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_intersect_test_intersect_3_544() {
    let compiled = compile(&"(1 | 2).intersect({}).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_intersect_test_intersect_4_545() {
    let compiled = compile(&"1.combine(1).intersect(1).count() = 1".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_exclude_test_exclude_1_546() {
    let compiled = compile(&"(1 | 2 | 3).exclude(2 | 4) = 1 | 3".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_exclude_test_exclude_2_547() {
    let compiled = compile(&"(1 | 2).exclude(4) = 1 | 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_exclude_test_exclude_3_548() {
    let compiled = compile(&"(1 | 2).exclude({}) = 1 | 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_exclude_test_exclude_4_549() {
    let compiled = compile(&"1.combine(1).exclude(2).count() = 2".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_in_test_in_1_550() {
    let compiled = compile(&"1 in (1 | 2 | 3)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_in_test_in_2_551() {
    let compiled = compile(&"1 in (2 | 3)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_in_test_in_3_552() {
    let compiled = compile(&"'a' in ('a' | 'c' | 'd')".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_in_test_in_4_553() {
    let compiled = compile(&"'b' in ('a' | 'c' | 'd')".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_contains_collection_test_contains_collection_1_554() {
    let compiled = compile(&"(1 | 2 | 3) contains 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_contains_collection_test_contains_collection_2_555() {
    let compiled = compile(&"(2 | 3) contains 1 ".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_contains_collection_test_contains_collection_3_556() {
    let compiled = compile(&"('a' | 'c' | 'd') contains 'a'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_contains_collection_test_contains_collection_4_557() {
    let compiled = compile(&"('a' | 'c' | 'd') contains 'b'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_and_test_boolean_logic_and_1_558() {
    let compiled = compile(&"(true and true) = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_and_test_boolean_logic_and_2_559() {
    let compiled = compile(&"(true and false) = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_and_test_boolean_logic_and_3_560() {
    let compiled = compile(&"(true and {}).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_and_test_boolean_logic_and_4_561() {
    let compiled = compile(&"(false and true) = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_and_test_boolean_logic_and_5_562() {
    let compiled = compile(&"(false and false) = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_and_test_boolean_logic_and_6_563() {
    let compiled = compile(&"(false and {}) = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_and_test_boolean_logic_and_7_564() {
    let compiled = compile(&"({} and true).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_and_test_boolean_logic_and_8_565() {
    let compiled = compile(&"({} and false) = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_and_test_boolean_logic_and_9_566() {
    let compiled = compile(&"({} and {}).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_or_test_boolean_logic_or_1_567() {
    let compiled = compile(&"(true or true) = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_or_test_boolean_logic_or_2_568() {
    let compiled = compile(&"(true or false) = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_or_test_boolean_logic_or_3_569() {
    let compiled = compile(&"(true or {}) = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_or_test_boolean_logic_or_4_570() {
    let compiled = compile(&"(false or true) = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_or_test_boolean_logic_or_5_571() {
    let compiled = compile(&"(false or false) = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_or_test_boolean_logic_or_6_572() {
    let compiled = compile(&"(false or {}).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_or_test_boolean_logic_or_7_573() {
    let compiled = compile(&"({} or true) = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_or_test_boolean_logic_or_8_574() {
    let compiled = compile(&"({} or false).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_or_test_boolean_logic_or_9_575() {
    let compiled = compile(&"({} or {}).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_x_or_test_boolean_logic_x_or_1_576() {
    let compiled = compile(&"(true xor true) = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_x_or_test_boolean_logic_x_or_2_577() {
    let compiled = compile(&"(true xor false) = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_x_or_test_boolean_logic_x_or_3_578() {
    let compiled = compile(&"(true xor {}).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_x_or_test_boolean_logic_x_or_4_579() {
    let compiled = compile(&"(false xor true) = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_x_or_test_boolean_logic_x_or_5_580() {
    let compiled = compile(&"(false xor false) = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_x_or_test_boolean_logic_x_or_6_581() {
    let compiled = compile(&"(false xor {}).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_x_or_test_boolean_logic_x_or_7_582() {
    let compiled = compile(&"({} xor true).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_x_or_test_boolean_logic_x_or_8_583() {
    let compiled = compile(&"({} xor false).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_logic_x_or_test_boolean_logic_x_or_9_584() {
    let compiled = compile(&"({} xor {}).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_implies_test_boolean_implies_1_585() {
    let compiled = compile(&"(true implies true) = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_implies_test_boolean_implies_2_586() {
    let compiled = compile(&"(true implies false) = false".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_implies_test_boolean_implies_3_587() {
    let compiled = compile(&"(true implies {}).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_implies_test_boolean_implies_4_588() {
    let compiled = compile(&"(false implies true) = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_implies_test_boolean_implies_5_589() {
    let compiled = compile(&"(false implies false) = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_implies_test_boolean_implies_6_590() {
    let compiled = compile(&"(false implies {}) = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_implies_test_boolean_implies_7_591() {
    let compiled = compile(&"({} implies true) = true".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_implies_test_boolean_implies_8_592() {
    let compiled = compile(&"({} implies false).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_boolean_implies_test_boolean_implies_9_593() {
    let compiled = compile(&"({} implies {}).empty()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_plus_test_plus_1_594() {
    let compiled = compile(&"1 + 1 = 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_plus_test_plus_2_595() {
    let compiled = compile(&"1 + 0 = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_plus_test_plus_3_596() {
    let compiled = compile(&"1.2 + 1.8 = 3.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_plus_test_plus_4_597() {
    let compiled = compile(&"'a'+'b' = 'ab'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_concatenate_test_concatenate_1_598() {
    let compiled = compile(&"'a' & 'b' = 'ab'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_concatenate_test_concatenate_2_599() {
    let compiled = compile(&"'1' & {} = '1'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_concatenate_test_concatenate_3_600() {
    let compiled = compile(&"{} & 'b' = 'b'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_concatenate_test_concatenate_4_601() {
    let compiled = compile(&"(1 | 2 | 3) & 'b' = '1,2,3b'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    let err = evaluate_result.err().unwrap();
    match err {
      FhirpathError::EvaluateError { msg } => panic!(),
      _ => {}
    }
  }
  #[test]
  fn test_minus_test_minus_1_602() {
    let compiled = compile(&"1 - 1 = 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_minus_test_minus_2_603() {
    let compiled = compile(&"1 - 0 = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_minus_test_minus_3_604() {
    let compiled = compile(&"1.8 - 1.2 = 0.6".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_minus_test_minus_4_605() {
    let compiled = compile(&"'a'-'b' = 'ab'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    let err = evaluate_result.err().unwrap();
    match err {
      FhirpathError::EvaluateError { msg } => panic!(),
      _ => {}
    }
  }
  #[test]
  fn test_multiply_test_multiply_1_606() {
    let compiled = compile(&"1.2 * 1.8 = 2.16".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_multiply_test_multiply_2_607() {
    let compiled = compile(&"1 * 1 = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_multiply_test_multiply_3_608() {
    let compiled = compile(&"1 * 0 = 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_divide_test_divide_1_609() {
    let compiled = compile(&"1 / 1 = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_divide_test_divide_2_610() {
    let compiled = compile(&"4 / 2 = 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_divide_test_divide_3_611() {
    let compiled = compile(&"4.0 / 2.0 = 2.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_divide_test_divide_4_612() {
    let compiled = compile(&"1 / 2 = 0.5".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_divide_test_divide_5_613() {
    let compiled = compile(&"1.2 / 1.8 = 0.66666667".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_divide_test_divide_6_614() {
    let compiled = compile(&"1 / 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_div_test_div_1_615() {
    let compiled = compile(&"1 div 1 = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_div_test_div_2_616() {
    let compiled = compile(&"4 div 2 = 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_div_test_div_3_617() {
    let compiled = compile(&"5 div 2 = 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_div_test_div_4_618() {
    let compiled = compile(&"2.2 div 1.8 = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_div_test_div_5_619() {
    let compiled = compile(&"5 div 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_mod_test_mod_1_620() {
    let compiled = compile(&"1 mod 1 = 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_mod_test_mod_2_621() {
    let compiled = compile(&"4 mod 2 = 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_mod_test_mod_3_622() {
    let compiled = compile(&"5 mod 2 = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_mod_test_mod_4_623() {
    let compiled = compile(&"2.2 mod 1.8 = 0.4".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_mod_test_mod_5_624() {
    let compiled = compile(&"5 mod 0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_round_test_round_1_625() {
    let compiled = compile(&"1.round() = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_round_test_round_2_626() {
    let compiled = compile(&"3.14159.round(3) = 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_sqrt_test_sqrt_1_627() {
    let compiled = compile(&"81.sqrt() = 9.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_sqrt_test_sqrt_2_628() {
    let compiled = compile(&"(-1).sqrt()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_abs_test_abs_1_629() {
    let compiled = compile(&"(-5).abs() = 5".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_abs_test_abs_2_630() {
    let compiled = compile(&"(-5.5).abs() = 5.5".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_abs_test_abs_3_631() {
    let compiled = compile(&"(-5.5 'mg').abs() = 5.5 'mg'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_ceiling_test_ceiling_1_632() {
    let compiled = compile(&"1.ceiling() = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_ceiling_test_ceiling_2_633() {
    let compiled = compile(&"(-1.1).ceiling() = -1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_ceiling_test_ceiling_3_634() {
    let compiled = compile(&"1.1.ceiling() = 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_exp_test_exp_1_635() {
    let compiled = compile(&"0.exp() = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_exp_test_exp_2_636() {
    let compiled = compile(&"(-0.0).exp() = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_floor_test_floor_1_637() {
    let compiled = compile(&"1.floor() = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_floor_test_floor_2_638() {
    let compiled = compile(&"2.1.floor() = 2".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_floor_test_floor_3_639() {
    let compiled = compile(&"(-2.1).floor() = -3".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_ln_test_ln_1_640() {
    let compiled = compile(&"1.ln() = 0.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_ln_test_ln_2_641() {
    let compiled = compile(&"1.0.ln() = 0.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_log_test_log_1_642() {
    let compiled = compile(&"16.log(2) = 4.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_log_test_log_2_643() {
    let compiled = compile(&"100.0.log(10.0) = 2.0".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_power_test_power_1_644() {
    let compiled = compile(&"2.power(3) = 8".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_power_test_power_2_645() {
    let compiled = compile(&"2.5.power(2) = 6.25".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_power_test_power_3_646() {
    let compiled = compile(&"(-1).power(0.5)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[]").unwrap()
        )
  }
  #[test]
  fn test_truncate_test_truncate_1_647() {
    let compiled = compile(&"101.truncate() = 101".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_truncate_test_truncate_2_648() {
    let compiled = compile(&"1.00000001.truncate() = 1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_truncate_test_truncate_3_649() {
    let compiled = compile(&"(-1.56).truncate() = -1".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_precedence_test_unary_precedence_650() {
    let compiled = compile(&"-1.convertsToInteger()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    let err = evaluate_result.err().unwrap();
    match err {
      FhirpathError::EvaluateError { msg } => panic!(),
      _ => {}
    }
  }
  #[test]
  fn test_precedence_test_precedence_2_651() {
    let compiled = compile(&"1+2*3+4 = 11".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_precedence_test_precedence_3_652() {
    let compiled = compile(&"1 > 2 is Boolean".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_precedence_test_precedence_4_653() {
    let compiled = compile(&"1 | 1 is Integer".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_variables_test_variables_1_654() {
    let compiled = compile(&"%sct = 'http://snomed.info/sct'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_variables_test_variables_2_655() {
    let compiled = compile(&"%loinc = 'http://loinc.org'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_variables_test_variables_3_656() {
    let compiled = compile(&"%ucum = 'http://unitsofmeasure.org'".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_variables_test_variables_4_657() {
    let compiled = compile(
      &"%`vs-administrative-gender` = 'http://hl7.org/fhir/ValueSet/administrative-gender'"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_extension_test_extension_1_658() {
    let compiled = compile(
      &"Patient.birthDate.extension('http://hl7.org/fhir/StructureDefinition/patient-birthTime').exists()"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_extension_test_extension_2_659() {
    let compiled = compile(
      &"Patient.birthDate.extension(%`ext-patient-birthTime`).exists()"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_extension_test_extension_3_660() {
    let compiled = compile(
      &"Patient.birthDate.extension('http://hl7.org/fhir/StructureDefinition/patient-birthTime1').empty()"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_1_661() {
    let compiled = compile(&"1.type().namespace = 'System'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_2_662() {
    let compiled = compile(&"1.type().name = 'Integer'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_3_663() {
    let compiled = compile(&"true.type().namespace = 'System'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_4_664() {
    let compiled = compile(&"true.type().name = 'Boolean'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_5_665() {
    let compiled = compile(&"true.is(Boolean)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_6_666() {
    let compiled = compile(&"true.is(System.Boolean)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_7_667() {
    let compiled = compile(&"true is Boolean".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_8_668() {
    let compiled = compile(&"true is System.Boolean".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_9_669() {
    let compiled = compile(&"Patient.active.type().namespace = 'FHIR'".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_10_670() {
    let compiled = compile(&"Patient.active.type().name = 'boolean'".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_11_671() {
    let compiled = compile(&"Patient.active.is(boolean)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_12_672() {
    let compiled = compile(&"Patient.active.is(Boolean).not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_13_673() {
    let compiled = compile(&"Patient.active.is(FHIR.boolean)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_14_674() {
    let compiled = compile(&"Patient.active.is(System.Boolean).not()".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_15_675() {
    let compiled = compile(&"Patient.type().namespace = 'FHIR'".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_16_676() {
    let compiled = compile(&"Patient.type().name = 'Patient'".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_17_677() {
    let compiled = compile(&"Patient.is(Patient)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_18_678() {
    let compiled = compile(&"Patient.is(FHIR.Patient)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_19_679() {
    let compiled = compile(&"Patient.is(FHIR.`Patient`)".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_20_680() {
    let compiled = compile(&"Patient.ofType(Patient).type().name".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[\"Patient\"]")
            .unwrap()
        )
  }
  #[test]
  fn test_type_test_type_21_681() {
    let compiled = compile(&"Patient.ofType(FHIR.Patient).type().name".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[\"Patient\"]")
            .unwrap()
        )
  }
  #[test]
  fn test_type_test_type_22_682() {
    let compiled = compile(&"Patient.is(System.Patient).not()".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_type_test_type_23_683() {
    let compiled = compile(&"Patient.ofType(FHIR.`Patient`).type().name".to_string())
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[\"Patient\"]")
            .unwrap()
        )
  }
  #[test]
  fn test_conforms_to_test_conforms_to_684() {
    let compiled = compile(
      &"conformsTo('http://hl7.org/fhir/StructureDefinition/Patient')"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[true]").unwrap()
        )
  }
  #[test]
  fn test_conforms_to_test_conforms_to_685() {
    let compiled = compile(
      &"conformsTo('http://hl7.org/fhir/StructureDefinition/Person')"
          .to_string(),
    )
        .unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    assert_json_eq!(
            evaluate_result.ok(), serde_json::from_str:: < Value > ("[false]").unwrap()
        )
  }
  #[test]
  fn test_conforms_to_test_conforms_to_686() {
    let compiled = compile(&"conformsTo('http://trash')".to_string()).unwrap();
    let input_value = INPUT_MAP.get("patient-example.xml").unwrap().clone();
    let evaluate_result = compiled
        .evaluate_single(
          input_value,
          Some(
            Arc::new(EvaluateOptions {
              trace_function: None,
              now: None,
              vars: None,
              model: Some(get_model_details(ModelType::R4).unwrap()),
            }),
          ),
        );
    let err = evaluate_result.err().unwrap();
    match err {
      FhirpathError::CompileError { msg } => panic!(),
      _ => {}
    }
  }
}
