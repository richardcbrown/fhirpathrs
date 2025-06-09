use std::collections::HashMap;
pub fn choice_type_paths() -> HashMap<String, Vec<String>> {
    let mut map = HashMap::<String, Vec<String>>::new();
    map.insert(
        "ActivityDefinition.product".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "ActivityDefinition.subject".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "ActivityDefinition.timing".to_string(),
        vec![
            "Age".to_string(), "dateTime".to_string(), "Duration".to_string(), "Period"
            .to_string(), "Range".to_string(), "Timing".to_string()
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
        "AuditEvent.entity.detail.value".to_string(),
        vec!["Base64Binary".to_string(), "string".to_string()],
    );
    map.insert(
        "BiologicallyDerivedProduct.collection.collected".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "BiologicallyDerivedProduct.manipulation.time".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "BiologicallyDerivedProduct.processing.time".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
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
        "ChargeItem.product".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
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
        "Claim.supportingInfo.timing".to_string(),
        vec!["Date".to_string(), "Period".to_string()],
    );
    map.insert(
        "Claim.supportingInfo.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "Quantity".to_string(),
            "Reference".to_string(), "string".to_string()
        ],
    );
    map.insert(
        "ClaimResponse.addItem.location".to_string(),
        vec![
            "Address".to_string(), "CodeableConcept".to_string(), "Reference".to_string()
        ],
    );
    map.insert(
        "ClaimResponse.addItem.serviced".to_string(),
        vec!["Date".to_string(), "Period".to_string()],
    );
    map.insert(
        "ClinicalImpression.effective".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "CodeSystem.concept.property.value".to_string(),
        vec![
            "Boolean".to_string(), "code".to_string(), "Coding".to_string(), "dateTime"
            .to_string(), "decimal".to_string(), "integer".to_string(), "string"
            .to_string()
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
        vec!["Canonical".to_string(), "uri".to_string()],
    );
    map.insert(
        "ConceptMap.target".to_string(),
        vec!["Canonical".to_string(), "uri".to_string()],
    );
    map.insert(
        "Condition.abatement".to_string(),
        vec![
            "Age".to_string(), "dateTime".to_string(), "Period".to_string(), "Range"
            .to_string(), "string".to_string()
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
        "Contract.legallyBinding".to_string(),
        vec!["Attachment".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Contract.rule.content".to_string(),
        vec!["Attachment".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Contract.term.action.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string(), "Timing".to_string()],
    );
    map.insert(
        "Contract.term.asset.valuedItem.entity".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Contract.term.offer.answer.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "Coding".to_string(), "date"
            .to_string(), "dateTime".to_string(), "decimal".to_string(), "integer"
            .to_string(), "Quantity".to_string(), "Reference".to_string(), "string"
            .to_string(), "time".to_string(), "uri".to_string()
        ],
    );
    map.insert(
        "Contract.term.topic".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Contract.topic".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Coverage.costToBeneficiary.value".to_string(),
        vec!["Money".to_string(), "Quantity".to_string()],
    );
    map.insert(
        "CoverageEligibilityRequest.item.diagnosis.diagnosis".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "CoverageEligibilityRequest.serviced".to_string(),
        vec!["Date".to_string(), "Period".to_string()],
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.benefit.allowed".to_string(),
        vec!["Money".to_string(), "string".to_string(), "unsignedInt".to_string()],
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.benefit.used".to_string(),
        vec!["Money".to_string(), "string".to_string(), "unsignedInt".to_string()],
    );
    map.insert(
        "CoverageEligibilityResponse.serviced".to_string(),
        vec!["Date".to_string(), "Period".to_string()],
    );
    map.insert(
        "DataRequirement.dateFilter.value".to_string(),
        vec!["DateTime".to_string(), "Duration".to_string(), "Period".to_string()],
    );
    map.insert(
        "DataRequirement.subject".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "DetectedIssue.identified".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "DeviceDefinition.manufacturer".to_string(),
        vec!["Reference".to_string(), "string".to_string()],
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
        "DeviceRequest.parameter.value".to_string(),
        vec![
            "Boolean".to_string(), "CodeableConcept".to_string(), "Quantity".to_string(),
            "Range".to_string()
        ],
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
        "Dosage.asNeeded".to_string(),
        vec!["Boolean".to_string(), "CodeableConcept".to_string()],
    );
    map.insert(
        "Dosage.doseAndRate.dose".to_string(),
        vec!["Quantity".to_string(), "Range".to_string()],
    );
    map.insert(
        "Dosage.doseAndRate.rate".to_string(),
        vec!["Quantity".to_string(), "Range".to_string(), "Ratio".to_string()],
    );
    map.insert(
        "ElementDefinition.defaultValue".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "canonical".to_string(), "code".to_string(), "CodeableConcept".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Contributor".to_string(), "Count".to_string(),
            "DataRequirement".to_string(), "date".to_string(), "dateTime".to_string(),
            "decimal".to_string(), "Distance".to_string(), "Dosage".to_string(),
            "Duration".to_string(), "Expression".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "markdown".to_string(), "Meta".to_string(), "Money"
            .to_string(), "oid".to_string(), "ParameterDefinition".to_string(), "Period"
            .to_string(), "positiveInt".to_string(), "Quantity".to_string(), "Range"
            .to_string(), "Ratio".to_string(), "Reference".to_string(), "RelatedArtifact"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "TriggerDefinition"
            .to_string(), "unsignedInt".to_string(), "uri".to_string(), "url"
            .to_string(), "UsageContext".to_string(), "uuid".to_string()
        ],
    );
    map.insert(
        "ElementDefinition.example.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "canonical".to_string(), "code".to_string(), "CodeableConcept".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Contributor".to_string(), "Count".to_string(),
            "DataRequirement".to_string(), "date".to_string(), "dateTime".to_string(),
            "decimal".to_string(), "Distance".to_string(), "Dosage".to_string(),
            "Duration".to_string(), "Expression".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "markdown".to_string(), "Meta".to_string(), "Money"
            .to_string(), "oid".to_string(), "ParameterDefinition".to_string(), "Period"
            .to_string(), "positiveInt".to_string(), "Quantity".to_string(), "Range"
            .to_string(), "Ratio".to_string(), "Reference".to_string(), "RelatedArtifact"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "TriggerDefinition"
            .to_string(), "unsignedInt".to_string(), "uri".to_string(), "url"
            .to_string(), "UsageContext".to_string(), "uuid".to_string()
        ],
    );
    map.insert(
        "ElementDefinition.fixed".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "canonical".to_string(), "code".to_string(), "CodeableConcept".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Contributor".to_string(), "Count".to_string(),
            "DataRequirement".to_string(), "date".to_string(), "dateTime".to_string(),
            "decimal".to_string(), "Distance".to_string(), "Dosage".to_string(),
            "Duration".to_string(), "Expression".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "markdown".to_string(), "Meta".to_string(), "Money"
            .to_string(), "oid".to_string(), "ParameterDefinition".to_string(), "Period"
            .to_string(), "positiveInt".to_string(), "Quantity".to_string(), "Range"
            .to_string(), "Ratio".to_string(), "Reference".to_string(), "RelatedArtifact"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "TriggerDefinition"
            .to_string(), "unsignedInt".to_string(), "uri".to_string(), "url"
            .to_string(), "UsageContext".to_string(), "uuid".to_string()
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
            "canonical".to_string(), "code".to_string(), "CodeableConcept".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Contributor".to_string(), "Count".to_string(),
            "DataRequirement".to_string(), "date".to_string(), "dateTime".to_string(),
            "decimal".to_string(), "Distance".to_string(), "Dosage".to_string(),
            "Duration".to_string(), "Expression".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "markdown".to_string(), "Meta".to_string(), "Money"
            .to_string(), "oid".to_string(), "ParameterDefinition".to_string(), "Period"
            .to_string(), "positiveInt".to_string(), "Quantity".to_string(), "Range"
            .to_string(), "Ratio".to_string(), "Reference".to_string(), "RelatedArtifact"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "TriggerDefinition"
            .to_string(), "unsignedInt".to_string(), "uri".to_string(), "url"
            .to_string(), "UsageContext".to_string(), "uuid".to_string()
        ],
    );
    map.insert(
        "EventDefinition.subject".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "EvidenceVariable.characteristic.definition".to_string(),
        vec![
            "Canonical".to_string(), "CodeableConcept".to_string(), "DataRequirement"
            .to_string(), "Expression".to_string(), "Reference".to_string(),
            "TriggerDefinition".to_string()
        ],
    );
    map.insert(
        "EvidenceVariable.characteristic.participantEffective".to_string(),
        vec![
            "DateTime".to_string(), "Duration".to_string(), "Period".to_string(),
            "Timing".to_string()
        ],
    );
    map.insert(
        "ExplanationOfBenefit.accident.location".to_string(),
        vec!["Address".to_string(), "Reference".to_string()],
    );
    map.insert(
        "ExplanationOfBenefit.addItem.location".to_string(),
        vec![
            "Address".to_string(), "CodeableConcept".to_string(), "Reference".to_string()
        ],
    );
    map.insert(
        "ExplanationOfBenefit.addItem.serviced".to_string(),
        vec!["Date".to_string(), "Period".to_string()],
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
        "ExplanationOfBenefit.supportingInfo.timing".to_string(),
        vec!["Date".to_string(), "Period".to_string()],
    );
    map.insert(
        "ExplanationOfBenefit.supportingInfo.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "Quantity".to_string(),
            "Reference".to_string(), "string".to_string()
        ],
    );
    map.insert(
        "Extension.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "canonical".to_string(), "code".to_string(), "CodeableConcept".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Contributor".to_string(), "Count".to_string(),
            "DataRequirement".to_string(), "date".to_string(), "dateTime".to_string(),
            "decimal".to_string(), "Distance".to_string(), "Dosage".to_string(),
            "Duration".to_string(), "Expression".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "markdown".to_string(), "Meta".to_string(), "Money"
            .to_string(), "oid".to_string(), "ParameterDefinition".to_string(), "Period"
            .to_string(), "positiveInt".to_string(), "Quantity".to_string(), "Range"
            .to_string(), "Ratio".to_string(), "Reference".to_string(), "RelatedArtifact"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "TriggerDefinition"
            .to_string(), "unsignedInt".to_string(), "uri".to_string(), "url"
            .to_string(), "UsageContext".to_string(), "uuid".to_string()
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
        vec![
            "Boolean".to_string(), "CodeableConcept".to_string(), "integer".to_string(),
            "Quantity".to_string(), "Range".to_string(), "Ratio".to_string(), "string"
            .to_string()
        ],
    );
    map.insert(
        "Goal.target.due".to_string(),
        vec!["Date".to_string(), "Duration".to_string()],
    );
    map.insert(
        "Group.characteristic.value".to_string(),
        vec![
            "Boolean".to_string(), "CodeableConcept".to_string(), "Quantity".to_string(),
            "Range".to_string(), "Reference".to_string()
        ],
    );
    map.insert(
        "GuidanceResponse.module".to_string(),
        vec!["Canonical".to_string(), "CodeableConcept".to_string(), "uri".to_string()],
    );
    map.insert(
        "Immunization.occurrence".to_string(),
        vec!["DateTime".to_string(), "string".to_string()],
    );
    map.insert(
        "Immunization.protocolApplied.doseNumber".to_string(),
        vec!["PositiveInt".to_string(), "string".to_string()],
    );
    map.insert(
        "Immunization.protocolApplied.seriesDoses".to_string(),
        vec!["PositiveInt".to_string(), "string".to_string()],
    );
    map.insert(
        "ImmunizationEvaluation.doseNumber".to_string(),
        vec!["PositiveInt".to_string(), "string".to_string()],
    );
    map.insert(
        "ImmunizationEvaluation.seriesDoses".to_string(),
        vec!["PositiveInt".to_string(), "string".to_string()],
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.doseNumber".to_string(),
        vec!["PositiveInt".to_string(), "string".to_string()],
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.seriesDoses".to_string(),
        vec!["PositiveInt".to_string(), "string".to_string()],
    );
    map.insert(
        "ImplementationGuide.definition.page.name".to_string(),
        vec!["Reference".to_string(), "url".to_string()],
    );
    map.insert(
        "ImplementationGuide.definition.resource.example".to_string(),
        vec!["Boolean".to_string(), "canonical".to_string()],
    );
    map.insert(
        "ImplementationGuide.manifest.resource.example".to_string(),
        vec!["Boolean".to_string(), "canonical".to_string()],
    );
    map.insert(
        "Invoice.lineItem.chargeItem".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Library.subject".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Measure.subject".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Media.created".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "Medication.ingredient.item".to_string(),
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
        "MedicationDispense.statusReason".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.indication".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.patientCharacteristics.characteristic"
            .to_string(),
        vec!["CodeableConcept".to_string(), "Quantity".to_string()],
    );
    map.insert(
        "MedicationKnowledge.drugCharacteristic.value".to_string(),
        vec![
            "Base64Binary".to_string(), "CodeableConcept".to_string(), "Quantity"
            .to_string(), "string".to_string()
        ],
    );
    map.insert(
        "MedicationKnowledge.ingredient.item".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "MedicationRequest.medication".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "MedicationRequest.reported".to_string(),
        vec!["Boolean".to_string(), "Reference".to_string()],
    );
    map.insert(
        "MedicationRequest.substitution.allowed".to_string(),
        vec!["Boolean".to_string(), "CodeableConcept".to_string()],
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
        "MedicinalProduct.specialDesignation.indication".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "MedicinalProductAuthorization.procedure.date".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "MedicinalProductContraindication.otherTherapy.medication".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "MedicinalProductIndication.otherTherapy.medication".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "MedicinalProductInteraction.interactant.item".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "MessageDefinition.event".to_string(),
        vec!["Coding".to_string(), "uri".to_string()],
    );
    map.insert(
        "MessageHeader.event".to_string(),
        vec!["Coding".to_string(), "uri".to_string()],
    );
    map.insert(
        "NutritionOrder.enteralFormula.administration.rate".to_string(),
        vec!["Quantity".to_string(), "Ratio".to_string()],
    );
    map.insert(
        "Observation.component.value".to_string(),
        vec![
            "Boolean".to_string(), "CodeableConcept".to_string(), "dateTime".to_string(),
            "integer".to_string(), "Period".to_string(), "Quantity".to_string(), "Range"
            .to_string(), "Ratio".to_string(), "SampledData".to_string(), "string"
            .to_string(), "time".to_string()
        ],
    );
    map.insert(
        "Observation.effective".to_string(),
        vec![
            "DateTime".to_string(), "instant".to_string(), "Period".to_string(), "Timing"
            .to_string()
        ],
    );
    map.insert(
        "Observation.value".to_string(),
        vec![
            "Boolean".to_string(), "CodeableConcept".to_string(), "dateTime".to_string(),
            "integer".to_string(), "Period".to_string(), "Quantity".to_string(), "Range"
            .to_string(), "Ratio".to_string(), "SampledData".to_string(), "string"
            .to_string(), "time".to_string()
        ],
    );
    map.insert(
        "Parameters.parameter.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "canonical".to_string(), "code".to_string(), "CodeableConcept".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Contributor".to_string(), "Count".to_string(),
            "DataRequirement".to_string(), "date".to_string(), "dateTime".to_string(),
            "decimal".to_string(), "Distance".to_string(), "Dosage".to_string(),
            "Duration".to_string(), "Expression".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "markdown".to_string(), "Meta".to_string(), "Money"
            .to_string(), "oid".to_string(), "ParameterDefinition".to_string(), "Period"
            .to_string(), "positiveInt".to_string(), "Quantity".to_string(), "Range"
            .to_string(), "Ratio".to_string(), "Reference".to_string(), "RelatedArtifact"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "TriggerDefinition"
            .to_string(), "unsignedInt".to_string(), "uri".to_string(), "url"
            .to_string(), "UsageContext".to_string(), "uuid".to_string()
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
        "PlanDefinition.action.definition".to_string(),
        vec!["Canonical".to_string(), "uri".to_string()],
    );
    map.insert(
        "PlanDefinition.action.relatedAction.offset".to_string(),
        vec!["Duration".to_string(), "Range".to_string()],
    );
    map.insert(
        "PlanDefinition.action.subject".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "PlanDefinition.action.timing".to_string(),
        vec![
            "Age".to_string(), "dateTime".to_string(), "Duration".to_string(), "Period"
            .to_string(), "Range".to_string(), "Timing".to_string()
        ],
    );
    map.insert(
        "PlanDefinition.goal.target.detail".to_string(),
        vec!["CodeableConcept".to_string(), "Quantity".to_string(), "Range".to_string()],
    );
    map.insert(
        "PlanDefinition.subject".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Population.age".to_string(),
        vec!["CodeableConcept".to_string(), "Range".to_string()],
    );
    map.insert(
        "Procedure.performed".to_string(),
        vec![
            "Age".to_string(), "dateTime".to_string(), "Period".to_string(), "Range"
            .to_string(), "string".to_string()
        ],
    );
    map.insert(
        "Provenance.occurred".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "Questionnaire.item.answerOption.value".to_string(),
        vec![
            "Coding".to_string(), "date".to_string(), "integer".to_string(), "Reference"
            .to_string(), "string".to_string(), "time".to_string()
        ],
    );
    map.insert(
        "Questionnaire.item.enableWhen.answer".to_string(),
        vec![
            "Boolean".to_string(), "Coding".to_string(), "date".to_string(), "dateTime"
            .to_string(), "decimal".to_string(), "integer".to_string(), "Quantity"
            .to_string(), "Reference".to_string(), "string".to_string(), "time"
            .to_string()
        ],
    );
    map.insert(
        "Questionnaire.item.initial.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "Coding".to_string(), "date"
            .to_string(), "dateTime".to_string(), "decimal".to_string(), "integer"
            .to_string(), "Quantity".to_string(), "Reference".to_string(), "string"
            .to_string(), "time".to_string(), "uri".to_string()
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
        "RequestGroup.action.relatedAction.offset".to_string(),
        vec!["Duration".to_string(), "Range".to_string()],
    );
    map.insert(
        "RequestGroup.action.timing".to_string(),
        vec![
            "Age".to_string(), "dateTime".to_string(), "Duration".to_string(), "Period"
            .to_string(), "Range".to_string(), "Timing".to_string()
        ],
    );
    map.insert(
        "ResearchDefinition.subject".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "ResearchElementDefinition.characteristic.definition".to_string(),
        vec![
            "Canonical".to_string(), "CodeableConcept".to_string(), "DataRequirement"
            .to_string(), "Expression".to_string()
        ],
    );
    map.insert(
        "ResearchElementDefinition.characteristic.participantEffective".to_string(),
        vec![
            "DateTime".to_string(), "Duration".to_string(), "Period".to_string(),
            "Timing".to_string()
        ],
    );
    map.insert(
        "ResearchElementDefinition.characteristic.studyEffective".to_string(),
        vec![
            "DateTime".to_string(), "Duration".to_string(), "Period".to_string(),
            "Timing".to_string()
        ],
    );
    map.insert(
        "ResearchElementDefinition.subject".to_string(),
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
        "ServiceRequest.asNeeded".to_string(),
        vec!["Boolean".to_string(), "CodeableConcept".to_string()],
    );
    map.insert(
        "ServiceRequest.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string(), "Timing".to_string()],
    );
    map.insert(
        "ServiceRequest.quantity".to_string(),
        vec!["Quantity".to_string(), "Range".to_string(), "Ratio".to_string()],
    );
    map.insert(
        "Specimen.collection.collected".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "Specimen.collection.fastingStatus".to_string(),
        vec!["CodeableConcept".to_string(), "Duration".to_string()],
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
        "SpecimenDefinition.typeTested.container.additive.additive".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.minimumVolume".to_string(),
        vec!["Quantity".to_string(), "string".to_string()],
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValue".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "canonical".to_string(), "code".to_string(), "CodeableConcept".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Contributor".to_string(), "Count".to_string(),
            "DataRequirement".to_string(), "date".to_string(), "dateTime".to_string(),
            "decimal".to_string(), "Distance".to_string(), "Dosage".to_string(),
            "Duration".to_string(), "Expression".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "markdown".to_string(), "Meta".to_string(), "Money"
            .to_string(), "oid".to_string(), "ParameterDefinition".to_string(), "Period"
            .to_string(), "positiveInt".to_string(), "Quantity".to_string(), "Range"
            .to_string(), "Ratio".to_string(), "Reference".to_string(), "RelatedArtifact"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "TriggerDefinition"
            .to_string(), "unsignedInt".to_string(), "uri".to_string(), "url"
            .to_string(), "UsageContext".to_string(), "uuid".to_string()
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
        "SubstanceAmount.amount".to_string(),
        vec!["Quantity".to_string(), "Range".to_string(), "string".to_string()],
    );
    map.insert(
        "SubstanceReferenceInformation.target.amount".to_string(),
        vec!["Quantity".to_string(), "Range".to_string(), "string".to_string()],
    );
    map.insert(
        "SubstanceSpecification.moiety.amount".to_string(),
        vec!["Quantity".to_string(), "string".to_string()],
    );
    map.insert(
        "SubstanceSpecification.property.amount".to_string(),
        vec!["Quantity".to_string(), "string".to_string()],
    );
    map.insert(
        "SubstanceSpecification.property.definingSubstance".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "SubstanceSpecification.relationship.amount".to_string(),
        vec![
            "Quantity".to_string(), "Range".to_string(), "Ratio".to_string(), "string"
            .to_string()
        ],
    );
    map.insert(
        "SubstanceSpecification.relationship.substance".to_string(),
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
        "SupplyRequest.item".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "SupplyRequest.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string(), "Timing".to_string()],
    );
    map.insert(
        "SupplyRequest.parameter.value".to_string(),
        vec![
            "Boolean".to_string(), "CodeableConcept".to_string(), "Quantity".to_string(),
            "Range".to_string()
        ],
    );
    map.insert(
        "Task.input.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "canonical".to_string(), "code".to_string(), "CodeableConcept".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Contributor".to_string(), "Count".to_string(),
            "DataRequirement".to_string(), "date".to_string(), "dateTime".to_string(),
            "decimal".to_string(), "Distance".to_string(), "Dosage".to_string(),
            "Duration".to_string(), "Expression".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "markdown".to_string(), "Meta".to_string(), "Money"
            .to_string(), "oid".to_string(), "ParameterDefinition".to_string(), "Period"
            .to_string(), "positiveInt".to_string(), "Quantity".to_string(), "Range"
            .to_string(), "Ratio".to_string(), "Reference".to_string(), "RelatedArtifact"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "TriggerDefinition"
            .to_string(), "unsignedInt".to_string(), "uri".to_string(), "url"
            .to_string(), "UsageContext".to_string(), "uuid".to_string()
        ],
    );
    map.insert(
        "Task.output.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "canonical".to_string(), "code".to_string(), "CodeableConcept".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Contributor".to_string(), "Count".to_string(),
            "DataRequirement".to_string(), "date".to_string(), "dateTime".to_string(),
            "decimal".to_string(), "Distance".to_string(), "Dosage".to_string(),
            "Duration".to_string(), "Expression".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "markdown".to_string(), "Meta".to_string(), "Money"
            .to_string(), "oid".to_string(), "ParameterDefinition".to_string(), "Period"
            .to_string(), "positiveInt".to_string(), "Quantity".to_string(), "Range"
            .to_string(), "Ratio".to_string(), "Reference".to_string(), "RelatedArtifact"
            .to_string(), "SampledData".to_string(), "Signature".to_string(), "string"
            .to_string(), "time".to_string(), "Timing".to_string(), "TriggerDefinition"
            .to_string(), "unsignedInt".to_string(), "uri".to_string(), "url"
            .to_string(), "UsageContext".to_string(), "uuid".to_string()
        ],
    );
    map.insert(
        "Timing.repeat.bounds".to_string(),
        vec!["Duration".to_string(), "Period".to_string(), "Range".to_string()],
    );
    map.insert(
        "TriggerDefinition.timing".to_string(),
        vec![
            "Date".to_string(), "dateTime".to_string(), "Reference".to_string(), "Timing"
            .to_string()
        ],
    );
    map.insert(
        "UsageContext.value".to_string(),
        vec![
            "CodeableConcept".to_string(), "Quantity".to_string(), "Range".to_string(),
            "Reference".to_string()
        ],
    );
    map.insert(
        "ValueSet.expansion.parameter.value".to_string(),
        vec![
            "Boolean".to_string(), "code".to_string(), "dateTime".to_string(), "decimal"
            .to_string(), "integer".to_string(), "string".to_string(), "uri".to_string()
        ],
    );
    map
}
