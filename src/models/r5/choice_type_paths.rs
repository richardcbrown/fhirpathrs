use std::collections::HashMap;
pub fn choice_type_paths() -> HashMap<String, Vec<String>> {
    let mut map = HashMap::<String, Vec<String>>::new();
    map.insert(
        "ActivityDefinition.asNeeded".to_string(),
        vec!["Boolean".to_string(), "CodeableConcept".to_string()],
    );
    map.insert(
        "ActivityDefinition.product".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "ActivityDefinition.subject".to_string(),
        vec![
            "Canonical".to_string(), "CodeableConcept".to_string(), "Reference"
            .to_string()
        ],
    );
    map.insert(
        "ActivityDefinition.timing".to_string(),
        vec![
            "Age".to_string(), "Duration".to_string(), "Range".to_string(), "Timing"
            .to_string()
        ],
    );
    map.insert(
        "ActivityDefinition.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "ActorDefinition.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "AdministrableProductDefinition.property.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "CodeableConcept"
            .to_string(), "date".to_string(), "markdown".to_string(), "Quantity"
            .to_string(), "Reference".to_string()
        ],
    );
    map.insert(
        "AdverseEvent.contributingFactor.item".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "AdverseEvent.mitigatingAction.item".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "AdverseEvent.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string(), "Timing".to_string()],
    );
    map.insert(
        "AdverseEvent.preventiveAction.item".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "AdverseEvent.supportingInfo.item".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "AdverseEvent.suspectEntity.instance".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
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
        "ArtifactAssessment.artifact".to_string(),
        vec!["Canonical".to_string(), "Reference".to_string(), "uri".to_string()],
    );
    map.insert(
        "ArtifactAssessment.citeAs".to_string(),
        vec!["Markdown".to_string(), "Reference".to_string()],
    );
    map.insert(
        "AuditEvent.agent.network".to_string(),
        vec!["Reference".to_string(), "string".to_string(), "uri".to_string()],
    );
    map.insert(
        "AuditEvent.entity.detail.value".to_string(),
        vec![
            "Base64Binary".to_string(), "boolean".to_string(), "CodeableConcept"
            .to_string(), "dateTime".to_string(), "integer".to_string(), "Period"
            .to_string(), "Quantity".to_string(), "Range".to_string(), "Ratio"
            .to_string(), "string".to_string(), "time".to_string()
        ],
    );
    map.insert(
        "AuditEvent.occurred".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "BiologicallyDerivedProduct.collection.collected".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "BiologicallyDerivedProduct.property.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "CodeableConcept"
            .to_string(), "integer".to_string(), "Period".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "string".to_string()
        ],
    );
    map.insert(
        "CanonicalResource.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "CapabilityStatement.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "CareTeam.participant.coverage".to_string(),
        vec!["Period".to_string(), "Timing".to_string()],
    );
    map.insert(
        "ChargeItem.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string(), "Timing".to_string()],
    );
    map.insert(
        "ChargeItemDefinition.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "Citation.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
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
        "Claim.event.when".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
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
            "Attachment".to_string(), "boolean".to_string(), "Identifier".to_string(),
            "Quantity".to_string(), "Reference".to_string(), "string".to_string()
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
        "ClaimResponse.event.when".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "ClinicalImpression.effective".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "ClinicalUseDefinition.indication.duration".to_string(),
        vec!["Range".to_string(), "string".to_string()],
    );
    map.insert(
        "ClinicalUseDefinition.interaction.interactant.item".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
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
        "CodeSystem.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "Communication.payload.content".to_string(),
        vec![
            "Attachment".to_string(), "CodeableConcept".to_string(), "Reference"
            .to_string()
        ],
    );
    map.insert(
        "CommunicationRequest.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "CommunicationRequest.payload.content".to_string(),
        vec![
            "Attachment".to_string(), "CodeableConcept".to_string(), "Reference"
            .to_string()
        ],
    );
    map.insert(
        "CompartmentDefinition.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "ConceptMap.group.element.target.dependsOn.value".to_string(),
        vec![
            "Boolean".to_string(), "code".to_string(), "Coding".to_string(), "Quantity"
            .to_string(), "string".to_string()
        ],
    );
    map.insert(
        "ConceptMap.group.element.target.property.value".to_string(),
        vec![
            "Boolean".to_string(), "code".to_string(), "Coding".to_string(), "dateTime"
            .to_string(), "decimal".to_string(), "integer".to_string(), "string"
            .to_string()
        ],
    );
    map.insert(
        "ConceptMap.sourceScope".to_string(),
        vec!["Canonical".to_string(), "uri".to_string()],
    );
    map.insert(
        "ConceptMap.targetScope".to_string(),
        vec!["Canonical".to_string(), "uri".to_string()],
    );
    map.insert(
        "ConceptMap.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
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
        "ConditionDefinition.precondition.value".to_string(),
        vec!["CodeableConcept".to_string(), "Quantity".to_string()],
    );
    map.insert(
        "ConditionDefinition.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
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
        "CoverageEligibilityRequest.event.when".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
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
        "CoverageEligibilityResponse.event.when".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
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
        "DataRequirement.valueFilter.value".to_string(),
        vec!["DateTime".to_string(), "Duration".to_string(), "Period".to_string()],
    );
    map.insert(
        "DetectedIssue.identified".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "Device.property.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "CodeableConcept"
            .to_string(), "integer".to_string(), "Quantity".to_string(), "Range"
            .to_string(), "string".to_string()
        ],
    );
    map.insert(
        "DeviceDefinition.property.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "CodeableConcept"
            .to_string(), "integer".to_string(), "Quantity".to_string(), "Range"
            .to_string(), "string".to_string()
        ],
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
        "DeviceUsage.timing".to_string(),
        vec!["DateTime".to_string(), "Period".to_string(), "Timing".to_string()],
    );
    map.insert(
        "DiagnosticReport.effective".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "DocumentReference.content.profile.value".to_string(),
        vec!["Canonical".to_string(), "Coding".to_string(), "uri".to_string()],
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
            "Attachment".to_string(), "Availability".to_string(), "base64Binary"
            .to_string(), "boolean".to_string(), "canonical".to_string(), "code"
            .to_string(), "CodeableConcept".to_string(), "CodeableReference".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Count".to_string(), "DataRequirement".to_string(), "date"
            .to_string(), "dateTime".to_string(), "decimal".to_string(), "Distance"
            .to_string(), "Dosage".to_string(), "Duration".to_string(), "Expression"
            .to_string(), "ExtendedContactDetail".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "integer64".to_string(), "markdown".to_string(), "Meta"
            .to_string(), "Money".to_string(), "oid".to_string(), "ParameterDefinition"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "RatioRange"
            .to_string(), "Reference".to_string(), "RelatedArtifact".to_string(),
            "SampledData".to_string(), "Signature".to_string(), "string".to_string(),
            "time".to_string(), "Timing".to_string(), "TriggerDefinition".to_string(),
            "unsignedInt".to_string(), "uri".to_string(), "url".to_string(),
            "UsageContext".to_string(), "uuid".to_string()
        ],
    );
    map.insert(
        "ElementDefinition.example.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "Availability".to_string(), "base64Binary"
            .to_string(), "boolean".to_string(), "canonical".to_string(), "code"
            .to_string(), "CodeableConcept".to_string(), "CodeableReference".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Count".to_string(), "DataRequirement".to_string(), "date"
            .to_string(), "dateTime".to_string(), "decimal".to_string(), "Distance"
            .to_string(), "Dosage".to_string(), "Duration".to_string(), "Expression"
            .to_string(), "ExtendedContactDetail".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "integer64".to_string(), "markdown".to_string(), "Meta"
            .to_string(), "Money".to_string(), "oid".to_string(), "ParameterDefinition"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "RatioRange"
            .to_string(), "Reference".to_string(), "RelatedArtifact".to_string(),
            "SampledData".to_string(), "Signature".to_string(), "string".to_string(),
            "time".to_string(), "Timing".to_string(), "TriggerDefinition".to_string(),
            "unsignedInt".to_string(), "uri".to_string(), "url".to_string(),
            "UsageContext".to_string(), "uuid".to_string()
        ],
    );
    map.insert(
        "ElementDefinition.fixed".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "Availability".to_string(), "base64Binary"
            .to_string(), "boolean".to_string(), "canonical".to_string(), "code"
            .to_string(), "CodeableConcept".to_string(), "CodeableReference".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Count".to_string(), "DataRequirement".to_string(), "date"
            .to_string(), "dateTime".to_string(), "decimal".to_string(), "Distance"
            .to_string(), "Dosage".to_string(), "Duration".to_string(), "Expression"
            .to_string(), "ExtendedContactDetail".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "integer64".to_string(), "markdown".to_string(), "Meta"
            .to_string(), "Money".to_string(), "oid".to_string(), "ParameterDefinition"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "RatioRange"
            .to_string(), "Reference".to_string(), "RelatedArtifact".to_string(),
            "SampledData".to_string(), "Signature".to_string(), "string".to_string(),
            "time".to_string(), "Timing".to_string(), "TriggerDefinition".to_string(),
            "unsignedInt".to_string(), "uri".to_string(), "url".to_string(),
            "UsageContext".to_string(), "uuid".to_string()
        ],
    );
    map.insert(
        "ElementDefinition.maxValue".to_string(),
        vec![
            "Date".to_string(), "dateTime".to_string(), "decimal".to_string(), "instant"
            .to_string(), "integer".to_string(), "integer64".to_string(), "positiveInt"
            .to_string(), "Quantity".to_string(), "time".to_string(), "unsignedInt"
            .to_string()
        ],
    );
    map.insert(
        "ElementDefinition.minValue".to_string(),
        vec![
            "Date".to_string(), "dateTime".to_string(), "decimal".to_string(), "instant"
            .to_string(), "integer".to_string(), "integer64".to_string(), "positiveInt"
            .to_string(), "Quantity".to_string(), "time".to_string(), "unsignedInt"
            .to_string()
        ],
    );
    map.insert(
        "ElementDefinition.pattern".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "Availability".to_string(), "base64Binary"
            .to_string(), "boolean".to_string(), "canonical".to_string(), "code"
            .to_string(), "CodeableConcept".to_string(), "CodeableReference".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Count".to_string(), "DataRequirement".to_string(), "date"
            .to_string(), "dateTime".to_string(), "decimal".to_string(), "Distance"
            .to_string(), "Dosage".to_string(), "Duration".to_string(), "Expression"
            .to_string(), "ExtendedContactDetail".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "integer64".to_string(), "markdown".to_string(), "Meta"
            .to_string(), "Money".to_string(), "oid".to_string(), "ParameterDefinition"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "RatioRange"
            .to_string(), "Reference".to_string(), "RelatedArtifact".to_string(),
            "SampledData".to_string(), "Signature".to_string(), "string".to_string(),
            "time".to_string(), "Timing".to_string(), "TriggerDefinition".to_string(),
            "unsignedInt".to_string(), "uri".to_string(), "url".to_string(),
            "UsageContext".to_string(), "uuid".to_string()
        ],
    );
    map.insert(
        "EventDefinition.subject".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "EventDefinition.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "Evidence.citeAs".to_string(),
        vec!["Markdown".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Evidence.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "EvidenceReport.citeAs".to_string(),
        vec!["Markdown".to_string(), "Reference".to_string()],
    );
    map.insert(
        "EvidenceReport.subject.characteristic.value".to_string(),
        vec![
            "Boolean".to_string(), "CodeableConcept".to_string(), "Quantity".to_string(),
            "Range".to_string(), "Reference".to_string()
        ],
    );
    map.insert(
        "EvidenceVariable.category.value".to_string(),
        vec!["CodeableConcept".to_string(), "Quantity".to_string(), "Range".to_string()],
    );
    map.insert(
        "EvidenceVariable.characteristic.definitionByTypeAndValue.value".to_string(),
        vec![
            "Boolean".to_string(), "CodeableConcept".to_string(), "id".to_string(),
            "Quantity".to_string(), "Range".to_string(), "Reference".to_string()
        ],
    );
    map.insert(
        "EvidenceVariable.characteristic.duration".to_string(),
        vec!["Quantity".to_string(), "Range".to_string()],
    );
    map.insert(
        "EvidenceVariable.characteristic.instances".to_string(),
        vec!["Quantity".to_string(), "Range".to_string()],
    );
    map.insert(
        "EvidenceVariable.characteristic.timeFromEvent.event".to_string(),
        vec![
            "CodeableConcept".to_string(), "dateTime".to_string(), "id".to_string(),
            "Reference".to_string()
        ],
    );
    map.insert(
        "EvidenceVariable.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "ExampleScenario.instance.structureProfile".to_string(),
        vec!["Canonical".to_string(), "uri".to_string()],
    );
    map.insert(
        "ExampleScenario.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
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
        "ExplanationOfBenefit.event.when".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
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
            "Attachment".to_string(), "boolean".to_string(), "Identifier".to_string(),
            "Quantity".to_string(), "Reference".to_string(), "string".to_string()
        ],
    );
    map.insert(
        "Extension.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "Availability".to_string(), "base64Binary"
            .to_string(), "boolean".to_string(), "canonical".to_string(), "code"
            .to_string(), "CodeableConcept".to_string(), "CodeableReference".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Count".to_string(), "DataRequirement".to_string(), "date"
            .to_string(), "dateTime".to_string(), "decimal".to_string(), "Distance"
            .to_string(), "Dosage".to_string(), "Duration".to_string(), "Expression"
            .to_string(), "ExtendedContactDetail".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "integer64".to_string(), "markdown".to_string(), "Meta"
            .to_string(), "Money".to_string(), "oid".to_string(), "ParameterDefinition"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "RatioRange"
            .to_string(), "Reference".to_string(), "RelatedArtifact".to_string(),
            "SampledData".to_string(), "Signature".to_string(), "string".to_string(),
            "time".to_string(), "Timing".to_string(), "TriggerDefinition".to_string(),
            "unsignedInt".to_string(), "uri".to_string(), "url".to_string(),
            "UsageContext".to_string(), "uuid".to_string()
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
        "FamilyMemberHistory.procedure.performed".to_string(),
        vec![
            "Age".to_string(), "dateTime".to_string(), "Period".to_string(), "Range"
            .to_string(), "string".to_string()
        ],
    );
    map.insert(
        "GenomicStudy.analysis.input.generatedBy".to_string(),
        vec!["Identifier".to_string(), "Reference".to_string()],
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
        "GraphDefinition.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
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
        "ImplementationGuide.definition.page.source".to_string(),
        vec!["Markdown".to_string(), "string".to_string(), "url".to_string()],
    );
    map.insert(
        "ImplementationGuide.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "Ingredient.substance.strength.concentration".to_string(),
        vec![
            "CodeableConcept".to_string(), "Quantity".to_string(), "Ratio".to_string(),
            "RatioRange".to_string()
        ],
    );
    map.insert(
        "Ingredient.substance.strength.presentation".to_string(),
        vec![
            "CodeableConcept".to_string(), "Quantity".to_string(), "Ratio".to_string(),
            "RatioRange".to_string()
        ],
    );
    map.insert(
        "Ingredient.substance.strength.referenceStrength.strength".to_string(),
        vec!["Quantity".to_string(), "Ratio".to_string(), "RatioRange".to_string()],
    );
    map.insert(
        "InventoryItem.characteristic.value".to_string(),
        vec![
            "Address".to_string(), "Annotation".to_string(), "boolean".to_string(),
            "CodeableConcept".to_string(), "dateTime".to_string(), "decimal".to_string(),
            "Duration".to_string(), "integer".to_string(), "Quantity".to_string(),
            "Range".to_string(), "Ratio".to_string(), "string".to_string(), "url"
            .to_string()
        ],
    );
    map.insert(
        "Invoice.lineItem.chargeItem".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Invoice.lineItem.serviced".to_string(),
        vec!["Date".to_string(), "Period".to_string()],
    );
    map.insert(
        "Invoice.period".to_string(),
        vec!["Date".to_string(), "Period".to_string()],
    );
    map.insert(
        "Library.subject".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Library.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "ManufacturedItemDefinition.property.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "CodeableConcept"
            .to_string(), "date".to_string(), "markdown".to_string(), "Quantity"
            .to_string(), "Reference".to_string()
        ],
    );
    map.insert(
        "Measure.group.subject".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Measure.subject".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Measure.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "MeasureReport.group.measureScore".to_string(),
        vec![
            "CodeableConcept".to_string(), "dateTime".to_string(), "Duration"
            .to_string(), "Period".to_string(), "Quantity".to_string(), "Range"
            .to_string()
        ],
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.component.value".to_string(),
        vec![
            "Boolean".to_string(), "CodeableConcept".to_string(), "Quantity".to_string(),
            "Range".to_string(), "Reference".to_string()
        ],
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.measureScore".to_string(),
        vec![
            "CodeableConcept".to_string(), "dateTime".to_string(), "Duration"
            .to_string(), "Period".to_string(), "Quantity".to_string(), "Range"
            .to_string()
        ],
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.value".to_string(),
        vec![
            "Boolean".to_string(), "CodeableConcept".to_string(), "Quantity".to_string(),
            "Range".to_string(), "Reference".to_string()
        ],
    );
    map.insert(
        "Medication.ingredient.strength".to_string(),
        vec!["CodeableConcept".to_string(), "Quantity".to_string(), "Ratio".to_string()],
    );
    map.insert(
        "MedicationAdministration.dosage.rate".to_string(),
        vec!["Quantity".to_string(), "Ratio".to_string()],
    );
    map.insert(
        "MedicationAdministration.occurence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string(), "Timing".to_string()],
    );
    map.insert(
        "MedicationKnowledge.cost.cost".to_string(),
        vec!["CodeableConcept".to_string(), "Money".to_string()],
    );
    map.insert(
        "MedicationKnowledge.definitional.drugCharacteristic.value".to_string(),
        vec![
            "Attachment".to_string(), "base64Binary".to_string(), "CodeableConcept"
            .to_string(), "Quantity".to_string(), "string".to_string()
        ],
    );
    map.insert(
        "MedicationKnowledge.definitional.ingredient.strength".to_string(),
        vec!["CodeableConcept".to_string(), "Quantity".to_string(), "Ratio".to_string()],
    );
    map.insert(
        "MedicationKnowledge.indicationGuideline.dosingGuideline.patientCharacteristic.value"
            .to_string(),
        vec!["CodeableConcept".to_string(), "Quantity".to_string(), "Range".to_string()],
    );
    map.insert(
        "MedicationKnowledge.medicineClassification.source".to_string(),
        vec!["String".to_string(), "uri".to_string()],
    );
    map.insert(
        "MedicationKnowledge.storageGuideline.environmentalSetting.value".to_string(),
        vec!["CodeableConcept".to_string(), "Quantity".to_string(), "Range".to_string()],
    );
    map.insert(
        "MedicationRequest.substitution.allowed".to_string(),
        vec!["Boolean".to_string(), "CodeableConcept".to_string()],
    );
    map.insert(
        "MedicationStatement.effective".to_string(),
        vec!["DateTime".to_string(), "Period".to_string(), "Timing".to_string()],
    );
    map.insert(
        "MedicinalProductDefinition.characteristic.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "CodeableConcept"
            .to_string(), "date".to_string(), "integer".to_string(), "markdown"
            .to_string(), "Quantity".to_string()
        ],
    );
    map.insert(
        "MessageDefinition.event".to_string(),
        vec!["Coding".to_string(), "uri".to_string()],
    );
    map.insert(
        "MessageDefinition.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "MessageHeader.destination.endpoint".to_string(),
        vec!["Reference".to_string(), "url".to_string()],
    );
    map.insert(
        "MessageHeader.event".to_string(),
        vec!["Canonical".to_string(), "Coding".to_string()],
    );
    map.insert(
        "MessageHeader.source.endpoint".to_string(),
        vec!["Reference".to_string(), "url".to_string()],
    );
    map.insert(
        "MetadataResource.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "MolecularSequence.relative.startingSequence.sequence".to_string(),
        vec![
            "CodeableConcept".to_string(), "Reference".to_string(), "string".to_string()
        ],
    );
    map.insert(
        "NamingSystem.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "NutritionIntake.occurrence".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "NutritionIntake.reported".to_string(),
        vec!["Boolean".to_string(), "Reference".to_string()],
    );
    map.insert(
        "NutritionOrder.enteralFormula.administration.rate".to_string(),
        vec!["Quantity".to_string(), "Ratio".to_string()],
    );
    map.insert(
        "NutritionProduct.characteristic.value".to_string(),
        vec![
            "Attachment".to_string(), "base64Binary".to_string(), "boolean".to_string(),
            "CodeableConcept".to_string(), "Quantity".to_string(), "string".to_string()
        ],
    );
    map.insert(
        "Observation.component.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "CodeableConcept"
            .to_string(), "dateTime".to_string(), "integer".to_string(), "Period"
            .to_string(), "Quantity".to_string(), "Range".to_string(), "Ratio"
            .to_string(), "Reference".to_string(), "SampledData".to_string(), "string"
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
        "Observation.instantiates".to_string(),
        vec!["Canonical".to_string(), "Reference".to_string()],
    );
    map.insert(
        "Observation.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "CodeableConcept"
            .to_string(), "dateTime".to_string(), "integer".to_string(), "Period"
            .to_string(), "Quantity".to_string(), "Range".to_string(), "Ratio"
            .to_string(), "Reference".to_string(), "SampledData".to_string(), "string"
            .to_string(), "time".to_string()
        ],
    );
    map.insert(
        "ObservationDefinition.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "OperationDefinition.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "PackagedProductDefinition.packaging.property.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "CodeableConcept"
            .to_string(), "date".to_string(), "Quantity".to_string()
        ],
    );
    map.insert(
        "Parameters.parameter.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "Availability".to_string(), "base64Binary"
            .to_string(), "boolean".to_string(), "canonical".to_string(), "code"
            .to_string(), "CodeableConcept".to_string(), "CodeableReference".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Count".to_string(), "DataRequirement".to_string(), "date"
            .to_string(), "dateTime".to_string(), "decimal".to_string(), "Distance"
            .to_string(), "Dosage".to_string(), "Duration".to_string(), "Expression"
            .to_string(), "ExtendedContactDetail".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "integer64".to_string(), "markdown".to_string(), "Meta"
            .to_string(), "Money".to_string(), "oid".to_string(), "ParameterDefinition"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "RatioRange"
            .to_string(), "Reference".to_string(), "RelatedArtifact".to_string(),
            "SampledData".to_string(), "Signature".to_string(), "string".to_string(),
            "time".to_string(), "Timing".to_string(), "TriggerDefinition".to_string(),
            "unsignedInt".to_string(), "uri".to_string(), "url".to_string(),
            "UsageContext".to_string(), "uuid".to_string()
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
        "PaymentReconciliation.allocation.targetItem".to_string(),
        vec!["Identifier".to_string(), "positiveInt".to_string(), "string".to_string()],
    );
    map.insert(
        "Person.deceased".to_string(),
        vec!["Boolean".to_string(), "dateTime".to_string()],
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
        vec![
            "Canonical".to_string(), "CodeableConcept".to_string(), "Reference"
            .to_string()
        ],
    );
    map.insert(
        "PlanDefinition.action.timing".to_string(),
        vec![
            "Age".to_string(), "Duration".to_string(), "Range".to_string(), "Timing"
            .to_string()
        ],
    );
    map.insert(
        "PlanDefinition.asNeeded".to_string(),
        vec!["Boolean".to_string(), "CodeableConcept".to_string()],
    );
    map.insert(
        "PlanDefinition.goal.target.detail".to_string(),
        vec![
            "Boolean".to_string(), "CodeableConcept".to_string(), "integer".to_string(),
            "Quantity".to_string(), "Range".to_string(), "Ratio".to_string(), "string"
            .to_string()
        ],
    );
    map.insert(
        "PlanDefinition.subject".to_string(),
        vec![
            "Canonical".to_string(), "CodeableConcept".to_string(), "Reference"
            .to_string()
        ],
    );
    map.insert(
        "PlanDefinition.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "Practitioner.deceased".to_string(),
        vec!["Boolean".to_string(), "dateTime".to_string()],
    );
    map.insert(
        "Procedure.occurrence".to_string(),
        vec![
            "Age".to_string(), "dateTime".to_string(), "Period".to_string(), "Range"
            .to_string(), "string".to_string(), "Timing".to_string()
        ],
    );
    map.insert(
        "Procedure.reported".to_string(),
        vec!["Boolean".to_string(), "Reference".to_string()],
    );
    map.insert(
        "ProductShelfLife.period".to_string(),
        vec!["Duration".to_string(), "string".to_string()],
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
        "Questionnaire.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
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
        "RegulatedAuthorization.case.date".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "RequestOrchestration.action.definition".to_string(),
        vec!["Canonical".to_string(), "uri".to_string()],
    );
    map.insert(
        "RequestOrchestration.action.participant.actor".to_string(),
        vec!["Canonical".to_string(), "Reference".to_string()],
    );
    map.insert(
        "RequestOrchestration.action.relatedAction.offset".to_string(),
        vec!["Duration".to_string(), "Range".to_string()],
    );
    map.insert(
        "RequestOrchestration.action.timing".to_string(),
        vec![
            "Age".to_string(), "dateTime".to_string(), "Duration".to_string(), "Period"
            .to_string(), "Range".to_string(), "Timing".to_string()
        ],
    );
    map.insert(
        "Requirements.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
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
        "SearchParameter.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
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
        "ServiceRequest.orderDetail.parameter.value".to_string(),
        vec![
            "Boolean".to_string(), "CodeableConcept".to_string(), "Period".to_string(),
            "Quantity".to_string(), "Range".to_string(), "Ratio".to_string(), "string"
            .to_string()
        ],
    );
    map.insert(
        "ServiceRequest.patientInstruction.instruction".to_string(),
        vec!["Markdown".to_string(), "Reference".to_string()],
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
        "Specimen.processing.time".to_string(),
        vec!["DateTime".to_string(), "Period".to_string()],
    );
    map.insert(
        "SpecimenDefinition.subject".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
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
        "SpecimenDefinition.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "StructureDefinition.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "StructureMap.group.rule.target.parameter.value".to_string(),
        vec![
            "Boolean".to_string(), "date".to_string(), "dateTime".to_string(), "decimal"
            .to_string(), "id".to_string(), "integer".to_string(), "string".to_string(),
            "time".to_string()
        ],
    );
    map.insert(
        "StructureMap.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "SubscriptionTopic.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "Substance.ingredient.substance".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "SubstanceDefinition.moiety.amount".to_string(),
        vec!["Quantity".to_string(), "string".to_string()],
    );
    map.insert(
        "SubstanceDefinition.property.value".to_string(),
        vec![
            "Attachment".to_string(), "boolean".to_string(), "CodeableConcept"
            .to_string(), "date".to_string(), "Quantity".to_string()
        ],
    );
    map.insert(
        "SubstanceDefinition.relationship.amount".to_string(),
        vec!["Quantity".to_string(), "Ratio".to_string(), "string".to_string()],
    );
    map.insert(
        "SubstanceDefinition.relationship.substanceDefinition".to_string(),
        vec!["CodeableConcept".to_string(), "Reference".to_string()],
    );
    map.insert(
        "SubstanceReferenceInformation.target.amount".to_string(),
        vec!["Quantity".to_string(), "Range".to_string(), "string".to_string()],
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
            "Attachment".to_string(), "Availability".to_string(), "base64Binary"
            .to_string(), "boolean".to_string(), "canonical".to_string(), "code"
            .to_string(), "CodeableConcept".to_string(), "CodeableReference".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Count".to_string(), "DataRequirement".to_string(), "date"
            .to_string(), "dateTime".to_string(), "decimal".to_string(), "Distance"
            .to_string(), "Dosage".to_string(), "Duration".to_string(), "Expression"
            .to_string(), "ExtendedContactDetail".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "integer64".to_string(), "markdown".to_string(), "Meta"
            .to_string(), "Money".to_string(), "oid".to_string(), "ParameterDefinition"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "RatioRange"
            .to_string(), "Reference".to_string(), "RelatedArtifact".to_string(),
            "SampledData".to_string(), "Signature".to_string(), "string".to_string(),
            "time".to_string(), "Timing".to_string(), "TriggerDefinition".to_string(),
            "unsignedInt".to_string(), "uri".to_string(), "url".to_string(),
            "UsageContext".to_string(), "uuid".to_string()
        ],
    );
    map.insert(
        "Task.output.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "Availability".to_string(), "base64Binary"
            .to_string(), "boolean".to_string(), "canonical".to_string(), "code"
            .to_string(), "CodeableConcept".to_string(), "CodeableReference".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Count".to_string(), "DataRequirement".to_string(), "date"
            .to_string(), "dateTime".to_string(), "decimal".to_string(), "Distance"
            .to_string(), "Dosage".to_string(), "Duration".to_string(), "Expression"
            .to_string(), "ExtendedContactDetail".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "integer64".to_string(), "markdown".to_string(), "Meta"
            .to_string(), "Money".to_string(), "oid".to_string(), "ParameterDefinition"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "RatioRange"
            .to_string(), "Reference".to_string(), "RelatedArtifact".to_string(),
            "SampledData".to_string(), "Signature".to_string(), "string".to_string(),
            "time".to_string(), "Timing".to_string(), "TriggerDefinition".to_string(),
            "unsignedInt".to_string(), "uri".to_string(), "url".to_string(),
            "UsageContext".to_string(), "uuid".to_string()
        ],
    );
    map.insert(
        "TerminologyCapabilities.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "TestPlan.testCase.testData.source".to_string(),
        vec!["Reference".to_string(), "string".to_string()],
    );
    map.insert(
        "TestPlan.testCase.testRun.script.source".to_string(),
        vec!["Reference".to_string(), "string".to_string()],
    );
    map.insert(
        "TestPlan.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "TestReport.setup.action.assert.requirement.link".to_string(),
        vec!["Canonical".to_string(), "uri".to_string()],
    );
    map.insert(
        "TestScript.setup.action.assert.requirement.link".to_string(),
        vec!["Canonical".to_string(), "uri".to_string()],
    );
    map.insert(
        "TestScript.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "Timing.repeat.bounds".to_string(),
        vec!["Duration".to_string(), "Period".to_string(), "Range".to_string()],
    );
    map.insert(
        "Transport.input.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "Availability".to_string(), "base64Binary"
            .to_string(), "boolean".to_string(), "canonical".to_string(), "code"
            .to_string(), "CodeableConcept".to_string(), "CodeableReference".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Count".to_string(), "DataRequirement".to_string(), "date"
            .to_string(), "dateTime".to_string(), "decimal".to_string(), "Distance"
            .to_string(), "Dosage".to_string(), "Duration".to_string(), "Expression"
            .to_string(), "ExtendedContactDetail".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "integer64".to_string(), "markdown".to_string(), "Meta"
            .to_string(), "Money".to_string(), "oid".to_string(), "ParameterDefinition"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "RatioRange"
            .to_string(), "Reference".to_string(), "RelatedArtifact".to_string(),
            "SampledData".to_string(), "Signature".to_string(), "string".to_string(),
            "time".to_string(), "Timing".to_string(), "TriggerDefinition".to_string(),
            "unsignedInt".to_string(), "uri".to_string(), "url".to_string(),
            "UsageContext".to_string(), "uuid".to_string()
        ],
    );
    map.insert(
        "Transport.output.value".to_string(),
        vec![
            "Address".to_string(), "Age".to_string(), "Annotation".to_string(),
            "Attachment".to_string(), "Availability".to_string(), "base64Binary"
            .to_string(), "boolean".to_string(), "canonical".to_string(), "code"
            .to_string(), "CodeableConcept".to_string(), "CodeableReference".to_string(),
            "Coding".to_string(), "ContactDetail".to_string(), "ContactPoint"
            .to_string(), "Count".to_string(), "DataRequirement".to_string(), "date"
            .to_string(), "dateTime".to_string(), "decimal".to_string(), "Distance"
            .to_string(), "Dosage".to_string(), "Duration".to_string(), "Expression"
            .to_string(), "ExtendedContactDetail".to_string(), "HumanName".to_string(),
            "id".to_string(), "Identifier".to_string(), "instant".to_string(), "integer"
            .to_string(), "integer64".to_string(), "markdown".to_string(), "Meta"
            .to_string(), "Money".to_string(), "oid".to_string(), "ParameterDefinition"
            .to_string(), "Period".to_string(), "positiveInt".to_string(), "Quantity"
            .to_string(), "Range".to_string(), "Ratio".to_string(), "RatioRange"
            .to_string(), "Reference".to_string(), "RelatedArtifact".to_string(),
            "SampledData".to_string(), "Signature".to_string(), "string".to_string(),
            "time".to_string(), "Timing".to_string(), "TriggerDefinition".to_string(),
            "unsignedInt".to_string(), "uri".to_string(), "url".to_string(),
            "UsageContext".to_string(), "uuid".to_string()
        ],
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
        "ValueSet.expansion.contains.property.subProperty.value".to_string(),
        vec![
            "Boolean".to_string(), "code".to_string(), "Coding".to_string(), "dateTime"
            .to_string(), "decimal".to_string(), "integer".to_string(), "string"
            .to_string()
        ],
    );
    map.insert(
        "ValueSet.expansion.contains.property.value".to_string(),
        vec![
            "Boolean".to_string(), "code".to_string(), "Coding".to_string(), "dateTime"
            .to_string(), "decimal".to_string(), "integer".to_string(), "string"
            .to_string()
        ],
    );
    map.insert(
        "ValueSet.expansion.parameter.value".to_string(),
        vec![
            "Boolean".to_string(), "code".to_string(), "dateTime".to_string(), "decimal"
            .to_string(), "integer".to_string(), "string".to_string(), "uri".to_string()
        ],
    );
    map.insert(
        "ValueSet.versionAlgorithm".to_string(),
        vec!["Coding".to_string(), "string".to_string()],
    );
    map.insert(
        "VirtualServiceDetail.address".to_string(),
        vec![
            "ContactPoint".to_string(), "ExtendedContactDetail".to_string(), "string"
            .to_string(), "url".to_string()
        ],
    );
    map
}
