use std::collections::HashMap;
pub fn choice_type_paths() -> HashMap<String, Vec<String>> {
    let mut map = HashMap::<String, Vec<String>>::new();
    map.insert(
        "ActivityDefinition.product".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "ActivityDefinition.timing".to_string(),
        vec![
            "DateTime".to_string(), "Period".to_string(), "Range".to_string(), "Timing"
            .to_string()
        ],
    );
    map.insert(
        "AllergyIntolerance.onset".to_string(),
        vec![
            "Age".to_string(), "dateTime".to_string(), "Period".to_string(), "Range"
            .to_string(), "string".to_string()
        ],
    );
    map.insert(
        "Annotation.author".to_string(),
        vec!["Reference".to_string(), "string".to_string()],
    );
    map.insert(
        "CarePlan.activity.detail.product".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "CarePlan.activity.detail.scheduled".to_string(),
        vec!["Period".to_string(), "string".to_string(), "Timing".to_string()],
    );
    map.insert(
        "ChargeItem.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string(), "Timing".to_string()],
    );
    map.insert(
        "Claim.accident.location".to_string(),
        vec!["Address".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Claim.diagnosis.diagnosis".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Claim.information.timing".to_string(),
        vec!["Date".to_string(), "Period".to_string()],
    );
    map.insert(
        "Claim.information.value".to_string(),
        vec![
            "Attachment".to_string(), "Quantity".to_string(), "Reference".to_string(),
            "string".to_string()
        ],
    );
    map.insert(
        "Claim.item.location".to_string(),
        vec![
            "Address".to_string(), "CodeableConcept".to_string(), "Reference".to_string()
        ],
    );
    map.insert(
        "Claim.item.serviced".to_string(),
        vec!["Date".to_string(), "Period".to_string()],
    );
    map.insert(
        "Claim.procedure.procedure".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "ClinicalImpression.effective".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "ClinicalImpression.finding.item".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "CodeSystem.concept.property.value".to_string(),
        vec![
            "Boolean".to_string(), "code".to_string(), "Coding".to_string(), "dateTime"
            .to_string(), "integer".to_string(), "string".to_string()
        ],
    );
    map.insert(
        "Communication.payload.content".to_string(),
        vec!["Attachment".to_string(), "Reference".to_string(), "string".to_string()],
    );
    map.insert(
        "CommunicationRequest.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "CommunicationRequest.payload.content".to_string(),
        vec!["Attachment".to_string(), "Reference".to_string(), "string".to_string()],
    );
    map.insert(
        "Composition.relatesTo.target".to_string(),
        vec!["Identifier".to_string(), "Reference".to_string()],
    );
    map.insert(
        "ConceptMap.source".to_string(),
        vec!["Reference".to_string(), "uri".to_string()],
    );
    map.insert(
        "ConceptMap.target".to_string(),
        vec!["Reference".to_string(), "uri".to_string()],
    );
    map.insert(
        "Condition.abatement".to_string(),
        vec![
            "Age".to_string(), "boolean".to_string(), "dateTime".to_string(), "Period"
            .to_string(), "Range".to_string(), "string".to_string()
        ],
    );
    map.insert(
        "Condition.onset".to_string(),
        vec![
            "Age".to_string(), "dateTime".to_string(), "Period".to_string(), "Range"
            .to_string(), "string".to_string()
        ],
    );
    map.insert(
        "Consent.source".to_string(),
        vec!["Attachment".to_string(), "Identifier".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Contract.binding".to_string(),
        vec!["Attachment".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Contract.friendly.content".to_string(),
        vec!["Attachment".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Contract.legal.content".to_string(),
        vec!["Attachment".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Contract.rule.content".to_string(),
        vec!["Attachment".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Contract.term.valuedItem.entity".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Contract.valuedItem.entity".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "DataRequirement.codeFilter.valueSet".to_string(),
        vec!["Reference".to_string(), "string".to_string()],
    );
    map.insert(
        "DataRequirement.dateFilter.value".to_string(),
        vec!["DateTime".to_string(), "Duration".to_string(), "Period".to_string()],
    );
    map.insert(
        "DeviceRequest.code".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "DeviceRequest.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string(), "Timing".to_string()],
    );
    map.insert(
        "DeviceUseStatement.timing".to_string(),
        vec!["DateTime".to_string(), "Period".to_string(), "Timing".to_string()],
    );
    map.insert(
        "DiagnosticReport.effective".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "DocumentManifest.content.p".to_string(),
        vec!["Attachment".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Dosage.asNeeded".to_string(),
        vec!["Boolean".to_string(), "CodeableConcept".to_string()],
    );
    map.insert(
        "Dosage.dose".to_string(),
        vec!["Quantity".to_string(), "Range".to_string()],
    );
    map.insert(
        "Dosage.rate".to_string(),
        vec!["Quantity".to_string(), "Range".to_string(), "Ratio".to_string()],
    );
    map.insert(
        "ElementDefinition.binding.valueSet".to_string(),
        vec!["Reference".to_string(), "uri".to_string()],
    );
    map.insert(
        "ElementDefinition.defaultValue".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "code".to_string(), "CodeableConcept".to_string(), "Coding".to_string(),
            "ContactPoint".to_string(), "Count".to_string(), "date".to_string(),
            "dateTime".to_string(), "decimal".to_string(), "Distance".to_string(),
            "Duration".to_string(), "HumanName".to_string(), "id".to_string(),
            "Identifier".to_string(), "instant".to_string(), "integer".to_string(),
            "markdown".to_string(), "Meta".to_string(), "Money".to_string(), "oid"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "Reference"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "unsignedInt"
            .to_string(), "uri".to_string()
        ],
    );
    map.insert(
        "ElementDefinition.example.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "code".to_string(), "CodeableConcept".to_string(), "Coding".to_string(),
            "ContactPoint".to_string(), "Count".to_string(), "date".to_string(),
            "dateTime".to_string(), "decimal".to_string(), "Distance".to_string(),
            "Duration".to_string(), "HumanName".to_string(), "id".to_string(),
            "Identifier".to_string(), "instant".to_string(), "integer".to_string(),
            "markdown".to_string(), "Meta".to_string(), "Money".to_string(), "oid"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "Reference"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "unsignedInt"
            .to_string(), "uri".to_string()
        ],
    );
    map.insert(
        "ElementDefinition.fixed".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "code".to_string(), "CodeableConcept".to_string(), "Coding".to_string(),
            "ContactPoint".to_string(), "Count".to_string(), "date".to_string(),
            "dateTime".to_string(), "decimal".to_string(), "Distance".to_string(),
            "Duration".to_string(), "HumanName".to_string(), "id".to_string(),
            "Identifier".to_string(), "instant".to_string(), "integer".to_string(),
            "markdown".to_string(), "Meta".to_string(), "Money".to_string(), "oid"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "Reference"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "unsignedInt"
            .to_string(), "uri".to_string()
        ],
    );
    map.insert(
        "ElementDefinition.maxValue".to_string(),
        vec![
            "Date".to_string(), "dateTime".to_string(), "decimal".to_string(), "instant"
            .to_string(), "integer".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "time".to_string(), "unsignedInt".to_string()
        ],
    );
    map.insert(
        "ElementDefinition.minValue".to_string(),
        vec![
            "Date".to_string(), "dateTime".to_string(), "decimal".to_string(), "instant"
            .to_string(), "integer".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "time".to_string(), "unsignedInt".to_string()
        ],
    );
    map.insert(
        "ElementDefinition.pattern".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "code".to_string(), "CodeableConcept".to_string(), "Coding".to_string(),
            "ContactPoint".to_string(), "Count".to_string(), "date".to_string(),
            "dateTime".to_string(), "decimal".to_string(), "Distance".to_string(),
            "Duration".to_string(), "HumanName".to_string(), "id".to_string(),
            "Identifier".to_string(), "instant".to_string(), "integer".to_string(),
            "markdown".to_string(), "Meta".to_string(), "Money".to_string(), "oid"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "Reference"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "unsignedInt"
            .to_string(), "uri".to_string()
        ],
    );
    map.insert(
        "EligibilityRequest.serviced".to_string(),
        vec!["Date".to_string(), "Period".to_string()],
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.financial.allowed".to_string(),
        vec!["Money".to_string(), "string".to_string(), "unsignedInt".to_string()],
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.financial.used".to_string(),
        vec!["Money".to_string(), "unsignedInt".to_string()],
    );
    map.insert(
        "ExplanationOfBenefit.accident.location".to_string(),
        vec!["Address".to_string(), "Reference".to_string()],
    );
    map.insert(
        "ExplanationOfBenefit.benefitBalance.financial.allowed".to_string(),
        vec!["Money".to_string(), "string".to_string(), "unsignedInt".to_string()],
    );
    map.insert(
        "ExplanationOfBenefit.benefitBalance.financial.used".to_string(),
        vec!["Money".to_string(), "unsignedInt".to_string()],
    );
    map.insert(
        "ExplanationOfBenefit.diagnosis.diagnosis".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "ExplanationOfBenefit.information.timing".to_string(),
        vec!["Date".to_string(), "Period".to_string()],
    );
    map.insert(
        "ExplanationOfBenefit.information.value".to_string(),
        vec![
            "Attachment".to_string(), "Quantity".to_string(), "Reference".to_string(),
            "string".to_string()
        ],
    );
    map.insert(
        "ExplanationOfBenefit.item.location".to_string(),
        vec![
            "Address".to_string(), "CodeableConcept".to_string(), "Reference".to_string()
        ],
    );
    map.insert(
        "ExplanationOfBenefit.item.serviced".to_string(),
        vec!["Date".to_string(), "Period".to_string()],
    );
    map.insert(
        "ExplanationOfBenefit.procedure.procedure".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Extension.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "code".to_string(), "CodeableConcept".to_string(), "Coding".to_string(),
            "ContactPoint".to_string(), "Count".to_string(), "date".to_string(),
            "dateTime".to_string(), "decimal".to_string(), "Distance".to_string(),
            "Duration".to_string(), "HumanName".to_string(), "id".to_string(),
            "Identifier".to_string(), "instant".to_string(), "integer".to_string(),
            "markdown".to_string(), "Meta".to_string(), "Money".to_string(), "oid"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "Reference"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "unsignedInt"
            .to_string(), "uri".to_string()
        ],
    );
    map.insert(
        "FamilyMemberHistory.age".to_string(),
        vec!["Age".to_string(), "Range".to_string(), "string".to_string()],
    );
    map.insert(
        "FamilyMemberHistory.born".to_string(),
        vec!["Date".to_string(), "Period".to_string(), "string".to_string()],
    );
    map.insert(
        "FamilyMemberHistory.condition.onset".to_string(),
        vec![
            "Age".to_string(), "Period".to_string(), "Range".to_string(), "string"
            .to_string()
        ],
    );
    map.insert(
        "FamilyMemberHistory.deceased".to_string(),
        vec![
            "Age".to_string(), "boolean".to_string(), "date".to_string(), "Range"
            .to_string(), "string".to_string()
        ],
    );
    map.insert(
        "Goal.start".to_string(),
        vec!["CodeableConcept".to_string(), "date".to_string()],
    );
    map.insert(
        "Goal.target.detail".to_string(),
        vec!["CodeableConcept".to_string(), "Quantity".to_string(), "Range".to_string()],
    );
    map.insert(
        "Goal.target.due".to_string(),
        vec!["Date".to_string(), "Duration".to_string()],
    );
    map.insert(
        "Group.characteristic.value".to_string(),
        vec![
            "Boolean".to_string(), "CodeableConcept".to_string(), "Quantity".to_string(),
            "Range".to_string()
        ],
    );
    map.insert(
        "GuidanceResponse.reason".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "ImplementationGuide.package.resource.source".to_string(),
        vec!["Reference".to_string(), "uri".to_string()],
    );
    map.insert(
        "Media.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "Medication.ingredient.item".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Medication.package.content.item".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "MedicationAdministration.dosage.rate".to_string(),
        vec!["Quantity".to_string(), "Ratio".to_string()],
    );
    map.insert(
        "MedicationAdministration.effective".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "MedicationAdministration.medication".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "MedicationDispense.medication".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "MedicationDispense.notDoneReason".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "MedicationRequest.medication".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "MedicationStatement.effective".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "MedicationStatement.medication".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "NutritionOrder.enteralFormula.administration.rate".to_string(),
        vec!["Quantity".to_string(), "Ratio".to_string()],
    );
    map.insert(
        "Observation.component.value".to_string(),
        vec![
            "Attachment".to_string(), "CodeableConcept".to_string(), "dateTime"
            .to_string(), "Period".to_string(), "Quantity".to_string(), "Range"
            .to_string(), "Ratio".to_string(), "SampledData".to_string(), "string"
            .to_string(), "time".to_string()
        ],
    );
    map.insert(
        "Observation.effective".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "Observation.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "CodeableConcept"
            .to_string(), "dateTime".to_string(), "Period".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "SampledData"
            .to_string(), "string".to_string(), "time".to_string()
        ],
    );
    map.insert(
        "OperationDefinition.parameter.binding.valueSet".to_string(),
        vec!["Reference".to_string(), "uri".to_string()],
    );
    map.insert(
        "Parameters.parameter.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "code".to_string(), "CodeableConcept".to_string(), "Coding".to_string(),
            "ContactPoint".to_string(), "Count".to_string(), "date".to_string(),
            "dateTime".to_string(), "decimal".to_string(), "Distance".to_string(),
            "Duration".to_string(), "HumanName".to_string(), "id".to_string(),
            "Identifier".to_string(), "instant".to_string(), "integer".to_string(),
            "markdown".to_string(), "Meta".to_string(), "Money".to_string(), "oid"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "Reference"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "unsignedInt"
            .to_string(), "uri".to_string()
        ],
    );
    map.insert(
        "Patient.deceased".to_string(),
        vec!["Boolean".to_string(), "dateTime".to_string()],
    );
    map.insert(
        "Patient.multipleBirth".to_string(),
        vec!["Boolean".to_string(), "integer".to_string()],
    );
    map.insert(
        "PlanDefinition.action.relatedAction.offset".to_string(),
        vec!["Duration".to_string(), "Range".to_string()],
    );
    map.insert(
        "PlanDefinition.action.timing".to_string(),
        vec![
            "DateTime".to_string(), "Duration".to_string(), "Period".to_string(), "Range"
            .to_string(), "Timing".to_string()
        ],
    );
    map.insert(
        "PlanDefinition.goal.target.detail".to_string(),
        vec!["CodeableConcept".to_string(), "Quantity".to_string(), "Range".to_string()],
    );
    map.insert(
        "Procedure.performed".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "ProcedureRequest.asNeeded".to_string(),
        vec!["Boolean".to_string(), "CodeableConcept".to_string()],
    );
    map.insert(
        "ProcedureRequest.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string(), "Timing".to_string()],
    );
    map.insert(
        "Provenance.agent.onBehalfOf".to_string(),
        vec!["Reference".to_string(), "uri".to_string()],
    );
    map.insert(
        "Provenance.agent.who".to_string(),
        vec!["Reference".to_string(), "uri".to_string()],
    );
    map.insert(
        "Provenance.entity.what".to_string(),
        vec!["Identifier".to_string(), "Reference".to_string(), "uri".to_string()],
    );
    map.insert(
        "Questionnaire.item.enableWhen.answer".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "Coding".to_string(), "date"
            .to_string(), "dateTime".to_string(), "decimal".to_string(), "integer"
            .to_string(), "Quantity".to_string(), "Reference".to_string(), "string"
            .to_string(), "time".to_string(), "uri".to_string()
        ],
    );
    map.insert(
        "Questionnaire.item.initial".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "Coding".to_string(), "date"
            .to_string(), "dateTime".to_string(), "decimal".to_string(), "integer"
            .to_string(), "Quantity".to_string(), "Reference".to_string(), "string"
            .to_string(), "time".to_string(), "uri".to_string()
        ],
    );
    map.insert(
        "Questionnaire.item.option.value".to_string(),
        vec![
            "Coding".to_string(), "date".to_string(), "integer".to_string(), "string"
            .to_string(), "time".to_string()
        ],
    );
    map.insert(
        "QuestionnaireResponse.item.answer.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "Coding".to_string(), "date"
            .to_string(), "dateTime".to_string(), "decimal".to_string(), "integer"
            .to_string(), "Quantity".to_string(), "Reference".to_string(), "string"
            .to_string(), "time".to_string(), "uri".to_string()
        ],
    );
    map.insert(
        "ReferralRequest.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "RequestGroup.action.relatedAction.offset".to_string(),
        vec!["Duration".to_string(), "Range".to_string()],
    );
    map.insert(
        "RequestGroup.action.timing".to_string(),
        vec![
            "DateTime".to_string(), "Duration".to_string(), "Period".to_string(), "Range"
            .to_string(), "Timing".to_string()
        ],
    );
    map.insert(
        "RequestGroup.reason".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "RiskAssessment.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "RiskAssessment.prediction.probability".to_string(),
        vec!["Decimal".to_string(), "Range".to_string()],
    );
    map.insert(
        "RiskAssessment.prediction.when".to_string(),
        vec!["Period".to_string(), "Range".to_string()],
    );
    map.insert(
        "RiskAssessment.reason".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Signature.onBehalfOf".to_string(),
        vec!["Reference".to_string(), "uri".to_string()],
    );
    map.insert(
        "Signature.who".to_string(),
        vec!["Reference".to_string(), "uri".to_string()],
    );
    map.insert(
        "Specimen.collection.collected".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "Specimen.container.additive".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Specimen.processing.time".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValue".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "code".to_string(), "CodeableConcept".to_string(), "Coding".to_string(),
            "ContactPoint".to_string(), "Count".to_string(), "date".to_string(),
            "dateTime".to_string(), "decimal".to_string(), "Distance".to_string(),
            "Duration".to_string(), "HumanName".to_string(), "id".to_string(),
            "Identifier".to_string(), "instant".to_string(), "integer".to_string(),
            "markdown".to_string(), "Meta".to_string(), "Money".to_string(), "oid"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "Reference"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "unsignedInt"
            .to_string(), "uri".to_string()
        ],
    );
    map.insert(
        "StructureMap.group.rule.target.parameter.value".to_string(),
        vec![
            "Boolean".to_string(), "decimal".to_string(), "id".to_string(), "integer"
            .to_string(), "string".to_string()
        ],
    );
    map.insert(
        "Substance.ingredient.substance".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "SupplyDelivery.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string(), "Timing".to_string()],
    );
    map.insert(
        "SupplyDelivery.suppliedItem.item".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "SupplyRequest.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string(), "Timing".to_string()],
    );
    map.insert(
        "SupplyRequest.orderedItem.item".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "SupplyRequest.reason".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Task.definition".to_string(),
        vec!["Reference".to_string(), "uri".to_string()],
    );
    map.insert(
        "Task.input.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "code".to_string(), "CodeableConcept".to_string(), "Coding".to_string(),
            "ContactPoint".to_string(), "Count".to_string(), "date".to_string(),
            "dateTime".to_string(), "decimal".to_string(), "Distance".to_string(),
            "Duration".to_string(), "HumanName".to_string(), "id".to_string(),
            "Identifier".to_string(), "instant".to_string(), "integer".to_string(),
            "markdown".to_string(), "Meta".to_string(), "Money".to_string(), "oid"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "Reference"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "unsignedInt"
            .to_string(), "uri".to_string()
        ],
    );
    map.insert(
        "Task.output.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "code".to_string(), "CodeableConcept".to_string(), "Coding".to_string(),
            "ContactPoint".to_string(), "Count".to_string(), "date".to_string(),
            "dateTime".to_string(), "decimal".to_string(), "Distance".to_string(),
            "Duration".to_string(), "HumanName".to_string(), "id".to_string(),
            "Identifier".to_string(), "instant".to_string(), "integer".to_string(),
            "markdown".to_string(), "Meta".to_string(), "Money".to_string(), "oid"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "Reference"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "unsignedInt"
            .to_string(), "uri".to_string()
        ],
    );
    map.insert(
        "Timing.repeat.bounds".to_string(),
        vec!["Duration".to_string(), "Period".to_string(), "Range".to_string()],
    );
    map.insert(
        "TriggerDefinition.eventTiming".to_string(),
        vec![
            "Date".to_string(), "dateTime".to_string(), "Reference".to_string(), "Timing"
            .to_string()
        ],
    );
    map.insert(
        "UsageContext.value".to_string(),
        vec!["CodeableConcept".to_string(), "Quantity".to_string(), "Range".to_string()],
    );
    map.insert(
        "ValueSet.expansion.parameter.value".to_string(),
        vec![
            "Boolean".to_string(), "code".to_string(), "decimal".to_string(), "integer"
            .to_string(), "string".to_string(), "uri".to_string()
        ],
    );
    map.insert(
        "VisionPrescription.reason".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map
}
