use std::collections::HashMap;
pub fn type_to_parent() -> HashMap<String, String> {
    let mut map = HashMap::<String, String>::new();
    map.insert("Account".to_string(), "DomainResource".to_string());
    map.insert("ActivityDefinition".to_string(), "DomainResource".to_string());
    map.insert("ActorDefinition".to_string(), "DomainResource".to_string());
    map.insert("Address".to_string(), "DataType".to_string());
    map.insert(
        "AdministrableProductDefinition".to_string(),
        "DomainResource".to_string(),
    );
    map.insert("AdverseEvent".to_string(), "DomainResource".to_string());
    map.insert("Age".to_string(), "Quantity".to_string());
    map.insert("AllergyIntolerance".to_string(), "DomainResource".to_string());
    map.insert("Annotation".to_string(), "DataType".to_string());
    map.insert("Appointment".to_string(), "DomainResource".to_string());
    map.insert("AppointmentResponse".to_string(), "DomainResource".to_string());
    map.insert("ArtifactAssessment".to_string(), "DomainResource".to_string());
    map.insert("Attachment".to_string(), "DataType".to_string());
    map.insert("AuditEvent".to_string(), "DomainResource".to_string());
    map.insert("Availability".to_string(), "DataType".to_string());
    map.insert("BackboneElement".to_string(), "Element".to_string());
    map.insert("BackboneType".to_string(), "DataType".to_string());
    map.insert("Basic".to_string(), "DomainResource".to_string());
    map.insert("Binary".to_string(), "Resource".to_string());
    map.insert("BiologicallyDerivedProduct".to_string(), "DomainResource".to_string());
    map.insert(
        "BiologicallyDerivedProductDispense".to_string(),
        "DomainResource".to_string(),
    );
    map.insert("BodyStructure".to_string(), "DomainResource".to_string());
    map.insert("Bundle".to_string(), "Resource".to_string());
    map.insert("CanonicalResource".to_string(), "DomainResource".to_string());
    map.insert("CapabilityStatement".to_string(), "DomainResource".to_string());
    map.insert("CarePlan".to_string(), "DomainResource".to_string());
    map.insert("CareTeam".to_string(), "DomainResource".to_string());
    map.insert("ChargeItem".to_string(), "DomainResource".to_string());
    map.insert("ChargeItemDefinition".to_string(), "DomainResource".to_string());
    map.insert("Citation".to_string(), "DomainResource".to_string());
    map.insert("Claim".to_string(), "DomainResource".to_string());
    map.insert("ClaimResponse".to_string(), "DomainResource".to_string());
    map.insert("ClinicalImpression".to_string(), "DomainResource".to_string());
    map.insert("ClinicalUseDefinition".to_string(), "DomainResource".to_string());
    map.insert("CodeSystem".to_string(), "DomainResource".to_string());
    map.insert("CodeableConcept".to_string(), "DataType".to_string());
    map.insert("CodeableReference".to_string(), "DataType".to_string());
    map.insert("Coding".to_string(), "DataType".to_string());
    map.insert("Communication".to_string(), "DomainResource".to_string());
    map.insert("CommunicationRequest".to_string(), "DomainResource".to_string());
    map.insert("CompartmentDefinition".to_string(), "DomainResource".to_string());
    map.insert("Composition".to_string(), "DomainResource".to_string());
    map.insert("ConceptMap".to_string(), "DomainResource".to_string());
    map.insert("Condition".to_string(), "DomainResource".to_string());
    map.insert("ConditionDefinition".to_string(), "DomainResource".to_string());
    map.insert("Consent".to_string(), "DomainResource".to_string());
    map.insert("ContactDetail".to_string(), "DataType".to_string());
    map.insert("ContactPoint".to_string(), "DataType".to_string());
    map.insert("Contract".to_string(), "DomainResource".to_string());
    map.insert("Contributor".to_string(), "DataType".to_string());
    map.insert("Count".to_string(), "Quantity".to_string());
    map.insert("Coverage".to_string(), "DomainResource".to_string());
    map.insert("CoverageEligibilityRequest".to_string(), "DomainResource".to_string());
    map.insert("CoverageEligibilityResponse".to_string(), "DomainResource".to_string());
    map.insert("DataRequirement".to_string(), "DataType".to_string());
    map.insert("DataType".to_string(), "Element".to_string());
    map.insert("DetectedIssue".to_string(), "DomainResource".to_string());
    map.insert("Device".to_string(), "DomainResource".to_string());
    map.insert("DeviceAssociation".to_string(), "DomainResource".to_string());
    map.insert("DeviceDefinition".to_string(), "DomainResource".to_string());
    map.insert("DeviceDispense".to_string(), "DomainResource".to_string());
    map.insert("DeviceMetric".to_string(), "DomainResource".to_string());
    map.insert("DeviceRequest".to_string(), "DomainResource".to_string());
    map.insert("DeviceUsage".to_string(), "DomainResource".to_string());
    map.insert("DiagnosticReport".to_string(), "DomainResource".to_string());
    map.insert("Distance".to_string(), "Quantity".to_string());
    map.insert("DocumentReference".to_string(), "DomainResource".to_string());
    map.insert("DomainResource".to_string(), "Resource".to_string());
    map.insert("Dosage".to_string(), "BackboneType".to_string());
    map.insert("Duration".to_string(), "Quantity".to_string());
    map.insert("Element".to_string(), "Base".to_string());
    map.insert("ElementDefinition".to_string(), "BackboneType".to_string());
    map.insert("Encounter".to_string(), "DomainResource".to_string());
    map.insert("EncounterHistory".to_string(), "DomainResource".to_string());
    map.insert("Endpoint".to_string(), "DomainResource".to_string());
    map.insert("EnrollmentRequest".to_string(), "DomainResource".to_string());
    map.insert("EnrollmentResponse".to_string(), "DomainResource".to_string());
    map.insert("EpisodeOfCare".to_string(), "DomainResource".to_string());
    map.insert("EventDefinition".to_string(), "DomainResource".to_string());
    map.insert("Evidence".to_string(), "DomainResource".to_string());
    map.insert("EvidenceReport".to_string(), "DomainResource".to_string());
    map.insert("EvidenceVariable".to_string(), "DomainResource".to_string());
    map.insert("ExampleScenario".to_string(), "DomainResource".to_string());
    map.insert("ExplanationOfBenefit".to_string(), "DomainResource".to_string());
    map.insert("Expression".to_string(), "DataType".to_string());
    map.insert("ExtendedContactDetail".to_string(), "DataType".to_string());
    map.insert("Extension".to_string(), "DataType".to_string());
    map.insert("FamilyMemberHistory".to_string(), "DomainResource".to_string());
    map.insert("Flag".to_string(), "DomainResource".to_string());
    map.insert("FormularyItem".to_string(), "DomainResource".to_string());
    map.insert("GenomicStudy".to_string(), "DomainResource".to_string());
    map.insert("Goal".to_string(), "DomainResource".to_string());
    map.insert("GraphDefinition".to_string(), "DomainResource".to_string());
    map.insert("Group".to_string(), "DomainResource".to_string());
    map.insert("GuidanceResponse".to_string(), "DomainResource".to_string());
    map.insert("HealthcareService".to_string(), "DomainResource".to_string());
    map.insert("HumanName".to_string(), "DataType".to_string());
    map.insert("Identifier".to_string(), "DataType".to_string());
    map.insert("ImagingSelection".to_string(), "DomainResource".to_string());
    map.insert("ImagingStudy".to_string(), "DomainResource".to_string());
    map.insert("Immunization".to_string(), "DomainResource".to_string());
    map.insert("ImmunizationEvaluation".to_string(), "DomainResource".to_string());
    map.insert("ImmunizationRecommendation".to_string(), "DomainResource".to_string());
    map.insert("ImplementationGuide".to_string(), "DomainResource".to_string());
    map.insert("Ingredient".to_string(), "DomainResource".to_string());
    map.insert("InsurancePlan".to_string(), "DomainResource".to_string());
    map.insert("InventoryItem".to_string(), "DomainResource".to_string());
    map.insert("InventoryReport".to_string(), "DomainResource".to_string());
    map.insert("Invoice".to_string(), "DomainResource".to_string());
    map.insert("Library".to_string(), "DomainResource".to_string());
    map.insert("Linkage".to_string(), "DomainResource".to_string());
    map.insert("List".to_string(), "DomainResource".to_string());
    map.insert("Location".to_string(), "DomainResource".to_string());
    map.insert("ManufacturedItemDefinition".to_string(), "DomainResource".to_string());
    map.insert("MarketingStatus".to_string(), "BackboneType".to_string());
    map.insert("Measure".to_string(), "DomainResource".to_string());
    map.insert("MeasureReport".to_string(), "DomainResource".to_string());
    map.insert("Medication".to_string(), "DomainResource".to_string());
    map.insert("MedicationAdministration".to_string(), "DomainResource".to_string());
    map.insert("MedicationDispense".to_string(), "DomainResource".to_string());
    map.insert("MedicationKnowledge".to_string(), "DomainResource".to_string());
    map.insert("MedicationRequest".to_string(), "DomainResource".to_string());
    map.insert("MedicationStatement".to_string(), "DomainResource".to_string());
    map.insert("MedicinalProductDefinition".to_string(), "DomainResource".to_string());
    map.insert("MessageDefinition".to_string(), "DomainResource".to_string());
    map.insert("MessageHeader".to_string(), "DomainResource".to_string());
    map.insert("Meta".to_string(), "DataType".to_string());
    map.insert("MetadataResource".to_string(), "DomainResource".to_string());
    map.insert("MolecularSequence".to_string(), "DomainResource".to_string());
    map.insert("MonetaryComponent".to_string(), "DataType".to_string());
    map.insert("Money".to_string(), "DataType".to_string());
    map.insert("NamingSystem".to_string(), "DomainResource".to_string());
    map.insert("Narrative".to_string(), "DataType".to_string());
    map.insert("NutritionIntake".to_string(), "DomainResource".to_string());
    map.insert("NutritionOrder".to_string(), "DomainResource".to_string());
    map.insert("NutritionProduct".to_string(), "DomainResource".to_string());
    map.insert("Observation".to_string(), "DomainResource".to_string());
    map.insert("ObservationDefinition".to_string(), "DomainResource".to_string());
    map.insert("OperationDefinition".to_string(), "DomainResource".to_string());
    map.insert("OperationOutcome".to_string(), "DomainResource".to_string());
    map.insert("Organization".to_string(), "DomainResource".to_string());
    map.insert("OrganizationAffiliation".to_string(), "DomainResource".to_string());
    map.insert("PackagedProductDefinition".to_string(), "DomainResource".to_string());
    map.insert("ParameterDefinition".to_string(), "DataType".to_string());
    map.insert("Parameters".to_string(), "Resource".to_string());
    map.insert("Patient".to_string(), "DomainResource".to_string());
    map.insert("PaymentNotice".to_string(), "DomainResource".to_string());
    map.insert("PaymentReconciliation".to_string(), "DomainResource".to_string());
    map.insert("Period".to_string(), "DataType".to_string());
    map.insert("Permission".to_string(), "DomainResource".to_string());
    map.insert("Person".to_string(), "DomainResource".to_string());
    map.insert("PlanDefinition".to_string(), "DomainResource".to_string());
    map.insert("Practitioner".to_string(), "DomainResource".to_string());
    map.insert("PractitionerRole".to_string(), "DomainResource".to_string());
    map.insert("PrimitiveType".to_string(), "DataType".to_string());
    map.insert("Procedure".to_string(), "DomainResource".to_string());
    map.insert("ProductShelfLife".to_string(), "BackboneType".to_string());
    map.insert("Provenance".to_string(), "DomainResource".to_string());
    map.insert("Quantity".to_string(), "DataType".to_string());
    map.insert("Questionnaire".to_string(), "DomainResource".to_string());
    map.insert("QuestionnaireResponse".to_string(), "DomainResource".to_string());
    map.insert("Range".to_string(), "DataType".to_string());
    map.insert("Ratio".to_string(), "DataType".to_string());
    map.insert("RatioRange".to_string(), "DataType".to_string());
    map.insert("Reference".to_string(), "DataType".to_string());
    map.insert("RegulatedAuthorization".to_string(), "DomainResource".to_string());
    map.insert("RelatedArtifact".to_string(), "DataType".to_string());
    map.insert("RelatedPerson".to_string(), "DomainResource".to_string());
    map.insert("RequestOrchestration".to_string(), "DomainResource".to_string());
    map.insert("Requirements".to_string(), "DomainResource".to_string());
    map.insert("ResearchStudy".to_string(), "DomainResource".to_string());
    map.insert("ResearchSubject".to_string(), "DomainResource".to_string());
    map.insert("Resource".to_string(), "Base".to_string());
    map.insert("RiskAssessment".to_string(), "DomainResource".to_string());
    map.insert("SampledData".to_string(), "DataType".to_string());
    map.insert("Schedule".to_string(), "DomainResource".to_string());
    map.insert("SearchParameter".to_string(), "DomainResource".to_string());
    map.insert("ServiceRequest".to_string(), "DomainResource".to_string());
    map.insert("Signature".to_string(), "DataType".to_string());
    map.insert("Slot".to_string(), "DomainResource".to_string());
    map.insert("Specimen".to_string(), "DomainResource".to_string());
    map.insert("SpecimenDefinition".to_string(), "DomainResource".to_string());
    map.insert("StructureDefinition".to_string(), "DomainResource".to_string());
    map.insert("StructureMap".to_string(), "DomainResource".to_string());
    map.insert("Subscription".to_string(), "DomainResource".to_string());
    map.insert("SubscriptionStatus".to_string(), "DomainResource".to_string());
    map.insert("SubscriptionTopic".to_string(), "DomainResource".to_string());
    map.insert("Substance".to_string(), "DomainResource".to_string());
    map.insert("SubstanceDefinition".to_string(), "DomainResource".to_string());
    map.insert("SubstanceNucleicAcid".to_string(), "DomainResource".to_string());
    map.insert("SubstancePolymer".to_string(), "DomainResource".to_string());
    map.insert("SubstanceProtein".to_string(), "DomainResource".to_string());
    map.insert(
        "SubstanceReferenceInformation".to_string(),
        "DomainResource".to_string(),
    );
    map.insert("SubstanceSourceMaterial".to_string(), "DomainResource".to_string());
    map.insert("SupplyDelivery".to_string(), "DomainResource".to_string());
    map.insert("SupplyRequest".to_string(), "DomainResource".to_string());
    map.insert("Task".to_string(), "DomainResource".to_string());
    map.insert("TerminologyCapabilities".to_string(), "DomainResource".to_string());
    map.insert("TestPlan".to_string(), "DomainResource".to_string());
    map.insert("TestReport".to_string(), "DomainResource".to_string());
    map.insert("TestScript".to_string(), "DomainResource".to_string());
    map.insert("Timing".to_string(), "BackboneType".to_string());
    map.insert("Transport".to_string(), "DomainResource".to_string());
    map.insert("TriggerDefinition".to_string(), "DataType".to_string());
    map.insert("UsageContext".to_string(), "DataType".to_string());
    map.insert("ValueSet".to_string(), "DomainResource".to_string());
    map.insert("VerificationResult".to_string(), "DomainResource".to_string());
    map.insert("VirtualServiceDetail".to_string(), "DataType".to_string());
    map.insert("VisionPrescription".to_string(), "DomainResource".to_string());
    map.insert("base64Binary".to_string(), "PrimitiveType".to_string());
    map.insert("boolean".to_string(), "PrimitiveType".to_string());
    map.insert("canonical".to_string(), "uri".to_string());
    map.insert("code".to_string(), "string".to_string());
    map.insert("date".to_string(), "PrimitiveType".to_string());
    map.insert("dateTime".to_string(), "PrimitiveType".to_string());
    map.insert("decimal".to_string(), "PrimitiveType".to_string());
    map.insert("id".to_string(), "string".to_string());
    map.insert("instant".to_string(), "PrimitiveType".to_string());
    map.insert("integer".to_string(), "PrimitiveType".to_string());
    map.insert("integer64".to_string(), "PrimitiveType".to_string());
    map.insert("markdown".to_string(), "string".to_string());
    map.insert("oid".to_string(), "uri".to_string());
    map.insert("positiveInt".to_string(), "integer".to_string());
    map.insert("string".to_string(), "PrimitiveType".to_string());
    map.insert("time".to_string(), "PrimitiveType".to_string());
    map.insert("unsignedInt".to_string(), "integer".to_string());
    map.insert("uri".to_string(), "PrimitiveType".to_string());
    map.insert("url".to_string(), "uri".to_string());
    map.insert("uuid".to_string(), "uri".to_string());
    map.insert("xhtml".to_string(), "Element".to_string());
    map
}
