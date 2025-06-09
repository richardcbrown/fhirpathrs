use std::collections::HashMap;
pub fn path_cardinality() -> HashMap<String, String> {
    let mut map = HashMap::<String, String>::new();
    map.insert("Account.contained".to_string(), "*".to_string());
    map.insert("Account.coverage".to_string(), "*".to_string());
    map.insert("Account.coverage.coverage".to_string(), "1".to_string());
    map.insert("Account.coverage.extension".to_string(), "*".to_string());
    map.insert("Account.coverage.id".to_string(), "1".to_string());
    map.insert("Account.coverage.modifierExtension".to_string(), "*".to_string());
    map.insert("Account.coverage.priority".to_string(), "1".to_string());
    map.insert("Account.description".to_string(), "1".to_string());
    map.insert("Account.extension".to_string(), "*".to_string());
    map.insert("Account.guarantor".to_string(), "*".to_string());
    map.insert("Account.guarantor.extension".to_string(), "*".to_string());
    map.insert("Account.guarantor.id".to_string(), "1".to_string());
    map.insert("Account.guarantor.modifierExtension".to_string(), "*".to_string());
    map.insert("Account.guarantor.onHold".to_string(), "1".to_string());
    map.insert("Account.guarantor.party".to_string(), "1".to_string());
    map.insert("Account.guarantor.period".to_string(), "1".to_string());
    map.insert("Account.id".to_string(), "1".to_string());
    map.insert("Account.identifier".to_string(), "*".to_string());
    map.insert("Account.implicitRules".to_string(), "1".to_string());
    map.insert("Account.language".to_string(), "1".to_string());
    map.insert("Account.meta".to_string(), "1".to_string());
    map.insert("Account.modifierExtension".to_string(), "*".to_string());
    map.insert("Account.name".to_string(), "1".to_string());
    map.insert("Account.owner".to_string(), "1".to_string());
    map.insert("Account.partOf".to_string(), "1".to_string());
    map.insert("Account.servicePeriod".to_string(), "1".to_string());
    map.insert("Account.status".to_string(), "1".to_string());
    map.insert("Account.subject".to_string(), "*".to_string());
    map.insert("Account.text".to_string(), "1".to_string());
    map.insert("Account.type".to_string(), "1".to_string());
    map.insert("ActivityDefinition.approvalDate".to_string(), "1".to_string());
    map.insert("ActivityDefinition.author".to_string(), "*".to_string());
    map.insert("ActivityDefinition.bodySite".to_string(), "*".to_string());
    map.insert("ActivityDefinition.code".to_string(), "1".to_string());
    map.insert("ActivityDefinition.contact".to_string(), "*".to_string());
    map.insert("ActivityDefinition.contained".to_string(), "*".to_string());
    map.insert("ActivityDefinition.copyright".to_string(), "1".to_string());
    map.insert("ActivityDefinition.date".to_string(), "1".to_string());
    map.insert("ActivityDefinition.description".to_string(), "1".to_string());
    map.insert("ActivityDefinition.doNotPerform".to_string(), "1".to_string());
    map.insert("ActivityDefinition.dosage".to_string(), "*".to_string());
    map.insert("ActivityDefinition.dynamicValue".to_string(), "*".to_string());
    map.insert(
        "ActivityDefinition.dynamicValue.expression".to_string(),
        "1".to_string(),
    );
    map.insert("ActivityDefinition.dynamicValue.extension".to_string(), "*".to_string());
    map.insert("ActivityDefinition.dynamicValue.id".to_string(), "1".to_string());
    map.insert(
        "ActivityDefinition.dynamicValue.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ActivityDefinition.dynamicValue.path".to_string(), "1".to_string());
    map.insert("ActivityDefinition.editor".to_string(), "*".to_string());
    map.insert("ActivityDefinition.effectivePeriod".to_string(), "1".to_string());
    map.insert("ActivityDefinition.endorser".to_string(), "*".to_string());
    map.insert("ActivityDefinition.experimental".to_string(), "1".to_string());
    map.insert("ActivityDefinition.extension".to_string(), "*".to_string());
    map.insert("ActivityDefinition.id".to_string(), "1".to_string());
    map.insert("ActivityDefinition.identifier".to_string(), "*".to_string());
    map.insert("ActivityDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("ActivityDefinition.intent".to_string(), "1".to_string());
    map.insert("ActivityDefinition.jurisdiction".to_string(), "*".to_string());
    map.insert("ActivityDefinition.kind".to_string(), "1".to_string());
    map.insert("ActivityDefinition.language".to_string(), "1".to_string());
    map.insert("ActivityDefinition.lastReviewDate".to_string(), "1".to_string());
    map.insert("ActivityDefinition.library".to_string(), "*".to_string());
    map.insert("ActivityDefinition.location".to_string(), "1".to_string());
    map.insert("ActivityDefinition.meta".to_string(), "1".to_string());
    map.insert("ActivityDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("ActivityDefinition.name".to_string(), "1".to_string());
    map.insert("ActivityDefinition.observationRequirement".to_string(), "*".to_string());
    map.insert(
        "ActivityDefinition.observationResultRequirement".to_string(),
        "*".to_string(),
    );
    map.insert("ActivityDefinition.participant".to_string(), "*".to_string());
    map.insert("ActivityDefinition.participant.extension".to_string(), "*".to_string());
    map.insert("ActivityDefinition.participant.id".to_string(), "1".to_string());
    map.insert(
        "ActivityDefinition.participant.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ActivityDefinition.participant.role".to_string(), "1".to_string());
    map.insert("ActivityDefinition.participant.type".to_string(), "1".to_string());
    map.insert("ActivityDefinition.priority".to_string(), "1".to_string());
    map.insert("ActivityDefinition.productCodeableConcept".to_string(), "1".to_string());
    map.insert("ActivityDefinition.productReference".to_string(), "1".to_string());
    map.insert("ActivityDefinition.profile".to_string(), "1".to_string());
    map.insert("ActivityDefinition.publisher".to_string(), "1".to_string());
    map.insert("ActivityDefinition.purpose".to_string(), "1".to_string());
    map.insert("ActivityDefinition.quantity".to_string(), "1".to_string());
    map.insert("ActivityDefinition.relatedArtifact".to_string(), "*".to_string());
    map.insert("ActivityDefinition.reviewer".to_string(), "*".to_string());
    map.insert("ActivityDefinition.specimenRequirement".to_string(), "*".to_string());
    map.insert("ActivityDefinition.status".to_string(), "1".to_string());
    map.insert("ActivityDefinition.subjectCodeableConcept".to_string(), "1".to_string());
    map.insert("ActivityDefinition.subjectReference".to_string(), "1".to_string());
    map.insert("ActivityDefinition.subtitle".to_string(), "1".to_string());
    map.insert("ActivityDefinition.text".to_string(), "1".to_string());
    map.insert("ActivityDefinition.timingAge".to_string(), "1".to_string());
    map.insert("ActivityDefinition.timingDateTime".to_string(), "1".to_string());
    map.insert("ActivityDefinition.timingDuration".to_string(), "1".to_string());
    map.insert("ActivityDefinition.timingPeriod".to_string(), "1".to_string());
    map.insert("ActivityDefinition.timingRange".to_string(), "1".to_string());
    map.insert("ActivityDefinition.timingTiming".to_string(), "1".to_string());
    map.insert("ActivityDefinition.title".to_string(), "1".to_string());
    map.insert("ActivityDefinition.topic".to_string(), "*".to_string());
    map.insert("ActivityDefinition.transform".to_string(), "1".to_string());
    map.insert("ActivityDefinition.url".to_string(), "1".to_string());
    map.insert("ActivityDefinition.usage".to_string(), "1".to_string());
    map.insert("ActivityDefinition.useContext".to_string(), "*".to_string());
    map.insert("ActivityDefinition.version".to_string(), "1".to_string());
    map.insert("Address.city".to_string(), "1".to_string());
    map.insert("Address.country".to_string(), "1".to_string());
    map.insert("Address.district".to_string(), "1".to_string());
    map.insert("Address.extension".to_string(), "*".to_string());
    map.insert("Address.id".to_string(), "1".to_string());
    map.insert("Address.line".to_string(), "*".to_string());
    map.insert("Address.period".to_string(), "1".to_string());
    map.insert("Address.postalCode".to_string(), "1".to_string());
    map.insert("Address.state".to_string(), "1".to_string());
    map.insert("Address.text".to_string(), "1".to_string());
    map.insert("Address.type".to_string(), "1".to_string());
    map.insert("Address.use".to_string(), "1".to_string());
    map.insert("AdverseEvent.actuality".to_string(), "1".to_string());
    map.insert("AdverseEvent.category".to_string(), "*".to_string());
    map.insert("AdverseEvent.contained".to_string(), "*".to_string());
    map.insert("AdverseEvent.contributor".to_string(), "*".to_string());
    map.insert("AdverseEvent.date".to_string(), "1".to_string());
    map.insert("AdverseEvent.detected".to_string(), "1".to_string());
    map.insert("AdverseEvent.encounter".to_string(), "1".to_string());
    map.insert("AdverseEvent.event".to_string(), "1".to_string());
    map.insert("AdverseEvent.extension".to_string(), "*".to_string());
    map.insert("AdverseEvent.id".to_string(), "1".to_string());
    map.insert("AdverseEvent.identifier".to_string(), "1".to_string());
    map.insert("AdverseEvent.implicitRules".to_string(), "1".to_string());
    map.insert("AdverseEvent.language".to_string(), "1".to_string());
    map.insert("AdverseEvent.location".to_string(), "1".to_string());
    map.insert("AdverseEvent.meta".to_string(), "1".to_string());
    map.insert("AdverseEvent.modifierExtension".to_string(), "*".to_string());
    map.insert("AdverseEvent.outcome".to_string(), "1".to_string());
    map.insert("AdverseEvent.recordedDate".to_string(), "1".to_string());
    map.insert("AdverseEvent.recorder".to_string(), "1".to_string());
    map.insert("AdverseEvent.referenceDocument".to_string(), "*".to_string());
    map.insert("AdverseEvent.resultingCondition".to_string(), "*".to_string());
    map.insert("AdverseEvent.seriousness".to_string(), "1".to_string());
    map.insert("AdverseEvent.severity".to_string(), "1".to_string());
    map.insert("AdverseEvent.study".to_string(), "*".to_string());
    map.insert("AdverseEvent.subject".to_string(), "1".to_string());
    map.insert("AdverseEvent.subjectMedicalHistory".to_string(), "*".to_string());
    map.insert("AdverseEvent.suspectEntity".to_string(), "*".to_string());
    map.insert("AdverseEvent.suspectEntity.causality".to_string(), "*".to_string());
    map.insert(
        "AdverseEvent.suspectEntity.causality.assessment".to_string(),
        "1".to_string(),
    );
    map.insert(
        "AdverseEvent.suspectEntity.causality.author".to_string(),
        "1".to_string(),
    );
    map.insert(
        "AdverseEvent.suspectEntity.causality.extension".to_string(),
        "*".to_string(),
    );
    map.insert("AdverseEvent.suspectEntity.causality.id".to_string(), "1".to_string());
    map.insert(
        "AdverseEvent.suspectEntity.causality.method".to_string(),
        "1".to_string(),
    );
    map.insert(
        "AdverseEvent.suspectEntity.causality.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "AdverseEvent.suspectEntity.causality.productRelatedness".to_string(),
        "1".to_string(),
    );
    map.insert("AdverseEvent.suspectEntity.extension".to_string(), "*".to_string());
    map.insert("AdverseEvent.suspectEntity.id".to_string(), "1".to_string());
    map.insert("AdverseEvent.suspectEntity.instance".to_string(), "1".to_string());
    map.insert(
        "AdverseEvent.suspectEntity.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("AdverseEvent.text".to_string(), "1".to_string());
    map.insert("Age.code".to_string(), "1".to_string());
    map.insert("Age.comparator".to_string(), "1".to_string());
    map.insert("Age.extension".to_string(), "*".to_string());
    map.insert("Age.id".to_string(), "1".to_string());
    map.insert("Age.system".to_string(), "1".to_string());
    map.insert("Age.unit".to_string(), "1".to_string());
    map.insert("Age.value".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.asserter".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.category".to_string(), "*".to_string());
    map.insert("AllergyIntolerance.clinicalStatus".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.code".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.contained".to_string(), "*".to_string());
    map.insert("AllergyIntolerance.criticality".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.encounter".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.extension".to_string(), "*".to_string());
    map.insert("AllergyIntolerance.id".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.identifier".to_string(), "*".to_string());
    map.insert("AllergyIntolerance.implicitRules".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.language".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.lastOccurrence".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.meta".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.modifierExtension".to_string(), "*".to_string());
    map.insert("AllergyIntolerance.note".to_string(), "*".to_string());
    map.insert("AllergyIntolerance.onsetAge".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.onsetDateTime".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.onsetPeriod".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.onsetRange".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.onsetString".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.patient".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.reaction".to_string(), "*".to_string());
    map.insert("AllergyIntolerance.reaction.description".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.reaction.exposureRoute".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.reaction.extension".to_string(), "*".to_string());
    map.insert("AllergyIntolerance.reaction.id".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.reaction.manifestation".to_string(), "*".to_string());
    map.insert(
        "AllergyIntolerance.reaction.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("AllergyIntolerance.reaction.note".to_string(), "*".to_string());
    map.insert("AllergyIntolerance.reaction.onset".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.reaction.severity".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.reaction.substance".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.recordedDate".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.recorder".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.text".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.type".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.verificationStatus".to_string(), "1".to_string());
    map.insert("Annotation.authorReference".to_string(), "1".to_string());
    map.insert("Annotation.authorString".to_string(), "1".to_string());
    map.insert("Annotation.extension".to_string(), "*".to_string());
    map.insert("Annotation.id".to_string(), "1".to_string());
    map.insert("Annotation.text".to_string(), "1".to_string());
    map.insert("Annotation.time".to_string(), "1".to_string());
    map.insert("Appointment.appointmentType".to_string(), "1".to_string());
    map.insert("Appointment.basedOn".to_string(), "*".to_string());
    map.insert("Appointment.cancelationReason".to_string(), "1".to_string());
    map.insert("Appointment.comment".to_string(), "1".to_string());
    map.insert("Appointment.contained".to_string(), "*".to_string());
    map.insert("Appointment.created".to_string(), "1".to_string());
    map.insert("Appointment.description".to_string(), "1".to_string());
    map.insert("Appointment.end".to_string(), "1".to_string());
    map.insert("Appointment.extension".to_string(), "*".to_string());
    map.insert("Appointment.id".to_string(), "1".to_string());
    map.insert("Appointment.identifier".to_string(), "*".to_string());
    map.insert("Appointment.implicitRules".to_string(), "1".to_string());
    map.insert("Appointment.language".to_string(), "1".to_string());
    map.insert("Appointment.meta".to_string(), "1".to_string());
    map.insert("Appointment.minutesDuration".to_string(), "1".to_string());
    map.insert("Appointment.modifierExtension".to_string(), "*".to_string());
    map.insert("Appointment.participant".to_string(), "*".to_string());
    map.insert("Appointment.participant.actor".to_string(), "1".to_string());
    map.insert("Appointment.participant.extension".to_string(), "*".to_string());
    map.insert("Appointment.participant.id".to_string(), "1".to_string());
    map.insert("Appointment.participant.modifierExtension".to_string(), "*".to_string());
    map.insert("Appointment.participant.period".to_string(), "1".to_string());
    map.insert("Appointment.participant.required".to_string(), "1".to_string());
    map.insert("Appointment.participant.status".to_string(), "1".to_string());
    map.insert("Appointment.participant.type".to_string(), "*".to_string());
    map.insert("Appointment.patientInstruction".to_string(), "1".to_string());
    map.insert("Appointment.priority".to_string(), "1".to_string());
    map.insert("Appointment.reasonCode".to_string(), "*".to_string());
    map.insert("Appointment.reasonReference".to_string(), "*".to_string());
    map.insert("Appointment.requestedPeriod".to_string(), "*".to_string());
    map.insert("Appointment.serviceCategory".to_string(), "*".to_string());
    map.insert("Appointment.serviceType".to_string(), "*".to_string());
    map.insert("Appointment.slot".to_string(), "*".to_string());
    map.insert("Appointment.specialty".to_string(), "*".to_string());
    map.insert("Appointment.start".to_string(), "1".to_string());
    map.insert("Appointment.status".to_string(), "1".to_string());
    map.insert("Appointment.supportingInformation".to_string(), "*".to_string());
    map.insert("Appointment.text".to_string(), "1".to_string());
    map.insert("AppointmentResponse.actor".to_string(), "1".to_string());
    map.insert("AppointmentResponse.appointment".to_string(), "1".to_string());
    map.insert("AppointmentResponse.comment".to_string(), "1".to_string());
    map.insert("AppointmentResponse.contained".to_string(), "*".to_string());
    map.insert("AppointmentResponse.end".to_string(), "1".to_string());
    map.insert("AppointmentResponse.extension".to_string(), "*".to_string());
    map.insert("AppointmentResponse.id".to_string(), "1".to_string());
    map.insert("AppointmentResponse.identifier".to_string(), "*".to_string());
    map.insert("AppointmentResponse.implicitRules".to_string(), "1".to_string());
    map.insert("AppointmentResponse.language".to_string(), "1".to_string());
    map.insert("AppointmentResponse.meta".to_string(), "1".to_string());
    map.insert("AppointmentResponse.modifierExtension".to_string(), "*".to_string());
    map.insert("AppointmentResponse.participantStatus".to_string(), "1".to_string());
    map.insert("AppointmentResponse.participantType".to_string(), "*".to_string());
    map.insert("AppointmentResponse.start".to_string(), "1".to_string());
    map.insert("AppointmentResponse.text".to_string(), "1".to_string());
    map.insert("Attachment.contentType".to_string(), "1".to_string());
    map.insert("Attachment.creation".to_string(), "1".to_string());
    map.insert("Attachment.data".to_string(), "1".to_string());
    map.insert("Attachment.extension".to_string(), "*".to_string());
    map.insert("Attachment.hash".to_string(), "1".to_string());
    map.insert("Attachment.id".to_string(), "1".to_string());
    map.insert("Attachment.language".to_string(), "1".to_string());
    map.insert("Attachment.size".to_string(), "1".to_string());
    map.insert("Attachment.title".to_string(), "1".to_string());
    map.insert("Attachment.url".to_string(), "1".to_string());
    map.insert("AuditEvent.action".to_string(), "1".to_string());
    map.insert("AuditEvent.agent".to_string(), "*".to_string());
    map.insert("AuditEvent.agent.altId".to_string(), "1".to_string());
    map.insert("AuditEvent.agent.extension".to_string(), "*".to_string());
    map.insert("AuditEvent.agent.id".to_string(), "1".to_string());
    map.insert("AuditEvent.agent.location".to_string(), "1".to_string());
    map.insert("AuditEvent.agent.media".to_string(), "1".to_string());
    map.insert("AuditEvent.agent.modifierExtension".to_string(), "*".to_string());
    map.insert("AuditEvent.agent.name".to_string(), "1".to_string());
    map.insert("AuditEvent.agent.network".to_string(), "1".to_string());
    map.insert("AuditEvent.agent.network.address".to_string(), "1".to_string());
    map.insert("AuditEvent.agent.network.extension".to_string(), "*".to_string());
    map.insert("AuditEvent.agent.network.id".to_string(), "1".to_string());
    map.insert(
        "AuditEvent.agent.network.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("AuditEvent.agent.network.type".to_string(), "1".to_string());
    map.insert("AuditEvent.agent.policy".to_string(), "*".to_string());
    map.insert("AuditEvent.agent.purposeOfUse".to_string(), "*".to_string());
    map.insert("AuditEvent.agent.requestor".to_string(), "1".to_string());
    map.insert("AuditEvent.agent.role".to_string(), "*".to_string());
    map.insert("AuditEvent.agent.type".to_string(), "1".to_string());
    map.insert("AuditEvent.agent.who".to_string(), "1".to_string());
    map.insert("AuditEvent.contained".to_string(), "*".to_string());
    map.insert("AuditEvent.entity".to_string(), "*".to_string());
    map.insert("AuditEvent.entity.description".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.detail".to_string(), "*".to_string());
    map.insert("AuditEvent.entity.detail.extension".to_string(), "*".to_string());
    map.insert("AuditEvent.entity.detail.id".to_string(), "1".to_string());
    map.insert(
        "AuditEvent.entity.detail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("AuditEvent.entity.detail.type".to_string(), "1".to_string());
    map.insert(
        "AuditEvent.entity.detail.valueBase64Binary".to_string(),
        "1".to_string(),
    );
    map.insert("AuditEvent.entity.detail.valueString".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.extension".to_string(), "*".to_string());
    map.insert("AuditEvent.entity.id".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.lifecycle".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.modifierExtension".to_string(), "*".to_string());
    map.insert("AuditEvent.entity.name".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.query".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.role".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.securityLabel".to_string(), "*".to_string());
    map.insert("AuditEvent.entity.type".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.what".to_string(), "1".to_string());
    map.insert("AuditEvent.extension".to_string(), "*".to_string());
    map.insert("AuditEvent.id".to_string(), "1".to_string());
    map.insert("AuditEvent.implicitRules".to_string(), "1".to_string());
    map.insert("AuditEvent.language".to_string(), "1".to_string());
    map.insert("AuditEvent.meta".to_string(), "1".to_string());
    map.insert("AuditEvent.modifierExtension".to_string(), "*".to_string());
    map.insert("AuditEvent.outcome".to_string(), "1".to_string());
    map.insert("AuditEvent.outcomeDesc".to_string(), "1".to_string());
    map.insert("AuditEvent.period".to_string(), "1".to_string());
    map.insert("AuditEvent.purposeOfEvent".to_string(), "*".to_string());
    map.insert("AuditEvent.recorded".to_string(), "1".to_string());
    map.insert("AuditEvent.source".to_string(), "1".to_string());
    map.insert("AuditEvent.source.extension".to_string(), "*".to_string());
    map.insert("AuditEvent.source.id".to_string(), "1".to_string());
    map.insert("AuditEvent.source.modifierExtension".to_string(), "*".to_string());
    map.insert("AuditEvent.source.observer".to_string(), "1".to_string());
    map.insert("AuditEvent.source.site".to_string(), "1".to_string());
    map.insert("AuditEvent.source.type".to_string(), "*".to_string());
    map.insert("AuditEvent.subtype".to_string(), "*".to_string());
    map.insert("AuditEvent.text".to_string(), "1".to_string());
    map.insert("AuditEvent.type".to_string(), "1".to_string());
    map.insert("BackboneElement.extension".to_string(), "*".to_string());
    map.insert("BackboneElement.id".to_string(), "1".to_string());
    map.insert("BackboneElement.modifierExtension".to_string(), "*".to_string());
    map.insert("Basic.author".to_string(), "1".to_string());
    map.insert("Basic.code".to_string(), "1".to_string());
    map.insert("Basic.contained".to_string(), "*".to_string());
    map.insert("Basic.created".to_string(), "1".to_string());
    map.insert("Basic.extension".to_string(), "*".to_string());
    map.insert("Basic.id".to_string(), "1".to_string());
    map.insert("Basic.identifier".to_string(), "*".to_string());
    map.insert("Basic.implicitRules".to_string(), "1".to_string());
    map.insert("Basic.language".to_string(), "1".to_string());
    map.insert("Basic.meta".to_string(), "1".to_string());
    map.insert("Basic.modifierExtension".to_string(), "*".to_string());
    map.insert("Basic.subject".to_string(), "1".to_string());
    map.insert("Basic.text".to_string(), "1".to_string());
    map.insert("Binary.contentType".to_string(), "1".to_string());
    map.insert("Binary.data".to_string(), "1".to_string());
    map.insert("Binary.id".to_string(), "1".to_string());
    map.insert("Binary.implicitRules".to_string(), "1".to_string());
    map.insert("Binary.language".to_string(), "1".to_string());
    map.insert("Binary.meta".to_string(), "1".to_string());
    map.insert("Binary.securityContext".to_string(), "1".to_string());
    map.insert("BiologicallyDerivedProduct.collection".to_string(), "1".to_string());
    map.insert(
        "BiologicallyDerivedProduct.collection.collectedDateTime".to_string(),
        "1".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.collection.collectedPeriod".to_string(),
        "1".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.collection.collector".to_string(),
        "1".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.collection.extension".to_string(),
        "*".to_string(),
    );
    map.insert("BiologicallyDerivedProduct.collection.id".to_string(), "1".to_string());
    map.insert(
        "BiologicallyDerivedProduct.collection.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.collection.source".to_string(),
        "1".to_string(),
    );
    map.insert("BiologicallyDerivedProduct.contained".to_string(), "*".to_string());
    map.insert("BiologicallyDerivedProduct.extension".to_string(), "*".to_string());
    map.insert("BiologicallyDerivedProduct.id".to_string(), "1".to_string());
    map.insert("BiologicallyDerivedProduct.identifier".to_string(), "*".to_string());
    map.insert("BiologicallyDerivedProduct.implicitRules".to_string(), "1".to_string());
    map.insert("BiologicallyDerivedProduct.language".to_string(), "1".to_string());
    map.insert("BiologicallyDerivedProduct.manipulation".to_string(), "1".to_string());
    map.insert(
        "BiologicallyDerivedProduct.manipulation.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.manipulation.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.manipulation.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.manipulation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.manipulation.timeDateTime".to_string(),
        "1".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.manipulation.timePeriod".to_string(),
        "1".to_string(),
    );
    map.insert("BiologicallyDerivedProduct.meta".to_string(), "1".to_string());
    map.insert(
        "BiologicallyDerivedProduct.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("BiologicallyDerivedProduct.parent".to_string(), "*".to_string());
    map.insert("BiologicallyDerivedProduct.processing".to_string(), "*".to_string());
    map.insert(
        "BiologicallyDerivedProduct.processing.additive".to_string(),
        "1".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.processing.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.processing.extension".to_string(),
        "*".to_string(),
    );
    map.insert("BiologicallyDerivedProduct.processing.id".to_string(), "1".to_string());
    map.insert(
        "BiologicallyDerivedProduct.processing.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.processing.procedure".to_string(),
        "1".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.processing.timeDateTime".to_string(),
        "1".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.processing.timePeriod".to_string(),
        "1".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.productCategory".to_string(),
        "1".to_string(),
    );
    map.insert("BiologicallyDerivedProduct.productCode".to_string(), "1".to_string());
    map.insert("BiologicallyDerivedProduct.quantity".to_string(), "1".to_string());
    map.insert("BiologicallyDerivedProduct.request".to_string(), "*".to_string());
    map.insert("BiologicallyDerivedProduct.status".to_string(), "1".to_string());
    map.insert("BiologicallyDerivedProduct.storage".to_string(), "*".to_string());
    map.insert(
        "BiologicallyDerivedProduct.storage.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.storage.duration".to_string(),
        "1".to_string(),
    );
    map.insert(
        "BiologicallyDerivedProduct.storage.extension".to_string(),
        "*".to_string(),
    );
    map.insert("BiologicallyDerivedProduct.storage.id".to_string(), "1".to_string());
    map.insert(
        "BiologicallyDerivedProduct.storage.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("BiologicallyDerivedProduct.storage.scale".to_string(), "1".to_string());
    map.insert(
        "BiologicallyDerivedProduct.storage.temperature".to_string(),
        "1".to_string(),
    );
    map.insert("BiologicallyDerivedProduct.text".to_string(), "1".to_string());
    map.insert("BodyStructure.active".to_string(), "1".to_string());
    map.insert("BodyStructure.contained".to_string(), "*".to_string());
    map.insert("BodyStructure.description".to_string(), "1".to_string());
    map.insert("BodyStructure.extension".to_string(), "*".to_string());
    map.insert("BodyStructure.id".to_string(), "1".to_string());
    map.insert("BodyStructure.identifier".to_string(), "*".to_string());
    map.insert("BodyStructure.image".to_string(), "*".to_string());
    map.insert("BodyStructure.implicitRules".to_string(), "1".to_string());
    map.insert("BodyStructure.language".to_string(), "1".to_string());
    map.insert("BodyStructure.location".to_string(), "1".to_string());
    map.insert("BodyStructure.locationQualifier".to_string(), "*".to_string());
    map.insert("BodyStructure.meta".to_string(), "1".to_string());
    map.insert("BodyStructure.modifierExtension".to_string(), "*".to_string());
    map.insert("BodyStructure.morphology".to_string(), "1".to_string());
    map.insert("BodyStructure.patient".to_string(), "1".to_string());
    map.insert("BodyStructure.text".to_string(), "1".to_string());
    map.insert("Bundle.entry".to_string(), "*".to_string());
    map.insert("Bundle.entry.extension".to_string(), "*".to_string());
    map.insert("Bundle.entry.fullUrl".to_string(), "1".to_string());
    map.insert("Bundle.entry.id".to_string(), "1".to_string());
    map.insert("Bundle.entry.link".to_string(), "*".to_string());
    map.insert("Bundle.entry.modifierExtension".to_string(), "*".to_string());
    map.insert("Bundle.entry.request".to_string(), "1".to_string());
    map.insert("Bundle.entry.request.extension".to_string(), "*".to_string());
    map.insert("Bundle.entry.request.id".to_string(), "1".to_string());
    map.insert("Bundle.entry.request.ifMatch".to_string(), "1".to_string());
    map.insert("Bundle.entry.request.ifModifiedSince".to_string(), "1".to_string());
    map.insert("Bundle.entry.request.ifNoneExist".to_string(), "1".to_string());
    map.insert("Bundle.entry.request.ifNoneMatch".to_string(), "1".to_string());
    map.insert("Bundle.entry.request.method".to_string(), "1".to_string());
    map.insert("Bundle.entry.request.modifierExtension".to_string(), "*".to_string());
    map.insert("Bundle.entry.request.url".to_string(), "1".to_string());
    map.insert("Bundle.entry.resource".to_string(), "1".to_string());
    map.insert("Bundle.entry.response".to_string(), "1".to_string());
    map.insert("Bundle.entry.response.etag".to_string(), "1".to_string());
    map.insert("Bundle.entry.response.extension".to_string(), "*".to_string());
    map.insert("Bundle.entry.response.id".to_string(), "1".to_string());
    map.insert("Bundle.entry.response.lastModified".to_string(), "1".to_string());
    map.insert("Bundle.entry.response.location".to_string(), "1".to_string());
    map.insert("Bundle.entry.response.modifierExtension".to_string(), "*".to_string());
    map.insert("Bundle.entry.response.outcome".to_string(), "1".to_string());
    map.insert("Bundle.entry.response.status".to_string(), "1".to_string());
    map.insert("Bundle.entry.search".to_string(), "1".to_string());
    map.insert("Bundle.entry.search.extension".to_string(), "*".to_string());
    map.insert("Bundle.entry.search.id".to_string(), "1".to_string());
    map.insert("Bundle.entry.search.mode".to_string(), "1".to_string());
    map.insert("Bundle.entry.search.modifierExtension".to_string(), "*".to_string());
    map.insert("Bundle.entry.search.score".to_string(), "1".to_string());
    map.insert("Bundle.id".to_string(), "1".to_string());
    map.insert("Bundle.identifier".to_string(), "1".to_string());
    map.insert("Bundle.implicitRules".to_string(), "1".to_string());
    map.insert("Bundle.language".to_string(), "1".to_string());
    map.insert("Bundle.link".to_string(), "*".to_string());
    map.insert("Bundle.link.extension".to_string(), "*".to_string());
    map.insert("Bundle.link.id".to_string(), "1".to_string());
    map.insert("Bundle.link.modifierExtension".to_string(), "*".to_string());
    map.insert("Bundle.link.relation".to_string(), "1".to_string());
    map.insert("Bundle.link.url".to_string(), "1".to_string());
    map.insert("Bundle.meta".to_string(), "1".to_string());
    map.insert("Bundle.signature".to_string(), "1".to_string());
    map.insert("Bundle.timestamp".to_string(), "1".to_string());
    map.insert("Bundle.total".to_string(), "1".to_string());
    map.insert("Bundle.type".to_string(), "1".to_string());
    map.insert("CapabilityStatement.contact".to_string(), "*".to_string());
    map.insert("CapabilityStatement.contained".to_string(), "*".to_string());
    map.insert("CapabilityStatement.copyright".to_string(), "1".to_string());
    map.insert("CapabilityStatement.date".to_string(), "1".to_string());
    map.insert("CapabilityStatement.description".to_string(), "1".to_string());
    map.insert("CapabilityStatement.document".to_string(), "*".to_string());
    map.insert(
        "CapabilityStatement.document.documentation".to_string(),
        "1".to_string(),
    );
    map.insert("CapabilityStatement.document.extension".to_string(), "*".to_string());
    map.insert("CapabilityStatement.document.id".to_string(), "1".to_string());
    map.insert("CapabilityStatement.document.mode".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.document.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.document.profile".to_string(), "1".to_string());
    map.insert("CapabilityStatement.experimental".to_string(), "1".to_string());
    map.insert("CapabilityStatement.extension".to_string(), "*".to_string());
    map.insert("CapabilityStatement.fhirVersion".to_string(), "1".to_string());
    map.insert("CapabilityStatement.format".to_string(), "*".to_string());
    map.insert("CapabilityStatement.id".to_string(), "1".to_string());
    map.insert("CapabilityStatement.implementation".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.implementation.custodian".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.implementation.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.implementation.extension".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.implementation.id".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.implementation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.implementation.url".to_string(), "1".to_string());
    map.insert("CapabilityStatement.implementationGuide".to_string(), "*".to_string());
    map.insert("CapabilityStatement.implicitRules".to_string(), "1".to_string());
    map.insert("CapabilityStatement.imports".to_string(), "*".to_string());
    map.insert("CapabilityStatement.instantiates".to_string(), "*".to_string());
    map.insert("CapabilityStatement.jurisdiction".to_string(), "*".to_string());
    map.insert("CapabilityStatement.kind".to_string(), "1".to_string());
    map.insert("CapabilityStatement.language".to_string(), "1".to_string());
    map.insert("CapabilityStatement.messaging".to_string(), "*".to_string());
    map.insert(
        "CapabilityStatement.messaging.documentation".to_string(),
        "1".to_string(),
    );
    map.insert("CapabilityStatement.messaging.endpoint".to_string(), "*".to_string());
    map.insert(
        "CapabilityStatement.messaging.endpoint.address".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.messaging.endpoint.extension".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.messaging.endpoint.id".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.messaging.endpoint.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.messaging.endpoint.protocol".to_string(),
        "1".to_string(),
    );
    map.insert("CapabilityStatement.messaging.extension".to_string(), "*".to_string());
    map.insert("CapabilityStatement.messaging.id".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.messaging.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.messaging.reliableCache".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.messaging.supportedMessage".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.messaging.supportedMessage.definition".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.messaging.supportedMessage.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.messaging.supportedMessage.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.messaging.supportedMessage.mode".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.messaging.supportedMessage.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.meta".to_string(), "1".to_string());
    map.insert("CapabilityStatement.modifierExtension".to_string(), "*".to_string());
    map.insert("CapabilityStatement.name".to_string(), "1".to_string());
    map.insert("CapabilityStatement.patchFormat".to_string(), "*".to_string());
    map.insert("CapabilityStatement.publisher".to_string(), "1".to_string());
    map.insert("CapabilityStatement.purpose".to_string(), "1".to_string());
    map.insert("CapabilityStatement.rest".to_string(), "*".to_string());
    map.insert("CapabilityStatement.rest.compartment".to_string(), "*".to_string());
    map.insert("CapabilityStatement.rest.documentation".to_string(), "1".to_string());
    map.insert("CapabilityStatement.rest.extension".to_string(), "*".to_string());
    map.insert("CapabilityStatement.rest.id".to_string(), "1".to_string());
    map.insert("CapabilityStatement.rest.interaction".to_string(), "*".to_string());
    map.insert("CapabilityStatement.rest.interaction.code".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.rest.interaction.documentation".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.interaction.extension".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.rest.interaction.id".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.rest.interaction.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.rest.mode".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.rest.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.rest.operation".to_string(), "*".to_string());
    map.insert("CapabilityStatement.rest.resource".to_string(), "*".to_string());
    map.insert(
        "CapabilityStatement.rest.resource.conditionalCreate".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.conditionalDelete".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.conditionalRead".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.conditionalUpdate".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.documentation".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.extension".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.rest.resource.id".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.rest.resource.interaction".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.interaction.code".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.interaction.documentation".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.interaction.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.interaction.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.interaction.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.operation".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.operation.definition".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.operation.documentation".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.operation.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.operation.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.operation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.operation.name".to_string(),
        "1".to_string(),
    );
    map.insert("CapabilityStatement.rest.resource.profile".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.rest.resource.readHistory".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.referencePolicy".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.searchInclude".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.searchParam".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.searchParam.definition".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.searchParam.documentation".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.searchParam.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.searchParam.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.searchParam.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.searchParam.name".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.searchParam.type".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.searchRevInclude".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.supportedProfile".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.rest.resource.type".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.rest.resource.updateCreate".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.resource.versioning".to_string(),
        "1".to_string(),
    );
    map.insert("CapabilityStatement.rest.searchParam".to_string(), "*".to_string());
    map.insert("CapabilityStatement.rest.security".to_string(), "1".to_string());
    map.insert("CapabilityStatement.rest.security.cors".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.rest.security.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.security.extension".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.rest.security.id".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.rest.security.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.rest.security.service".to_string(), "*".to_string());
    map.insert("CapabilityStatement.software".to_string(), "1".to_string());
    map.insert("CapabilityStatement.software.extension".to_string(), "*".to_string());
    map.insert("CapabilityStatement.software.id".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.software.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.software.name".to_string(), "1".to_string());
    map.insert("CapabilityStatement.software.releaseDate".to_string(), "1".to_string());
    map.insert("CapabilityStatement.software.version".to_string(), "1".to_string());
    map.insert("CapabilityStatement.status".to_string(), "1".to_string());
    map.insert("CapabilityStatement.text".to_string(), "1".to_string());
    map.insert("CapabilityStatement.title".to_string(), "1".to_string());
    map.insert("CapabilityStatement.url".to_string(), "1".to_string());
    map.insert("CapabilityStatement.useContext".to_string(), "*".to_string());
    map.insert("CapabilityStatement.version".to_string(), "1".to_string());
    map.insert("CarePlan.activity".to_string(), "*".to_string());
    map.insert("CarePlan.activity.detail".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.code".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.dailyAmount".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.description".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.doNotPerform".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.extension".to_string(), "*".to_string());
    map.insert("CarePlan.activity.detail.goal".to_string(), "*".to_string());
    map.insert("CarePlan.activity.detail.id".to_string(), "1".to_string());
    map.insert(
        "CarePlan.activity.detail.instantiatesCanonical".to_string(),
        "*".to_string(),
    );
    map.insert("CarePlan.activity.detail.instantiatesUri".to_string(), "*".to_string());
    map.insert("CarePlan.activity.detail.kind".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.location".to_string(), "1".to_string());
    map.insert(
        "CarePlan.activity.detail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CarePlan.activity.detail.performer".to_string(), "*".to_string());
    map.insert(
        "CarePlan.activity.detail.productCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("CarePlan.activity.detail.productReference".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.quantity".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.reasonCode".to_string(), "*".to_string());
    map.insert("CarePlan.activity.detail.reasonReference".to_string(), "*".to_string());
    map.insert("CarePlan.activity.detail.scheduledPeriod".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.scheduledString".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.scheduledTiming".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.status".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.statusReason".to_string(), "1".to_string());
    map.insert("CarePlan.activity.extension".to_string(), "*".to_string());
    map.insert("CarePlan.activity.id".to_string(), "1".to_string());
    map.insert("CarePlan.activity.modifierExtension".to_string(), "*".to_string());
    map.insert("CarePlan.activity.outcomeCodeableConcept".to_string(), "*".to_string());
    map.insert("CarePlan.activity.outcomeReference".to_string(), "*".to_string());
    map.insert("CarePlan.activity.progress".to_string(), "*".to_string());
    map.insert("CarePlan.activity.reference".to_string(), "1".to_string());
    map.insert("CarePlan.addresses".to_string(), "*".to_string());
    map.insert("CarePlan.author".to_string(), "1".to_string());
    map.insert("CarePlan.basedOn".to_string(), "*".to_string());
    map.insert("CarePlan.careTeam".to_string(), "*".to_string());
    map.insert("CarePlan.category".to_string(), "*".to_string());
    map.insert("CarePlan.contained".to_string(), "*".to_string());
    map.insert("CarePlan.contributor".to_string(), "*".to_string());
    map.insert("CarePlan.created".to_string(), "1".to_string());
    map.insert("CarePlan.description".to_string(), "1".to_string());
    map.insert("CarePlan.encounter".to_string(), "1".to_string());
    map.insert("CarePlan.extension".to_string(), "*".to_string());
    map.insert("CarePlan.goal".to_string(), "*".to_string());
    map.insert("CarePlan.id".to_string(), "1".to_string());
    map.insert("CarePlan.identifier".to_string(), "*".to_string());
    map.insert("CarePlan.implicitRules".to_string(), "1".to_string());
    map.insert("CarePlan.instantiatesCanonical".to_string(), "*".to_string());
    map.insert("CarePlan.instantiatesUri".to_string(), "*".to_string());
    map.insert("CarePlan.intent".to_string(), "1".to_string());
    map.insert("CarePlan.language".to_string(), "1".to_string());
    map.insert("CarePlan.meta".to_string(), "1".to_string());
    map.insert("CarePlan.modifierExtension".to_string(), "*".to_string());
    map.insert("CarePlan.note".to_string(), "*".to_string());
    map.insert("CarePlan.partOf".to_string(), "*".to_string());
    map.insert("CarePlan.period".to_string(), "1".to_string());
    map.insert("CarePlan.replaces".to_string(), "*".to_string());
    map.insert("CarePlan.status".to_string(), "1".to_string());
    map.insert("CarePlan.subject".to_string(), "1".to_string());
    map.insert("CarePlan.supportingInfo".to_string(), "*".to_string());
    map.insert("CarePlan.text".to_string(), "1".to_string());
    map.insert("CarePlan.title".to_string(), "1".to_string());
    map.insert("CareTeam.category".to_string(), "*".to_string());
    map.insert("CareTeam.contained".to_string(), "*".to_string());
    map.insert("CareTeam.encounter".to_string(), "1".to_string());
    map.insert("CareTeam.extension".to_string(), "*".to_string());
    map.insert("CareTeam.id".to_string(), "1".to_string());
    map.insert("CareTeam.identifier".to_string(), "*".to_string());
    map.insert("CareTeam.implicitRules".to_string(), "1".to_string());
    map.insert("CareTeam.language".to_string(), "1".to_string());
    map.insert("CareTeam.managingOrganization".to_string(), "*".to_string());
    map.insert("CareTeam.meta".to_string(), "1".to_string());
    map.insert("CareTeam.modifierExtension".to_string(), "*".to_string());
    map.insert("CareTeam.name".to_string(), "1".to_string());
    map.insert("CareTeam.note".to_string(), "*".to_string());
    map.insert("CareTeam.participant".to_string(), "*".to_string());
    map.insert("CareTeam.participant.extension".to_string(), "*".to_string());
    map.insert("CareTeam.participant.id".to_string(), "1".to_string());
    map.insert("CareTeam.participant.member".to_string(), "1".to_string());
    map.insert("CareTeam.participant.modifierExtension".to_string(), "*".to_string());
    map.insert("CareTeam.participant.onBehalfOf".to_string(), "1".to_string());
    map.insert("CareTeam.participant.period".to_string(), "1".to_string());
    map.insert("CareTeam.participant.role".to_string(), "*".to_string());
    map.insert("CareTeam.period".to_string(), "1".to_string());
    map.insert("CareTeam.reasonCode".to_string(), "*".to_string());
    map.insert("CareTeam.reasonReference".to_string(), "*".to_string());
    map.insert("CareTeam.status".to_string(), "1".to_string());
    map.insert("CareTeam.subject".to_string(), "1".to_string());
    map.insert("CareTeam.telecom".to_string(), "*".to_string());
    map.insert("CareTeam.text".to_string(), "1".to_string());
    map.insert("CatalogEntry.additionalCharacteristic".to_string(), "*".to_string());
    map.insert("CatalogEntry.additionalClassification".to_string(), "*".to_string());
    map.insert("CatalogEntry.additionalIdentifier".to_string(), "*".to_string());
    map.insert("CatalogEntry.classification".to_string(), "*".to_string());
    map.insert("CatalogEntry.contained".to_string(), "*".to_string());
    map.insert("CatalogEntry.extension".to_string(), "*".to_string());
    map.insert("CatalogEntry.id".to_string(), "1".to_string());
    map.insert("CatalogEntry.identifier".to_string(), "*".to_string());
    map.insert("CatalogEntry.implicitRules".to_string(), "1".to_string());
    map.insert("CatalogEntry.language".to_string(), "1".to_string());
    map.insert("CatalogEntry.lastUpdated".to_string(), "1".to_string());
    map.insert("CatalogEntry.meta".to_string(), "1".to_string());
    map.insert("CatalogEntry.modifierExtension".to_string(), "*".to_string());
    map.insert("CatalogEntry.orderable".to_string(), "1".to_string());
    map.insert("CatalogEntry.referencedItem".to_string(), "1".to_string());
    map.insert("CatalogEntry.relatedEntry".to_string(), "*".to_string());
    map.insert("CatalogEntry.relatedEntry.extension".to_string(), "*".to_string());
    map.insert("CatalogEntry.relatedEntry.id".to_string(), "1".to_string());
    map.insert("CatalogEntry.relatedEntry.item".to_string(), "1".to_string());
    map.insert(
        "CatalogEntry.relatedEntry.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CatalogEntry.relatedEntry.relationtype".to_string(), "1".to_string());
    map.insert("CatalogEntry.status".to_string(), "1".to_string());
    map.insert("CatalogEntry.text".to_string(), "1".to_string());
    map.insert("CatalogEntry.type".to_string(), "1".to_string());
    map.insert("CatalogEntry.validTo".to_string(), "1".to_string());
    map.insert("CatalogEntry.validityPeriod".to_string(), "1".to_string());
    map.insert("ChargeItem.account".to_string(), "*".to_string());
    map.insert("ChargeItem.bodysite".to_string(), "*".to_string());
    map.insert("ChargeItem.code".to_string(), "1".to_string());
    map.insert("ChargeItem.contained".to_string(), "*".to_string());
    map.insert("ChargeItem.context".to_string(), "1".to_string());
    map.insert("ChargeItem.costCenter".to_string(), "1".to_string());
    map.insert("ChargeItem.definitionCanonical".to_string(), "*".to_string());
    map.insert("ChargeItem.definitionUri".to_string(), "*".to_string());
    map.insert("ChargeItem.enteredDate".to_string(), "1".to_string());
    map.insert("ChargeItem.enterer".to_string(), "1".to_string());
    map.insert("ChargeItem.extension".to_string(), "*".to_string());
    map.insert("ChargeItem.factorOverride".to_string(), "1".to_string());
    map.insert("ChargeItem.id".to_string(), "1".to_string());
    map.insert("ChargeItem.identifier".to_string(), "*".to_string());
    map.insert("ChargeItem.implicitRules".to_string(), "1".to_string());
    map.insert("ChargeItem.language".to_string(), "1".to_string());
    map.insert("ChargeItem.meta".to_string(), "1".to_string());
    map.insert("ChargeItem.modifierExtension".to_string(), "*".to_string());
    map.insert("ChargeItem.note".to_string(), "*".to_string());
    map.insert("ChargeItem.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("ChargeItem.occurrencePeriod".to_string(), "1".to_string());
    map.insert("ChargeItem.occurrenceTiming".to_string(), "1".to_string());
    map.insert("ChargeItem.overrideReason".to_string(), "1".to_string());
    map.insert("ChargeItem.partOf".to_string(), "*".to_string());
    map.insert("ChargeItem.performer".to_string(), "*".to_string());
    map.insert("ChargeItem.performer.actor".to_string(), "1".to_string());
    map.insert("ChargeItem.performer.extension".to_string(), "*".to_string());
    map.insert("ChargeItem.performer.function".to_string(), "1".to_string());
    map.insert("ChargeItem.performer.id".to_string(), "1".to_string());
    map.insert("ChargeItem.performer.modifierExtension".to_string(), "*".to_string());
    map.insert("ChargeItem.performingOrganization".to_string(), "1".to_string());
    map.insert("ChargeItem.priceOverride".to_string(), "1".to_string());
    map.insert("ChargeItem.productCodeableConcept".to_string(), "1".to_string());
    map.insert("ChargeItem.productReference".to_string(), "1".to_string());
    map.insert("ChargeItem.quantity".to_string(), "1".to_string());
    map.insert("ChargeItem.reason".to_string(), "*".to_string());
    map.insert("ChargeItem.requestingOrganization".to_string(), "1".to_string());
    map.insert("ChargeItem.service".to_string(), "*".to_string());
    map.insert("ChargeItem.status".to_string(), "1".to_string());
    map.insert("ChargeItem.subject".to_string(), "1".to_string());
    map.insert("ChargeItem.supportingInformation".to_string(), "*".to_string());
    map.insert("ChargeItem.text".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.applicability".to_string(), "*".to_string());
    map.insert(
        "ChargeItemDefinition.applicability.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ChargeItemDefinition.applicability.expression".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ChargeItemDefinition.applicability.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ChargeItemDefinition.applicability.id".to_string(), "1".to_string());
    map.insert(
        "ChargeItemDefinition.applicability.language".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ChargeItemDefinition.applicability.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ChargeItemDefinition.approvalDate".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.code".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.contact".to_string(), "*".to_string());
    map.insert("ChargeItemDefinition.contained".to_string(), "*".to_string());
    map.insert("ChargeItemDefinition.copyright".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.date".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.derivedFromUri".to_string(), "*".to_string());
    map.insert("ChargeItemDefinition.description".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.effectivePeriod".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.experimental".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.extension".to_string(), "*".to_string());
    map.insert("ChargeItemDefinition.id".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.identifier".to_string(), "*".to_string());
    map.insert("ChargeItemDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.instance".to_string(), "*".to_string());
    map.insert("ChargeItemDefinition.jurisdiction".to_string(), "*".to_string());
    map.insert("ChargeItemDefinition.language".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.lastReviewDate".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.meta".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("ChargeItemDefinition.partOf".to_string(), "*".to_string());
    map.insert("ChargeItemDefinition.propertyGroup".to_string(), "*".to_string());
    map.insert(
        "ChargeItemDefinition.propertyGroup.applicability".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ChargeItemDefinition.propertyGroup.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ChargeItemDefinition.propertyGroup.id".to_string(), "1".to_string());
    map.insert(
        "ChargeItemDefinition.propertyGroup.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ChargeItemDefinition.propertyGroup.priceComponent".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ChargeItemDefinition.propertyGroup.priceComponent.amount".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ChargeItemDefinition.propertyGroup.priceComponent.code".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ChargeItemDefinition.propertyGroup.priceComponent.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ChargeItemDefinition.propertyGroup.priceComponent.factor".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ChargeItemDefinition.propertyGroup.priceComponent.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ChargeItemDefinition.propertyGroup.priceComponent.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "ChargeItemDefinition.propertyGroup.priceComponent.type".to_string(),
        "1".to_string(),
    );
    map.insert("ChargeItemDefinition.publisher".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.replaces".to_string(), "*".to_string());
    map.insert("ChargeItemDefinition.status".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.text".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.title".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.url".to_string(), "1".to_string());
    map.insert("ChargeItemDefinition.useContext".to_string(), "*".to_string());
    map.insert("ChargeItemDefinition.version".to_string(), "1".to_string());
    map.insert("Claim.accident".to_string(), "1".to_string());
    map.insert("Claim.accident.date".to_string(), "1".to_string());
    map.insert("Claim.accident.extension".to_string(), "*".to_string());
    map.insert("Claim.accident.id".to_string(), "1".to_string());
    map.insert("Claim.accident.locationAddress".to_string(), "1".to_string());
    map.insert("Claim.accident.locationReference".to_string(), "1".to_string());
    map.insert("Claim.accident.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.accident.type".to_string(), "1".to_string());
    map.insert("Claim.billablePeriod".to_string(), "1".to_string());
    map.insert("Claim.careTeam".to_string(), "*".to_string());
    map.insert("Claim.careTeam.extension".to_string(), "*".to_string());
    map.insert("Claim.careTeam.id".to_string(), "1".to_string());
    map.insert("Claim.careTeam.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.careTeam.provider".to_string(), "1".to_string());
    map.insert("Claim.careTeam.qualification".to_string(), "1".to_string());
    map.insert("Claim.careTeam.responsible".to_string(), "1".to_string());
    map.insert("Claim.careTeam.role".to_string(), "1".to_string());
    map.insert("Claim.careTeam.sequence".to_string(), "1".to_string());
    map.insert("Claim.contained".to_string(), "*".to_string());
    map.insert("Claim.created".to_string(), "1".to_string());
    map.insert("Claim.diagnosis".to_string(), "*".to_string());
    map.insert("Claim.diagnosis.diagnosisCodeableConcept".to_string(), "1".to_string());
    map.insert("Claim.diagnosis.diagnosisReference".to_string(), "1".to_string());
    map.insert("Claim.diagnosis.extension".to_string(), "*".to_string());
    map.insert("Claim.diagnosis.id".to_string(), "1".to_string());
    map.insert("Claim.diagnosis.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.diagnosis.onAdmission".to_string(), "1".to_string());
    map.insert("Claim.diagnosis.packageCode".to_string(), "1".to_string());
    map.insert("Claim.diagnosis.sequence".to_string(), "1".to_string());
    map.insert("Claim.diagnosis.type".to_string(), "*".to_string());
    map.insert("Claim.enterer".to_string(), "1".to_string());
    map.insert("Claim.extension".to_string(), "*".to_string());
    map.insert("Claim.facility".to_string(), "1".to_string());
    map.insert("Claim.fundsReserve".to_string(), "1".to_string());
    map.insert("Claim.id".to_string(), "1".to_string());
    map.insert("Claim.identifier".to_string(), "*".to_string());
    map.insert("Claim.implicitRules".to_string(), "1".to_string());
    map.insert("Claim.insurance".to_string(), "*".to_string());
    map.insert("Claim.insurance.businessArrangement".to_string(), "1".to_string());
    map.insert("Claim.insurance.claimResponse".to_string(), "1".to_string());
    map.insert("Claim.insurance.coverage".to_string(), "1".to_string());
    map.insert("Claim.insurance.extension".to_string(), "*".to_string());
    map.insert("Claim.insurance.focal".to_string(), "1".to_string());
    map.insert("Claim.insurance.id".to_string(), "1".to_string());
    map.insert("Claim.insurance.identifier".to_string(), "1".to_string());
    map.insert("Claim.insurance.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.insurance.preAuthRef".to_string(), "*".to_string());
    map.insert("Claim.insurance.sequence".to_string(), "1".to_string());
    map.insert("Claim.insurer".to_string(), "1".to_string());
    map.insert("Claim.item".to_string(), "*".to_string());
    map.insert("Claim.item.bodySite".to_string(), "1".to_string());
    map.insert("Claim.item.careTeamSequence".to_string(), "*".to_string());
    map.insert("Claim.item.category".to_string(), "1".to_string());
    map.insert("Claim.item.detail".to_string(), "*".to_string());
    map.insert("Claim.item.detail.category".to_string(), "1".to_string());
    map.insert("Claim.item.detail.extension".to_string(), "*".to_string());
    map.insert("Claim.item.detail.factor".to_string(), "1".to_string());
    map.insert("Claim.item.detail.id".to_string(), "1".to_string());
    map.insert("Claim.item.detail.modifier".to_string(), "*".to_string());
    map.insert("Claim.item.detail.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.item.detail.net".to_string(), "1".to_string());
    map.insert("Claim.item.detail.productOrService".to_string(), "1".to_string());
    map.insert("Claim.item.detail.programCode".to_string(), "*".to_string());
    map.insert("Claim.item.detail.quantity".to_string(), "1".to_string());
    map.insert("Claim.item.detail.revenue".to_string(), "1".to_string());
    map.insert("Claim.item.detail.sequence".to_string(), "1".to_string());
    map.insert("Claim.item.detail.subDetail".to_string(), "*".to_string());
    map.insert("Claim.item.detail.subDetail.category".to_string(), "1".to_string());
    map.insert("Claim.item.detail.subDetail.extension".to_string(), "*".to_string());
    map.insert("Claim.item.detail.subDetail.factor".to_string(), "1".to_string());
    map.insert("Claim.item.detail.subDetail.id".to_string(), "1".to_string());
    map.insert("Claim.item.detail.subDetail.modifier".to_string(), "*".to_string());
    map.insert(
        "Claim.item.detail.subDetail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Claim.item.detail.subDetail.net".to_string(), "1".to_string());
    map.insert(
        "Claim.item.detail.subDetail.productOrService".to_string(),
        "1".to_string(),
    );
    map.insert("Claim.item.detail.subDetail.programCode".to_string(), "*".to_string());
    map.insert("Claim.item.detail.subDetail.quantity".to_string(), "1".to_string());
    map.insert("Claim.item.detail.subDetail.revenue".to_string(), "1".to_string());
    map.insert("Claim.item.detail.subDetail.sequence".to_string(), "1".to_string());
    map.insert("Claim.item.detail.subDetail.udi".to_string(), "*".to_string());
    map.insert("Claim.item.detail.subDetail.unitPrice".to_string(), "1".to_string());
    map.insert("Claim.item.detail.udi".to_string(), "*".to_string());
    map.insert("Claim.item.detail.unitPrice".to_string(), "1".to_string());
    map.insert("Claim.item.diagnosisSequence".to_string(), "*".to_string());
    map.insert("Claim.item.encounter".to_string(), "*".to_string());
    map.insert("Claim.item.extension".to_string(), "*".to_string());
    map.insert("Claim.item.factor".to_string(), "1".to_string());
    map.insert("Claim.item.id".to_string(), "1".to_string());
    map.insert("Claim.item.informationSequence".to_string(), "*".to_string());
    map.insert("Claim.item.locationAddress".to_string(), "1".to_string());
    map.insert("Claim.item.locationCodeableConcept".to_string(), "1".to_string());
    map.insert("Claim.item.locationReference".to_string(), "1".to_string());
    map.insert("Claim.item.modifier".to_string(), "*".to_string());
    map.insert("Claim.item.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.item.net".to_string(), "1".to_string());
    map.insert("Claim.item.procedureSequence".to_string(), "*".to_string());
    map.insert("Claim.item.productOrService".to_string(), "1".to_string());
    map.insert("Claim.item.programCode".to_string(), "*".to_string());
    map.insert("Claim.item.quantity".to_string(), "1".to_string());
    map.insert("Claim.item.revenue".to_string(), "1".to_string());
    map.insert("Claim.item.sequence".to_string(), "1".to_string());
    map.insert("Claim.item.servicedDate".to_string(), "1".to_string());
    map.insert("Claim.item.servicedPeriod".to_string(), "1".to_string());
    map.insert("Claim.item.subSite".to_string(), "*".to_string());
    map.insert("Claim.item.udi".to_string(), "*".to_string());
    map.insert("Claim.item.unitPrice".to_string(), "1".to_string());
    map.insert("Claim.language".to_string(), "1".to_string());
    map.insert("Claim.meta".to_string(), "1".to_string());
    map.insert("Claim.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.originalPrescription".to_string(), "1".to_string());
    map.insert("Claim.patient".to_string(), "1".to_string());
    map.insert("Claim.payee".to_string(), "1".to_string());
    map.insert("Claim.payee.extension".to_string(), "*".to_string());
    map.insert("Claim.payee.id".to_string(), "1".to_string());
    map.insert("Claim.payee.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.payee.party".to_string(), "1".to_string());
    map.insert("Claim.payee.type".to_string(), "1".to_string());
    map.insert("Claim.prescription".to_string(), "1".to_string());
    map.insert("Claim.priority".to_string(), "1".to_string());
    map.insert("Claim.procedure".to_string(), "*".to_string());
    map.insert("Claim.procedure.date".to_string(), "1".to_string());
    map.insert("Claim.procedure.extension".to_string(), "*".to_string());
    map.insert("Claim.procedure.id".to_string(), "1".to_string());
    map.insert("Claim.procedure.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.procedure.procedureCodeableConcept".to_string(), "1".to_string());
    map.insert("Claim.procedure.procedureReference".to_string(), "1".to_string());
    map.insert("Claim.procedure.sequence".to_string(), "1".to_string());
    map.insert("Claim.procedure.type".to_string(), "*".to_string());
    map.insert("Claim.procedure.udi".to_string(), "*".to_string());
    map.insert("Claim.provider".to_string(), "1".to_string());
    map.insert("Claim.referral".to_string(), "1".to_string());
    map.insert("Claim.related".to_string(), "*".to_string());
    map.insert("Claim.related.claim".to_string(), "1".to_string());
    map.insert("Claim.related.extension".to_string(), "*".to_string());
    map.insert("Claim.related.id".to_string(), "1".to_string());
    map.insert("Claim.related.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.related.reference".to_string(), "1".to_string());
    map.insert("Claim.related.relationship".to_string(), "1".to_string());
    map.insert("Claim.status".to_string(), "1".to_string());
    map.insert("Claim.subType".to_string(), "1".to_string());
    map.insert("Claim.supportingInfo".to_string(), "*".to_string());
    map.insert("Claim.supportingInfo.category".to_string(), "1".to_string());
    map.insert("Claim.supportingInfo.code".to_string(), "1".to_string());
    map.insert("Claim.supportingInfo.extension".to_string(), "*".to_string());
    map.insert("Claim.supportingInfo.id".to_string(), "1".to_string());
    map.insert("Claim.supportingInfo.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.supportingInfo.reason".to_string(), "1".to_string());
    map.insert("Claim.supportingInfo.sequence".to_string(), "1".to_string());
    map.insert("Claim.supportingInfo.timingDate".to_string(), "1".to_string());
    map.insert("Claim.supportingInfo.timingPeriod".to_string(), "1".to_string());
    map.insert("Claim.supportingInfo.valueAttachment".to_string(), "1".to_string());
    map.insert("Claim.supportingInfo.valueBoolean".to_string(), "1".to_string());
    map.insert("Claim.supportingInfo.valueQuantity".to_string(), "1".to_string());
    map.insert("Claim.supportingInfo.valueReference".to_string(), "1".to_string());
    map.insert("Claim.supportingInfo.valueString".to_string(), "1".to_string());
    map.insert("Claim.text".to_string(), "1".to_string());
    map.insert("Claim.total".to_string(), "1".to_string());
    map.insert("Claim.type".to_string(), "1".to_string());
    map.insert("Claim.use".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.adjudication".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.bodySite".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.detail".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.detail.adjudication".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.detail.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.detail.factor".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.detail.id".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.detail.modifier".to_string(), "*".to_string());
    map.insert(
        "ClaimResponse.addItem.detail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ClaimResponse.addItem.detail.net".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.detail.noteNumber".to_string(), "*".to_string());
    map.insert(
        "ClaimResponse.addItem.detail.productOrService".to_string(),
        "1".to_string(),
    );
    map.insert("ClaimResponse.addItem.detail.quantity".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.detail.subDetail".to_string(), "*".to_string());
    map.insert(
        "ClaimResponse.addItem.detail.subDetail.adjudication".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ClaimResponse.addItem.detail.subDetail.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ClaimResponse.addItem.detail.subDetail.factor".to_string(),
        "1".to_string(),
    );
    map.insert("ClaimResponse.addItem.detail.subDetail.id".to_string(), "1".to_string());
    map.insert(
        "ClaimResponse.addItem.detail.subDetail.modifier".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ClaimResponse.addItem.detail.subDetail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ClaimResponse.addItem.detail.subDetail.net".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ClaimResponse.addItem.detail.subDetail.noteNumber".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ClaimResponse.addItem.detail.subDetail.productOrService".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ClaimResponse.addItem.detail.subDetail.quantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ClaimResponse.addItem.detail.subDetail.unitPrice".to_string(),
        "1".to_string(),
    );
    map.insert("ClaimResponse.addItem.detail.unitPrice".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.detailSequence".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.factor".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.id".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.itemSequence".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.locationAddress".to_string(), "1".to_string());
    map.insert(
        "ClaimResponse.addItem.locationCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("ClaimResponse.addItem.locationReference".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.modifier".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.modifierExtension".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.net".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.noteNumber".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.productOrService".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.programCode".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.provider".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.quantity".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.servicedDate".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.servicedPeriod".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.subSite".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.subdetailSequence".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.unitPrice".to_string(), "1".to_string());
    map.insert("ClaimResponse.adjudication".to_string(), "*".to_string());
    map.insert("ClaimResponse.communicationRequest".to_string(), "*".to_string());
    map.insert("ClaimResponse.contained".to_string(), "*".to_string());
    map.insert("ClaimResponse.created".to_string(), "1".to_string());
    map.insert("ClaimResponse.disposition".to_string(), "1".to_string());
    map.insert("ClaimResponse.error".to_string(), "*".to_string());
    map.insert("ClaimResponse.error.code".to_string(), "1".to_string());
    map.insert("ClaimResponse.error.detailSequence".to_string(), "1".to_string());
    map.insert("ClaimResponse.error.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.error.id".to_string(), "1".to_string());
    map.insert("ClaimResponse.error.itemSequence".to_string(), "1".to_string());
    map.insert("ClaimResponse.error.modifierExtension".to_string(), "*".to_string());
    map.insert("ClaimResponse.error.subDetailSequence".to_string(), "1".to_string());
    map.insert("ClaimResponse.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.form".to_string(), "1".to_string());
    map.insert("ClaimResponse.formCode".to_string(), "1".to_string());
    map.insert("ClaimResponse.fundsReserve".to_string(), "1".to_string());
    map.insert("ClaimResponse.id".to_string(), "1".to_string());
    map.insert("ClaimResponse.identifier".to_string(), "*".to_string());
    map.insert("ClaimResponse.implicitRules".to_string(), "1".to_string());
    map.insert("ClaimResponse.insurance".to_string(), "*".to_string());
    map.insert(
        "ClaimResponse.insurance.businessArrangement".to_string(),
        "1".to_string(),
    );
    map.insert("ClaimResponse.insurance.claimResponse".to_string(), "1".to_string());
    map.insert("ClaimResponse.insurance.coverage".to_string(), "1".to_string());
    map.insert("ClaimResponse.insurance.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.insurance.focal".to_string(), "1".to_string());
    map.insert("ClaimResponse.insurance.id".to_string(), "1".to_string());
    map.insert("ClaimResponse.insurance.modifierExtension".to_string(), "*".to_string());
    map.insert("ClaimResponse.insurance.sequence".to_string(), "1".to_string());
    map.insert("ClaimResponse.insurer".to_string(), "1".to_string());
    map.insert("ClaimResponse.item".to_string(), "*".to_string());
    map.insert("ClaimResponse.item.adjudication".to_string(), "*".to_string());
    map.insert("ClaimResponse.item.adjudication.amount".to_string(), "1".to_string());
    map.insert("ClaimResponse.item.adjudication.category".to_string(), "1".to_string());
    map.insert("ClaimResponse.item.adjudication.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.item.adjudication.id".to_string(), "1".to_string());
    map.insert(
        "ClaimResponse.item.adjudication.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ClaimResponse.item.adjudication.reason".to_string(), "1".to_string());
    map.insert("ClaimResponse.item.adjudication.value".to_string(), "1".to_string());
    map.insert("ClaimResponse.item.detail".to_string(), "*".to_string());
    map.insert("ClaimResponse.item.detail.adjudication".to_string(), "*".to_string());
    map.insert("ClaimResponse.item.detail.detailSequence".to_string(), "1".to_string());
    map.insert("ClaimResponse.item.detail.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.item.detail.id".to_string(), "1".to_string());
    map.insert(
        "ClaimResponse.item.detail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ClaimResponse.item.detail.noteNumber".to_string(), "*".to_string());
    map.insert("ClaimResponse.item.detail.subDetail".to_string(), "*".to_string());
    map.insert(
        "ClaimResponse.item.detail.subDetail.adjudication".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ClaimResponse.item.detail.subDetail.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ClaimResponse.item.detail.subDetail.id".to_string(), "1".to_string());
    map.insert(
        "ClaimResponse.item.detail.subDetail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ClaimResponse.item.detail.subDetail.noteNumber".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ClaimResponse.item.detail.subDetail.subDetailSequence".to_string(),
        "1".to_string(),
    );
    map.insert("ClaimResponse.item.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.item.id".to_string(), "1".to_string());
    map.insert("ClaimResponse.item.itemSequence".to_string(), "1".to_string());
    map.insert("ClaimResponse.item.modifierExtension".to_string(), "*".to_string());
    map.insert("ClaimResponse.item.noteNumber".to_string(), "*".to_string());
    map.insert("ClaimResponse.language".to_string(), "1".to_string());
    map.insert("ClaimResponse.meta".to_string(), "1".to_string());
    map.insert("ClaimResponse.modifierExtension".to_string(), "*".to_string());
    map.insert("ClaimResponse.outcome".to_string(), "1".to_string());
    map.insert("ClaimResponse.patient".to_string(), "1".to_string());
    map.insert("ClaimResponse.payeeType".to_string(), "1".to_string());
    map.insert("ClaimResponse.payment".to_string(), "1".to_string());
    map.insert("ClaimResponse.payment.adjustment".to_string(), "1".to_string());
    map.insert("ClaimResponse.payment.adjustmentReason".to_string(), "1".to_string());
    map.insert("ClaimResponse.payment.amount".to_string(), "1".to_string());
    map.insert("ClaimResponse.payment.date".to_string(), "1".to_string());
    map.insert("ClaimResponse.payment.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.payment.id".to_string(), "1".to_string());
    map.insert("ClaimResponse.payment.identifier".to_string(), "1".to_string());
    map.insert("ClaimResponse.payment.modifierExtension".to_string(), "*".to_string());
    map.insert("ClaimResponse.payment.type".to_string(), "1".to_string());
    map.insert("ClaimResponse.preAuthPeriod".to_string(), "1".to_string());
    map.insert("ClaimResponse.preAuthRef".to_string(), "1".to_string());
    map.insert("ClaimResponse.processNote".to_string(), "*".to_string());
    map.insert("ClaimResponse.processNote.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.processNote.id".to_string(), "1".to_string());
    map.insert("ClaimResponse.processNote.language".to_string(), "1".to_string());
    map.insert(
        "ClaimResponse.processNote.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ClaimResponse.processNote.number".to_string(), "1".to_string());
    map.insert("ClaimResponse.processNote.text".to_string(), "1".to_string());
    map.insert("ClaimResponse.processNote.type".to_string(), "1".to_string());
    map.insert("ClaimResponse.request".to_string(), "1".to_string());
    map.insert("ClaimResponse.requestor".to_string(), "1".to_string());
    map.insert("ClaimResponse.status".to_string(), "1".to_string());
    map.insert("ClaimResponse.subType".to_string(), "1".to_string());
    map.insert("ClaimResponse.text".to_string(), "1".to_string());
    map.insert("ClaimResponse.total".to_string(), "*".to_string());
    map.insert("ClaimResponse.total.amount".to_string(), "1".to_string());
    map.insert("ClaimResponse.total.category".to_string(), "1".to_string());
    map.insert("ClaimResponse.total.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.total.id".to_string(), "1".to_string());
    map.insert("ClaimResponse.total.modifierExtension".to_string(), "*".to_string());
    map.insert("ClaimResponse.type".to_string(), "1".to_string());
    map.insert("ClaimResponse.use".to_string(), "1".to_string());
    map.insert("ClinicalImpression.assessor".to_string(), "1".to_string());
    map.insert("ClinicalImpression.code".to_string(), "1".to_string());
    map.insert("ClinicalImpression.contained".to_string(), "*".to_string());
    map.insert("ClinicalImpression.date".to_string(), "1".to_string());
    map.insert("ClinicalImpression.description".to_string(), "1".to_string());
    map.insert("ClinicalImpression.effectiveDateTime".to_string(), "1".to_string());
    map.insert("ClinicalImpression.effectivePeriod".to_string(), "1".to_string());
    map.insert("ClinicalImpression.encounter".to_string(), "1".to_string());
    map.insert("ClinicalImpression.extension".to_string(), "*".to_string());
    map.insert("ClinicalImpression.finding".to_string(), "*".to_string());
    map.insert("ClinicalImpression.finding.basis".to_string(), "1".to_string());
    map.insert("ClinicalImpression.finding.extension".to_string(), "*".to_string());
    map.insert("ClinicalImpression.finding.id".to_string(), "1".to_string());
    map.insert(
        "ClinicalImpression.finding.itemCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("ClinicalImpression.finding.itemReference".to_string(), "1".to_string());
    map.insert(
        "ClinicalImpression.finding.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ClinicalImpression.id".to_string(), "1".to_string());
    map.insert("ClinicalImpression.identifier".to_string(), "*".to_string());
    map.insert("ClinicalImpression.implicitRules".to_string(), "1".to_string());
    map.insert("ClinicalImpression.investigation".to_string(), "*".to_string());
    map.insert("ClinicalImpression.investigation.code".to_string(), "1".to_string());
    map.insert(
        "ClinicalImpression.investigation.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ClinicalImpression.investigation.id".to_string(), "1".to_string());
    map.insert("ClinicalImpression.investigation.item".to_string(), "*".to_string());
    map.insert(
        "ClinicalImpression.investigation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ClinicalImpression.language".to_string(), "1".to_string());
    map.insert("ClinicalImpression.meta".to_string(), "1".to_string());
    map.insert("ClinicalImpression.modifierExtension".to_string(), "*".to_string());
    map.insert("ClinicalImpression.note".to_string(), "*".to_string());
    map.insert("ClinicalImpression.previous".to_string(), "1".to_string());
    map.insert("ClinicalImpression.problem".to_string(), "*".to_string());
    map.insert(
        "ClinicalImpression.prognosisCodeableConcept".to_string(),
        "*".to_string(),
    );
    map.insert("ClinicalImpression.prognosisReference".to_string(), "*".to_string());
    map.insert("ClinicalImpression.protocol".to_string(), "*".to_string());
    map.insert("ClinicalImpression.status".to_string(), "1".to_string());
    map.insert("ClinicalImpression.statusReason".to_string(), "1".to_string());
    map.insert("ClinicalImpression.subject".to_string(), "1".to_string());
    map.insert("ClinicalImpression.summary".to_string(), "1".to_string());
    map.insert("ClinicalImpression.supportingInfo".to_string(), "*".to_string());
    map.insert("ClinicalImpression.text".to_string(), "1".to_string());
    map.insert("CodeSystem.caseSensitive".to_string(), "1".to_string());
    map.insert("CodeSystem.compositional".to_string(), "1".to_string());
    map.insert("CodeSystem.concept".to_string(), "*".to_string());
    map.insert("CodeSystem.concept.code".to_string(), "1".to_string());
    map.insert("CodeSystem.concept.concept".to_string(), "*".to_string());
    map.insert("CodeSystem.concept.definition".to_string(), "1".to_string());
    map.insert("CodeSystem.concept.designation".to_string(), "*".to_string());
    map.insert("CodeSystem.concept.designation.extension".to_string(), "*".to_string());
    map.insert("CodeSystem.concept.designation.id".to_string(), "1".to_string());
    map.insert("CodeSystem.concept.designation.language".to_string(), "1".to_string());
    map.insert(
        "CodeSystem.concept.designation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CodeSystem.concept.designation.use".to_string(), "1".to_string());
    map.insert("CodeSystem.concept.designation.value".to_string(), "1".to_string());
    map.insert("CodeSystem.concept.display".to_string(), "1".to_string());
    map.insert("CodeSystem.concept.extension".to_string(), "*".to_string());
    map.insert("CodeSystem.concept.id".to_string(), "1".to_string());
    map.insert("CodeSystem.concept.modifierExtension".to_string(), "*".to_string());
    map.insert("CodeSystem.concept.property".to_string(), "*".to_string());
    map.insert("CodeSystem.concept.property.code".to_string(), "1".to_string());
    map.insert("CodeSystem.concept.property.extension".to_string(), "*".to_string());
    map.insert("CodeSystem.concept.property.id".to_string(), "1".to_string());
    map.insert(
        "CodeSystem.concept.property.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CodeSystem.concept.property.valueBoolean".to_string(), "1".to_string());
    map.insert("CodeSystem.concept.property.valueCode".to_string(), "1".to_string());
    map.insert("CodeSystem.concept.property.valueCoding".to_string(), "1".to_string());
    map.insert("CodeSystem.concept.property.valueDateTime".to_string(), "1".to_string());
    map.insert("CodeSystem.concept.property.valueDecimal".to_string(), "1".to_string());
    map.insert("CodeSystem.concept.property.valueInteger".to_string(), "1".to_string());
    map.insert("CodeSystem.concept.property.valueString".to_string(), "1".to_string());
    map.insert("CodeSystem.contact".to_string(), "*".to_string());
    map.insert("CodeSystem.contained".to_string(), "*".to_string());
    map.insert("CodeSystem.content".to_string(), "1".to_string());
    map.insert("CodeSystem.copyright".to_string(), "1".to_string());
    map.insert("CodeSystem.count".to_string(), "1".to_string());
    map.insert("CodeSystem.date".to_string(), "1".to_string());
    map.insert("CodeSystem.description".to_string(), "1".to_string());
    map.insert("CodeSystem.experimental".to_string(), "1".to_string());
    map.insert("CodeSystem.extension".to_string(), "*".to_string());
    map.insert("CodeSystem.filter".to_string(), "*".to_string());
    map.insert("CodeSystem.filter.code".to_string(), "1".to_string());
    map.insert("CodeSystem.filter.description".to_string(), "1".to_string());
    map.insert("CodeSystem.filter.extension".to_string(), "*".to_string());
    map.insert("CodeSystem.filter.id".to_string(), "1".to_string());
    map.insert("CodeSystem.filter.modifierExtension".to_string(), "*".to_string());
    map.insert("CodeSystem.filter.operator".to_string(), "*".to_string());
    map.insert("CodeSystem.filter.value".to_string(), "1".to_string());
    map.insert("CodeSystem.hierarchyMeaning".to_string(), "1".to_string());
    map.insert("CodeSystem.id".to_string(), "1".to_string());
    map.insert("CodeSystem.identifier".to_string(), "*".to_string());
    map.insert("CodeSystem.implicitRules".to_string(), "1".to_string());
    map.insert("CodeSystem.jurisdiction".to_string(), "*".to_string());
    map.insert("CodeSystem.language".to_string(), "1".to_string());
    map.insert("CodeSystem.meta".to_string(), "1".to_string());
    map.insert("CodeSystem.modifierExtension".to_string(), "*".to_string());
    map.insert("CodeSystem.name".to_string(), "1".to_string());
    map.insert("CodeSystem.property".to_string(), "*".to_string());
    map.insert("CodeSystem.property.code".to_string(), "1".to_string());
    map.insert("CodeSystem.property.description".to_string(), "1".to_string());
    map.insert("CodeSystem.property.extension".to_string(), "*".to_string());
    map.insert("CodeSystem.property.id".to_string(), "1".to_string());
    map.insert("CodeSystem.property.modifierExtension".to_string(), "*".to_string());
    map.insert("CodeSystem.property.type".to_string(), "1".to_string());
    map.insert("CodeSystem.property.uri".to_string(), "1".to_string());
    map.insert("CodeSystem.publisher".to_string(), "1".to_string());
    map.insert("CodeSystem.purpose".to_string(), "1".to_string());
    map.insert("CodeSystem.status".to_string(), "1".to_string());
    map.insert("CodeSystem.supplements".to_string(), "1".to_string());
    map.insert("CodeSystem.text".to_string(), "1".to_string());
    map.insert("CodeSystem.title".to_string(), "1".to_string());
    map.insert("CodeSystem.url".to_string(), "1".to_string());
    map.insert("CodeSystem.useContext".to_string(), "*".to_string());
    map.insert("CodeSystem.valueSet".to_string(), "1".to_string());
    map.insert("CodeSystem.version".to_string(), "1".to_string());
    map.insert("CodeSystem.versionNeeded".to_string(), "1".to_string());
    map.insert("CodeableConcept.coding".to_string(), "*".to_string());
    map.insert("CodeableConcept.extension".to_string(), "*".to_string());
    map.insert("CodeableConcept.id".to_string(), "1".to_string());
    map.insert("CodeableConcept.text".to_string(), "1".to_string());
    map.insert("Coding.code".to_string(), "1".to_string());
    map.insert("Coding.display".to_string(), "1".to_string());
    map.insert("Coding.extension".to_string(), "*".to_string());
    map.insert("Coding.id".to_string(), "1".to_string());
    map.insert("Coding.system".to_string(), "1".to_string());
    map.insert("Coding.userSelected".to_string(), "1".to_string());
    map.insert("Coding.version".to_string(), "1".to_string());
    map.insert("Communication.about".to_string(), "*".to_string());
    map.insert("Communication.basedOn".to_string(), "*".to_string());
    map.insert("Communication.category".to_string(), "*".to_string());
    map.insert("Communication.contained".to_string(), "*".to_string());
    map.insert("Communication.encounter".to_string(), "1".to_string());
    map.insert("Communication.extension".to_string(), "*".to_string());
    map.insert("Communication.id".to_string(), "1".to_string());
    map.insert("Communication.identifier".to_string(), "*".to_string());
    map.insert("Communication.implicitRules".to_string(), "1".to_string());
    map.insert("Communication.inResponseTo".to_string(), "*".to_string());
    map.insert("Communication.instantiatesCanonical".to_string(), "*".to_string());
    map.insert("Communication.instantiatesUri".to_string(), "*".to_string());
    map.insert("Communication.language".to_string(), "1".to_string());
    map.insert("Communication.medium".to_string(), "*".to_string());
    map.insert("Communication.meta".to_string(), "1".to_string());
    map.insert("Communication.modifierExtension".to_string(), "*".to_string());
    map.insert("Communication.note".to_string(), "*".to_string());
    map.insert("Communication.partOf".to_string(), "*".to_string());
    map.insert("Communication.payload".to_string(), "*".to_string());
    map.insert("Communication.payload.contentAttachment".to_string(), "1".to_string());
    map.insert("Communication.payload.contentReference".to_string(), "1".to_string());
    map.insert("Communication.payload.contentString".to_string(), "1".to_string());
    map.insert("Communication.payload.extension".to_string(), "*".to_string());
    map.insert("Communication.payload.id".to_string(), "1".to_string());
    map.insert("Communication.payload.modifierExtension".to_string(), "*".to_string());
    map.insert("Communication.priority".to_string(), "1".to_string());
    map.insert("Communication.reasonCode".to_string(), "*".to_string());
    map.insert("Communication.reasonReference".to_string(), "*".to_string());
    map.insert("Communication.received".to_string(), "1".to_string());
    map.insert("Communication.recipient".to_string(), "*".to_string());
    map.insert("Communication.sender".to_string(), "1".to_string());
    map.insert("Communication.sent".to_string(), "1".to_string());
    map.insert("Communication.status".to_string(), "1".to_string());
    map.insert("Communication.statusReason".to_string(), "1".to_string());
    map.insert("Communication.subject".to_string(), "1".to_string());
    map.insert("Communication.text".to_string(), "1".to_string());
    map.insert("Communication.topic".to_string(), "1".to_string());
    map.insert("CommunicationRequest.about".to_string(), "*".to_string());
    map.insert("CommunicationRequest.authoredOn".to_string(), "1".to_string());
    map.insert("CommunicationRequest.basedOn".to_string(), "*".to_string());
    map.insert("CommunicationRequest.category".to_string(), "*".to_string());
    map.insert("CommunicationRequest.contained".to_string(), "*".to_string());
    map.insert("CommunicationRequest.doNotPerform".to_string(), "1".to_string());
    map.insert("CommunicationRequest.encounter".to_string(), "1".to_string());
    map.insert("CommunicationRequest.extension".to_string(), "*".to_string());
    map.insert("CommunicationRequest.groupIdentifier".to_string(), "1".to_string());
    map.insert("CommunicationRequest.id".to_string(), "1".to_string());
    map.insert("CommunicationRequest.identifier".to_string(), "*".to_string());
    map.insert("CommunicationRequest.implicitRules".to_string(), "1".to_string());
    map.insert("CommunicationRequest.language".to_string(), "1".to_string());
    map.insert("CommunicationRequest.medium".to_string(), "*".to_string());
    map.insert("CommunicationRequest.meta".to_string(), "1".to_string());
    map.insert("CommunicationRequest.modifierExtension".to_string(), "*".to_string());
    map.insert("CommunicationRequest.note".to_string(), "*".to_string());
    map.insert("CommunicationRequest.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("CommunicationRequest.occurrencePeriod".to_string(), "1".to_string());
    map.insert("CommunicationRequest.payload".to_string(), "*".to_string());
    map.insert(
        "CommunicationRequest.payload.contentAttachment".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CommunicationRequest.payload.contentReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CommunicationRequest.payload.contentString".to_string(),
        "1".to_string(),
    );
    map.insert("CommunicationRequest.payload.extension".to_string(), "*".to_string());
    map.insert("CommunicationRequest.payload.id".to_string(), "1".to_string());
    map.insert(
        "CommunicationRequest.payload.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CommunicationRequest.priority".to_string(), "1".to_string());
    map.insert("CommunicationRequest.reasonCode".to_string(), "*".to_string());
    map.insert("CommunicationRequest.reasonReference".to_string(), "*".to_string());
    map.insert("CommunicationRequest.recipient".to_string(), "*".to_string());
    map.insert("CommunicationRequest.replaces".to_string(), "*".to_string());
    map.insert("CommunicationRequest.requester".to_string(), "1".to_string());
    map.insert("CommunicationRequest.sender".to_string(), "1".to_string());
    map.insert("CommunicationRequest.status".to_string(), "1".to_string());
    map.insert("CommunicationRequest.statusReason".to_string(), "1".to_string());
    map.insert("CommunicationRequest.subject".to_string(), "1".to_string());
    map.insert("CommunicationRequest.text".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.code".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.contact".to_string(), "*".to_string());
    map.insert("CompartmentDefinition.contained".to_string(), "*".to_string());
    map.insert("CompartmentDefinition.date".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.description".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.experimental".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.extension".to_string(), "*".to_string());
    map.insert("CompartmentDefinition.id".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.language".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.meta".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("CompartmentDefinition.name".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.publisher".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.purpose".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.resource".to_string(), "*".to_string());
    map.insert("CompartmentDefinition.resource.code".to_string(), "1".to_string());
    map.insert(
        "CompartmentDefinition.resource.documentation".to_string(),
        "1".to_string(),
    );
    map.insert("CompartmentDefinition.resource.extension".to_string(), "*".to_string());
    map.insert("CompartmentDefinition.resource.id".to_string(), "1".to_string());
    map.insert(
        "CompartmentDefinition.resource.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CompartmentDefinition.resource.param".to_string(), "*".to_string());
    map.insert("CompartmentDefinition.search".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.status".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.text".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.url".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.useContext".to_string(), "*".to_string());
    map.insert("CompartmentDefinition.version".to_string(), "1".to_string());
    map.insert("Composition.attester".to_string(), "*".to_string());
    map.insert("Composition.attester.extension".to_string(), "*".to_string());
    map.insert("Composition.attester.id".to_string(), "1".to_string());
    map.insert("Composition.attester.mode".to_string(), "1".to_string());
    map.insert("Composition.attester.modifierExtension".to_string(), "*".to_string());
    map.insert("Composition.attester.party".to_string(), "1".to_string());
    map.insert("Composition.attester.time".to_string(), "1".to_string());
    map.insert("Composition.author".to_string(), "*".to_string());
    map.insert("Composition.category".to_string(), "*".to_string());
    map.insert("Composition.confidentiality".to_string(), "1".to_string());
    map.insert("Composition.contained".to_string(), "*".to_string());
    map.insert("Composition.custodian".to_string(), "1".to_string());
    map.insert("Composition.date".to_string(), "1".to_string());
    map.insert("Composition.encounter".to_string(), "1".to_string());
    map.insert("Composition.event".to_string(), "*".to_string());
    map.insert("Composition.event.code".to_string(), "*".to_string());
    map.insert("Composition.event.detail".to_string(), "*".to_string());
    map.insert("Composition.event.extension".to_string(), "*".to_string());
    map.insert("Composition.event.id".to_string(), "1".to_string());
    map.insert("Composition.event.modifierExtension".to_string(), "*".to_string());
    map.insert("Composition.event.period".to_string(), "1".to_string());
    map.insert("Composition.extension".to_string(), "*".to_string());
    map.insert("Composition.id".to_string(), "1".to_string());
    map.insert("Composition.identifier".to_string(), "1".to_string());
    map.insert("Composition.implicitRules".to_string(), "1".to_string());
    map.insert("Composition.language".to_string(), "1".to_string());
    map.insert("Composition.meta".to_string(), "1".to_string());
    map.insert("Composition.modifierExtension".to_string(), "*".to_string());
    map.insert("Composition.relatesTo".to_string(), "*".to_string());
    map.insert("Composition.relatesTo.code".to_string(), "1".to_string());
    map.insert("Composition.relatesTo.extension".to_string(), "*".to_string());
    map.insert("Composition.relatesTo.id".to_string(), "1".to_string());
    map.insert("Composition.relatesTo.modifierExtension".to_string(), "*".to_string());
    map.insert("Composition.relatesTo.targetIdentifier".to_string(), "1".to_string());
    map.insert("Composition.relatesTo.targetReference".to_string(), "1".to_string());
    map.insert("Composition.section".to_string(), "*".to_string());
    map.insert("Composition.section.author".to_string(), "*".to_string());
    map.insert("Composition.section.code".to_string(), "1".to_string());
    map.insert("Composition.section.emptyReason".to_string(), "1".to_string());
    map.insert("Composition.section.entry".to_string(), "*".to_string());
    map.insert("Composition.section.extension".to_string(), "*".to_string());
    map.insert("Composition.section.focus".to_string(), "1".to_string());
    map.insert("Composition.section.id".to_string(), "1".to_string());
    map.insert("Composition.section.mode".to_string(), "1".to_string());
    map.insert("Composition.section.modifierExtension".to_string(), "*".to_string());
    map.insert("Composition.section.orderedBy".to_string(), "1".to_string());
    map.insert("Composition.section.section".to_string(), "*".to_string());
    map.insert("Composition.section.text".to_string(), "1".to_string());
    map.insert("Composition.section.title".to_string(), "1".to_string());
    map.insert("Composition.status".to_string(), "1".to_string());
    map.insert("Composition.subject".to_string(), "1".to_string());
    map.insert("Composition.text".to_string(), "1".to_string());
    map.insert("Composition.title".to_string(), "1".to_string());
    map.insert("Composition.type".to_string(), "1".to_string());
    map.insert("ConceptMap.contact".to_string(), "*".to_string());
    map.insert("ConceptMap.contained".to_string(), "*".to_string());
    map.insert("ConceptMap.copyright".to_string(), "1".to_string());
    map.insert("ConceptMap.date".to_string(), "1".to_string());
    map.insert("ConceptMap.description".to_string(), "1".to_string());
    map.insert("ConceptMap.experimental".to_string(), "1".to_string());
    map.insert("ConceptMap.extension".to_string(), "*".to_string());
    map.insert("ConceptMap.group".to_string(), "*".to_string());
    map.insert("ConceptMap.group.element".to_string(), "*".to_string());
    map.insert("ConceptMap.group.element.code".to_string(), "1".to_string());
    map.insert("ConceptMap.group.element.display".to_string(), "1".to_string());
    map.insert("ConceptMap.group.element.extension".to_string(), "*".to_string());
    map.insert("ConceptMap.group.element.id".to_string(), "1".to_string());
    map.insert(
        "ConceptMap.group.element.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ConceptMap.group.element.target".to_string(), "*".to_string());
    map.insert("ConceptMap.group.element.target.code".to_string(), "1".to_string());
    map.insert("ConceptMap.group.element.target.comment".to_string(), "1".to_string());
    map.insert("ConceptMap.group.element.target.dependsOn".to_string(), "*".to_string());
    map.insert(
        "ConceptMap.group.element.target.dependsOn.display".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ConceptMap.group.element.target.dependsOn.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ConceptMap.group.element.target.dependsOn.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ConceptMap.group.element.target.dependsOn.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ConceptMap.group.element.target.dependsOn.property".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ConceptMap.group.element.target.dependsOn.system".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ConceptMap.group.element.target.dependsOn.value".to_string(),
        "1".to_string(),
    );
    map.insert("ConceptMap.group.element.target.display".to_string(), "1".to_string());
    map.insert(
        "ConceptMap.group.element.target.equivalence".to_string(),
        "1".to_string(),
    );
    map.insert("ConceptMap.group.element.target.extension".to_string(), "*".to_string());
    map.insert("ConceptMap.group.element.target.id".to_string(), "1".to_string());
    map.insert(
        "ConceptMap.group.element.target.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ConceptMap.group.element.target.product".to_string(), "*".to_string());
    map.insert("ConceptMap.group.extension".to_string(), "*".to_string());
    map.insert("ConceptMap.group.id".to_string(), "1".to_string());
    map.insert("ConceptMap.group.modifierExtension".to_string(), "*".to_string());
    map.insert("ConceptMap.group.source".to_string(), "1".to_string());
    map.insert("ConceptMap.group.sourceVersion".to_string(), "1".to_string());
    map.insert("ConceptMap.group.target".to_string(), "1".to_string());
    map.insert("ConceptMap.group.targetVersion".to_string(), "1".to_string());
    map.insert("ConceptMap.group.unmapped".to_string(), "1".to_string());
    map.insert("ConceptMap.group.unmapped.code".to_string(), "1".to_string());
    map.insert("ConceptMap.group.unmapped.display".to_string(), "1".to_string());
    map.insert("ConceptMap.group.unmapped.extension".to_string(), "*".to_string());
    map.insert("ConceptMap.group.unmapped.id".to_string(), "1".to_string());
    map.insert("ConceptMap.group.unmapped.mode".to_string(), "1".to_string());
    map.insert(
        "ConceptMap.group.unmapped.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ConceptMap.group.unmapped.url".to_string(), "1".to_string());
    map.insert("ConceptMap.id".to_string(), "1".to_string());
    map.insert("ConceptMap.identifier".to_string(), "1".to_string());
    map.insert("ConceptMap.implicitRules".to_string(), "1".to_string());
    map.insert("ConceptMap.jurisdiction".to_string(), "*".to_string());
    map.insert("ConceptMap.language".to_string(), "1".to_string());
    map.insert("ConceptMap.meta".to_string(), "1".to_string());
    map.insert("ConceptMap.modifierExtension".to_string(), "*".to_string());
    map.insert("ConceptMap.name".to_string(), "1".to_string());
    map.insert("ConceptMap.publisher".to_string(), "1".to_string());
    map.insert("ConceptMap.purpose".to_string(), "1".to_string());
    map.insert("ConceptMap.sourceCanonical".to_string(), "1".to_string());
    map.insert("ConceptMap.sourceUri".to_string(), "1".to_string());
    map.insert("ConceptMap.status".to_string(), "1".to_string());
    map.insert("ConceptMap.targetCanonical".to_string(), "1".to_string());
    map.insert("ConceptMap.targetUri".to_string(), "1".to_string());
    map.insert("ConceptMap.text".to_string(), "1".to_string());
    map.insert("ConceptMap.title".to_string(), "1".to_string());
    map.insert("ConceptMap.url".to_string(), "1".to_string());
    map.insert("ConceptMap.useContext".to_string(), "*".to_string());
    map.insert("ConceptMap.version".to_string(), "1".to_string());
    map.insert("Condition.abatementAge".to_string(), "1".to_string());
    map.insert("Condition.abatementDateTime".to_string(), "1".to_string());
    map.insert("Condition.abatementPeriod".to_string(), "1".to_string());
    map.insert("Condition.abatementRange".to_string(), "1".to_string());
    map.insert("Condition.abatementString".to_string(), "1".to_string());
    map.insert("Condition.asserter".to_string(), "1".to_string());
    map.insert("Condition.bodySite".to_string(), "*".to_string());
    map.insert("Condition.category".to_string(), "*".to_string());
    map.insert("Condition.clinicalStatus".to_string(), "1".to_string());
    map.insert("Condition.code".to_string(), "1".to_string());
    map.insert("Condition.contained".to_string(), "*".to_string());
    map.insert("Condition.encounter".to_string(), "1".to_string());
    map.insert("Condition.evidence".to_string(), "*".to_string());
    map.insert("Condition.evidence.code".to_string(), "*".to_string());
    map.insert("Condition.evidence.detail".to_string(), "*".to_string());
    map.insert("Condition.evidence.extension".to_string(), "*".to_string());
    map.insert("Condition.evidence.id".to_string(), "1".to_string());
    map.insert("Condition.evidence.modifierExtension".to_string(), "*".to_string());
    map.insert("Condition.extension".to_string(), "*".to_string());
    map.insert("Condition.id".to_string(), "1".to_string());
    map.insert("Condition.identifier".to_string(), "*".to_string());
    map.insert("Condition.implicitRules".to_string(), "1".to_string());
    map.insert("Condition.language".to_string(), "1".to_string());
    map.insert("Condition.meta".to_string(), "1".to_string());
    map.insert("Condition.modifierExtension".to_string(), "*".to_string());
    map.insert("Condition.note".to_string(), "*".to_string());
    map.insert("Condition.onsetAge".to_string(), "1".to_string());
    map.insert("Condition.onsetDateTime".to_string(), "1".to_string());
    map.insert("Condition.onsetPeriod".to_string(), "1".to_string());
    map.insert("Condition.onsetRange".to_string(), "1".to_string());
    map.insert("Condition.onsetString".to_string(), "1".to_string());
    map.insert("Condition.recordedDate".to_string(), "1".to_string());
    map.insert("Condition.recorder".to_string(), "1".to_string());
    map.insert("Condition.severity".to_string(), "1".to_string());
    map.insert("Condition.stage".to_string(), "*".to_string());
    map.insert("Condition.stage.assessment".to_string(), "*".to_string());
    map.insert("Condition.stage.extension".to_string(), "*".to_string());
    map.insert("Condition.stage.id".to_string(), "1".to_string());
    map.insert("Condition.stage.modifierExtension".to_string(), "*".to_string());
    map.insert("Condition.stage.summary".to_string(), "1".to_string());
    map.insert("Condition.stage.type".to_string(), "1".to_string());
    map.insert("Condition.subject".to_string(), "1".to_string());
    map.insert("Condition.text".to_string(), "1".to_string());
    map.insert("Condition.verificationStatus".to_string(), "1".to_string());
    map.insert("Consent.category".to_string(), "*".to_string());
    map.insert("Consent.contained".to_string(), "*".to_string());
    map.insert("Consent.dateTime".to_string(), "1".to_string());
    map.insert("Consent.extension".to_string(), "*".to_string());
    map.insert("Consent.id".to_string(), "1".to_string());
    map.insert("Consent.identifier".to_string(), "*".to_string());
    map.insert("Consent.implicitRules".to_string(), "1".to_string());
    map.insert("Consent.language".to_string(), "1".to_string());
    map.insert("Consent.meta".to_string(), "1".to_string());
    map.insert("Consent.modifierExtension".to_string(), "*".to_string());
    map.insert("Consent.organization".to_string(), "*".to_string());
    map.insert("Consent.patient".to_string(), "1".to_string());
    map.insert("Consent.performer".to_string(), "*".to_string());
    map.insert("Consent.policy".to_string(), "*".to_string());
    map.insert("Consent.policy.authority".to_string(), "1".to_string());
    map.insert("Consent.policy.extension".to_string(), "*".to_string());
    map.insert("Consent.policy.id".to_string(), "1".to_string());
    map.insert("Consent.policy.modifierExtension".to_string(), "*".to_string());
    map.insert("Consent.policy.uri".to_string(), "1".to_string());
    map.insert("Consent.policyRule".to_string(), "1".to_string());
    map.insert("Consent.provision".to_string(), "1".to_string());
    map.insert("Consent.provision.action".to_string(), "*".to_string());
    map.insert("Consent.provision.actor".to_string(), "*".to_string());
    map.insert("Consent.provision.actor.extension".to_string(), "*".to_string());
    map.insert("Consent.provision.actor.id".to_string(), "1".to_string());
    map.insert("Consent.provision.actor.modifierExtension".to_string(), "*".to_string());
    map.insert("Consent.provision.actor.reference".to_string(), "1".to_string());
    map.insert("Consent.provision.actor.role".to_string(), "1".to_string());
    map.insert("Consent.provision.class".to_string(), "*".to_string());
    map.insert("Consent.provision.code".to_string(), "*".to_string());
    map.insert("Consent.provision.data".to_string(), "*".to_string());
    map.insert("Consent.provision.data.extension".to_string(), "*".to_string());
    map.insert("Consent.provision.data.id".to_string(), "1".to_string());
    map.insert("Consent.provision.data.meaning".to_string(), "1".to_string());
    map.insert("Consent.provision.data.modifierExtension".to_string(), "*".to_string());
    map.insert("Consent.provision.data.reference".to_string(), "1".to_string());
    map.insert("Consent.provision.dataPeriod".to_string(), "1".to_string());
    map.insert("Consent.provision.extension".to_string(), "*".to_string());
    map.insert("Consent.provision.id".to_string(), "1".to_string());
    map.insert("Consent.provision.modifierExtension".to_string(), "*".to_string());
    map.insert("Consent.provision.period".to_string(), "1".to_string());
    map.insert("Consent.provision.provision".to_string(), "*".to_string());
    map.insert("Consent.provision.purpose".to_string(), "*".to_string());
    map.insert("Consent.provision.securityLabel".to_string(), "*".to_string());
    map.insert("Consent.provision.type".to_string(), "1".to_string());
    map.insert("Consent.scope".to_string(), "1".to_string());
    map.insert("Consent.sourceAttachment".to_string(), "1".to_string());
    map.insert("Consent.sourceReference".to_string(), "1".to_string());
    map.insert("Consent.status".to_string(), "1".to_string());
    map.insert("Consent.text".to_string(), "1".to_string());
    map.insert("Consent.verification".to_string(), "*".to_string());
    map.insert("Consent.verification.extension".to_string(), "*".to_string());
    map.insert("Consent.verification.id".to_string(), "1".to_string());
    map.insert("Consent.verification.modifierExtension".to_string(), "*".to_string());
    map.insert("Consent.verification.verificationDate".to_string(), "1".to_string());
    map.insert("Consent.verification.verified".to_string(), "1".to_string());
    map.insert("Consent.verification.verifiedWith".to_string(), "1".to_string());
    map.insert("ContactDetail.extension".to_string(), "*".to_string());
    map.insert("ContactDetail.id".to_string(), "1".to_string());
    map.insert("ContactDetail.name".to_string(), "1".to_string());
    map.insert("ContactDetail.telecom".to_string(), "*".to_string());
    map.insert("ContactPoint.extension".to_string(), "*".to_string());
    map.insert("ContactPoint.id".to_string(), "1".to_string());
    map.insert("ContactPoint.period".to_string(), "1".to_string());
    map.insert("ContactPoint.rank".to_string(), "1".to_string());
    map.insert("ContactPoint.system".to_string(), "1".to_string());
    map.insert("ContactPoint.use".to_string(), "1".to_string());
    map.insert("ContactPoint.value".to_string(), "1".to_string());
    map.insert("Contract.alias".to_string(), "*".to_string());
    map.insert("Contract.applies".to_string(), "1".to_string());
    map.insert("Contract.author".to_string(), "1".to_string());
    map.insert("Contract.authority".to_string(), "*".to_string());
    map.insert("Contract.contained".to_string(), "*".to_string());
    map.insert("Contract.contentDefinition".to_string(), "1".to_string());
    map.insert("Contract.contentDefinition.copyright".to_string(), "1".to_string());
    map.insert("Contract.contentDefinition.extension".to_string(), "*".to_string());
    map.insert("Contract.contentDefinition.id".to_string(), "1".to_string());
    map.insert(
        "Contract.contentDefinition.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "Contract.contentDefinition.publicationDate".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Contract.contentDefinition.publicationStatus".to_string(),
        "1".to_string(),
    );
    map.insert("Contract.contentDefinition.publisher".to_string(), "1".to_string());
    map.insert("Contract.contentDefinition.subType".to_string(), "1".to_string());
    map.insert("Contract.contentDefinition.type".to_string(), "1".to_string());
    map.insert("Contract.contentDerivative".to_string(), "1".to_string());
    map.insert("Contract.domain".to_string(), "*".to_string());
    map.insert("Contract.expirationType".to_string(), "1".to_string());
    map.insert("Contract.extension".to_string(), "*".to_string());
    map.insert("Contract.friendly".to_string(), "*".to_string());
    map.insert("Contract.friendly.contentAttachment".to_string(), "1".to_string());
    map.insert("Contract.friendly.contentReference".to_string(), "1".to_string());
    map.insert("Contract.friendly.extension".to_string(), "*".to_string());
    map.insert("Contract.friendly.id".to_string(), "1".to_string());
    map.insert("Contract.friendly.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.id".to_string(), "1".to_string());
    map.insert("Contract.identifier".to_string(), "*".to_string());
    map.insert("Contract.implicitRules".to_string(), "1".to_string());
    map.insert("Contract.instantiatesCanonical".to_string(), "1".to_string());
    map.insert("Contract.instantiatesUri".to_string(), "1".to_string());
    map.insert("Contract.issued".to_string(), "1".to_string());
    map.insert("Contract.language".to_string(), "1".to_string());
    map.insert("Contract.legal".to_string(), "*".to_string());
    map.insert("Contract.legal.contentAttachment".to_string(), "1".to_string());
    map.insert("Contract.legal.contentReference".to_string(), "1".to_string());
    map.insert("Contract.legal.extension".to_string(), "*".to_string());
    map.insert("Contract.legal.id".to_string(), "1".to_string());
    map.insert("Contract.legal.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.legalState".to_string(), "1".to_string());
    map.insert("Contract.legallyBindingAttachment".to_string(), "1".to_string());
    map.insert("Contract.legallyBindingReference".to_string(), "1".to_string());
    map.insert("Contract.meta".to_string(), "1".to_string());
    map.insert("Contract.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.name".to_string(), "1".to_string());
    map.insert("Contract.relevantHistory".to_string(), "*".to_string());
    map.insert("Contract.rule".to_string(), "*".to_string());
    map.insert("Contract.rule.contentAttachment".to_string(), "1".to_string());
    map.insert("Contract.rule.contentReference".to_string(), "1".to_string());
    map.insert("Contract.rule.extension".to_string(), "*".to_string());
    map.insert("Contract.rule.id".to_string(), "1".to_string());
    map.insert("Contract.rule.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.scope".to_string(), "1".to_string());
    map.insert("Contract.signer".to_string(), "*".to_string());
    map.insert("Contract.signer.extension".to_string(), "*".to_string());
    map.insert("Contract.signer.id".to_string(), "1".to_string());
    map.insert("Contract.signer.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.signer.party".to_string(), "1".to_string());
    map.insert("Contract.signer.signature".to_string(), "*".to_string());
    map.insert("Contract.signer.type".to_string(), "1".to_string());
    map.insert("Contract.site".to_string(), "*".to_string());
    map.insert("Contract.status".to_string(), "1".to_string());
    map.insert("Contract.subType".to_string(), "*".to_string());
    map.insert("Contract.subject".to_string(), "*".to_string());
    map.insert("Contract.subtitle".to_string(), "1".to_string());
    map.insert("Contract.supportingInfo".to_string(), "*".to_string());
    map.insert("Contract.term".to_string(), "*".to_string());
    map.insert("Contract.term.action".to_string(), "*".to_string());
    map.insert("Contract.term.action.context".to_string(), "1".to_string());
    map.insert("Contract.term.action.contextLinkId".to_string(), "*".to_string());
    map.insert("Contract.term.action.doNotPerform".to_string(), "1".to_string());
    map.insert("Contract.term.action.extension".to_string(), "*".to_string());
    map.insert("Contract.term.action.id".to_string(), "1".to_string());
    map.insert("Contract.term.action.intent".to_string(), "1".to_string());
    map.insert("Contract.term.action.linkId".to_string(), "*".to_string());
    map.insert("Contract.term.action.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.term.action.note".to_string(), "*".to_string());
    map.insert("Contract.term.action.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("Contract.term.action.occurrencePeriod".to_string(), "1".to_string());
    map.insert("Contract.term.action.occurrenceTiming".to_string(), "1".to_string());
    map.insert("Contract.term.action.performer".to_string(), "1".to_string());
    map.insert("Contract.term.action.performerLinkId".to_string(), "*".to_string());
    map.insert("Contract.term.action.performerRole".to_string(), "1".to_string());
    map.insert("Contract.term.action.performerType".to_string(), "*".to_string());
    map.insert("Contract.term.action.reason".to_string(), "*".to_string());
    map.insert("Contract.term.action.reasonCode".to_string(), "*".to_string());
    map.insert("Contract.term.action.reasonLinkId".to_string(), "*".to_string());
    map.insert("Contract.term.action.reasonReference".to_string(), "*".to_string());
    map.insert("Contract.term.action.requester".to_string(), "*".to_string());
    map.insert("Contract.term.action.requesterLinkId".to_string(), "*".to_string());
    map.insert("Contract.term.action.securityLabelNumber".to_string(), "*".to_string());
    map.insert("Contract.term.action.status".to_string(), "1".to_string());
    map.insert("Contract.term.action.subject".to_string(), "*".to_string());
    map.insert("Contract.term.action.subject.extension".to_string(), "*".to_string());
    map.insert("Contract.term.action.subject.id".to_string(), "1".to_string());
    map.insert(
        "Contract.term.action.subject.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Contract.term.action.subject.reference".to_string(), "*".to_string());
    map.insert("Contract.term.action.subject.role".to_string(), "1".to_string());
    map.insert("Contract.term.action.type".to_string(), "1".to_string());
    map.insert("Contract.term.applies".to_string(), "1".to_string());
    map.insert("Contract.term.asset".to_string(), "*".to_string());
    map.insert("Contract.term.asset.answer".to_string(), "*".to_string());
    map.insert("Contract.term.asset.condition".to_string(), "1".to_string());
    map.insert("Contract.term.asset.context".to_string(), "*".to_string());
    map.insert("Contract.term.asset.context.code".to_string(), "*".to_string());
    map.insert("Contract.term.asset.context.extension".to_string(), "*".to_string());
    map.insert("Contract.term.asset.context.id".to_string(), "1".to_string());
    map.insert(
        "Contract.term.asset.context.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Contract.term.asset.context.reference".to_string(), "1".to_string());
    map.insert("Contract.term.asset.context.text".to_string(), "1".to_string());
    map.insert("Contract.term.asset.extension".to_string(), "*".to_string());
    map.insert("Contract.term.asset.id".to_string(), "1".to_string());
    map.insert("Contract.term.asset.linkId".to_string(), "*".to_string());
    map.insert("Contract.term.asset.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.term.asset.period".to_string(), "*".to_string());
    map.insert("Contract.term.asset.periodType".to_string(), "*".to_string());
    map.insert("Contract.term.asset.relationship".to_string(), "1".to_string());
    map.insert("Contract.term.asset.scope".to_string(), "1".to_string());
    map.insert("Contract.term.asset.securityLabelNumber".to_string(), "*".to_string());
    map.insert("Contract.term.asset.subtype".to_string(), "*".to_string());
    map.insert("Contract.term.asset.text".to_string(), "1".to_string());
    map.insert("Contract.term.asset.type".to_string(), "*".to_string());
    map.insert("Contract.term.asset.typeReference".to_string(), "*".to_string());
    map.insert("Contract.term.asset.usePeriod".to_string(), "*".to_string());
    map.insert("Contract.term.asset.valuedItem".to_string(), "*".to_string());
    map.insert(
        "Contract.term.asset.valuedItem.effectiveTime".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Contract.term.asset.valuedItem.entityCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Contract.term.asset.valuedItem.entityReference".to_string(),
        "1".to_string(),
    );
    map.insert("Contract.term.asset.valuedItem.extension".to_string(), "*".to_string());
    map.insert("Contract.term.asset.valuedItem.factor".to_string(), "1".to_string());
    map.insert("Contract.term.asset.valuedItem.id".to_string(), "1".to_string());
    map.insert("Contract.term.asset.valuedItem.identifier".to_string(), "1".to_string());
    map.insert("Contract.term.asset.valuedItem.linkId".to_string(), "*".to_string());
    map.insert(
        "Contract.term.asset.valuedItem.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Contract.term.asset.valuedItem.net".to_string(), "1".to_string());
    map.insert("Contract.term.asset.valuedItem.payment".to_string(), "1".to_string());
    map.insert(
        "Contract.term.asset.valuedItem.paymentDate".to_string(),
        "1".to_string(),
    );
    map.insert("Contract.term.asset.valuedItem.points".to_string(), "1".to_string());
    map.insert("Contract.term.asset.valuedItem.quantity".to_string(), "1".to_string());
    map.insert("Contract.term.asset.valuedItem.recipient".to_string(), "1".to_string());
    map.insert(
        "Contract.term.asset.valuedItem.responsible".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Contract.term.asset.valuedItem.securityLabelNumber".to_string(),
        "*".to_string(),
    );
    map.insert("Contract.term.asset.valuedItem.unitPrice".to_string(), "1".to_string());
    map.insert("Contract.term.extension".to_string(), "*".to_string());
    map.insert("Contract.term.group".to_string(), "*".to_string());
    map.insert("Contract.term.id".to_string(), "1".to_string());
    map.insert("Contract.term.identifier".to_string(), "1".to_string());
    map.insert("Contract.term.issued".to_string(), "1".to_string());
    map.insert("Contract.term.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.term.offer".to_string(), "1".to_string());
    map.insert("Contract.term.offer.answer".to_string(), "*".to_string());
    map.insert("Contract.term.offer.answer.extension".to_string(), "*".to_string());
    map.insert("Contract.term.offer.answer.id".to_string(), "1".to_string());
    map.insert(
        "Contract.term.offer.answer.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "Contract.term.offer.answer.valueAttachment".to_string(),
        "1".to_string(),
    );
    map.insert("Contract.term.offer.answer.valueBoolean".to_string(), "1".to_string());
    map.insert("Contract.term.offer.answer.valueCoding".to_string(), "1".to_string());
    map.insert("Contract.term.offer.answer.valueDate".to_string(), "1".to_string());
    map.insert("Contract.term.offer.answer.valueDateTime".to_string(), "1".to_string());
    map.insert("Contract.term.offer.answer.valueDecimal".to_string(), "1".to_string());
    map.insert("Contract.term.offer.answer.valueInteger".to_string(), "1".to_string());
    map.insert("Contract.term.offer.answer.valueQuantity".to_string(), "1".to_string());
    map.insert("Contract.term.offer.answer.valueReference".to_string(), "1".to_string());
    map.insert("Contract.term.offer.answer.valueString".to_string(), "1".to_string());
    map.insert("Contract.term.offer.answer.valueTime".to_string(), "1".to_string());
    map.insert("Contract.term.offer.answer.valueUri".to_string(), "1".to_string());
    map.insert("Contract.term.offer.decision".to_string(), "1".to_string());
    map.insert("Contract.term.offer.decisionMode".to_string(), "*".to_string());
    map.insert("Contract.term.offer.extension".to_string(), "*".to_string());
    map.insert("Contract.term.offer.id".to_string(), "1".to_string());
    map.insert("Contract.term.offer.identifier".to_string(), "*".to_string());
    map.insert("Contract.term.offer.linkId".to_string(), "*".to_string());
    map.insert("Contract.term.offer.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.term.offer.party".to_string(), "*".to_string());
    map.insert("Contract.term.offer.party.extension".to_string(), "*".to_string());
    map.insert("Contract.term.offer.party.id".to_string(), "1".to_string());
    map.insert(
        "Contract.term.offer.party.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Contract.term.offer.party.reference".to_string(), "*".to_string());
    map.insert("Contract.term.offer.party.role".to_string(), "1".to_string());
    map.insert("Contract.term.offer.securityLabelNumber".to_string(), "*".to_string());
    map.insert("Contract.term.offer.text".to_string(), "1".to_string());
    map.insert("Contract.term.offer.topic".to_string(), "1".to_string());
    map.insert("Contract.term.offer.type".to_string(), "1".to_string());
    map.insert("Contract.term.securityLabel".to_string(), "*".to_string());
    map.insert("Contract.term.securityLabel.category".to_string(), "*".to_string());
    map.insert(
        "Contract.term.securityLabel.classification".to_string(),
        "1".to_string(),
    );
    map.insert("Contract.term.securityLabel.control".to_string(), "*".to_string());
    map.insert("Contract.term.securityLabel.extension".to_string(), "*".to_string());
    map.insert("Contract.term.securityLabel.id".to_string(), "1".to_string());
    map.insert(
        "Contract.term.securityLabel.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Contract.term.securityLabel.number".to_string(), "*".to_string());
    map.insert("Contract.term.subType".to_string(), "1".to_string());
    map.insert("Contract.term.text".to_string(), "1".to_string());
    map.insert("Contract.term.topicCodeableConcept".to_string(), "1".to_string());
    map.insert("Contract.term.topicReference".to_string(), "1".to_string());
    map.insert("Contract.term.type".to_string(), "1".to_string());
    map.insert("Contract.text".to_string(), "1".to_string());
    map.insert("Contract.title".to_string(), "1".to_string());
    map.insert("Contract.topicCodeableConcept".to_string(), "1".to_string());
    map.insert("Contract.topicReference".to_string(), "1".to_string());
    map.insert("Contract.type".to_string(), "1".to_string());
    map.insert("Contract.url".to_string(), "1".to_string());
    map.insert("Contract.version".to_string(), "1".to_string());
    map.insert("Contributor.contact".to_string(), "*".to_string());
    map.insert("Contributor.extension".to_string(), "*".to_string());
    map.insert("Contributor.id".to_string(), "1".to_string());
    map.insert("Contributor.name".to_string(), "1".to_string());
    map.insert("Contributor.type".to_string(), "1".to_string());
    map.insert("Count.code".to_string(), "1".to_string());
    map.insert("Count.comparator".to_string(), "1".to_string());
    map.insert("Count.extension".to_string(), "*".to_string());
    map.insert("Count.id".to_string(), "1".to_string());
    map.insert("Count.system".to_string(), "1".to_string());
    map.insert("Count.unit".to_string(), "1".to_string());
    map.insert("Count.value".to_string(), "1".to_string());
    map.insert("Coverage.beneficiary".to_string(), "1".to_string());
    map.insert("Coverage.class".to_string(), "*".to_string());
    map.insert("Coverage.class.extension".to_string(), "*".to_string());
    map.insert("Coverage.class.id".to_string(), "1".to_string());
    map.insert("Coverage.class.modifierExtension".to_string(), "*".to_string());
    map.insert("Coverage.class.name".to_string(), "1".to_string());
    map.insert("Coverage.class.type".to_string(), "1".to_string());
    map.insert("Coverage.class.value".to_string(), "1".to_string());
    map.insert("Coverage.contained".to_string(), "*".to_string());
    map.insert("Coverage.contract".to_string(), "*".to_string());
    map.insert("Coverage.costToBeneficiary".to_string(), "*".to_string());
    map.insert("Coverage.costToBeneficiary.exception".to_string(), "*".to_string());
    map.insert(
        "Coverage.costToBeneficiary.exception.extension".to_string(),
        "*".to_string(),
    );
    map.insert("Coverage.costToBeneficiary.exception.id".to_string(), "1".to_string());
    map.insert(
        "Coverage.costToBeneficiary.exception.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "Coverage.costToBeneficiary.exception.period".to_string(),
        "1".to_string(),
    );
    map.insert("Coverage.costToBeneficiary.exception.type".to_string(), "1".to_string());
    map.insert("Coverage.costToBeneficiary.extension".to_string(), "*".to_string());
    map.insert("Coverage.costToBeneficiary.id".to_string(), "1".to_string());
    map.insert(
        "Coverage.costToBeneficiary.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Coverage.costToBeneficiary.type".to_string(), "1".to_string());
    map.insert("Coverage.costToBeneficiary.valueMoney".to_string(), "1".to_string());
    map.insert("Coverage.costToBeneficiary.valueQuantity".to_string(), "1".to_string());
    map.insert("Coverage.dependent".to_string(), "1".to_string());
    map.insert("Coverage.extension".to_string(), "*".to_string());
    map.insert("Coverage.id".to_string(), "1".to_string());
    map.insert("Coverage.identifier".to_string(), "*".to_string());
    map.insert("Coverage.implicitRules".to_string(), "1".to_string());
    map.insert("Coverage.language".to_string(), "1".to_string());
    map.insert("Coverage.meta".to_string(), "1".to_string());
    map.insert("Coverage.modifierExtension".to_string(), "*".to_string());
    map.insert("Coverage.network".to_string(), "1".to_string());
    map.insert("Coverage.order".to_string(), "1".to_string());
    map.insert("Coverage.payor".to_string(), "*".to_string());
    map.insert("Coverage.period".to_string(), "1".to_string());
    map.insert("Coverage.policyHolder".to_string(), "1".to_string());
    map.insert("Coverage.relationship".to_string(), "1".to_string());
    map.insert("Coverage.status".to_string(), "1".to_string());
    map.insert("Coverage.subrogation".to_string(), "1".to_string());
    map.insert("Coverage.subscriber".to_string(), "1".to_string());
    map.insert("Coverage.subscriberId".to_string(), "1".to_string());
    map.insert("Coverage.text".to_string(), "1".to_string());
    map.insert("Coverage.type".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.contained".to_string(), "*".to_string());
    map.insert("CoverageEligibilityRequest.created".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.enterer".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.extension".to_string(), "*".to_string());
    map.insert("CoverageEligibilityRequest.facility".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.id".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.identifier".to_string(), "*".to_string());
    map.insert("CoverageEligibilityRequest.implicitRules".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.insurance".to_string(), "*".to_string());
    map.insert(
        "CoverageEligibilityRequest.insurance.businessArrangement".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityRequest.insurance.coverage".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityRequest.insurance.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CoverageEligibilityRequest.insurance.focal".to_string(),
        "1".to_string(),
    );
    map.insert("CoverageEligibilityRequest.insurance.id".to_string(), "1".to_string());
    map.insert(
        "CoverageEligibilityRequest.insurance.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CoverageEligibilityRequest.insurer".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.item".to_string(), "*".to_string());
    map.insert("CoverageEligibilityRequest.item.category".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.item.detail".to_string(), "*".to_string());
    map.insert("CoverageEligibilityRequest.item.diagnosis".to_string(), "*".to_string());
    map.insert(
        "CoverageEligibilityRequest.item.diagnosis.diagnosisCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityRequest.item.diagnosis.diagnosisReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityRequest.item.diagnosis.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CoverageEligibilityRequest.item.diagnosis.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityRequest.item.diagnosis.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CoverageEligibilityRequest.item.extension".to_string(), "*".to_string());
    map.insert("CoverageEligibilityRequest.item.facility".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.item.id".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.item.modifier".to_string(), "*".to_string());
    map.insert(
        "CoverageEligibilityRequest.item.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CoverageEligibilityRequest.item.productOrService".to_string(),
        "1".to_string(),
    );
    map.insert("CoverageEligibilityRequest.item.provider".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.item.quantity".to_string(), "1".to_string());
    map.insert(
        "CoverageEligibilityRequest.item.supportingInfoSequence".to_string(),
        "*".to_string(),
    );
    map.insert("CoverageEligibilityRequest.item.unitPrice".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.language".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.meta".to_string(), "1".to_string());
    map.insert(
        "CoverageEligibilityRequest.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CoverageEligibilityRequest.patient".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.priority".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.provider".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.purpose".to_string(), "*".to_string());
    map.insert("CoverageEligibilityRequest.servicedDate".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.servicedPeriod".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.status".to_string(), "1".to_string());
    map.insert("CoverageEligibilityRequest.supportingInfo".to_string(), "*".to_string());
    map.insert(
        "CoverageEligibilityRequest.supportingInfo.appliesToAll".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityRequest.supportingInfo.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CoverageEligibilityRequest.supportingInfo.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityRequest.supportingInfo.information".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityRequest.supportingInfo.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CoverageEligibilityRequest.supportingInfo.sequence".to_string(),
        "1".to_string(),
    );
    map.insert("CoverageEligibilityRequest.text".to_string(), "1".to_string());
    map.insert("CoverageEligibilityResponse.contained".to_string(), "*".to_string());
    map.insert("CoverageEligibilityResponse.created".to_string(), "1".to_string());
    map.insert("CoverageEligibilityResponse.disposition".to_string(), "1".to_string());
    map.insert("CoverageEligibilityResponse.error".to_string(), "*".to_string());
    map.insert("CoverageEligibilityResponse.error.code".to_string(), "1".to_string());
    map.insert(
        "CoverageEligibilityResponse.error.extension".to_string(),
        "*".to_string(),
    );
    map.insert("CoverageEligibilityResponse.error.id".to_string(), "1".to_string());
    map.insert(
        "CoverageEligibilityResponse.error.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CoverageEligibilityResponse.extension".to_string(), "*".to_string());
    map.insert("CoverageEligibilityResponse.form".to_string(), "1".to_string());
    map.insert("CoverageEligibilityResponse.id".to_string(), "1".to_string());
    map.insert("CoverageEligibilityResponse.identifier".to_string(), "*".to_string());
    map.insert("CoverageEligibilityResponse.implicitRules".to_string(), "1".to_string());
    map.insert("CoverageEligibilityResponse.insurance".to_string(), "*".to_string());
    map.insert(
        "CoverageEligibilityResponse.insurance.benefitPeriod".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.coverage".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.extension".to_string(),
        "*".to_string(),
    );
    map.insert("CoverageEligibilityResponse.insurance.id".to_string(), "1".to_string());
    map.insert(
        "CoverageEligibilityResponse.insurance.inforce".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.authorizationRequired".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.authorizationSupporting".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.authorizationUrl".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.benefit".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.benefit.allowedMoney".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.benefit.allowedString".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.benefit.allowedUnsignedInt"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.benefit.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.benefit.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.benefit.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.benefit.type".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.benefit.usedMoney".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.benefit.usedString".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.benefit.usedUnsignedInt".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.category".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.excluded".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.modifier".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.name".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.network".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.productOrService".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.provider".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.term".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.item.unit".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CoverageEligibilityResponse.insurance.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CoverageEligibilityResponse.insurer".to_string(), "1".to_string());
    map.insert("CoverageEligibilityResponse.language".to_string(), "1".to_string());
    map.insert("CoverageEligibilityResponse.meta".to_string(), "1".to_string());
    map.insert(
        "CoverageEligibilityResponse.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CoverageEligibilityResponse.outcome".to_string(), "1".to_string());
    map.insert("CoverageEligibilityResponse.patient".to_string(), "1".to_string());
    map.insert("CoverageEligibilityResponse.preAuthRef".to_string(), "1".to_string());
    map.insert("CoverageEligibilityResponse.purpose".to_string(), "*".to_string());
    map.insert("CoverageEligibilityResponse.request".to_string(), "1".to_string());
    map.insert("CoverageEligibilityResponse.requestor".to_string(), "1".to_string());
    map.insert("CoverageEligibilityResponse.servicedDate".to_string(), "1".to_string());
    map.insert(
        "CoverageEligibilityResponse.servicedPeriod".to_string(),
        "1".to_string(),
    );
    map.insert("CoverageEligibilityResponse.status".to_string(), "1".to_string());
    map.insert("CoverageEligibilityResponse.text".to_string(), "1".to_string());
    map.insert("DataRequirement.codeFilter".to_string(), "*".to_string());
    map.insert("DataRequirement.codeFilter.code".to_string(), "*".to_string());
    map.insert("DataRequirement.codeFilter.extension".to_string(), "*".to_string());
    map.insert("DataRequirement.codeFilter.id".to_string(), "1".to_string());
    map.insert("DataRequirement.codeFilter.path".to_string(), "1".to_string());
    map.insert("DataRequirement.codeFilter.searchParam".to_string(), "1".to_string());
    map.insert("DataRequirement.codeFilter.valueSet".to_string(), "1".to_string());
    map.insert("DataRequirement.dateFilter".to_string(), "*".to_string());
    map.insert("DataRequirement.dateFilter.extension".to_string(), "*".to_string());
    map.insert("DataRequirement.dateFilter.id".to_string(), "1".to_string());
    map.insert("DataRequirement.dateFilter.path".to_string(), "1".to_string());
    map.insert("DataRequirement.dateFilter.searchParam".to_string(), "1".to_string());
    map.insert("DataRequirement.dateFilter.valueDateTime".to_string(), "1".to_string());
    map.insert("DataRequirement.dateFilter.valueDuration".to_string(), "1".to_string());
    map.insert("DataRequirement.dateFilter.valuePeriod".to_string(), "1".to_string());
    map.insert("DataRequirement.extension".to_string(), "*".to_string());
    map.insert("DataRequirement.id".to_string(), "1".to_string());
    map.insert("DataRequirement.limit".to_string(), "1".to_string());
    map.insert("DataRequirement.mustSupport".to_string(), "*".to_string());
    map.insert("DataRequirement.profile".to_string(), "*".to_string());
    map.insert("DataRequirement.sort".to_string(), "*".to_string());
    map.insert("DataRequirement.sort.direction".to_string(), "1".to_string());
    map.insert("DataRequirement.sort.extension".to_string(), "*".to_string());
    map.insert("DataRequirement.sort.id".to_string(), "1".to_string());
    map.insert("DataRequirement.sort.path".to_string(), "1".to_string());
    map.insert("DataRequirement.subjectCodeableConcept".to_string(), "1".to_string());
    map.insert("DataRequirement.subjectReference".to_string(), "1".to_string());
    map.insert("DataRequirement.type".to_string(), "1".to_string());
    map.insert("DetectedIssue.author".to_string(), "1".to_string());
    map.insert("DetectedIssue.code".to_string(), "1".to_string());
    map.insert("DetectedIssue.contained".to_string(), "*".to_string());
    map.insert("DetectedIssue.detail".to_string(), "1".to_string());
    map.insert("DetectedIssue.evidence".to_string(), "*".to_string());
    map.insert("DetectedIssue.evidence.code".to_string(), "*".to_string());
    map.insert("DetectedIssue.evidence.detail".to_string(), "*".to_string());
    map.insert("DetectedIssue.evidence.extension".to_string(), "*".to_string());
    map.insert("DetectedIssue.evidence.id".to_string(), "1".to_string());
    map.insert("DetectedIssue.evidence.modifierExtension".to_string(), "*".to_string());
    map.insert("DetectedIssue.extension".to_string(), "*".to_string());
    map.insert("DetectedIssue.id".to_string(), "1".to_string());
    map.insert("DetectedIssue.identifiedDateTime".to_string(), "1".to_string());
    map.insert("DetectedIssue.identifiedPeriod".to_string(), "1".to_string());
    map.insert("DetectedIssue.identifier".to_string(), "*".to_string());
    map.insert("DetectedIssue.implicated".to_string(), "*".to_string());
    map.insert("DetectedIssue.implicitRules".to_string(), "1".to_string());
    map.insert("DetectedIssue.language".to_string(), "1".to_string());
    map.insert("DetectedIssue.meta".to_string(), "1".to_string());
    map.insert("DetectedIssue.mitigation".to_string(), "*".to_string());
    map.insert("DetectedIssue.mitigation.action".to_string(), "1".to_string());
    map.insert("DetectedIssue.mitigation.author".to_string(), "1".to_string());
    map.insert("DetectedIssue.mitigation.date".to_string(), "1".to_string());
    map.insert("DetectedIssue.mitigation.extension".to_string(), "*".to_string());
    map.insert("DetectedIssue.mitigation.id".to_string(), "1".to_string());
    map.insert(
        "DetectedIssue.mitigation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("DetectedIssue.modifierExtension".to_string(), "*".to_string());
    map.insert("DetectedIssue.patient".to_string(), "1".to_string());
    map.insert("DetectedIssue.reference".to_string(), "1".to_string());
    map.insert("DetectedIssue.severity".to_string(), "1".to_string());
    map.insert("DetectedIssue.status".to_string(), "1".to_string());
    map.insert("DetectedIssue.text".to_string(), "1".to_string());
    map.insert("Device.contact".to_string(), "*".to_string());
    map.insert("Device.contained".to_string(), "*".to_string());
    map.insert("Device.definition".to_string(), "1".to_string());
    map.insert("Device.deviceName".to_string(), "*".to_string());
    map.insert("Device.deviceName.extension".to_string(), "*".to_string());
    map.insert("Device.deviceName.id".to_string(), "1".to_string());
    map.insert("Device.deviceName.modifierExtension".to_string(), "*".to_string());
    map.insert("Device.deviceName.name".to_string(), "1".to_string());
    map.insert("Device.deviceName.type".to_string(), "1".to_string());
    map.insert("Device.distinctIdentifier".to_string(), "1".to_string());
    map.insert("Device.expirationDate".to_string(), "1".to_string());
    map.insert("Device.extension".to_string(), "*".to_string());
    map.insert("Device.id".to_string(), "1".to_string());
    map.insert("Device.identifier".to_string(), "*".to_string());
    map.insert("Device.implicitRules".to_string(), "1".to_string());
    map.insert("Device.language".to_string(), "1".to_string());
    map.insert("Device.location".to_string(), "1".to_string());
    map.insert("Device.lotNumber".to_string(), "1".to_string());
    map.insert("Device.manufactureDate".to_string(), "1".to_string());
    map.insert("Device.manufacturer".to_string(), "1".to_string());
    map.insert("Device.meta".to_string(), "1".to_string());
    map.insert("Device.modelNumber".to_string(), "1".to_string());
    map.insert("Device.modifierExtension".to_string(), "*".to_string());
    map.insert("Device.note".to_string(), "*".to_string());
    map.insert("Device.owner".to_string(), "1".to_string());
    map.insert("Device.parent".to_string(), "1".to_string());
    map.insert("Device.partNumber".to_string(), "1".to_string());
    map.insert("Device.patient".to_string(), "1".to_string());
    map.insert("Device.property".to_string(), "*".to_string());
    map.insert("Device.property.extension".to_string(), "*".to_string());
    map.insert("Device.property.id".to_string(), "1".to_string());
    map.insert("Device.property.modifierExtension".to_string(), "*".to_string());
    map.insert("Device.property.type".to_string(), "1".to_string());
    map.insert("Device.property.valueCode".to_string(), "*".to_string());
    map.insert("Device.property.valueQuantity".to_string(), "*".to_string());
    map.insert("Device.safety".to_string(), "*".to_string());
    map.insert("Device.serialNumber".to_string(), "1".to_string());
    map.insert("Device.specialization".to_string(), "*".to_string());
    map.insert("Device.specialization.extension".to_string(), "*".to_string());
    map.insert("Device.specialization.id".to_string(), "1".to_string());
    map.insert("Device.specialization.modifierExtension".to_string(), "*".to_string());
    map.insert("Device.specialization.systemType".to_string(), "1".to_string());
    map.insert("Device.specialization.version".to_string(), "1".to_string());
    map.insert("Device.status".to_string(), "1".to_string());
    map.insert("Device.statusReason".to_string(), "*".to_string());
    map.insert("Device.text".to_string(), "1".to_string());
    map.insert("Device.type".to_string(), "1".to_string());
    map.insert("Device.udiCarrier".to_string(), "*".to_string());
    map.insert("Device.udiCarrier.carrierAIDC".to_string(), "1".to_string());
    map.insert("Device.udiCarrier.carrierHRF".to_string(), "1".to_string());
    map.insert("Device.udiCarrier.deviceIdentifier".to_string(), "1".to_string());
    map.insert("Device.udiCarrier.entryType".to_string(), "1".to_string());
    map.insert("Device.udiCarrier.extension".to_string(), "*".to_string());
    map.insert("Device.udiCarrier.id".to_string(), "1".to_string());
    map.insert("Device.udiCarrier.issuer".to_string(), "1".to_string());
    map.insert("Device.udiCarrier.jurisdiction".to_string(), "1".to_string());
    map.insert("Device.udiCarrier.modifierExtension".to_string(), "*".to_string());
    map.insert("Device.url".to_string(), "1".to_string());
    map.insert("Device.version".to_string(), "*".to_string());
    map.insert("Device.version.component".to_string(), "1".to_string());
    map.insert("Device.version.extension".to_string(), "*".to_string());
    map.insert("Device.version.id".to_string(), "1".to_string());
    map.insert("Device.version.modifierExtension".to_string(), "*".to_string());
    map.insert("Device.version.type".to_string(), "1".to_string());
    map.insert("Device.version.value".to_string(), "1".to_string());
    map.insert("DeviceDefinition.capability".to_string(), "*".to_string());
    map.insert("DeviceDefinition.capability.description".to_string(), "*".to_string());
    map.insert("DeviceDefinition.capability.extension".to_string(), "*".to_string());
    map.insert("DeviceDefinition.capability.id".to_string(), "1".to_string());
    map.insert(
        "DeviceDefinition.capability.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("DeviceDefinition.capability.type".to_string(), "1".to_string());
    map.insert("DeviceDefinition.contact".to_string(), "*".to_string());
    map.insert("DeviceDefinition.contained".to_string(), "*".to_string());
    map.insert("DeviceDefinition.deviceName".to_string(), "*".to_string());
    map.insert("DeviceDefinition.deviceName.extension".to_string(), "*".to_string());
    map.insert("DeviceDefinition.deviceName.id".to_string(), "1".to_string());
    map.insert(
        "DeviceDefinition.deviceName.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("DeviceDefinition.deviceName.name".to_string(), "1".to_string());
    map.insert("DeviceDefinition.deviceName.type".to_string(), "1".to_string());
    map.insert("DeviceDefinition.extension".to_string(), "*".to_string());
    map.insert("DeviceDefinition.id".to_string(), "1".to_string());
    map.insert("DeviceDefinition.identifier".to_string(), "*".to_string());
    map.insert("DeviceDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("DeviceDefinition.language".to_string(), "1".to_string());
    map.insert("DeviceDefinition.languageCode".to_string(), "*".to_string());
    map.insert("DeviceDefinition.manufacturerReference".to_string(), "1".to_string());
    map.insert("DeviceDefinition.manufacturerString".to_string(), "1".to_string());
    map.insert("DeviceDefinition.material".to_string(), "*".to_string());
    map.insert(
        "DeviceDefinition.material.allergenicIndicator".to_string(),
        "1".to_string(),
    );
    map.insert("DeviceDefinition.material.alternate".to_string(), "1".to_string());
    map.insert("DeviceDefinition.material.extension".to_string(), "*".to_string());
    map.insert("DeviceDefinition.material.id".to_string(), "1".to_string());
    map.insert(
        "DeviceDefinition.material.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("DeviceDefinition.material.substance".to_string(), "1".to_string());
    map.insert("DeviceDefinition.meta".to_string(), "1".to_string());
    map.insert("DeviceDefinition.modelNumber".to_string(), "1".to_string());
    map.insert("DeviceDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("DeviceDefinition.note".to_string(), "*".to_string());
    map.insert("DeviceDefinition.onlineInformation".to_string(), "1".to_string());
    map.insert("DeviceDefinition.owner".to_string(), "1".to_string());
    map.insert("DeviceDefinition.parentDevice".to_string(), "1".to_string());
    map.insert("DeviceDefinition.physicalCharacteristics".to_string(), "1".to_string());
    map.insert("DeviceDefinition.property".to_string(), "*".to_string());
    map.insert("DeviceDefinition.property.extension".to_string(), "*".to_string());
    map.insert("DeviceDefinition.property.id".to_string(), "1".to_string());
    map.insert(
        "DeviceDefinition.property.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("DeviceDefinition.property.type".to_string(), "1".to_string());
    map.insert("DeviceDefinition.property.valueCode".to_string(), "*".to_string());
    map.insert("DeviceDefinition.property.valueQuantity".to_string(), "*".to_string());
    map.insert("DeviceDefinition.quantity".to_string(), "1".to_string());
    map.insert("DeviceDefinition.safety".to_string(), "*".to_string());
    map.insert("DeviceDefinition.shelfLifeStorage".to_string(), "*".to_string());
    map.insert("DeviceDefinition.specialization".to_string(), "*".to_string());
    map.insert("DeviceDefinition.specialization.extension".to_string(), "*".to_string());
    map.insert("DeviceDefinition.specialization.id".to_string(), "1".to_string());
    map.insert(
        "DeviceDefinition.specialization.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "DeviceDefinition.specialization.systemType".to_string(),
        "1".to_string(),
    );
    map.insert("DeviceDefinition.specialization.version".to_string(), "1".to_string());
    map.insert("DeviceDefinition.text".to_string(), "1".to_string());
    map.insert("DeviceDefinition.type".to_string(), "1".to_string());
    map.insert("DeviceDefinition.udiDeviceIdentifier".to_string(), "*".to_string());
    map.insert(
        "DeviceDefinition.udiDeviceIdentifier.deviceIdentifier".to_string(),
        "1".to_string(),
    );
    map.insert(
        "DeviceDefinition.udiDeviceIdentifier.extension".to_string(),
        "*".to_string(),
    );
    map.insert("DeviceDefinition.udiDeviceIdentifier.id".to_string(), "1".to_string());
    map.insert(
        "DeviceDefinition.udiDeviceIdentifier.issuer".to_string(),
        "1".to_string(),
    );
    map.insert(
        "DeviceDefinition.udiDeviceIdentifier.jurisdiction".to_string(),
        "1".to_string(),
    );
    map.insert(
        "DeviceDefinition.udiDeviceIdentifier.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("DeviceDefinition.url".to_string(), "1".to_string());
    map.insert("DeviceDefinition.version".to_string(), "*".to_string());
    map.insert("DeviceMetric.calibration".to_string(), "*".to_string());
    map.insert("DeviceMetric.calibration.extension".to_string(), "*".to_string());
    map.insert("DeviceMetric.calibration.id".to_string(), "1".to_string());
    map.insert(
        "DeviceMetric.calibration.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("DeviceMetric.calibration.state".to_string(), "1".to_string());
    map.insert("DeviceMetric.calibration.time".to_string(), "1".to_string());
    map.insert("DeviceMetric.calibration.type".to_string(), "1".to_string());
    map.insert("DeviceMetric.category".to_string(), "1".to_string());
    map.insert("DeviceMetric.color".to_string(), "1".to_string());
    map.insert("DeviceMetric.contained".to_string(), "*".to_string());
    map.insert("DeviceMetric.extension".to_string(), "*".to_string());
    map.insert("DeviceMetric.id".to_string(), "1".to_string());
    map.insert("DeviceMetric.identifier".to_string(), "*".to_string());
    map.insert("DeviceMetric.implicitRules".to_string(), "1".to_string());
    map.insert("DeviceMetric.language".to_string(), "1".to_string());
    map.insert("DeviceMetric.measurementPeriod".to_string(), "1".to_string());
    map.insert("DeviceMetric.meta".to_string(), "1".to_string());
    map.insert("DeviceMetric.modifierExtension".to_string(), "*".to_string());
    map.insert("DeviceMetric.operationalStatus".to_string(), "1".to_string());
    map.insert("DeviceMetric.parent".to_string(), "1".to_string());
    map.insert("DeviceMetric.source".to_string(), "1".to_string());
    map.insert("DeviceMetric.text".to_string(), "1".to_string());
    map.insert("DeviceMetric.type".to_string(), "1".to_string());
    map.insert("DeviceMetric.unit".to_string(), "1".to_string());
    map.insert("DeviceRequest.authoredOn".to_string(), "1".to_string());
    map.insert("DeviceRequest.basedOn".to_string(), "*".to_string());
    map.insert("DeviceRequest.codeCodeableConcept".to_string(), "1".to_string());
    map.insert("DeviceRequest.codeReference".to_string(), "1".to_string());
    map.insert("DeviceRequest.contained".to_string(), "*".to_string());
    map.insert("DeviceRequest.encounter".to_string(), "1".to_string());
    map.insert("DeviceRequest.extension".to_string(), "*".to_string());
    map.insert("DeviceRequest.groupIdentifier".to_string(), "1".to_string());
    map.insert("DeviceRequest.id".to_string(), "1".to_string());
    map.insert("DeviceRequest.identifier".to_string(), "*".to_string());
    map.insert("DeviceRequest.implicitRules".to_string(), "1".to_string());
    map.insert("DeviceRequest.instantiatesCanonical".to_string(), "*".to_string());
    map.insert("DeviceRequest.instantiatesUri".to_string(), "*".to_string());
    map.insert("DeviceRequest.insurance".to_string(), "*".to_string());
    map.insert("DeviceRequest.intent".to_string(), "1".to_string());
    map.insert("DeviceRequest.language".to_string(), "1".to_string());
    map.insert("DeviceRequest.meta".to_string(), "1".to_string());
    map.insert("DeviceRequest.modifierExtension".to_string(), "*".to_string());
    map.insert("DeviceRequest.note".to_string(), "*".to_string());
    map.insert("DeviceRequest.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("DeviceRequest.occurrencePeriod".to_string(), "1".to_string());
    map.insert("DeviceRequest.occurrenceTiming".to_string(), "1".to_string());
    map.insert("DeviceRequest.parameter".to_string(), "*".to_string());
    map.insert("DeviceRequest.parameter.code".to_string(), "1".to_string());
    map.insert("DeviceRequest.parameter.extension".to_string(), "*".to_string());
    map.insert("DeviceRequest.parameter.id".to_string(), "1".to_string());
    map.insert("DeviceRequest.parameter.modifierExtension".to_string(), "*".to_string());
    map.insert("DeviceRequest.parameter.valueBoolean".to_string(), "1".to_string());
    map.insert(
        "DeviceRequest.parameter.valueCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("DeviceRequest.parameter.valueQuantity".to_string(), "1".to_string());
    map.insert("DeviceRequest.parameter.valueRange".to_string(), "1".to_string());
    map.insert("DeviceRequest.performer".to_string(), "1".to_string());
    map.insert("DeviceRequest.performerType".to_string(), "1".to_string());
    map.insert("DeviceRequest.priorRequest".to_string(), "*".to_string());
    map.insert("DeviceRequest.priority".to_string(), "1".to_string());
    map.insert("DeviceRequest.reasonCode".to_string(), "*".to_string());
    map.insert("DeviceRequest.reasonReference".to_string(), "*".to_string());
    map.insert("DeviceRequest.relevantHistory".to_string(), "*".to_string());
    map.insert("DeviceRequest.requester".to_string(), "1".to_string());
    map.insert("DeviceRequest.status".to_string(), "1".to_string());
    map.insert("DeviceRequest.subject".to_string(), "1".to_string());
    map.insert("DeviceRequest.supportingInfo".to_string(), "*".to_string());
    map.insert("DeviceRequest.text".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.basedOn".to_string(), "*".to_string());
    map.insert("DeviceUseStatement.bodySite".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.contained".to_string(), "*".to_string());
    map.insert("DeviceUseStatement.derivedFrom".to_string(), "*".to_string());
    map.insert("DeviceUseStatement.device".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.extension".to_string(), "*".to_string());
    map.insert("DeviceUseStatement.id".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.identifier".to_string(), "*".to_string());
    map.insert("DeviceUseStatement.implicitRules".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.language".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.meta".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.modifierExtension".to_string(), "*".to_string());
    map.insert("DeviceUseStatement.note".to_string(), "*".to_string());
    map.insert("DeviceUseStatement.reasonCode".to_string(), "*".to_string());
    map.insert("DeviceUseStatement.reasonReference".to_string(), "*".to_string());
    map.insert("DeviceUseStatement.recordedOn".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.source".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.status".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.subject".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.text".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.timingDateTime".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.timingPeriod".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.timingTiming".to_string(), "1".to_string());
    map.insert("DiagnosticReport.basedOn".to_string(), "*".to_string());
    map.insert("DiagnosticReport.category".to_string(), "*".to_string());
    map.insert("DiagnosticReport.code".to_string(), "1".to_string());
    map.insert("DiagnosticReport.conclusion".to_string(), "1".to_string());
    map.insert("DiagnosticReport.conclusionCode".to_string(), "*".to_string());
    map.insert("DiagnosticReport.contained".to_string(), "*".to_string());
    map.insert("DiagnosticReport.effectiveDateTime".to_string(), "1".to_string());
    map.insert("DiagnosticReport.effectivePeriod".to_string(), "1".to_string());
    map.insert("DiagnosticReport.encounter".to_string(), "1".to_string());
    map.insert("DiagnosticReport.extension".to_string(), "*".to_string());
    map.insert("DiagnosticReport.id".to_string(), "1".to_string());
    map.insert("DiagnosticReport.identifier".to_string(), "*".to_string());
    map.insert("DiagnosticReport.imagingStudy".to_string(), "*".to_string());
    map.insert("DiagnosticReport.implicitRules".to_string(), "1".to_string());
    map.insert("DiagnosticReport.issued".to_string(), "1".to_string());
    map.insert("DiagnosticReport.language".to_string(), "1".to_string());
    map.insert("DiagnosticReport.media".to_string(), "*".to_string());
    map.insert("DiagnosticReport.media.comment".to_string(), "1".to_string());
    map.insert("DiagnosticReport.media.extension".to_string(), "*".to_string());
    map.insert("DiagnosticReport.media.id".to_string(), "1".to_string());
    map.insert("DiagnosticReport.media.link".to_string(), "1".to_string());
    map.insert("DiagnosticReport.media.modifierExtension".to_string(), "*".to_string());
    map.insert("DiagnosticReport.meta".to_string(), "1".to_string());
    map.insert("DiagnosticReport.modifierExtension".to_string(), "*".to_string());
    map.insert("DiagnosticReport.performer".to_string(), "*".to_string());
    map.insert("DiagnosticReport.presentedForm".to_string(), "*".to_string());
    map.insert("DiagnosticReport.result".to_string(), "*".to_string());
    map.insert("DiagnosticReport.resultsInterpreter".to_string(), "*".to_string());
    map.insert("DiagnosticReport.specimen".to_string(), "*".to_string());
    map.insert("DiagnosticReport.status".to_string(), "1".to_string());
    map.insert("DiagnosticReport.subject".to_string(), "1".to_string());
    map.insert("DiagnosticReport.text".to_string(), "1".to_string());
    map.insert("Distance.code".to_string(), "1".to_string());
    map.insert("Distance.comparator".to_string(), "1".to_string());
    map.insert("Distance.extension".to_string(), "*".to_string());
    map.insert("Distance.id".to_string(), "1".to_string());
    map.insert("Distance.system".to_string(), "1".to_string());
    map.insert("Distance.unit".to_string(), "1".to_string());
    map.insert("Distance.value".to_string(), "1".to_string());
    map.insert("DocumentManifest.author".to_string(), "*".to_string());
    map.insert("DocumentManifest.contained".to_string(), "*".to_string());
    map.insert("DocumentManifest.content".to_string(), "*".to_string());
    map.insert("DocumentManifest.created".to_string(), "1".to_string());
    map.insert("DocumentManifest.description".to_string(), "1".to_string());
    map.insert("DocumentManifest.extension".to_string(), "*".to_string());
    map.insert("DocumentManifest.id".to_string(), "1".to_string());
    map.insert("DocumentManifest.identifier".to_string(), "*".to_string());
    map.insert("DocumentManifest.implicitRules".to_string(), "1".to_string());
    map.insert("DocumentManifest.language".to_string(), "1".to_string());
    map.insert("DocumentManifest.masterIdentifier".to_string(), "1".to_string());
    map.insert("DocumentManifest.meta".to_string(), "1".to_string());
    map.insert("DocumentManifest.modifierExtension".to_string(), "*".to_string());
    map.insert("DocumentManifest.recipient".to_string(), "*".to_string());
    map.insert("DocumentManifest.related".to_string(), "*".to_string());
    map.insert("DocumentManifest.related.extension".to_string(), "*".to_string());
    map.insert("DocumentManifest.related.id".to_string(), "1".to_string());
    map.insert("DocumentManifest.related.identifier".to_string(), "1".to_string());
    map.insert(
        "DocumentManifest.related.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("DocumentManifest.related.ref".to_string(), "1".to_string());
    map.insert("DocumentManifest.source".to_string(), "1".to_string());
    map.insert("DocumentManifest.status".to_string(), "1".to_string());
    map.insert("DocumentManifest.subject".to_string(), "1".to_string());
    map.insert("DocumentManifest.text".to_string(), "1".to_string());
    map.insert("DocumentManifest.type".to_string(), "1".to_string());
    map.insert("DocumentReference.authenticator".to_string(), "1".to_string());
    map.insert("DocumentReference.author".to_string(), "*".to_string());
    map.insert("DocumentReference.category".to_string(), "*".to_string());
    map.insert("DocumentReference.contained".to_string(), "*".to_string());
    map.insert("DocumentReference.content".to_string(), "*".to_string());
    map.insert("DocumentReference.content.attachment".to_string(), "1".to_string());
    map.insert("DocumentReference.content.extension".to_string(), "*".to_string());
    map.insert("DocumentReference.content.format".to_string(), "1".to_string());
    map.insert("DocumentReference.content.id".to_string(), "1".to_string());
    map.insert(
        "DocumentReference.content.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("DocumentReference.context".to_string(), "1".to_string());
    map.insert("DocumentReference.context.encounter".to_string(), "*".to_string());
    map.insert("DocumentReference.context.event".to_string(), "*".to_string());
    map.insert("DocumentReference.context.extension".to_string(), "*".to_string());
    map.insert("DocumentReference.context.facilityType".to_string(), "1".to_string());
    map.insert("DocumentReference.context.id".to_string(), "1".to_string());
    map.insert(
        "DocumentReference.context.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("DocumentReference.context.period".to_string(), "1".to_string());
    map.insert("DocumentReference.context.practiceSetting".to_string(), "1".to_string());
    map.insert("DocumentReference.context.related".to_string(), "*".to_string());
    map.insert(
        "DocumentReference.context.sourcePatientInfo".to_string(),
        "1".to_string(),
    );
    map.insert("DocumentReference.custodian".to_string(), "1".to_string());
    map.insert("DocumentReference.date".to_string(), "1".to_string());
    map.insert("DocumentReference.description".to_string(), "1".to_string());
    map.insert("DocumentReference.docStatus".to_string(), "1".to_string());
    map.insert("DocumentReference.extension".to_string(), "*".to_string());
    map.insert("DocumentReference.id".to_string(), "1".to_string());
    map.insert("DocumentReference.identifier".to_string(), "*".to_string());
    map.insert("DocumentReference.implicitRules".to_string(), "1".to_string());
    map.insert("DocumentReference.language".to_string(), "1".to_string());
    map.insert("DocumentReference.masterIdentifier".to_string(), "1".to_string());
    map.insert("DocumentReference.meta".to_string(), "1".to_string());
    map.insert("DocumentReference.modifierExtension".to_string(), "*".to_string());
    map.insert("DocumentReference.relatesTo".to_string(), "*".to_string());
    map.insert("DocumentReference.relatesTo.code".to_string(), "1".to_string());
    map.insert("DocumentReference.relatesTo.extension".to_string(), "*".to_string());
    map.insert("DocumentReference.relatesTo.id".to_string(), "1".to_string());
    map.insert(
        "DocumentReference.relatesTo.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("DocumentReference.relatesTo.target".to_string(), "1".to_string());
    map.insert("DocumentReference.securityLabel".to_string(), "*".to_string());
    map.insert("DocumentReference.status".to_string(), "1".to_string());
    map.insert("DocumentReference.subject".to_string(), "1".to_string());
    map.insert("DocumentReference.text".to_string(), "1".to_string());
    map.insert("DocumentReference.type".to_string(), "1".to_string());
    map.insert("DomainResource.contained".to_string(), "*".to_string());
    map.insert("DomainResource.extension".to_string(), "*".to_string());
    map.insert("DomainResource.id".to_string(), "1".to_string());
    map.insert("DomainResource.implicitRules".to_string(), "1".to_string());
    map.insert("DomainResource.language".to_string(), "1".to_string());
    map.insert("DomainResource.meta".to_string(), "1".to_string());
    map.insert("DomainResource.modifierExtension".to_string(), "*".to_string());
    map.insert("DomainResource.text".to_string(), "1".to_string());
    map.insert("Dosage.additionalInstruction".to_string(), "*".to_string());
    map.insert("Dosage.asNeededBoolean".to_string(), "1".to_string());
    map.insert("Dosage.asNeededCodeableConcept".to_string(), "1".to_string());
    map.insert("Dosage.doseAndRate".to_string(), "*".to_string());
    map.insert("Dosage.doseAndRate.doseQuantity".to_string(), "1".to_string());
    map.insert("Dosage.doseAndRate.doseRange".to_string(), "1".to_string());
    map.insert("Dosage.doseAndRate.extension".to_string(), "*".to_string());
    map.insert("Dosage.doseAndRate.id".to_string(), "1".to_string());
    map.insert("Dosage.doseAndRate.rateQuantity".to_string(), "1".to_string());
    map.insert("Dosage.doseAndRate.rateRange".to_string(), "1".to_string());
    map.insert("Dosage.doseAndRate.rateRatio".to_string(), "1".to_string());
    map.insert("Dosage.doseAndRate.type".to_string(), "1".to_string());
    map.insert("Dosage.extension".to_string(), "*".to_string());
    map.insert("Dosage.id".to_string(), "1".to_string());
    map.insert("Dosage.maxDosePerAdministration".to_string(), "1".to_string());
    map.insert("Dosage.maxDosePerLifetime".to_string(), "1".to_string());
    map.insert("Dosage.maxDosePerPeriod".to_string(), "1".to_string());
    map.insert("Dosage.method".to_string(), "1".to_string());
    map.insert("Dosage.modifierExtension".to_string(), "*".to_string());
    map.insert("Dosage.patientInstruction".to_string(), "1".to_string());
    map.insert("Dosage.route".to_string(), "1".to_string());
    map.insert("Dosage.sequence".to_string(), "1".to_string());
    map.insert("Dosage.site".to_string(), "1".to_string());
    map.insert("Dosage.text".to_string(), "1".to_string());
    map.insert("Dosage.timing".to_string(), "1".to_string());
    map.insert("Duration.code".to_string(), "1".to_string());
    map.insert("Duration.comparator".to_string(), "1".to_string());
    map.insert("Duration.extension".to_string(), "*".to_string());
    map.insert("Duration.id".to_string(), "1".to_string());
    map.insert("Duration.system".to_string(), "1".to_string());
    map.insert("Duration.unit".to_string(), "1".to_string());
    map.insert("Duration.value".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.approvalDate".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.author".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.certainty".to_string(), "*".to_string());
    map.insert(
        "EffectEvidenceSynthesis.certainty.certaintySubcomponent".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.certainty.certaintySubcomponent.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.certainty.certaintySubcomponent.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.certainty.certaintySubcomponent.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.certainty.certaintySubcomponent.note".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.certainty.certaintySubcomponent.rating".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.certainty.certaintySubcomponent.type".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.certainty.extension".to_string(),
        "*".to_string(),
    );
    map.insert("EffectEvidenceSynthesis.certainty.id".to_string(), "1".to_string());
    map.insert(
        "EffectEvidenceSynthesis.certainty.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("EffectEvidenceSynthesis.certainty.note".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.certainty.rating".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.contact".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.contained".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.copyright".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.date".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.description".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.editor".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.effectEstimate".to_string(), "*".to_string());
    map.insert(
        "EffectEvidenceSynthesis.effectEstimate.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.effectEstimate.extension".to_string(),
        "*".to_string(),
    );
    map.insert("EffectEvidenceSynthesis.effectEstimate.id".to_string(), "1".to_string());
    map.insert(
        "EffectEvidenceSynthesis.effectEstimate.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.effectEstimate.precisionEstimate".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.effectEstimate.precisionEstimate.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.effectEstimate.precisionEstimate.from".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.effectEstimate.precisionEstimate.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.effectEstimate.precisionEstimate.level".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.effectEstimate.precisionEstimate.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.effectEstimate.precisionEstimate.to".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.effectEstimate.precisionEstimate.type".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.effectEstimate.type".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.effectEstimate.unitOfMeasure".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.effectEstimate.value".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.effectEstimate.variantState".to_string(),
        "1".to_string(),
    );
    map.insert("EffectEvidenceSynthesis.effectivePeriod".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.endorser".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.exposure".to_string(), "1".to_string());
    map.insert(
        "EffectEvidenceSynthesis.exposureAlternative".to_string(),
        "1".to_string(),
    );
    map.insert("EffectEvidenceSynthesis.extension".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.id".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.identifier".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.implicitRules".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.jurisdiction".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.language".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.lastReviewDate".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.meta".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.modifierExtension".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.name".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.note".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.outcome".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.population".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.publisher".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.relatedArtifact".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.resultsByExposure".to_string(), "*".to_string());
    map.insert(
        "EffectEvidenceSynthesis.resultsByExposure.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.resultsByExposure.exposureState".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.resultsByExposure.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.resultsByExposure.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.resultsByExposure.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.resultsByExposure.riskEvidenceSynthesis".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.resultsByExposure.variantState".to_string(),
        "1".to_string(),
    );
    map.insert("EffectEvidenceSynthesis.reviewer".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.sampleSize".to_string(), "1".to_string());
    map.insert(
        "EffectEvidenceSynthesis.sampleSize.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.sampleSize.extension".to_string(),
        "*".to_string(),
    );
    map.insert("EffectEvidenceSynthesis.sampleSize.id".to_string(), "1".to_string());
    map.insert(
        "EffectEvidenceSynthesis.sampleSize.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.sampleSize.numberOfParticipants".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EffectEvidenceSynthesis.sampleSize.numberOfStudies".to_string(),
        "1".to_string(),
    );
    map.insert("EffectEvidenceSynthesis.status".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.studyType".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.synthesisType".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.text".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.title".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.topic".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.url".to_string(), "1".to_string());
    map.insert("EffectEvidenceSynthesis.useContext".to_string(), "*".to_string());
    map.insert("EffectEvidenceSynthesis.version".to_string(), "1".to_string());
    map.insert("ElementDefinition.alias".to_string(), "*".to_string());
    map.insert("ElementDefinition.base".to_string(), "1".to_string());
    map.insert("ElementDefinition.base.extension".to_string(), "*".to_string());
    map.insert("ElementDefinition.base.id".to_string(), "1".to_string());
    map.insert("ElementDefinition.base.max".to_string(), "1".to_string());
    map.insert("ElementDefinition.base.min".to_string(), "1".to_string());
    map.insert("ElementDefinition.base.path".to_string(), "1".to_string());
    map.insert("ElementDefinition.binding".to_string(), "1".to_string());
    map.insert("ElementDefinition.binding.description".to_string(), "1".to_string());
    map.insert("ElementDefinition.binding.extension".to_string(), "*".to_string());
    map.insert("ElementDefinition.binding.id".to_string(), "1".to_string());
    map.insert("ElementDefinition.binding.strength".to_string(), "1".to_string());
    map.insert("ElementDefinition.binding.valueSet".to_string(), "1".to_string());
    map.insert("ElementDefinition.code".to_string(), "*".to_string());
    map.insert("ElementDefinition.comment".to_string(), "1".to_string());
    map.insert("ElementDefinition.condition".to_string(), "*".to_string());
    map.insert("ElementDefinition.constraint".to_string(), "*".to_string());
    map.insert("ElementDefinition.constraint.expression".to_string(), "1".to_string());
    map.insert("ElementDefinition.constraint.extension".to_string(), "*".to_string());
    map.insert("ElementDefinition.constraint.human".to_string(), "1".to_string());
    map.insert("ElementDefinition.constraint.id".to_string(), "1".to_string());
    map.insert("ElementDefinition.constraint.key".to_string(), "1".to_string());
    map.insert("ElementDefinition.constraint.requirements".to_string(), "1".to_string());
    map.insert("ElementDefinition.constraint.severity".to_string(), "1".to_string());
    map.insert("ElementDefinition.constraint.source".to_string(), "1".to_string());
    map.insert("ElementDefinition.constraint.xpath".to_string(), "1".to_string());
    map.insert("ElementDefinition.contentReference".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueAddress".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueAge".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueAnnotation".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueAttachment".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.defaultValueBase64Binary".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.defaultValueBoolean".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueCanonical".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueCode".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.defaultValueCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.defaultValueCoding".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.defaultValueContactDetail".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ElementDefinition.defaultValueContactPoint".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.defaultValueContributor".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueCount".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.defaultValueDataRequirement".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.defaultValueDate".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueDateTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueDecimal".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueDistance".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueDosage".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueDuration".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueExpression".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueHumanName".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueId".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueIdentifier".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueInstant".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueInteger".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueMarkdown".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueMeta".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueMoney".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueOid".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.defaultValueParameterDefinition".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.defaultValuePeriod".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValuePositiveInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueQuantity".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueRange".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueRatio".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueReference".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.defaultValueRelatedArtifact".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.defaultValueSampledData".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueSignature".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueString".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueTiming".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.defaultValueTriggerDefinition".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.defaultValueUnsignedInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueUri".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueUrl".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.defaultValueUsageContext".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.defaultValueUuid".to_string(), "1".to_string());
    map.insert("ElementDefinition.definition".to_string(), "1".to_string());
    map.insert("ElementDefinition.example".to_string(), "*".to_string());
    map.insert("ElementDefinition.example.extension".to_string(), "*".to_string());
    map.insert("ElementDefinition.example.id".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.label".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueAddress".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueAge".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueAnnotation".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueAttachment".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.example.valueBase64Binary".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.example.valueBoolean".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueCanonical".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueCode".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.example.valueCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.example.valueCoding".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.example.valueContactDetail".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ElementDefinition.example.valueContactPoint".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ElementDefinition.example.valueContributor".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.example.valueCount".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.example.valueDataRequirement".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.example.valueDate".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueDateTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueDecimal".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueDistance".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueDosage".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueDuration".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueExpression".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueHumanName".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueId".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueIdentifier".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueInstant".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueInteger".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueMarkdown".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueMeta".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueMoney".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueOid".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.example.valueParameterDefinition".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.example.valuePeriod".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.example.valuePositiveInt".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.example.valueQuantity".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueRange".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueRatio".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueReference".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.example.valueRelatedArtifact".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ElementDefinition.example.valueSampledData".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.example.valueSignature".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueString".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueTiming".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.example.valueTriggerDefinition".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ElementDefinition.example.valueUnsignedInt".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.example.valueUri".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueUrl".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.example.valueUsageContext".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.example.valueUuid".to_string(), "1".to_string());
    map.insert("ElementDefinition.extension".to_string(), "*".to_string());
    map.insert("ElementDefinition.fixedAddress".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedAge".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedAnnotation".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedAttachment".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedBase64Binary".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedBoolean".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedCanonical".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedCode".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedCodeableConcept".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedCoding".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedContactDetail".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedContactPoint".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedContributor".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedCount".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedDataRequirement".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedDate".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedDateTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedDecimal".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedDistance".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedDosage".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedDuration".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedExpression".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedHumanName".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedId".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedIdentifier".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedInstant".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedInteger".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedMarkdown".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedMeta".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedMoney".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedOid".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.fixedParameterDefinition".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.fixedPeriod".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedPositiveInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedQuantity".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedRange".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedRatio".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedReference".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedRelatedArtifact".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedSampledData".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedSignature".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedString".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedTiming".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedTriggerDefinition".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedUnsignedInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedUri".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedUrl".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedUsageContext".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedUuid".to_string(), "1".to_string());
    map.insert("ElementDefinition.id".to_string(), "1".to_string());
    map.insert("ElementDefinition.isModifier".to_string(), "1".to_string());
    map.insert("ElementDefinition.isModifierReason".to_string(), "1".to_string());
    map.insert("ElementDefinition.isSummary".to_string(), "1".to_string());
    map.insert("ElementDefinition.label".to_string(), "1".to_string());
    map.insert("ElementDefinition.mapping".to_string(), "*".to_string());
    map.insert("ElementDefinition.mapping.comment".to_string(), "1".to_string());
    map.insert("ElementDefinition.mapping.extension".to_string(), "*".to_string());
    map.insert("ElementDefinition.mapping.id".to_string(), "1".to_string());
    map.insert("ElementDefinition.mapping.identity".to_string(), "1".to_string());
    map.insert("ElementDefinition.mapping.language".to_string(), "1".to_string());
    map.insert("ElementDefinition.mapping.map".to_string(), "1".to_string());
    map.insert("ElementDefinition.max".to_string(), "1".to_string());
    map.insert("ElementDefinition.maxLength".to_string(), "1".to_string());
    map.insert("ElementDefinition.maxValueDate".to_string(), "1".to_string());
    map.insert("ElementDefinition.maxValueDateTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.maxValueDecimal".to_string(), "1".to_string());
    map.insert("ElementDefinition.maxValueInstant".to_string(), "1".to_string());
    map.insert("ElementDefinition.maxValueInteger".to_string(), "1".to_string());
    map.insert("ElementDefinition.maxValuePositiveInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.maxValueQuantity".to_string(), "1".to_string());
    map.insert("ElementDefinition.maxValueTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.maxValueUnsignedInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.meaningWhenMissing".to_string(), "1".to_string());
    map.insert("ElementDefinition.min".to_string(), "1".to_string());
    map.insert("ElementDefinition.minValueDate".to_string(), "1".to_string());
    map.insert("ElementDefinition.minValueDateTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.minValueDecimal".to_string(), "1".to_string());
    map.insert("ElementDefinition.minValueInstant".to_string(), "1".to_string());
    map.insert("ElementDefinition.minValueInteger".to_string(), "1".to_string());
    map.insert("ElementDefinition.minValuePositiveInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.minValueQuantity".to_string(), "1".to_string());
    map.insert("ElementDefinition.minValueTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.minValueUnsignedInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("ElementDefinition.mustSupport".to_string(), "1".to_string());
    map.insert("ElementDefinition.orderMeaning".to_string(), "1".to_string());
    map.insert("ElementDefinition.path".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternAddress".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternAge".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternAnnotation".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternAttachment".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternBase64Binary".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternBoolean".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternCanonical".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternCode".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternCodeableConcept".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternCoding".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternContactDetail".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternContactPoint".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternContributor".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternCount".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternDataRequirement".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternDate".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternDateTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternDecimal".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternDistance".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternDosage".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternDuration".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternExpression".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternHumanName".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternId".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternIdentifier".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternInstant".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternInteger".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternMarkdown".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternMeta".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternMoney".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternOid".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.patternParameterDefinition".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.patternPeriod".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternPositiveInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternQuantity".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternRange".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternRatio".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternReference".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternRelatedArtifact".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternSampledData".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternSignature".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternString".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternTiming".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.patternTriggerDefinition".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.patternUnsignedInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternUri".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternUrl".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternUsageContext".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternUuid".to_string(), "1".to_string());
    map.insert("ElementDefinition.representation".to_string(), "*".to_string());
    map.insert("ElementDefinition.requirements".to_string(), "1".to_string());
    map.insert("ElementDefinition.short".to_string(), "1".to_string());
    map.insert("ElementDefinition.sliceIsConstraining".to_string(), "1".to_string());
    map.insert("ElementDefinition.sliceName".to_string(), "1".to_string());
    map.insert("ElementDefinition.slicing".to_string(), "1".to_string());
    map.insert("ElementDefinition.slicing.description".to_string(), "1".to_string());
    map.insert("ElementDefinition.slicing.discriminator".to_string(), "*".to_string());
    map.insert(
        "ElementDefinition.slicing.discriminator.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ElementDefinition.slicing.discriminator.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ElementDefinition.slicing.discriminator.path".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ElementDefinition.slicing.discriminator.type".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.slicing.extension".to_string(), "*".to_string());
    map.insert("ElementDefinition.slicing.id".to_string(), "1".to_string());
    map.insert("ElementDefinition.slicing.ordered".to_string(), "1".to_string());
    map.insert("ElementDefinition.slicing.rules".to_string(), "1".to_string());
    map.insert("ElementDefinition.type".to_string(), "*".to_string());
    map.insert("ElementDefinition.type.aggregation".to_string(), "*".to_string());
    map.insert("ElementDefinition.type.code".to_string(), "1".to_string());
    map.insert("ElementDefinition.type.extension".to_string(), "*".to_string());
    map.insert("ElementDefinition.type.id".to_string(), "1".to_string());
    map.insert("ElementDefinition.type.profile".to_string(), "*".to_string());
    map.insert("ElementDefinition.type.targetProfile".to_string(), "*".to_string());
    map.insert("ElementDefinition.type.versioning".to_string(), "1".to_string());
    map.insert("Encounter.account".to_string(), "*".to_string());
    map.insert("Encounter.appointment".to_string(), "*".to_string());
    map.insert("Encounter.basedOn".to_string(), "*".to_string());
    map.insert("Encounter.class".to_string(), "1".to_string());
    map.insert("Encounter.classHistory".to_string(), "*".to_string());
    map.insert("Encounter.classHistory.class".to_string(), "1".to_string());
    map.insert("Encounter.classHistory.extension".to_string(), "*".to_string());
    map.insert("Encounter.classHistory.id".to_string(), "1".to_string());
    map.insert("Encounter.classHistory.modifierExtension".to_string(), "*".to_string());
    map.insert("Encounter.classHistory.period".to_string(), "1".to_string());
    map.insert("Encounter.contained".to_string(), "*".to_string());
    map.insert("Encounter.diagnosis".to_string(), "*".to_string());
    map.insert("Encounter.diagnosis.condition".to_string(), "1".to_string());
    map.insert("Encounter.diagnosis.extension".to_string(), "*".to_string());
    map.insert("Encounter.diagnosis.id".to_string(), "1".to_string());
    map.insert("Encounter.diagnosis.modifierExtension".to_string(), "*".to_string());
    map.insert("Encounter.diagnosis.rank".to_string(), "1".to_string());
    map.insert("Encounter.diagnosis.use".to_string(), "1".to_string());
    map.insert("Encounter.episodeOfCare".to_string(), "*".to_string());
    map.insert("Encounter.extension".to_string(), "*".to_string());
    map.insert("Encounter.hospitalization".to_string(), "1".to_string());
    map.insert("Encounter.hospitalization.admitSource".to_string(), "1".to_string());
    map.insert("Encounter.hospitalization.destination".to_string(), "1".to_string());
    map.insert("Encounter.hospitalization.dietPreference".to_string(), "*".to_string());
    map.insert(
        "Encounter.hospitalization.dischargeDisposition".to_string(),
        "1".to_string(),
    );
    map.insert("Encounter.hospitalization.extension".to_string(), "*".to_string());
    map.insert("Encounter.hospitalization.id".to_string(), "1".to_string());
    map.insert(
        "Encounter.hospitalization.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Encounter.hospitalization.origin".to_string(), "1".to_string());
    map.insert(
        "Encounter.hospitalization.preAdmissionIdentifier".to_string(),
        "1".to_string(),
    );
    map.insert("Encounter.hospitalization.reAdmission".to_string(), "1".to_string());
    map.insert(
        "Encounter.hospitalization.specialArrangement".to_string(),
        "*".to_string(),
    );
    map.insert("Encounter.hospitalization.specialCourtesy".to_string(), "*".to_string());
    map.insert("Encounter.id".to_string(), "1".to_string());
    map.insert("Encounter.identifier".to_string(), "*".to_string());
    map.insert("Encounter.implicitRules".to_string(), "1".to_string());
    map.insert("Encounter.language".to_string(), "1".to_string());
    map.insert("Encounter.length".to_string(), "1".to_string());
    map.insert("Encounter.location".to_string(), "*".to_string());
    map.insert("Encounter.location.extension".to_string(), "*".to_string());
    map.insert("Encounter.location.id".to_string(), "1".to_string());
    map.insert("Encounter.location.location".to_string(), "1".to_string());
    map.insert("Encounter.location.modifierExtension".to_string(), "*".to_string());
    map.insert("Encounter.location.period".to_string(), "1".to_string());
    map.insert("Encounter.location.physicalType".to_string(), "1".to_string());
    map.insert("Encounter.location.status".to_string(), "1".to_string());
    map.insert("Encounter.meta".to_string(), "1".to_string());
    map.insert("Encounter.modifierExtension".to_string(), "*".to_string());
    map.insert("Encounter.partOf".to_string(), "1".to_string());
    map.insert("Encounter.participant".to_string(), "*".to_string());
    map.insert("Encounter.participant.extension".to_string(), "*".to_string());
    map.insert("Encounter.participant.id".to_string(), "1".to_string());
    map.insert("Encounter.participant.individual".to_string(), "1".to_string());
    map.insert("Encounter.participant.modifierExtension".to_string(), "*".to_string());
    map.insert("Encounter.participant.period".to_string(), "1".to_string());
    map.insert("Encounter.participant.type".to_string(), "*".to_string());
    map.insert("Encounter.period".to_string(), "1".to_string());
    map.insert("Encounter.priority".to_string(), "1".to_string());
    map.insert("Encounter.reasonCode".to_string(), "*".to_string());
    map.insert("Encounter.reasonReference".to_string(), "*".to_string());
    map.insert("Encounter.serviceProvider".to_string(), "1".to_string());
    map.insert("Encounter.serviceType".to_string(), "1".to_string());
    map.insert("Encounter.status".to_string(), "1".to_string());
    map.insert("Encounter.statusHistory".to_string(), "*".to_string());
    map.insert("Encounter.statusHistory.extension".to_string(), "*".to_string());
    map.insert("Encounter.statusHistory.id".to_string(), "1".to_string());
    map.insert("Encounter.statusHistory.modifierExtension".to_string(), "*".to_string());
    map.insert("Encounter.statusHistory.period".to_string(), "1".to_string());
    map.insert("Encounter.statusHistory.status".to_string(), "1".to_string());
    map.insert("Encounter.subject".to_string(), "1".to_string());
    map.insert("Encounter.text".to_string(), "1".to_string());
    map.insert("Encounter.type".to_string(), "*".to_string());
    map.insert("Endpoint.address".to_string(), "1".to_string());
    map.insert("Endpoint.connectionType".to_string(), "1".to_string());
    map.insert("Endpoint.contact".to_string(), "*".to_string());
    map.insert("Endpoint.contained".to_string(), "*".to_string());
    map.insert("Endpoint.extension".to_string(), "*".to_string());
    map.insert("Endpoint.header".to_string(), "*".to_string());
    map.insert("Endpoint.id".to_string(), "1".to_string());
    map.insert("Endpoint.identifier".to_string(), "*".to_string());
    map.insert("Endpoint.implicitRules".to_string(), "1".to_string());
    map.insert("Endpoint.language".to_string(), "1".to_string());
    map.insert("Endpoint.managingOrganization".to_string(), "1".to_string());
    map.insert("Endpoint.meta".to_string(), "1".to_string());
    map.insert("Endpoint.modifierExtension".to_string(), "*".to_string());
    map.insert("Endpoint.name".to_string(), "1".to_string());
    map.insert("Endpoint.payloadMimeType".to_string(), "*".to_string());
    map.insert("Endpoint.payloadType".to_string(), "*".to_string());
    map.insert("Endpoint.period".to_string(), "1".to_string());
    map.insert("Endpoint.status".to_string(), "1".to_string());
    map.insert("Endpoint.text".to_string(), "1".to_string());
    map.insert("EnrollmentRequest.candidate".to_string(), "1".to_string());
    map.insert("EnrollmentRequest.contained".to_string(), "*".to_string());
    map.insert("EnrollmentRequest.coverage".to_string(), "1".to_string());
    map.insert("EnrollmentRequest.created".to_string(), "1".to_string());
    map.insert("EnrollmentRequest.extension".to_string(), "*".to_string());
    map.insert("EnrollmentRequest.id".to_string(), "1".to_string());
    map.insert("EnrollmentRequest.identifier".to_string(), "*".to_string());
    map.insert("EnrollmentRequest.implicitRules".to_string(), "1".to_string());
    map.insert("EnrollmentRequest.insurer".to_string(), "1".to_string());
    map.insert("EnrollmentRequest.language".to_string(), "1".to_string());
    map.insert("EnrollmentRequest.meta".to_string(), "1".to_string());
    map.insert("EnrollmentRequest.modifierExtension".to_string(), "*".to_string());
    map.insert("EnrollmentRequest.provider".to_string(), "1".to_string());
    map.insert("EnrollmentRequest.status".to_string(), "1".to_string());
    map.insert("EnrollmentRequest.text".to_string(), "1".to_string());
    map.insert("EnrollmentResponse.contained".to_string(), "*".to_string());
    map.insert("EnrollmentResponse.created".to_string(), "1".to_string());
    map.insert("EnrollmentResponse.disposition".to_string(), "1".to_string());
    map.insert("EnrollmentResponse.extension".to_string(), "*".to_string());
    map.insert("EnrollmentResponse.id".to_string(), "1".to_string());
    map.insert("EnrollmentResponse.identifier".to_string(), "*".to_string());
    map.insert("EnrollmentResponse.implicitRules".to_string(), "1".to_string());
    map.insert("EnrollmentResponse.language".to_string(), "1".to_string());
    map.insert("EnrollmentResponse.meta".to_string(), "1".to_string());
    map.insert("EnrollmentResponse.modifierExtension".to_string(), "*".to_string());
    map.insert("EnrollmentResponse.organization".to_string(), "1".to_string());
    map.insert("EnrollmentResponse.outcome".to_string(), "1".to_string());
    map.insert("EnrollmentResponse.request".to_string(), "1".to_string());
    map.insert("EnrollmentResponse.requestProvider".to_string(), "1".to_string());
    map.insert("EnrollmentResponse.status".to_string(), "1".to_string());
    map.insert("EnrollmentResponse.text".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.account".to_string(), "*".to_string());
    map.insert("EpisodeOfCare.careManager".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.contained".to_string(), "*".to_string());
    map.insert("EpisodeOfCare.diagnosis".to_string(), "*".to_string());
    map.insert("EpisodeOfCare.diagnosis.condition".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.diagnosis.extension".to_string(), "*".to_string());
    map.insert("EpisodeOfCare.diagnosis.id".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.diagnosis.modifierExtension".to_string(), "*".to_string());
    map.insert("EpisodeOfCare.diagnosis.rank".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.diagnosis.role".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.extension".to_string(), "*".to_string());
    map.insert("EpisodeOfCare.id".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.identifier".to_string(), "*".to_string());
    map.insert("EpisodeOfCare.implicitRules".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.language".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.managingOrganization".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.meta".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.modifierExtension".to_string(), "*".to_string());
    map.insert("EpisodeOfCare.patient".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.period".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.referralRequest".to_string(), "*".to_string());
    map.insert("EpisodeOfCare.status".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.statusHistory".to_string(), "*".to_string());
    map.insert("EpisodeOfCare.statusHistory.extension".to_string(), "*".to_string());
    map.insert("EpisodeOfCare.statusHistory.id".to_string(), "1".to_string());
    map.insert(
        "EpisodeOfCare.statusHistory.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("EpisodeOfCare.statusHistory.period".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.statusHistory.status".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.team".to_string(), "*".to_string());
    map.insert("EpisodeOfCare.text".to_string(), "1".to_string());
    map.insert("EpisodeOfCare.type".to_string(), "*".to_string());
    map.insert("EventDefinition.approvalDate".to_string(), "1".to_string());
    map.insert("EventDefinition.author".to_string(), "*".to_string());
    map.insert("EventDefinition.contact".to_string(), "*".to_string());
    map.insert("EventDefinition.contained".to_string(), "*".to_string());
    map.insert("EventDefinition.copyright".to_string(), "1".to_string());
    map.insert("EventDefinition.date".to_string(), "1".to_string());
    map.insert("EventDefinition.description".to_string(), "1".to_string());
    map.insert("EventDefinition.editor".to_string(), "*".to_string());
    map.insert("EventDefinition.effectivePeriod".to_string(), "1".to_string());
    map.insert("EventDefinition.endorser".to_string(), "*".to_string());
    map.insert("EventDefinition.experimental".to_string(), "1".to_string());
    map.insert("EventDefinition.extension".to_string(), "*".to_string());
    map.insert("EventDefinition.id".to_string(), "1".to_string());
    map.insert("EventDefinition.identifier".to_string(), "*".to_string());
    map.insert("EventDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("EventDefinition.jurisdiction".to_string(), "*".to_string());
    map.insert("EventDefinition.language".to_string(), "1".to_string());
    map.insert("EventDefinition.lastReviewDate".to_string(), "1".to_string());
    map.insert("EventDefinition.meta".to_string(), "1".to_string());
    map.insert("EventDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("EventDefinition.name".to_string(), "1".to_string());
    map.insert("EventDefinition.publisher".to_string(), "1".to_string());
    map.insert("EventDefinition.purpose".to_string(), "1".to_string());
    map.insert("EventDefinition.relatedArtifact".to_string(), "*".to_string());
    map.insert("EventDefinition.reviewer".to_string(), "*".to_string());
    map.insert("EventDefinition.status".to_string(), "1".to_string());
    map.insert("EventDefinition.subjectCodeableConcept".to_string(), "1".to_string());
    map.insert("EventDefinition.subjectReference".to_string(), "1".to_string());
    map.insert("EventDefinition.subtitle".to_string(), "1".to_string());
    map.insert("EventDefinition.text".to_string(), "1".to_string());
    map.insert("EventDefinition.title".to_string(), "1".to_string());
    map.insert("EventDefinition.topic".to_string(), "*".to_string());
    map.insert("EventDefinition.trigger".to_string(), "*".to_string());
    map.insert("EventDefinition.url".to_string(), "1".to_string());
    map.insert("EventDefinition.usage".to_string(), "1".to_string());
    map.insert("EventDefinition.useContext".to_string(), "*".to_string());
    map.insert("EventDefinition.version".to_string(), "1".to_string());
    map.insert("Evidence.approvalDate".to_string(), "1".to_string());
    map.insert("Evidence.author".to_string(), "*".to_string());
    map.insert("Evidence.contact".to_string(), "*".to_string());
    map.insert("Evidence.contained".to_string(), "*".to_string());
    map.insert("Evidence.copyright".to_string(), "1".to_string());
    map.insert("Evidence.date".to_string(), "1".to_string());
    map.insert("Evidence.description".to_string(), "1".to_string());
    map.insert("Evidence.editor".to_string(), "*".to_string());
    map.insert("Evidence.effectivePeriod".to_string(), "1".to_string());
    map.insert("Evidence.endorser".to_string(), "*".to_string());
    map.insert("Evidence.exposureBackground".to_string(), "1".to_string());
    map.insert("Evidence.exposureVariant".to_string(), "*".to_string());
    map.insert("Evidence.extension".to_string(), "*".to_string());
    map.insert("Evidence.id".to_string(), "1".to_string());
    map.insert("Evidence.identifier".to_string(), "*".to_string());
    map.insert("Evidence.implicitRules".to_string(), "1".to_string());
    map.insert("Evidence.jurisdiction".to_string(), "*".to_string());
    map.insert("Evidence.language".to_string(), "1".to_string());
    map.insert("Evidence.lastReviewDate".to_string(), "1".to_string());
    map.insert("Evidence.meta".to_string(), "1".to_string());
    map.insert("Evidence.modifierExtension".to_string(), "*".to_string());
    map.insert("Evidence.name".to_string(), "1".to_string());
    map.insert("Evidence.note".to_string(), "*".to_string());
    map.insert("Evidence.outcome".to_string(), "*".to_string());
    map.insert("Evidence.publisher".to_string(), "1".to_string());
    map.insert("Evidence.relatedArtifact".to_string(), "*".to_string());
    map.insert("Evidence.reviewer".to_string(), "*".to_string());
    map.insert("Evidence.shortTitle".to_string(), "1".to_string());
    map.insert("Evidence.status".to_string(), "1".to_string());
    map.insert("Evidence.subtitle".to_string(), "1".to_string());
    map.insert("Evidence.text".to_string(), "1".to_string());
    map.insert("Evidence.title".to_string(), "1".to_string());
    map.insert("Evidence.topic".to_string(), "*".to_string());
    map.insert("Evidence.url".to_string(), "1".to_string());
    map.insert("Evidence.useContext".to_string(), "*".to_string());
    map.insert("Evidence.version".to_string(), "1".to_string());
    map.insert("EvidenceVariable.approvalDate".to_string(), "1".to_string());
    map.insert("EvidenceVariable.author".to_string(), "*".to_string());
    map.insert("EvidenceVariable.characteristic".to_string(), "*".to_string());
    map.insert(
        "EvidenceVariable.characteristic.definitionCanonical".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EvidenceVariable.characteristic.definitionCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EvidenceVariable.characteristic.definitionDataRequirement".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EvidenceVariable.characteristic.definitionExpression".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EvidenceVariable.characteristic.definitionReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EvidenceVariable.characteristic.definitionTriggerDefinition".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EvidenceVariable.characteristic.description".to_string(),
        "1".to_string(),
    );
    map.insert("EvidenceVariable.characteristic.exclude".to_string(), "1".to_string());
    map.insert("EvidenceVariable.characteristic.extension".to_string(), "*".to_string());
    map.insert(
        "EvidenceVariable.characteristic.groupMeasure".to_string(),
        "1".to_string(),
    );
    map.insert("EvidenceVariable.characteristic.id".to_string(), "1".to_string());
    map.insert(
        "EvidenceVariable.characteristic.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EvidenceVariable.characteristic.participantEffectiveDateTime".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EvidenceVariable.characteristic.participantEffectiveDuration".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EvidenceVariable.characteristic.participantEffectivePeriod".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EvidenceVariable.characteristic.participantEffectiveTiming".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EvidenceVariable.characteristic.timeFromStart".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EvidenceVariable.characteristic.usageContext".to_string(),
        "*".to_string(),
    );
    map.insert("EvidenceVariable.contact".to_string(), "*".to_string());
    map.insert("EvidenceVariable.contained".to_string(), "*".to_string());
    map.insert("EvidenceVariable.copyright".to_string(), "1".to_string());
    map.insert("EvidenceVariable.date".to_string(), "1".to_string());
    map.insert("EvidenceVariable.description".to_string(), "1".to_string());
    map.insert("EvidenceVariable.editor".to_string(), "*".to_string());
    map.insert("EvidenceVariable.effectivePeriod".to_string(), "1".to_string());
    map.insert("EvidenceVariable.endorser".to_string(), "*".to_string());
    map.insert("EvidenceVariable.extension".to_string(), "*".to_string());
    map.insert("EvidenceVariable.id".to_string(), "1".to_string());
    map.insert("EvidenceVariable.identifier".to_string(), "*".to_string());
    map.insert("EvidenceVariable.implicitRules".to_string(), "1".to_string());
    map.insert("EvidenceVariable.jurisdiction".to_string(), "*".to_string());
    map.insert("EvidenceVariable.language".to_string(), "1".to_string());
    map.insert("EvidenceVariable.lastReviewDate".to_string(), "1".to_string());
    map.insert("EvidenceVariable.meta".to_string(), "1".to_string());
    map.insert("EvidenceVariable.modifierExtension".to_string(), "*".to_string());
    map.insert("EvidenceVariable.name".to_string(), "1".to_string());
    map.insert("EvidenceVariable.note".to_string(), "*".to_string());
    map.insert("EvidenceVariable.publisher".to_string(), "1".to_string());
    map.insert("EvidenceVariable.relatedArtifact".to_string(), "*".to_string());
    map.insert("EvidenceVariable.reviewer".to_string(), "*".to_string());
    map.insert("EvidenceVariable.shortTitle".to_string(), "1".to_string());
    map.insert("EvidenceVariable.status".to_string(), "1".to_string());
    map.insert("EvidenceVariable.subtitle".to_string(), "1".to_string());
    map.insert("EvidenceVariable.text".to_string(), "1".to_string());
    map.insert("EvidenceVariable.title".to_string(), "1".to_string());
    map.insert("EvidenceVariable.topic".to_string(), "*".to_string());
    map.insert("EvidenceVariable.type".to_string(), "1".to_string());
    map.insert("EvidenceVariable.url".to_string(), "1".to_string());
    map.insert("EvidenceVariable.useContext".to_string(), "*".to_string());
    map.insert("EvidenceVariable.version".to_string(), "1".to_string());
    map.insert("ExampleScenario.actor".to_string(), "*".to_string());
    map.insert("ExampleScenario.actor.actorId".to_string(), "1".to_string());
    map.insert("ExampleScenario.actor.description".to_string(), "1".to_string());
    map.insert("ExampleScenario.actor.extension".to_string(), "*".to_string());
    map.insert("ExampleScenario.actor.id".to_string(), "1".to_string());
    map.insert("ExampleScenario.actor.modifierExtension".to_string(), "*".to_string());
    map.insert("ExampleScenario.actor.name".to_string(), "1".to_string());
    map.insert("ExampleScenario.actor.type".to_string(), "1".to_string());
    map.insert("ExampleScenario.contact".to_string(), "*".to_string());
    map.insert("ExampleScenario.contained".to_string(), "*".to_string());
    map.insert("ExampleScenario.copyright".to_string(), "1".to_string());
    map.insert("ExampleScenario.date".to_string(), "1".to_string());
    map.insert("ExampleScenario.experimental".to_string(), "1".to_string());
    map.insert("ExampleScenario.extension".to_string(), "*".to_string());
    map.insert("ExampleScenario.id".to_string(), "1".to_string());
    map.insert("ExampleScenario.identifier".to_string(), "*".to_string());
    map.insert("ExampleScenario.implicitRules".to_string(), "1".to_string());
    map.insert("ExampleScenario.instance".to_string(), "*".to_string());
    map.insert(
        "ExampleScenario.instance.containedInstance".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExampleScenario.instance.containedInstance.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExampleScenario.instance.containedInstance.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExampleScenario.instance.containedInstance.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExampleScenario.instance.containedInstance.resourceId".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExampleScenario.instance.containedInstance.versionId".to_string(),
        "1".to_string(),
    );
    map.insert("ExampleScenario.instance.description".to_string(), "1".to_string());
    map.insert("ExampleScenario.instance.extension".to_string(), "*".to_string());
    map.insert("ExampleScenario.instance.id".to_string(), "1".to_string());
    map.insert(
        "ExampleScenario.instance.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExampleScenario.instance.name".to_string(), "1".to_string());
    map.insert("ExampleScenario.instance.resourceId".to_string(), "1".to_string());
    map.insert("ExampleScenario.instance.resourceType".to_string(), "1".to_string());
    map.insert("ExampleScenario.instance.version".to_string(), "*".to_string());
    map.insert(
        "ExampleScenario.instance.version.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExampleScenario.instance.version.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ExampleScenario.instance.version.id".to_string(), "1".to_string());
    map.insert(
        "ExampleScenario.instance.version.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExampleScenario.instance.version.versionId".to_string(),
        "1".to_string(),
    );
    map.insert("ExampleScenario.jurisdiction".to_string(), "*".to_string());
    map.insert("ExampleScenario.language".to_string(), "1".to_string());
    map.insert("ExampleScenario.meta".to_string(), "1".to_string());
    map.insert("ExampleScenario.modifierExtension".to_string(), "*".to_string());
    map.insert("ExampleScenario.name".to_string(), "1".to_string());
    map.insert("ExampleScenario.process".to_string(), "*".to_string());
    map.insert("ExampleScenario.process.description".to_string(), "1".to_string());
    map.insert("ExampleScenario.process.extension".to_string(), "*".to_string());
    map.insert("ExampleScenario.process.id".to_string(), "1".to_string());
    map.insert("ExampleScenario.process.modifierExtension".to_string(), "*".to_string());
    map.insert("ExampleScenario.process.postConditions".to_string(), "1".to_string());
    map.insert("ExampleScenario.process.preConditions".to_string(), "1".to_string());
    map.insert("ExampleScenario.process.step".to_string(), "*".to_string());
    map.insert("ExampleScenario.process.step.alternative".to_string(), "*".to_string());
    map.insert(
        "ExampleScenario.process.step.alternative.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExampleScenario.process.step.alternative.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExampleScenario.process.step.alternative.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExampleScenario.process.step.alternative.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExampleScenario.process.step.alternative.step".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExampleScenario.process.step.alternative.title".to_string(),
        "1".to_string(),
    );
    map.insert("ExampleScenario.process.step.extension".to_string(), "*".to_string());
    map.insert("ExampleScenario.process.step.id".to_string(), "1".to_string());
    map.insert(
        "ExampleScenario.process.step.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExampleScenario.process.step.operation".to_string(), "1".to_string());
    map.insert(
        "ExampleScenario.process.step.operation.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExampleScenario.process.step.operation.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ExampleScenario.process.step.operation.id".to_string(), "1".to_string());
    map.insert(
        "ExampleScenario.process.step.operation.initiator".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExampleScenario.process.step.operation.initiatorActive".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExampleScenario.process.step.operation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExampleScenario.process.step.operation.name".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExampleScenario.process.step.operation.number".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExampleScenario.process.step.operation.receiver".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExampleScenario.process.step.operation.receiverActive".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExampleScenario.process.step.operation.request".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExampleScenario.process.step.operation.response".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExampleScenario.process.step.operation.type".to_string(),
        "1".to_string(),
    );
    map.insert("ExampleScenario.process.step.pause".to_string(), "1".to_string());
    map.insert("ExampleScenario.process.step.process".to_string(), "*".to_string());
    map.insert("ExampleScenario.process.title".to_string(), "1".to_string());
    map.insert("ExampleScenario.publisher".to_string(), "1".to_string());
    map.insert("ExampleScenario.purpose".to_string(), "1".to_string());
    map.insert("ExampleScenario.status".to_string(), "1".to_string());
    map.insert("ExampleScenario.text".to_string(), "1".to_string());
    map.insert("ExampleScenario.url".to_string(), "1".to_string());
    map.insert("ExampleScenario.useContext".to_string(), "*".to_string());
    map.insert("ExampleScenario.version".to_string(), "1".to_string());
    map.insert("ExampleScenario.workflow".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.accident".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.accident.date".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.accident.extension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.accident.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.accident.locationAddress".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.accident.locationReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.accident.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.accident.type".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.addItem".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.addItem.adjudication".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.addItem.bodySite".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.addItem.detail".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.addItem.detail.adjudication".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.factor".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.addItem.detail.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.addItem.detail.modifier".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.addItem.detail.net".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.addItem.detail.noteNumber".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.productOrService".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.quantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.subDetail".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.subDetail.adjudication".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.subDetail.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.subDetail.factor".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.subDetail.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.subDetail.modifier".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.subDetail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.subDetail.net".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.subDetail.noteNumber".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.subDetail.productOrService".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.subDetail.quantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.subDetail.unitPrice".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.unitPrice".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detailSequence".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.addItem.extension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.addItem.factor".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.addItem.id".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.addItem.itemSequence".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.addItem.locationAddress".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.locationCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.locationReference".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.addItem.modifier".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.addItem.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.addItem.net".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.addItem.noteNumber".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.addItem.productOrService".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.addItem.programCode".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.addItem.provider".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.addItem.quantity".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.addItem.servicedDate".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.addItem.servicedPeriod".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.subDetailSequence".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.addItem.subSite".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.addItem.unitPrice".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.adjudication".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.benefitBalance".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.benefitBalance.category".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.benefitBalance.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.benefitBalance.excluded".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.benefitBalance.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.benefitBalance.financial".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.benefitBalance.financial.allowedMoney".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.benefitBalance.financial.allowedString".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.benefitBalance.financial.allowedUnsignedInt".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.benefitBalance.financial.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.benefitBalance.financial.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.benefitBalance.financial.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.benefitBalance.financial.type".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.benefitBalance.financial.usedMoney".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.benefitBalance.financial.usedUnsignedInt".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.benefitBalance.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.benefitBalance.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.benefitBalance.name".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.benefitBalance.network".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.benefitBalance.term".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.benefitBalance.unit".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.benefitPeriod".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.billablePeriod".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.careTeam".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.careTeam.extension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.careTeam.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.careTeam.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.careTeam.provider".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.careTeam.qualification".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.careTeam.responsible".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.careTeam.role".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.careTeam.sequence".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.claim".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.claimResponse".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.contained".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.created".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.diagnosis".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.diagnosis.diagnosisCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.diagnosis.diagnosisReference".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.diagnosis.extension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.diagnosis.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.diagnosis.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.diagnosis.onAdmission".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.diagnosis.packageCode".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.diagnosis.sequence".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.diagnosis.type".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.disposition".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.enterer".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.extension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.facility".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.form".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.formCode".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.fundsReserve".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.fundsReserveRequested".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.id".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.identifier".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.implicitRules".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.insurance".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.insurance.coverage".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.insurance.extension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.insurance.focal".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.insurance.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.insurance.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.insurance.preAuthRef".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.insurer".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.item.adjudication".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.item.adjudication.amount".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.adjudication.category".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.adjudication.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.item.adjudication.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.item.adjudication.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.adjudication.reason".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.adjudication.value".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.item.bodySite".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.item.careTeamSequence".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.item.category".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.detail".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.item.detail.adjudication".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.item.detail.category".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.item.detail.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.item.detail.factor".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.detail.id".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.detail.modifier".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.item.detail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.item.detail.net".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.item.detail.noteNumber".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.productOrService".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.programCode".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.item.detail.quantity".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.detail.revenue".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.detail.sequence".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.adjudication".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.category".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.factor".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.modifier".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.net".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.noteNumber".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.productOrService".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.programCode".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.quantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.revenue".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.sequence".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.udi".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.unitPrice".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.item.detail.udi".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.item.detail.unitPrice".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.diagnosisSequence".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.item.encounter".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.item.extension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.item.factor".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.item.informationSequence".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.item.locationAddress".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.item.locationCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.locationReference".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.item.modifier".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.item.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.item.net".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.noteNumber".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.item.procedureSequence".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.productOrService".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.item.programCode".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.item.quantity".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.revenue".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.sequence".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.servicedDate".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.servicedPeriod".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.subSite".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.item.udi".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.item.unitPrice".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.language".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.meta".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.modifierExtension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.originalPrescription".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.outcome".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.patient".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.payee".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.payee.extension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.payee.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.payee.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.payee.party".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.payee.type".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.payment".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.payment.adjustment".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.payment.adjustmentReason".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.payment.amount".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.payment.date".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.payment.extension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.payment.id".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.payment.identifier".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.payment.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.payment.type".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.preAuthRef".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.preAuthRefPeriod".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.precedence".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.prescription".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.priority".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.procedure".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.procedure.date".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.procedure.extension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.procedure.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.procedure.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.procedure.procedureCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.procedure.procedureReference".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.procedure.sequence".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.procedure.type".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.procedure.udi".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.processNote".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.processNote.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.processNote.id".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.processNote.language".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.processNote.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.processNote.number".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.processNote.text".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.processNote.type".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.provider".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.referral".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.related".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.related.claim".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.related.extension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.related.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.related.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.related.reference".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.related.relationship".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.status".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.subType".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.supportingInfo".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.supportingInfo.category".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.supportingInfo.code".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.supportingInfo.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.supportingInfo.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.supportingInfo.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.supportingInfo.reason".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.supportingInfo.sequence".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.supportingInfo.timingDate".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.supportingInfo.timingPeriod".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.supportingInfo.valueAttachment".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.supportingInfo.valueBoolean".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.supportingInfo.valueQuantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.supportingInfo.valueReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.supportingInfo.valueString".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.text".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.total".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.total.amount".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.total.category".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.total.extension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.total.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.total.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.type".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.use".to_string(), "1".to_string());
    map.insert("Expression.description".to_string(), "1".to_string());
    map.insert("Expression.expression".to_string(), "1".to_string());
    map.insert("Expression.extension".to_string(), "*".to_string());
    map.insert("Expression.id".to_string(), "1".to_string());
    map.insert("Expression.language".to_string(), "1".to_string());
    map.insert("Expression.name".to_string(), "1".to_string());
    map.insert("Expression.reference".to_string(), "1".to_string());
    map.insert("Extension.extension".to_string(), "*".to_string());
    map.insert("Extension.id".to_string(), "1".to_string());
    map.insert("Extension.url".to_string(), "1".to_string());
    map.insert("Extension.valueAddress".to_string(), "1".to_string());
    map.insert("Extension.valueAge".to_string(), "1".to_string());
    map.insert("Extension.valueAnnotation".to_string(), "1".to_string());
    map.insert("Extension.valueAttachment".to_string(), "1".to_string());
    map.insert("Extension.valueBase64Binary".to_string(), "1".to_string());
    map.insert("Extension.valueBoolean".to_string(), "1".to_string());
    map.insert("Extension.valueCanonical".to_string(), "1".to_string());
    map.insert("Extension.valueCode".to_string(), "1".to_string());
    map.insert("Extension.valueCodeableConcept".to_string(), "1".to_string());
    map.insert("Extension.valueCoding".to_string(), "1".to_string());
    map.insert("Extension.valueContactDetail".to_string(), "1".to_string());
    map.insert("Extension.valueContactPoint".to_string(), "1".to_string());
    map.insert("Extension.valueContributor".to_string(), "1".to_string());
    map.insert("Extension.valueCount".to_string(), "1".to_string());
    map.insert("Extension.valueDataRequirement".to_string(), "1".to_string());
    map.insert("Extension.valueDate".to_string(), "1".to_string());
    map.insert("Extension.valueDateTime".to_string(), "1".to_string());
    map.insert("Extension.valueDecimal".to_string(), "1".to_string());
    map.insert("Extension.valueDistance".to_string(), "1".to_string());
    map.insert("Extension.valueDosage".to_string(), "1".to_string());
    map.insert("Extension.valueDuration".to_string(), "1".to_string());
    map.insert("Extension.valueExpression".to_string(), "1".to_string());
    map.insert("Extension.valueHumanName".to_string(), "1".to_string());
    map.insert("Extension.valueId".to_string(), "1".to_string());
    map.insert("Extension.valueIdentifier".to_string(), "1".to_string());
    map.insert("Extension.valueInstant".to_string(), "1".to_string());
    map.insert("Extension.valueInteger".to_string(), "1".to_string());
    map.insert("Extension.valueMarkdown".to_string(), "1".to_string());
    map.insert("Extension.valueMeta".to_string(), "1".to_string());
    map.insert("Extension.valueMoney".to_string(), "1".to_string());
    map.insert("Extension.valueOid".to_string(), "1".to_string());
    map.insert("Extension.valueParameterDefinition".to_string(), "1".to_string());
    map.insert("Extension.valuePeriod".to_string(), "1".to_string());
    map.insert("Extension.valuePositiveInt".to_string(), "1".to_string());
    map.insert("Extension.valueQuantity".to_string(), "1".to_string());
    map.insert("Extension.valueRange".to_string(), "1".to_string());
    map.insert("Extension.valueRatio".to_string(), "1".to_string());
    map.insert("Extension.valueReference".to_string(), "1".to_string());
    map.insert("Extension.valueRelatedArtifact".to_string(), "1".to_string());
    map.insert("Extension.valueSampledData".to_string(), "1".to_string());
    map.insert("Extension.valueSignature".to_string(), "1".to_string());
    map.insert("Extension.valueString".to_string(), "1".to_string());
    map.insert("Extension.valueTime".to_string(), "1".to_string());
    map.insert("Extension.valueTiming".to_string(), "1".to_string());
    map.insert("Extension.valueTriggerDefinition".to_string(), "1".to_string());
    map.insert("Extension.valueUnsignedInt".to_string(), "1".to_string());
    map.insert("Extension.valueUri".to_string(), "1".to_string());
    map.insert("Extension.valueUrl".to_string(), "1".to_string());
    map.insert("Extension.valueUsageContext".to_string(), "1".to_string());
    map.insert("Extension.valueUuid".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.ageAge".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.ageRange".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.ageString".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.bornDate".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.bornPeriod".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.bornString".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.condition".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.condition.code".to_string(), "1".to_string());
    map.insert(
        "FamilyMemberHistory.condition.contributedToDeath".to_string(),
        "1".to_string(),
    );
    map.insert("FamilyMemberHistory.condition.extension".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.condition.id".to_string(), "1".to_string());
    map.insert(
        "FamilyMemberHistory.condition.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("FamilyMemberHistory.condition.note".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.condition.onsetAge".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.condition.onsetPeriod".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.condition.onsetRange".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.condition.onsetString".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.condition.outcome".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.contained".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.dataAbsentReason".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.date".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.deceasedAge".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.deceasedBoolean".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.deceasedDate".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.deceasedRange".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.deceasedString".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.estimatedAge".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.extension".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.id".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.identifier".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.implicitRules".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.instantiatesCanonical".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.instantiatesUri".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.language".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.meta".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.modifierExtension".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.name".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.note".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.patient".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.reasonCode".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.reasonReference".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.relationship".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.sex".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.status".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.text".to_string(), "1".to_string());
    map.insert("Flag.author".to_string(), "1".to_string());
    map.insert("Flag.category".to_string(), "*".to_string());
    map.insert("Flag.code".to_string(), "1".to_string());
    map.insert("Flag.contained".to_string(), "*".to_string());
    map.insert("Flag.encounter".to_string(), "1".to_string());
    map.insert("Flag.extension".to_string(), "*".to_string());
    map.insert("Flag.id".to_string(), "1".to_string());
    map.insert("Flag.identifier".to_string(), "*".to_string());
    map.insert("Flag.implicitRules".to_string(), "1".to_string());
    map.insert("Flag.language".to_string(), "1".to_string());
    map.insert("Flag.meta".to_string(), "1".to_string());
    map.insert("Flag.modifierExtension".to_string(), "*".to_string());
    map.insert("Flag.period".to_string(), "1".to_string());
    map.insert("Flag.status".to_string(), "1".to_string());
    map.insert("Flag.subject".to_string(), "1".to_string());
    map.insert("Flag.text".to_string(), "1".to_string());
    map.insert("Goal.achievementStatus".to_string(), "1".to_string());
    map.insert("Goal.addresses".to_string(), "*".to_string());
    map.insert("Goal.category".to_string(), "*".to_string());
    map.insert("Goal.contained".to_string(), "*".to_string());
    map.insert("Goal.description".to_string(), "1".to_string());
    map.insert("Goal.expressedBy".to_string(), "1".to_string());
    map.insert("Goal.extension".to_string(), "*".to_string());
    map.insert("Goal.id".to_string(), "1".to_string());
    map.insert("Goal.identifier".to_string(), "*".to_string());
    map.insert("Goal.implicitRules".to_string(), "1".to_string());
    map.insert("Goal.language".to_string(), "1".to_string());
    map.insert("Goal.lifecycleStatus".to_string(), "1".to_string());
    map.insert("Goal.meta".to_string(), "1".to_string());
    map.insert("Goal.modifierExtension".to_string(), "*".to_string());
    map.insert("Goal.note".to_string(), "*".to_string());
    map.insert("Goal.outcomeCode".to_string(), "*".to_string());
    map.insert("Goal.outcomeReference".to_string(), "*".to_string());
    map.insert("Goal.priority".to_string(), "1".to_string());
    map.insert("Goal.startCodeableConcept".to_string(), "1".to_string());
    map.insert("Goal.startDate".to_string(), "1".to_string());
    map.insert("Goal.statusDate".to_string(), "1".to_string());
    map.insert("Goal.statusReason".to_string(), "1".to_string());
    map.insert("Goal.subject".to_string(), "1".to_string());
    map.insert("Goal.target".to_string(), "*".to_string());
    map.insert("Goal.target.detailBoolean".to_string(), "1".to_string());
    map.insert("Goal.target.detailCodeableConcept".to_string(), "1".to_string());
    map.insert("Goal.target.detailInteger".to_string(), "1".to_string());
    map.insert("Goal.target.detailQuantity".to_string(), "1".to_string());
    map.insert("Goal.target.detailRange".to_string(), "1".to_string());
    map.insert("Goal.target.detailRatio".to_string(), "1".to_string());
    map.insert("Goal.target.detailString".to_string(), "1".to_string());
    map.insert("Goal.target.dueDate".to_string(), "1".to_string());
    map.insert("Goal.target.dueDuration".to_string(), "1".to_string());
    map.insert("Goal.target.extension".to_string(), "*".to_string());
    map.insert("Goal.target.id".to_string(), "1".to_string());
    map.insert("Goal.target.measure".to_string(), "1".to_string());
    map.insert("Goal.target.modifierExtension".to_string(), "*".to_string());
    map.insert("Goal.text".to_string(), "1".to_string());
    map.insert("GraphDefinition.contact".to_string(), "*".to_string());
    map.insert("GraphDefinition.contained".to_string(), "*".to_string());
    map.insert("GraphDefinition.date".to_string(), "1".to_string());
    map.insert("GraphDefinition.description".to_string(), "1".to_string());
    map.insert("GraphDefinition.experimental".to_string(), "1".to_string());
    map.insert("GraphDefinition.extension".to_string(), "*".to_string());
    map.insert("GraphDefinition.id".to_string(), "1".to_string());
    map.insert("GraphDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("GraphDefinition.jurisdiction".to_string(), "*".to_string());
    map.insert("GraphDefinition.language".to_string(), "1".to_string());
    map.insert("GraphDefinition.link".to_string(), "*".to_string());
    map.insert("GraphDefinition.link.description".to_string(), "1".to_string());
    map.insert("GraphDefinition.link.extension".to_string(), "*".to_string());
    map.insert("GraphDefinition.link.id".to_string(), "1".to_string());
    map.insert("GraphDefinition.link.max".to_string(), "1".to_string());
    map.insert("GraphDefinition.link.min".to_string(), "1".to_string());
    map.insert("GraphDefinition.link.modifierExtension".to_string(), "*".to_string());
    map.insert("GraphDefinition.link.path".to_string(), "1".to_string());
    map.insert("GraphDefinition.link.sliceName".to_string(), "1".to_string());
    map.insert("GraphDefinition.link.target".to_string(), "*".to_string());
    map.insert("GraphDefinition.link.target.compartment".to_string(), "*".to_string());
    map.insert(
        "GraphDefinition.link.target.compartment.code".to_string(),
        "1".to_string(),
    );
    map.insert(
        "GraphDefinition.link.target.compartment.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "GraphDefinition.link.target.compartment.expression".to_string(),
        "1".to_string(),
    );
    map.insert(
        "GraphDefinition.link.target.compartment.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "GraphDefinition.link.target.compartment.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "GraphDefinition.link.target.compartment.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "GraphDefinition.link.target.compartment.rule".to_string(),
        "1".to_string(),
    );
    map.insert(
        "GraphDefinition.link.target.compartment.use".to_string(),
        "1".to_string(),
    );
    map.insert("GraphDefinition.link.target.extension".to_string(), "*".to_string());
    map.insert("GraphDefinition.link.target.id".to_string(), "1".to_string());
    map.insert("GraphDefinition.link.target.link".to_string(), "*".to_string());
    map.insert(
        "GraphDefinition.link.target.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("GraphDefinition.link.target.params".to_string(), "1".to_string());
    map.insert("GraphDefinition.link.target.profile".to_string(), "1".to_string());
    map.insert("GraphDefinition.link.target.type".to_string(), "1".to_string());
    map.insert("GraphDefinition.meta".to_string(), "1".to_string());
    map.insert("GraphDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("GraphDefinition.name".to_string(), "1".to_string());
    map.insert("GraphDefinition.profile".to_string(), "1".to_string());
    map.insert("GraphDefinition.publisher".to_string(), "1".to_string());
    map.insert("GraphDefinition.purpose".to_string(), "1".to_string());
    map.insert("GraphDefinition.start".to_string(), "1".to_string());
    map.insert("GraphDefinition.status".to_string(), "1".to_string());
    map.insert("GraphDefinition.text".to_string(), "1".to_string());
    map.insert("GraphDefinition.url".to_string(), "1".to_string());
    map.insert("GraphDefinition.useContext".to_string(), "*".to_string());
    map.insert("GraphDefinition.version".to_string(), "1".to_string());
    map.insert("Group.active".to_string(), "1".to_string());
    map.insert("Group.actual".to_string(), "1".to_string());
    map.insert("Group.characteristic".to_string(), "*".to_string());
    map.insert("Group.characteristic.code".to_string(), "1".to_string());
    map.insert("Group.characteristic.exclude".to_string(), "1".to_string());
    map.insert("Group.characteristic.extension".to_string(), "*".to_string());
    map.insert("Group.characteristic.id".to_string(), "1".to_string());
    map.insert("Group.characteristic.modifierExtension".to_string(), "*".to_string());
    map.insert("Group.characteristic.period".to_string(), "1".to_string());
    map.insert("Group.characteristic.valueBoolean".to_string(), "1".to_string());
    map.insert("Group.characteristic.valueCodeableConcept".to_string(), "1".to_string());
    map.insert("Group.characteristic.valueQuantity".to_string(), "1".to_string());
    map.insert("Group.characteristic.valueRange".to_string(), "1".to_string());
    map.insert("Group.characteristic.valueReference".to_string(), "1".to_string());
    map.insert("Group.code".to_string(), "1".to_string());
    map.insert("Group.contained".to_string(), "*".to_string());
    map.insert("Group.extension".to_string(), "*".to_string());
    map.insert("Group.id".to_string(), "1".to_string());
    map.insert("Group.identifier".to_string(), "*".to_string());
    map.insert("Group.implicitRules".to_string(), "1".to_string());
    map.insert("Group.language".to_string(), "1".to_string());
    map.insert("Group.managingEntity".to_string(), "1".to_string());
    map.insert("Group.member".to_string(), "*".to_string());
    map.insert("Group.member.entity".to_string(), "1".to_string());
    map.insert("Group.member.extension".to_string(), "*".to_string());
    map.insert("Group.member.id".to_string(), "1".to_string());
    map.insert("Group.member.inactive".to_string(), "1".to_string());
    map.insert("Group.member.modifierExtension".to_string(), "*".to_string());
    map.insert("Group.member.period".to_string(), "1".to_string());
    map.insert("Group.meta".to_string(), "1".to_string());
    map.insert("Group.modifierExtension".to_string(), "*".to_string());
    map.insert("Group.name".to_string(), "1".to_string());
    map.insert("Group.quantity".to_string(), "1".to_string());
    map.insert("Group.text".to_string(), "1".to_string());
    map.insert("Group.type".to_string(), "1".to_string());
    map.insert("GuidanceResponse.contained".to_string(), "*".to_string());
    map.insert("GuidanceResponse.dataRequirement".to_string(), "*".to_string());
    map.insert("GuidanceResponse.encounter".to_string(), "1".to_string());
    map.insert("GuidanceResponse.evaluationMessage".to_string(), "*".to_string());
    map.insert("GuidanceResponse.extension".to_string(), "*".to_string());
    map.insert("GuidanceResponse.id".to_string(), "1".to_string());
    map.insert("GuidanceResponse.identifier".to_string(), "*".to_string());
    map.insert("GuidanceResponse.implicitRules".to_string(), "1".to_string());
    map.insert("GuidanceResponse.language".to_string(), "1".to_string());
    map.insert("GuidanceResponse.meta".to_string(), "1".to_string());
    map.insert("GuidanceResponse.modifierExtension".to_string(), "*".to_string());
    map.insert("GuidanceResponse.moduleCanonical".to_string(), "1".to_string());
    map.insert("GuidanceResponse.moduleCodeableConcept".to_string(), "1".to_string());
    map.insert("GuidanceResponse.moduleUri".to_string(), "1".to_string());
    map.insert("GuidanceResponse.note".to_string(), "*".to_string());
    map.insert("GuidanceResponse.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("GuidanceResponse.outputParameters".to_string(), "1".to_string());
    map.insert("GuidanceResponse.performer".to_string(), "1".to_string());
    map.insert("GuidanceResponse.reasonCode".to_string(), "*".to_string());
    map.insert("GuidanceResponse.reasonReference".to_string(), "*".to_string());
    map.insert("GuidanceResponse.requestIdentifier".to_string(), "1".to_string());
    map.insert("GuidanceResponse.result".to_string(), "1".to_string());
    map.insert("GuidanceResponse.status".to_string(), "1".to_string());
    map.insert("GuidanceResponse.subject".to_string(), "1".to_string());
    map.insert("GuidanceResponse.text".to_string(), "1".to_string());
    map.insert("HealthcareService.active".to_string(), "1".to_string());
    map.insert("HealthcareService.appointmentRequired".to_string(), "1".to_string());
    map.insert("HealthcareService.availabilityExceptions".to_string(), "1".to_string());
    map.insert("HealthcareService.availableTime".to_string(), "*".to_string());
    map.insert("HealthcareService.availableTime.allDay".to_string(), "1".to_string());
    map.insert(
        "HealthcareService.availableTime.availableEndTime".to_string(),
        "1".to_string(),
    );
    map.insert(
        "HealthcareService.availableTime.availableStartTime".to_string(),
        "1".to_string(),
    );
    map.insert(
        "HealthcareService.availableTime.daysOfWeek".to_string(),
        "*".to_string(),
    );
    map.insert("HealthcareService.availableTime.extension".to_string(), "*".to_string());
    map.insert("HealthcareService.availableTime.id".to_string(), "1".to_string());
    map.insert(
        "HealthcareService.availableTime.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("HealthcareService.category".to_string(), "*".to_string());
    map.insert("HealthcareService.characteristic".to_string(), "*".to_string());
    map.insert("HealthcareService.comment".to_string(), "1".to_string());
    map.insert("HealthcareService.communication".to_string(), "*".to_string());
    map.insert("HealthcareService.contained".to_string(), "*".to_string());
    map.insert("HealthcareService.coverageArea".to_string(), "*".to_string());
    map.insert("HealthcareService.eligibility".to_string(), "*".to_string());
    map.insert("HealthcareService.eligibility.code".to_string(), "1".to_string());
    map.insert("HealthcareService.eligibility.comment".to_string(), "1".to_string());
    map.insert("HealthcareService.eligibility.extension".to_string(), "*".to_string());
    map.insert("HealthcareService.eligibility.id".to_string(), "1".to_string());
    map.insert(
        "HealthcareService.eligibility.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("HealthcareService.endpoint".to_string(), "*".to_string());
    map.insert("HealthcareService.extension".to_string(), "*".to_string());
    map.insert("HealthcareService.extraDetails".to_string(), "1".to_string());
    map.insert("HealthcareService.id".to_string(), "1".to_string());
    map.insert("HealthcareService.identifier".to_string(), "*".to_string());
    map.insert("HealthcareService.implicitRules".to_string(), "1".to_string());
    map.insert("HealthcareService.language".to_string(), "1".to_string());
    map.insert("HealthcareService.location".to_string(), "*".to_string());
    map.insert("HealthcareService.meta".to_string(), "1".to_string());
    map.insert("HealthcareService.modifierExtension".to_string(), "*".to_string());
    map.insert("HealthcareService.name".to_string(), "1".to_string());
    map.insert("HealthcareService.notAvailable".to_string(), "*".to_string());
    map.insert(
        "HealthcareService.notAvailable.description".to_string(),
        "1".to_string(),
    );
    map.insert("HealthcareService.notAvailable.during".to_string(), "1".to_string());
    map.insert("HealthcareService.notAvailable.extension".to_string(), "*".to_string());
    map.insert("HealthcareService.notAvailable.id".to_string(), "1".to_string());
    map.insert(
        "HealthcareService.notAvailable.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("HealthcareService.photo".to_string(), "1".to_string());
    map.insert("HealthcareService.program".to_string(), "*".to_string());
    map.insert("HealthcareService.providedBy".to_string(), "1".to_string());
    map.insert("HealthcareService.referralMethod".to_string(), "*".to_string());
    map.insert("HealthcareService.serviceProvisionCode".to_string(), "*".to_string());
    map.insert("HealthcareService.specialty".to_string(), "*".to_string());
    map.insert("HealthcareService.telecom".to_string(), "*".to_string());
    map.insert("HealthcareService.text".to_string(), "1".to_string());
    map.insert("HealthcareService.type".to_string(), "*".to_string());
    map.insert("HumanName.extension".to_string(), "*".to_string());
    map.insert("HumanName.family".to_string(), "1".to_string());
    map.insert("HumanName.given".to_string(), "*".to_string());
    map.insert("HumanName.id".to_string(), "1".to_string());
    map.insert("HumanName.period".to_string(), "1".to_string());
    map.insert("HumanName.prefix".to_string(), "*".to_string());
    map.insert("HumanName.suffix".to_string(), "*".to_string());
    map.insert("HumanName.text".to_string(), "1".to_string());
    map.insert("HumanName.use".to_string(), "1".to_string());
    map.insert("Identifier.assigner".to_string(), "1".to_string());
    map.insert("Identifier.extension".to_string(), "*".to_string());
    map.insert("Identifier.id".to_string(), "1".to_string());
    map.insert("Identifier.period".to_string(), "1".to_string());
    map.insert("Identifier.system".to_string(), "1".to_string());
    map.insert("Identifier.type".to_string(), "1".to_string());
    map.insert("Identifier.use".to_string(), "1".to_string());
    map.insert("Identifier.value".to_string(), "1".to_string());
    map.insert("ImagingStudy.basedOn".to_string(), "*".to_string());
    map.insert("ImagingStudy.contained".to_string(), "*".to_string());
    map.insert("ImagingStudy.description".to_string(), "1".to_string());
    map.insert("ImagingStudy.encounter".to_string(), "1".to_string());
    map.insert("ImagingStudy.endpoint".to_string(), "*".to_string());
    map.insert("ImagingStudy.extension".to_string(), "*".to_string());
    map.insert("ImagingStudy.id".to_string(), "1".to_string());
    map.insert("ImagingStudy.identifier".to_string(), "*".to_string());
    map.insert("ImagingStudy.implicitRules".to_string(), "1".to_string());
    map.insert("ImagingStudy.interpreter".to_string(), "*".to_string());
    map.insert("ImagingStudy.language".to_string(), "1".to_string());
    map.insert("ImagingStudy.location".to_string(), "1".to_string());
    map.insert("ImagingStudy.meta".to_string(), "1".to_string());
    map.insert("ImagingStudy.modality".to_string(), "*".to_string());
    map.insert("ImagingStudy.modifierExtension".to_string(), "*".to_string());
    map.insert("ImagingStudy.note".to_string(), "*".to_string());
    map.insert("ImagingStudy.numberOfInstances".to_string(), "1".to_string());
    map.insert("ImagingStudy.numberOfSeries".to_string(), "1".to_string());
    map.insert("ImagingStudy.procedureCode".to_string(), "*".to_string());
    map.insert("ImagingStudy.procedureReference".to_string(), "1".to_string());
    map.insert("ImagingStudy.reasonCode".to_string(), "*".to_string());
    map.insert("ImagingStudy.reasonReference".to_string(), "*".to_string());
    map.insert("ImagingStudy.referrer".to_string(), "1".to_string());
    map.insert("ImagingStudy.series".to_string(), "*".to_string());
    map.insert("ImagingStudy.series.bodySite".to_string(), "1".to_string());
    map.insert("ImagingStudy.series.description".to_string(), "1".to_string());
    map.insert("ImagingStudy.series.endpoint".to_string(), "*".to_string());
    map.insert("ImagingStudy.series.extension".to_string(), "*".to_string());
    map.insert("ImagingStudy.series.id".to_string(), "1".to_string());
    map.insert("ImagingStudy.series.instance".to_string(), "*".to_string());
    map.insert("ImagingStudy.series.instance.extension".to_string(), "*".to_string());
    map.insert("ImagingStudy.series.instance.id".to_string(), "1".to_string());
    map.insert(
        "ImagingStudy.series.instance.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ImagingStudy.series.instance.number".to_string(), "1".to_string());
    map.insert("ImagingStudy.series.instance.sopClass".to_string(), "1".to_string());
    map.insert("ImagingStudy.series.instance.title".to_string(), "1".to_string());
    map.insert("ImagingStudy.series.instance.uid".to_string(), "1".to_string());
    map.insert("ImagingStudy.series.laterality".to_string(), "1".to_string());
    map.insert("ImagingStudy.series.modality".to_string(), "1".to_string());
    map.insert("ImagingStudy.series.modifierExtension".to_string(), "*".to_string());
    map.insert("ImagingStudy.series.number".to_string(), "1".to_string());
    map.insert("ImagingStudy.series.numberOfInstances".to_string(), "1".to_string());
    map.insert("ImagingStudy.series.performer".to_string(), "*".to_string());
    map.insert("ImagingStudy.series.performer.actor".to_string(), "1".to_string());
    map.insert("ImagingStudy.series.performer.extension".to_string(), "*".to_string());
    map.insert("ImagingStudy.series.performer.function".to_string(), "1".to_string());
    map.insert("ImagingStudy.series.performer.id".to_string(), "1".to_string());
    map.insert(
        "ImagingStudy.series.performer.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ImagingStudy.series.specimen".to_string(), "*".to_string());
    map.insert("ImagingStudy.series.started".to_string(), "1".to_string());
    map.insert("ImagingStudy.series.uid".to_string(), "1".to_string());
    map.insert("ImagingStudy.started".to_string(), "1".to_string());
    map.insert("ImagingStudy.status".to_string(), "1".to_string());
    map.insert("ImagingStudy.subject".to_string(), "1".to_string());
    map.insert("ImagingStudy.text".to_string(), "1".to_string());
    map.insert("Immunization.contained".to_string(), "*".to_string());
    map.insert("Immunization.doseQuantity".to_string(), "1".to_string());
    map.insert("Immunization.education".to_string(), "*".to_string());
    map.insert("Immunization.education.documentType".to_string(), "1".to_string());
    map.insert("Immunization.education.extension".to_string(), "*".to_string());
    map.insert("Immunization.education.id".to_string(), "1".to_string());
    map.insert("Immunization.education.modifierExtension".to_string(), "*".to_string());
    map.insert("Immunization.education.presentationDate".to_string(), "1".to_string());
    map.insert("Immunization.education.publicationDate".to_string(), "1".to_string());
    map.insert("Immunization.education.reference".to_string(), "1".to_string());
    map.insert("Immunization.encounter".to_string(), "1".to_string());
    map.insert("Immunization.expirationDate".to_string(), "1".to_string());
    map.insert("Immunization.extension".to_string(), "*".to_string());
    map.insert("Immunization.fundingSource".to_string(), "1".to_string());
    map.insert("Immunization.id".to_string(), "1".to_string());
    map.insert("Immunization.identifier".to_string(), "*".to_string());
    map.insert("Immunization.implicitRules".to_string(), "1".to_string());
    map.insert("Immunization.isSubpotent".to_string(), "1".to_string());
    map.insert("Immunization.language".to_string(), "1".to_string());
    map.insert("Immunization.location".to_string(), "1".to_string());
    map.insert("Immunization.lotNumber".to_string(), "1".to_string());
    map.insert("Immunization.manufacturer".to_string(), "1".to_string());
    map.insert("Immunization.meta".to_string(), "1".to_string());
    map.insert("Immunization.modifierExtension".to_string(), "*".to_string());
    map.insert("Immunization.note".to_string(), "*".to_string());
    map.insert("Immunization.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("Immunization.occurrenceString".to_string(), "1".to_string());
    map.insert("Immunization.patient".to_string(), "1".to_string());
    map.insert("Immunization.performer".to_string(), "*".to_string());
    map.insert("Immunization.performer.actor".to_string(), "1".to_string());
    map.insert("Immunization.performer.extension".to_string(), "*".to_string());
    map.insert("Immunization.performer.function".to_string(), "1".to_string());
    map.insert("Immunization.performer.id".to_string(), "1".to_string());
    map.insert("Immunization.performer.modifierExtension".to_string(), "*".to_string());
    map.insert("Immunization.primarySource".to_string(), "1".to_string());
    map.insert("Immunization.programEligibility".to_string(), "*".to_string());
    map.insert("Immunization.protocolApplied".to_string(), "*".to_string());
    map.insert("Immunization.protocolApplied.authority".to_string(), "1".to_string());
    map.insert(
        "Immunization.protocolApplied.doseNumberPositiveInt".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Immunization.protocolApplied.doseNumberString".to_string(),
        "1".to_string(),
    );
    map.insert("Immunization.protocolApplied.extension".to_string(), "*".to_string());
    map.insert("Immunization.protocolApplied.id".to_string(), "1".to_string());
    map.insert(
        "Immunization.protocolApplied.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Immunization.protocolApplied.series".to_string(), "1".to_string());
    map.insert(
        "Immunization.protocolApplied.seriesDosesPositiveInt".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Immunization.protocolApplied.seriesDosesString".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Immunization.protocolApplied.targetDisease".to_string(),
        "*".to_string(),
    );
    map.insert("Immunization.reaction".to_string(), "*".to_string());
    map.insert("Immunization.reaction.date".to_string(), "1".to_string());
    map.insert("Immunization.reaction.detail".to_string(), "1".to_string());
    map.insert("Immunization.reaction.extension".to_string(), "*".to_string());
    map.insert("Immunization.reaction.id".to_string(), "1".to_string());
    map.insert("Immunization.reaction.modifierExtension".to_string(), "*".to_string());
    map.insert("Immunization.reaction.reported".to_string(), "1".to_string());
    map.insert("Immunization.reasonCode".to_string(), "*".to_string());
    map.insert("Immunization.reasonReference".to_string(), "*".to_string());
    map.insert("Immunization.recorded".to_string(), "1".to_string());
    map.insert("Immunization.reportOrigin".to_string(), "1".to_string());
    map.insert("Immunization.route".to_string(), "1".to_string());
    map.insert("Immunization.site".to_string(), "1".to_string());
    map.insert("Immunization.status".to_string(), "1".to_string());
    map.insert("Immunization.statusReason".to_string(), "1".to_string());
    map.insert("Immunization.subpotentReason".to_string(), "*".to_string());
    map.insert("Immunization.text".to_string(), "1".to_string());
    map.insert("Immunization.vaccineCode".to_string(), "1".to_string());
    map.insert("ImmunizationEvaluation.authority".to_string(), "1".to_string());
    map.insert("ImmunizationEvaluation.contained".to_string(), "*".to_string());
    map.insert("ImmunizationEvaluation.date".to_string(), "1".to_string());
    map.insert("ImmunizationEvaluation.description".to_string(), "1".to_string());
    map.insert(
        "ImmunizationEvaluation.doseNumberPositiveInt".to_string(),
        "1".to_string(),
    );
    map.insert("ImmunizationEvaluation.doseNumberString".to_string(), "1".to_string());
    map.insert("ImmunizationEvaluation.doseStatus".to_string(), "1".to_string());
    map.insert("ImmunizationEvaluation.doseStatusReason".to_string(), "*".to_string());
    map.insert("ImmunizationEvaluation.extension".to_string(), "*".to_string());
    map.insert("ImmunizationEvaluation.id".to_string(), "1".to_string());
    map.insert("ImmunizationEvaluation.identifier".to_string(), "*".to_string());
    map.insert("ImmunizationEvaluation.immunizationEvent".to_string(), "1".to_string());
    map.insert("ImmunizationEvaluation.implicitRules".to_string(), "1".to_string());
    map.insert("ImmunizationEvaluation.language".to_string(), "1".to_string());
    map.insert("ImmunizationEvaluation.meta".to_string(), "1".to_string());
    map.insert("ImmunizationEvaluation.modifierExtension".to_string(), "*".to_string());
    map.insert("ImmunizationEvaluation.patient".to_string(), "1".to_string());
    map.insert("ImmunizationEvaluation.series".to_string(), "1".to_string());
    map.insert(
        "ImmunizationEvaluation.seriesDosesPositiveInt".to_string(),
        "1".to_string(),
    );
    map.insert("ImmunizationEvaluation.seriesDosesString".to_string(), "1".to_string());
    map.insert("ImmunizationEvaluation.status".to_string(), "1".to_string());
    map.insert("ImmunizationEvaluation.targetDisease".to_string(), "1".to_string());
    map.insert("ImmunizationEvaluation.text".to_string(), "1".to_string());
    map.insert("ImmunizationRecommendation.authority".to_string(), "1".to_string());
    map.insert("ImmunizationRecommendation.contained".to_string(), "*".to_string());
    map.insert("ImmunizationRecommendation.date".to_string(), "1".to_string());
    map.insert("ImmunizationRecommendation.extension".to_string(), "*".to_string());
    map.insert("ImmunizationRecommendation.id".to_string(), "1".to_string());
    map.insert("ImmunizationRecommendation.identifier".to_string(), "*".to_string());
    map.insert("ImmunizationRecommendation.implicitRules".to_string(), "1".to_string());
    map.insert("ImmunizationRecommendation.language".to_string(), "1".to_string());
    map.insert("ImmunizationRecommendation.meta".to_string(), "1".to_string());
    map.insert(
        "ImmunizationRecommendation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ImmunizationRecommendation.patient".to_string(), "1".to_string());
    map.insert("ImmunizationRecommendation.recommendation".to_string(), "*".to_string());
    map.insert(
        "ImmunizationRecommendation.recommendation.contraindicatedVaccineCode"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.dateCriterion".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.dateCriterion.code".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.dateCriterion.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.dateCriterion.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.dateCriterion.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.dateCriterion.value".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.doseNumberPositiveInt".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.doseNumberString".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.forecastReason".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.forecastStatus".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.series".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.seriesDosesPositiveInt".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.seriesDosesString".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.supportingImmunization".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.supportingPatientInformation"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.targetDisease".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.vaccineCode".to_string(),
        "*".to_string(),
    );
    map.insert("ImmunizationRecommendation.text".to_string(), "1".to_string());
    map.insert("ImplementationGuide.contact".to_string(), "*".to_string());
    map.insert("ImplementationGuide.contained".to_string(), "*".to_string());
    map.insert("ImplementationGuide.copyright".to_string(), "1".to_string());
    map.insert("ImplementationGuide.date".to_string(), "1".to_string());
    map.insert("ImplementationGuide.definition".to_string(), "1".to_string());
    map.insert("ImplementationGuide.definition.extension".to_string(), "*".to_string());
    map.insert("ImplementationGuide.definition.grouping".to_string(), "*".to_string());
    map.insert(
        "ImplementationGuide.definition.grouping.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.grouping.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.grouping.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.grouping.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.grouping.name".to_string(),
        "1".to_string(),
    );
    map.insert("ImplementationGuide.definition.id".to_string(), "1".to_string());
    map.insert(
        "ImplementationGuide.definition.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ImplementationGuide.definition.page".to_string(), "1".to_string());
    map.insert(
        "ImplementationGuide.definition.page.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.page.generation".to_string(),
        "1".to_string(),
    );
    map.insert("ImplementationGuide.definition.page.id".to_string(), "1".to_string());
    map.insert(
        "ImplementationGuide.definition.page.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.page.nameReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.page.nameUrl".to_string(),
        "1".to_string(),
    );
    map.insert("ImplementationGuide.definition.page.page".to_string(), "*".to_string());
    map.insert("ImplementationGuide.definition.page.title".to_string(), "1".to_string());
    map.insert("ImplementationGuide.definition.parameter".to_string(), "*".to_string());
    map.insert(
        "ImplementationGuide.definition.parameter.code".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.parameter.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.parameter.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.parameter.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.parameter.value".to_string(),
        "1".to_string(),
    );
    map.insert("ImplementationGuide.definition.resource".to_string(), "*".to_string());
    map.insert(
        "ImplementationGuide.definition.resource.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.resource.exampleBoolean".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.resource.exampleCanonical".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.resource.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.resource.fhirVersion".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.resource.groupingId".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.resource.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.resource.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.resource.name".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.resource.reference".to_string(),
        "1".to_string(),
    );
    map.insert("ImplementationGuide.definition.template".to_string(), "*".to_string());
    map.insert(
        "ImplementationGuide.definition.template.code".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.template.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.template.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.template.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.template.scope".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.definition.template.source".to_string(),
        "1".to_string(),
    );
    map.insert("ImplementationGuide.dependsOn".to_string(), "*".to_string());
    map.insert("ImplementationGuide.dependsOn.extension".to_string(), "*".to_string());
    map.insert("ImplementationGuide.dependsOn.id".to_string(), "1".to_string());
    map.insert(
        "ImplementationGuide.dependsOn.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ImplementationGuide.dependsOn.packageId".to_string(), "1".to_string());
    map.insert("ImplementationGuide.dependsOn.uri".to_string(), "1".to_string());
    map.insert("ImplementationGuide.dependsOn.version".to_string(), "1".to_string());
    map.insert("ImplementationGuide.description".to_string(), "1".to_string());
    map.insert("ImplementationGuide.experimental".to_string(), "1".to_string());
    map.insert("ImplementationGuide.extension".to_string(), "*".to_string());
    map.insert("ImplementationGuide.fhirVersion".to_string(), "*".to_string());
    map.insert("ImplementationGuide.global".to_string(), "*".to_string());
    map.insert("ImplementationGuide.global.extension".to_string(), "*".to_string());
    map.insert("ImplementationGuide.global.id".to_string(), "1".to_string());
    map.insert(
        "ImplementationGuide.global.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ImplementationGuide.global.profile".to_string(), "1".to_string());
    map.insert("ImplementationGuide.global.type".to_string(), "1".to_string());
    map.insert("ImplementationGuide.id".to_string(), "1".to_string());
    map.insert("ImplementationGuide.implicitRules".to_string(), "1".to_string());
    map.insert("ImplementationGuide.jurisdiction".to_string(), "*".to_string());
    map.insert("ImplementationGuide.language".to_string(), "1".to_string());
    map.insert("ImplementationGuide.license".to_string(), "1".to_string());
    map.insert("ImplementationGuide.manifest".to_string(), "1".to_string());
    map.insert("ImplementationGuide.manifest.extension".to_string(), "*".to_string());
    map.insert("ImplementationGuide.manifest.id".to_string(), "1".to_string());
    map.insert("ImplementationGuide.manifest.image".to_string(), "*".to_string());
    map.insert(
        "ImplementationGuide.manifest.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ImplementationGuide.manifest.other".to_string(), "*".to_string());
    map.insert("ImplementationGuide.manifest.page".to_string(), "*".to_string());
    map.insert("ImplementationGuide.manifest.page.anchor".to_string(), "*".to_string());
    map.insert(
        "ImplementationGuide.manifest.page.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ImplementationGuide.manifest.page.id".to_string(), "1".to_string());
    map.insert(
        "ImplementationGuide.manifest.page.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ImplementationGuide.manifest.page.name".to_string(), "1".to_string());
    map.insert("ImplementationGuide.manifest.page.title".to_string(), "1".to_string());
    map.insert("ImplementationGuide.manifest.rendering".to_string(), "1".to_string());
    map.insert("ImplementationGuide.manifest.resource".to_string(), "*".to_string());
    map.insert(
        "ImplementationGuide.manifest.resource.exampleBoolean".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.manifest.resource.exampleCanonical".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.manifest.resource.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ImplementationGuide.manifest.resource.id".to_string(), "1".to_string());
    map.insert(
        "ImplementationGuide.manifest.resource.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImplementationGuide.manifest.resource.reference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.manifest.resource.relativePath".to_string(),
        "1".to_string(),
    );
    map.insert("ImplementationGuide.meta".to_string(), "1".to_string());
    map.insert("ImplementationGuide.modifierExtension".to_string(), "*".to_string());
    map.insert("ImplementationGuide.name".to_string(), "1".to_string());
    map.insert("ImplementationGuide.packageId".to_string(), "1".to_string());
    map.insert("ImplementationGuide.publisher".to_string(), "1".to_string());
    map.insert("ImplementationGuide.status".to_string(), "1".to_string());
    map.insert("ImplementationGuide.text".to_string(), "1".to_string());
    map.insert("ImplementationGuide.title".to_string(), "1".to_string());
    map.insert("ImplementationGuide.url".to_string(), "1".to_string());
    map.insert("ImplementationGuide.useContext".to_string(), "*".to_string());
    map.insert("ImplementationGuide.version".to_string(), "1".to_string());
    map.insert("InsurancePlan.administeredBy".to_string(), "1".to_string());
    map.insert("InsurancePlan.alias".to_string(), "*".to_string());
    map.insert("InsurancePlan.contact".to_string(), "*".to_string());
    map.insert("InsurancePlan.contact.address".to_string(), "1".to_string());
    map.insert("InsurancePlan.contact.extension".to_string(), "*".to_string());
    map.insert("InsurancePlan.contact.id".to_string(), "1".to_string());
    map.insert("InsurancePlan.contact.modifierExtension".to_string(), "*".to_string());
    map.insert("InsurancePlan.contact.name".to_string(), "1".to_string());
    map.insert("InsurancePlan.contact.purpose".to_string(), "1".to_string());
    map.insert("InsurancePlan.contact.telecom".to_string(), "*".to_string());
    map.insert("InsurancePlan.contained".to_string(), "*".to_string());
    map.insert("InsurancePlan.coverage".to_string(), "*".to_string());
    map.insert("InsurancePlan.coverage.benefit".to_string(), "*".to_string());
    map.insert("InsurancePlan.coverage.benefit.extension".to_string(), "*".to_string());
    map.insert("InsurancePlan.coverage.benefit.id".to_string(), "1".to_string());
    map.insert("InsurancePlan.coverage.benefit.limit".to_string(), "*".to_string());
    map.insert("InsurancePlan.coverage.benefit.limit.code".to_string(), "1".to_string());
    map.insert(
        "InsurancePlan.coverage.benefit.limit.extension".to_string(),
        "*".to_string(),
    );
    map.insert("InsurancePlan.coverage.benefit.limit.id".to_string(), "1".to_string());
    map.insert(
        "InsurancePlan.coverage.benefit.limit.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "InsurancePlan.coverage.benefit.limit.value".to_string(),
        "1".to_string(),
    );
    map.insert(
        "InsurancePlan.coverage.benefit.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "InsurancePlan.coverage.benefit.requirement".to_string(),
        "1".to_string(),
    );
    map.insert("InsurancePlan.coverage.benefit.type".to_string(), "1".to_string());
    map.insert("InsurancePlan.coverage.extension".to_string(), "*".to_string());
    map.insert("InsurancePlan.coverage.id".to_string(), "1".to_string());
    map.insert("InsurancePlan.coverage.modifierExtension".to_string(), "*".to_string());
    map.insert("InsurancePlan.coverage.network".to_string(), "*".to_string());
    map.insert("InsurancePlan.coverage.type".to_string(), "1".to_string());
    map.insert("InsurancePlan.coverageArea".to_string(), "*".to_string());
    map.insert("InsurancePlan.endpoint".to_string(), "*".to_string());
    map.insert("InsurancePlan.extension".to_string(), "*".to_string());
    map.insert("InsurancePlan.id".to_string(), "1".to_string());
    map.insert("InsurancePlan.identifier".to_string(), "*".to_string());
    map.insert("InsurancePlan.implicitRules".to_string(), "1".to_string());
    map.insert("InsurancePlan.language".to_string(), "1".to_string());
    map.insert("InsurancePlan.meta".to_string(), "1".to_string());
    map.insert("InsurancePlan.modifierExtension".to_string(), "*".to_string());
    map.insert("InsurancePlan.name".to_string(), "1".to_string());
    map.insert("InsurancePlan.network".to_string(), "*".to_string());
    map.insert("InsurancePlan.ownedBy".to_string(), "1".to_string());
    map.insert("InsurancePlan.period".to_string(), "1".to_string());
    map.insert("InsurancePlan.plan".to_string(), "*".to_string());
    map.insert("InsurancePlan.plan.coverageArea".to_string(), "*".to_string());
    map.insert("InsurancePlan.plan.extension".to_string(), "*".to_string());
    map.insert("InsurancePlan.plan.generalCost".to_string(), "*".to_string());
    map.insert("InsurancePlan.plan.generalCost.comment".to_string(), "1".to_string());
    map.insert("InsurancePlan.plan.generalCost.cost".to_string(), "1".to_string());
    map.insert("InsurancePlan.plan.generalCost.extension".to_string(), "*".to_string());
    map.insert("InsurancePlan.plan.generalCost.groupSize".to_string(), "1".to_string());
    map.insert("InsurancePlan.plan.generalCost.id".to_string(), "1".to_string());
    map.insert(
        "InsurancePlan.plan.generalCost.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("InsurancePlan.plan.generalCost.type".to_string(), "1".to_string());
    map.insert("InsurancePlan.plan.id".to_string(), "1".to_string());
    map.insert("InsurancePlan.plan.identifier".to_string(), "*".to_string());
    map.insert("InsurancePlan.plan.modifierExtension".to_string(), "*".to_string());
    map.insert("InsurancePlan.plan.network".to_string(), "*".to_string());
    map.insert("InsurancePlan.plan.specificCost".to_string(), "*".to_string());
    map.insert("InsurancePlan.plan.specificCost.benefit".to_string(), "*".to_string());
    map.insert(
        "InsurancePlan.plan.specificCost.benefit.cost".to_string(),
        "*".to_string(),
    );
    map.insert(
        "InsurancePlan.plan.specificCost.benefit.cost.applicability".to_string(),
        "1".to_string(),
    );
    map.insert(
        "InsurancePlan.plan.specificCost.benefit.cost.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "InsurancePlan.plan.specificCost.benefit.cost.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "InsurancePlan.plan.specificCost.benefit.cost.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "InsurancePlan.plan.specificCost.benefit.cost.qualifiers".to_string(),
        "*".to_string(),
    );
    map.insert(
        "InsurancePlan.plan.specificCost.benefit.cost.type".to_string(),
        "1".to_string(),
    );
    map.insert(
        "InsurancePlan.plan.specificCost.benefit.cost.value".to_string(),
        "1".to_string(),
    );
    map.insert(
        "InsurancePlan.plan.specificCost.benefit.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "InsurancePlan.plan.specificCost.benefit.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "InsurancePlan.plan.specificCost.benefit.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "InsurancePlan.plan.specificCost.benefit.type".to_string(),
        "1".to_string(),
    );
    map.insert("InsurancePlan.plan.specificCost.category".to_string(), "1".to_string());
    map.insert("InsurancePlan.plan.specificCost.extension".to_string(), "*".to_string());
    map.insert("InsurancePlan.plan.specificCost.id".to_string(), "1".to_string());
    map.insert(
        "InsurancePlan.plan.specificCost.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("InsurancePlan.plan.type".to_string(), "1".to_string());
    map.insert("InsurancePlan.status".to_string(), "1".to_string());
    map.insert("InsurancePlan.text".to_string(), "1".to_string());
    map.insert("InsurancePlan.type".to_string(), "*".to_string());
    map.insert("Invoice.account".to_string(), "1".to_string());
    map.insert("Invoice.cancelledReason".to_string(), "1".to_string());
    map.insert("Invoice.contained".to_string(), "*".to_string());
    map.insert("Invoice.date".to_string(), "1".to_string());
    map.insert("Invoice.extension".to_string(), "*".to_string());
    map.insert("Invoice.id".to_string(), "1".to_string());
    map.insert("Invoice.identifier".to_string(), "*".to_string());
    map.insert("Invoice.implicitRules".to_string(), "1".to_string());
    map.insert("Invoice.issuer".to_string(), "1".to_string());
    map.insert("Invoice.language".to_string(), "1".to_string());
    map.insert("Invoice.lineItem".to_string(), "*".to_string());
    map.insert(
        "Invoice.lineItem.chargeItemCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("Invoice.lineItem.chargeItemReference".to_string(), "1".to_string());
    map.insert("Invoice.lineItem.extension".to_string(), "*".to_string());
    map.insert("Invoice.lineItem.id".to_string(), "1".to_string());
    map.insert("Invoice.lineItem.modifierExtension".to_string(), "*".to_string());
    map.insert("Invoice.lineItem.priceComponent".to_string(), "*".to_string());
    map.insert("Invoice.lineItem.priceComponent.amount".to_string(), "1".to_string());
    map.insert("Invoice.lineItem.priceComponent.code".to_string(), "1".to_string());
    map.insert("Invoice.lineItem.priceComponent.extension".to_string(), "*".to_string());
    map.insert("Invoice.lineItem.priceComponent.factor".to_string(), "1".to_string());
    map.insert("Invoice.lineItem.priceComponent.id".to_string(), "1".to_string());
    map.insert(
        "Invoice.lineItem.priceComponent.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Invoice.lineItem.priceComponent.type".to_string(), "1".to_string());
    map.insert("Invoice.lineItem.sequence".to_string(), "1".to_string());
    map.insert("Invoice.meta".to_string(), "1".to_string());
    map.insert("Invoice.modifierExtension".to_string(), "*".to_string());
    map.insert("Invoice.note".to_string(), "*".to_string());
    map.insert("Invoice.participant".to_string(), "*".to_string());
    map.insert("Invoice.participant.actor".to_string(), "1".to_string());
    map.insert("Invoice.participant.extension".to_string(), "*".to_string());
    map.insert("Invoice.participant.id".to_string(), "1".to_string());
    map.insert("Invoice.participant.modifierExtension".to_string(), "*".to_string());
    map.insert("Invoice.participant.role".to_string(), "1".to_string());
    map.insert("Invoice.paymentTerms".to_string(), "1".to_string());
    map.insert("Invoice.recipient".to_string(), "1".to_string());
    map.insert("Invoice.status".to_string(), "1".to_string());
    map.insert("Invoice.subject".to_string(), "1".to_string());
    map.insert("Invoice.text".to_string(), "1".to_string());
    map.insert("Invoice.totalGross".to_string(), "1".to_string());
    map.insert("Invoice.totalNet".to_string(), "1".to_string());
    map.insert("Invoice.totalPriceComponent".to_string(), "*".to_string());
    map.insert("Invoice.type".to_string(), "1".to_string());
    map.insert("Library.approvalDate".to_string(), "1".to_string());
    map.insert("Library.author".to_string(), "*".to_string());
    map.insert("Library.contact".to_string(), "*".to_string());
    map.insert("Library.contained".to_string(), "*".to_string());
    map.insert("Library.content".to_string(), "*".to_string());
    map.insert("Library.copyright".to_string(), "1".to_string());
    map.insert("Library.dataRequirement".to_string(), "*".to_string());
    map.insert("Library.date".to_string(), "1".to_string());
    map.insert("Library.description".to_string(), "1".to_string());
    map.insert("Library.editor".to_string(), "*".to_string());
    map.insert("Library.effectivePeriod".to_string(), "1".to_string());
    map.insert("Library.endorser".to_string(), "*".to_string());
    map.insert("Library.experimental".to_string(), "1".to_string());
    map.insert("Library.extension".to_string(), "*".to_string());
    map.insert("Library.id".to_string(), "1".to_string());
    map.insert("Library.identifier".to_string(), "*".to_string());
    map.insert("Library.implicitRules".to_string(), "1".to_string());
    map.insert("Library.jurisdiction".to_string(), "*".to_string());
    map.insert("Library.language".to_string(), "1".to_string());
    map.insert("Library.lastReviewDate".to_string(), "1".to_string());
    map.insert("Library.meta".to_string(), "1".to_string());
    map.insert("Library.modifierExtension".to_string(), "*".to_string());
    map.insert("Library.name".to_string(), "1".to_string());
    map.insert("Library.parameter".to_string(), "*".to_string());
    map.insert("Library.publisher".to_string(), "1".to_string());
    map.insert("Library.purpose".to_string(), "1".to_string());
    map.insert("Library.relatedArtifact".to_string(), "*".to_string());
    map.insert("Library.reviewer".to_string(), "*".to_string());
    map.insert("Library.status".to_string(), "1".to_string());
    map.insert("Library.subjectCodeableConcept".to_string(), "1".to_string());
    map.insert("Library.subjectReference".to_string(), "1".to_string());
    map.insert("Library.subtitle".to_string(), "1".to_string());
    map.insert("Library.text".to_string(), "1".to_string());
    map.insert("Library.title".to_string(), "1".to_string());
    map.insert("Library.topic".to_string(), "*".to_string());
    map.insert("Library.type".to_string(), "1".to_string());
    map.insert("Library.url".to_string(), "1".to_string());
    map.insert("Library.usage".to_string(), "1".to_string());
    map.insert("Library.useContext".to_string(), "*".to_string());
    map.insert("Library.version".to_string(), "1".to_string());
    map.insert("Linkage.active".to_string(), "1".to_string());
    map.insert("Linkage.author".to_string(), "1".to_string());
    map.insert("Linkage.contained".to_string(), "*".to_string());
    map.insert("Linkage.extension".to_string(), "*".to_string());
    map.insert("Linkage.id".to_string(), "1".to_string());
    map.insert("Linkage.implicitRules".to_string(), "1".to_string());
    map.insert("Linkage.item".to_string(), "*".to_string());
    map.insert("Linkage.item.extension".to_string(), "*".to_string());
    map.insert("Linkage.item.id".to_string(), "1".to_string());
    map.insert("Linkage.item.modifierExtension".to_string(), "*".to_string());
    map.insert("Linkage.item.resource".to_string(), "1".to_string());
    map.insert("Linkage.item.type".to_string(), "1".to_string());
    map.insert("Linkage.language".to_string(), "1".to_string());
    map.insert("Linkage.meta".to_string(), "1".to_string());
    map.insert("Linkage.modifierExtension".to_string(), "*".to_string());
    map.insert("Linkage.text".to_string(), "1".to_string());
    map.insert("List.code".to_string(), "1".to_string());
    map.insert("List.contained".to_string(), "*".to_string());
    map.insert("List.date".to_string(), "1".to_string());
    map.insert("List.emptyReason".to_string(), "1".to_string());
    map.insert("List.encounter".to_string(), "1".to_string());
    map.insert("List.entry".to_string(), "*".to_string());
    map.insert("List.entry.date".to_string(), "1".to_string());
    map.insert("List.entry.deleted".to_string(), "1".to_string());
    map.insert("List.entry.extension".to_string(), "*".to_string());
    map.insert("List.entry.flag".to_string(), "1".to_string());
    map.insert("List.entry.id".to_string(), "1".to_string());
    map.insert("List.entry.item".to_string(), "1".to_string());
    map.insert("List.entry.modifierExtension".to_string(), "*".to_string());
    map.insert("List.extension".to_string(), "*".to_string());
    map.insert("List.id".to_string(), "1".to_string());
    map.insert("List.identifier".to_string(), "*".to_string());
    map.insert("List.implicitRules".to_string(), "1".to_string());
    map.insert("List.language".to_string(), "1".to_string());
    map.insert("List.meta".to_string(), "1".to_string());
    map.insert("List.mode".to_string(), "1".to_string());
    map.insert("List.modifierExtension".to_string(), "*".to_string());
    map.insert("List.note".to_string(), "*".to_string());
    map.insert("List.orderedBy".to_string(), "1".to_string());
    map.insert("List.source".to_string(), "1".to_string());
    map.insert("List.status".to_string(), "1".to_string());
    map.insert("List.subject".to_string(), "1".to_string());
    map.insert("List.text".to_string(), "1".to_string());
    map.insert("List.title".to_string(), "1".to_string());
    map.insert("Location.address".to_string(), "1".to_string());
    map.insert("Location.alias".to_string(), "*".to_string());
    map.insert("Location.availabilityExceptions".to_string(), "1".to_string());
    map.insert("Location.contained".to_string(), "*".to_string());
    map.insert("Location.description".to_string(), "1".to_string());
    map.insert("Location.endpoint".to_string(), "*".to_string());
    map.insert("Location.extension".to_string(), "*".to_string());
    map.insert("Location.hoursOfOperation".to_string(), "*".to_string());
    map.insert("Location.hoursOfOperation.allDay".to_string(), "1".to_string());
    map.insert("Location.hoursOfOperation.closingTime".to_string(), "1".to_string());
    map.insert("Location.hoursOfOperation.daysOfWeek".to_string(), "*".to_string());
    map.insert("Location.hoursOfOperation.extension".to_string(), "*".to_string());
    map.insert("Location.hoursOfOperation.id".to_string(), "1".to_string());
    map.insert(
        "Location.hoursOfOperation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Location.hoursOfOperation.openingTime".to_string(), "1".to_string());
    map.insert("Location.id".to_string(), "1".to_string());
    map.insert("Location.identifier".to_string(), "*".to_string());
    map.insert("Location.implicitRules".to_string(), "1".to_string());
    map.insert("Location.language".to_string(), "1".to_string());
    map.insert("Location.managingOrganization".to_string(), "1".to_string());
    map.insert("Location.meta".to_string(), "1".to_string());
    map.insert("Location.mode".to_string(), "1".to_string());
    map.insert("Location.modifierExtension".to_string(), "*".to_string());
    map.insert("Location.name".to_string(), "1".to_string());
    map.insert("Location.operationalStatus".to_string(), "1".to_string());
    map.insert("Location.partOf".to_string(), "1".to_string());
    map.insert("Location.physicalType".to_string(), "1".to_string());
    map.insert("Location.position".to_string(), "1".to_string());
    map.insert("Location.position.altitude".to_string(), "1".to_string());
    map.insert("Location.position.extension".to_string(), "*".to_string());
    map.insert("Location.position.id".to_string(), "1".to_string());
    map.insert("Location.position.latitude".to_string(), "1".to_string());
    map.insert("Location.position.longitude".to_string(), "1".to_string());
    map.insert("Location.position.modifierExtension".to_string(), "*".to_string());
    map.insert("Location.status".to_string(), "1".to_string());
    map.insert("Location.telecom".to_string(), "*".to_string());
    map.insert("Location.text".to_string(), "1".to_string());
    map.insert("Location.type".to_string(), "*".to_string());
    map.insert("MarketingStatus.country".to_string(), "1".to_string());
    map.insert("MarketingStatus.dateRange".to_string(), "1".to_string());
    map.insert("MarketingStatus.extension".to_string(), "*".to_string());
    map.insert("MarketingStatus.id".to_string(), "1".to_string());
    map.insert("MarketingStatus.jurisdiction".to_string(), "1".to_string());
    map.insert("MarketingStatus.modifierExtension".to_string(), "*".to_string());
    map.insert("MarketingStatus.restoreDate".to_string(), "1".to_string());
    map.insert("MarketingStatus.status".to_string(), "1".to_string());
    map.insert("Measure.approvalDate".to_string(), "1".to_string());
    map.insert("Measure.author".to_string(), "*".to_string());
    map.insert("Measure.clinicalRecommendationStatement".to_string(), "1".to_string());
    map.insert("Measure.compositeScoring".to_string(), "1".to_string());
    map.insert("Measure.contact".to_string(), "*".to_string());
    map.insert("Measure.contained".to_string(), "*".to_string());
    map.insert("Measure.copyright".to_string(), "1".to_string());
    map.insert("Measure.date".to_string(), "1".to_string());
    map.insert("Measure.definition".to_string(), "*".to_string());
    map.insert("Measure.description".to_string(), "1".to_string());
    map.insert("Measure.disclaimer".to_string(), "1".to_string());
    map.insert("Measure.editor".to_string(), "*".to_string());
    map.insert("Measure.effectivePeriod".to_string(), "1".to_string());
    map.insert("Measure.endorser".to_string(), "*".to_string());
    map.insert("Measure.experimental".to_string(), "1".to_string());
    map.insert("Measure.extension".to_string(), "*".to_string());
    map.insert("Measure.group".to_string(), "*".to_string());
    map.insert("Measure.group.code".to_string(), "1".to_string());
    map.insert("Measure.group.description".to_string(), "1".to_string());
    map.insert("Measure.group.extension".to_string(), "*".to_string());
    map.insert("Measure.group.id".to_string(), "1".to_string());
    map.insert("Measure.group.modifierExtension".to_string(), "*".to_string());
    map.insert("Measure.group.population".to_string(), "*".to_string());
    map.insert("Measure.group.population.code".to_string(), "1".to_string());
    map.insert("Measure.group.population.criteria".to_string(), "1".to_string());
    map.insert("Measure.group.population.description".to_string(), "1".to_string());
    map.insert("Measure.group.population.extension".to_string(), "*".to_string());
    map.insert("Measure.group.population.id".to_string(), "1".to_string());
    map.insert(
        "Measure.group.population.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Measure.group.stratifier".to_string(), "*".to_string());
    map.insert("Measure.group.stratifier.code".to_string(), "1".to_string());
    map.insert("Measure.group.stratifier.component".to_string(), "*".to_string());
    map.insert("Measure.group.stratifier.component.code".to_string(), "1".to_string());
    map.insert(
        "Measure.group.stratifier.component.criteria".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Measure.group.stratifier.component.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Measure.group.stratifier.component.extension".to_string(),
        "*".to_string(),
    );
    map.insert("Measure.group.stratifier.component.id".to_string(), "1".to_string());
    map.insert(
        "Measure.group.stratifier.component.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Measure.group.stratifier.criteria".to_string(), "1".to_string());
    map.insert("Measure.group.stratifier.description".to_string(), "1".to_string());
    map.insert("Measure.group.stratifier.extension".to_string(), "*".to_string());
    map.insert("Measure.group.stratifier.id".to_string(), "1".to_string());
    map.insert(
        "Measure.group.stratifier.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Measure.guidance".to_string(), "1".to_string());
    map.insert("Measure.id".to_string(), "1".to_string());
    map.insert("Measure.identifier".to_string(), "*".to_string());
    map.insert("Measure.implicitRules".to_string(), "1".to_string());
    map.insert("Measure.improvementNotation".to_string(), "1".to_string());
    map.insert("Measure.jurisdiction".to_string(), "*".to_string());
    map.insert("Measure.language".to_string(), "1".to_string());
    map.insert("Measure.lastReviewDate".to_string(), "1".to_string());
    map.insert("Measure.library".to_string(), "*".to_string());
    map.insert("Measure.meta".to_string(), "1".to_string());
    map.insert("Measure.modifierExtension".to_string(), "*".to_string());
    map.insert("Measure.name".to_string(), "1".to_string());
    map.insert("Measure.publisher".to_string(), "1".to_string());
    map.insert("Measure.purpose".to_string(), "1".to_string());
    map.insert("Measure.rateAggregation".to_string(), "1".to_string());
    map.insert("Measure.rationale".to_string(), "1".to_string());
    map.insert("Measure.relatedArtifact".to_string(), "*".to_string());
    map.insert("Measure.reviewer".to_string(), "*".to_string());
    map.insert("Measure.riskAdjustment".to_string(), "1".to_string());
    map.insert("Measure.scoring".to_string(), "1".to_string());
    map.insert("Measure.status".to_string(), "1".to_string());
    map.insert("Measure.subjectCodeableConcept".to_string(), "1".to_string());
    map.insert("Measure.subjectReference".to_string(), "1".to_string());
    map.insert("Measure.subtitle".to_string(), "1".to_string());
    map.insert("Measure.supplementalData".to_string(), "*".to_string());
    map.insert("Measure.supplementalData.code".to_string(), "1".to_string());
    map.insert("Measure.supplementalData.criteria".to_string(), "1".to_string());
    map.insert("Measure.supplementalData.description".to_string(), "1".to_string());
    map.insert("Measure.supplementalData.extension".to_string(), "*".to_string());
    map.insert("Measure.supplementalData.id".to_string(), "1".to_string());
    map.insert(
        "Measure.supplementalData.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Measure.supplementalData.usage".to_string(), "*".to_string());
    map.insert("Measure.text".to_string(), "1".to_string());
    map.insert("Measure.title".to_string(), "1".to_string());
    map.insert("Measure.topic".to_string(), "*".to_string());
    map.insert("Measure.type".to_string(), "*".to_string());
    map.insert("Measure.url".to_string(), "1".to_string());
    map.insert("Measure.usage".to_string(), "1".to_string());
    map.insert("Measure.useContext".to_string(), "*".to_string());
    map.insert("Measure.version".to_string(), "1".to_string());
    map.insert("MeasureReport.contained".to_string(), "*".to_string());
    map.insert("MeasureReport.date".to_string(), "1".to_string());
    map.insert("MeasureReport.evaluatedResource".to_string(), "*".to_string());
    map.insert("MeasureReport.extension".to_string(), "*".to_string());
    map.insert("MeasureReport.group".to_string(), "*".to_string());
    map.insert("MeasureReport.group.code".to_string(), "1".to_string());
    map.insert("MeasureReport.group.extension".to_string(), "*".to_string());
    map.insert("MeasureReport.group.id".to_string(), "1".to_string());
    map.insert("MeasureReport.group.measureScore".to_string(), "1".to_string());
    map.insert("MeasureReport.group.modifierExtension".to_string(), "*".to_string());
    map.insert("MeasureReport.group.population".to_string(), "*".to_string());
    map.insert("MeasureReport.group.population.code".to_string(), "1".to_string());
    map.insert("MeasureReport.group.population.count".to_string(), "1".to_string());
    map.insert("MeasureReport.group.population.extension".to_string(), "*".to_string());
    map.insert("MeasureReport.group.population.id".to_string(), "1".to_string());
    map.insert(
        "MeasureReport.group.population.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MeasureReport.group.population.subjectResults".to_string(),
        "1".to_string(),
    );
    map.insert("MeasureReport.group.stratifier".to_string(), "*".to_string());
    map.insert("MeasureReport.group.stratifier.code".to_string(), "*".to_string());
    map.insert("MeasureReport.group.stratifier.extension".to_string(), "*".to_string());
    map.insert("MeasureReport.group.stratifier.id".to_string(), "1".to_string());
    map.insert(
        "MeasureReport.group.stratifier.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MeasureReport.group.stratifier.stratum".to_string(), "*".to_string());
    map.insert(
        "MeasureReport.group.stratifier.stratum.component".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.component.code".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.component.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.component.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.component.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.component.value".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.extension".to_string(),
        "*".to_string(),
    );
    map.insert("MeasureReport.group.stratifier.stratum.id".to_string(), "1".to_string());
    map.insert(
        "MeasureReport.group.stratifier.stratum.measureScore".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.population".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.population.code".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.population.count".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.population.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.population.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.population.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.population.subjectResults".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.value".to_string(),
        "1".to_string(),
    );
    map.insert("MeasureReport.id".to_string(), "1".to_string());
    map.insert("MeasureReport.identifier".to_string(), "*".to_string());
    map.insert("MeasureReport.implicitRules".to_string(), "1".to_string());
    map.insert("MeasureReport.improvementNotation".to_string(), "1".to_string());
    map.insert("MeasureReport.language".to_string(), "1".to_string());
    map.insert("MeasureReport.measure".to_string(), "1".to_string());
    map.insert("MeasureReport.meta".to_string(), "1".to_string());
    map.insert("MeasureReport.modifierExtension".to_string(), "*".to_string());
    map.insert("MeasureReport.period".to_string(), "1".to_string());
    map.insert("MeasureReport.reporter".to_string(), "1".to_string());
    map.insert("MeasureReport.status".to_string(), "1".to_string());
    map.insert("MeasureReport.subject".to_string(), "1".to_string());
    map.insert("MeasureReport.text".to_string(), "1".to_string());
    map.insert("MeasureReport.type".to_string(), "1".to_string());
    map.insert("Media.basedOn".to_string(), "*".to_string());
    map.insert("Media.bodySite".to_string(), "1".to_string());
    map.insert("Media.contained".to_string(), "*".to_string());
    map.insert("Media.content".to_string(), "1".to_string());
    map.insert("Media.createdDateTime".to_string(), "1".to_string());
    map.insert("Media.createdPeriod".to_string(), "1".to_string());
    map.insert("Media.device".to_string(), "1".to_string());
    map.insert("Media.deviceName".to_string(), "1".to_string());
    map.insert("Media.duration".to_string(), "1".to_string());
    map.insert("Media.encounter".to_string(), "1".to_string());
    map.insert("Media.extension".to_string(), "*".to_string());
    map.insert("Media.frames".to_string(), "1".to_string());
    map.insert("Media.height".to_string(), "1".to_string());
    map.insert("Media.id".to_string(), "1".to_string());
    map.insert("Media.identifier".to_string(), "*".to_string());
    map.insert("Media.implicitRules".to_string(), "1".to_string());
    map.insert("Media.issued".to_string(), "1".to_string());
    map.insert("Media.language".to_string(), "1".to_string());
    map.insert("Media.meta".to_string(), "1".to_string());
    map.insert("Media.modality".to_string(), "1".to_string());
    map.insert("Media.modifierExtension".to_string(), "*".to_string());
    map.insert("Media.note".to_string(), "*".to_string());
    map.insert("Media.operator".to_string(), "1".to_string());
    map.insert("Media.partOf".to_string(), "*".to_string());
    map.insert("Media.reasonCode".to_string(), "*".to_string());
    map.insert("Media.status".to_string(), "1".to_string());
    map.insert("Media.subject".to_string(), "1".to_string());
    map.insert("Media.text".to_string(), "1".to_string());
    map.insert("Media.type".to_string(), "1".to_string());
    map.insert("Media.view".to_string(), "1".to_string());
    map.insert("Media.width".to_string(), "1".to_string());
    map.insert("Medication.amount".to_string(), "1".to_string());
    map.insert("Medication.batch".to_string(), "1".to_string());
    map.insert("Medication.batch.expirationDate".to_string(), "1".to_string());
    map.insert("Medication.batch.extension".to_string(), "*".to_string());
    map.insert("Medication.batch.id".to_string(), "1".to_string());
    map.insert("Medication.batch.lotNumber".to_string(), "1".to_string());
    map.insert("Medication.batch.modifierExtension".to_string(), "*".to_string());
    map.insert("Medication.code".to_string(), "1".to_string());
    map.insert("Medication.contained".to_string(), "*".to_string());
    map.insert("Medication.extension".to_string(), "*".to_string());
    map.insert("Medication.form".to_string(), "1".to_string());
    map.insert("Medication.id".to_string(), "1".to_string());
    map.insert("Medication.identifier".to_string(), "*".to_string());
    map.insert("Medication.implicitRules".to_string(), "1".to_string());
    map.insert("Medication.ingredient".to_string(), "*".to_string());
    map.insert("Medication.ingredient.extension".to_string(), "*".to_string());
    map.insert("Medication.ingredient.id".to_string(), "1".to_string());
    map.insert("Medication.ingredient.isActive".to_string(), "1".to_string());
    map.insert("Medication.ingredient.itemCodeableConcept".to_string(), "1".to_string());
    map.insert("Medication.ingredient.itemReference".to_string(), "1".to_string());
    map.insert("Medication.ingredient.modifierExtension".to_string(), "*".to_string());
    map.insert("Medication.ingredient.strength".to_string(), "1".to_string());
    map.insert("Medication.language".to_string(), "1".to_string());
    map.insert("Medication.manufacturer".to_string(), "1".to_string());
    map.insert("Medication.meta".to_string(), "1".to_string());
    map.insert("Medication.modifierExtension".to_string(), "*".to_string());
    map.insert("Medication.status".to_string(), "1".to_string());
    map.insert("Medication.text".to_string(), "1".to_string());
    map.insert("MedicationAdministration.category".to_string(), "1".to_string());
    map.insert("MedicationAdministration.contained".to_string(), "*".to_string());
    map.insert("MedicationAdministration.context".to_string(), "1".to_string());
    map.insert("MedicationAdministration.device".to_string(), "*".to_string());
    map.insert("MedicationAdministration.dosage".to_string(), "1".to_string());
    map.insert("MedicationAdministration.dosage.dose".to_string(), "1".to_string());
    map.insert("MedicationAdministration.dosage.extension".to_string(), "*".to_string());
    map.insert("MedicationAdministration.dosage.id".to_string(), "1".to_string());
    map.insert("MedicationAdministration.dosage.method".to_string(), "1".to_string());
    map.insert(
        "MedicationAdministration.dosage.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationAdministration.dosage.rateQuantity".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationAdministration.dosage.rateRatio".to_string(), "1".to_string());
    map.insert("MedicationAdministration.dosage.route".to_string(), "1".to_string());
    map.insert("MedicationAdministration.dosage.site".to_string(), "1".to_string());
    map.insert("MedicationAdministration.dosage.text".to_string(), "1".to_string());
    map.insert(
        "MedicationAdministration.effectiveDateTime".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationAdministration.effectivePeriod".to_string(), "1".to_string());
    map.insert("MedicationAdministration.eventHistory".to_string(), "*".to_string());
    map.insert("MedicationAdministration.extension".to_string(), "*".to_string());
    map.insert("MedicationAdministration.id".to_string(), "1".to_string());
    map.insert("MedicationAdministration.identifier".to_string(), "*".to_string());
    map.insert("MedicationAdministration.implicitRules".to_string(), "1".to_string());
    map.insert("MedicationAdministration.instantiates".to_string(), "*".to_string());
    map.insert("MedicationAdministration.language".to_string(), "1".to_string());
    map.insert(
        "MedicationAdministration.medicationCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationAdministration.medicationReference".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationAdministration.meta".to_string(), "1".to_string());
    map.insert(
        "MedicationAdministration.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationAdministration.note".to_string(), "*".to_string());
    map.insert("MedicationAdministration.partOf".to_string(), "*".to_string());
    map.insert("MedicationAdministration.performer".to_string(), "*".to_string());
    map.insert("MedicationAdministration.performer.actor".to_string(), "1".to_string());
    map.insert(
        "MedicationAdministration.performer.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationAdministration.performer.function".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationAdministration.performer.id".to_string(), "1".to_string());
    map.insert(
        "MedicationAdministration.performer.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationAdministration.reasonCode".to_string(), "*".to_string());
    map.insert("MedicationAdministration.reasonReference".to_string(), "*".to_string());
    map.insert("MedicationAdministration.request".to_string(), "1".to_string());
    map.insert("MedicationAdministration.status".to_string(), "1".to_string());
    map.insert("MedicationAdministration.statusReason".to_string(), "*".to_string());
    map.insert("MedicationAdministration.subject".to_string(), "1".to_string());
    map.insert(
        "MedicationAdministration.supportingInformation".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationAdministration.text".to_string(), "1".to_string());
    map.insert(
        "MedicationDispense.authorizingPrescription".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationDispense.category".to_string(), "1".to_string());
    map.insert("MedicationDispense.contained".to_string(), "*".to_string());
    map.insert("MedicationDispense.context".to_string(), "1".to_string());
    map.insert("MedicationDispense.daysSupply".to_string(), "1".to_string());
    map.insert("MedicationDispense.destination".to_string(), "1".to_string());
    map.insert("MedicationDispense.detectedIssue".to_string(), "*".to_string());
    map.insert("MedicationDispense.dosageInstruction".to_string(), "*".to_string());
    map.insert("MedicationDispense.eventHistory".to_string(), "*".to_string());
    map.insert("MedicationDispense.extension".to_string(), "*".to_string());
    map.insert("MedicationDispense.id".to_string(), "1".to_string());
    map.insert("MedicationDispense.identifier".to_string(), "*".to_string());
    map.insert("MedicationDispense.implicitRules".to_string(), "1".to_string());
    map.insert("MedicationDispense.language".to_string(), "1".to_string());
    map.insert("MedicationDispense.location".to_string(), "1".to_string());
    map.insert(
        "MedicationDispense.medicationCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationDispense.medicationReference".to_string(), "1".to_string());
    map.insert("MedicationDispense.meta".to_string(), "1".to_string());
    map.insert("MedicationDispense.modifierExtension".to_string(), "*".to_string());
    map.insert("MedicationDispense.note".to_string(), "*".to_string());
    map.insert("MedicationDispense.partOf".to_string(), "*".to_string());
    map.insert("MedicationDispense.performer".to_string(), "*".to_string());
    map.insert("MedicationDispense.performer.actor".to_string(), "1".to_string());
    map.insert("MedicationDispense.performer.extension".to_string(), "*".to_string());
    map.insert("MedicationDispense.performer.function".to_string(), "1".to_string());
    map.insert("MedicationDispense.performer.id".to_string(), "1".to_string());
    map.insert(
        "MedicationDispense.performer.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationDispense.quantity".to_string(), "1".to_string());
    map.insert("MedicationDispense.receiver".to_string(), "*".to_string());
    map.insert("MedicationDispense.status".to_string(), "1".to_string());
    map.insert(
        "MedicationDispense.statusReasonCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationDispense.statusReasonReference".to_string(), "1".to_string());
    map.insert("MedicationDispense.subject".to_string(), "1".to_string());
    map.insert("MedicationDispense.substitution".to_string(), "1".to_string());
    map.insert("MedicationDispense.substitution.extension".to_string(), "*".to_string());
    map.insert("MedicationDispense.substitution.id".to_string(), "1".to_string());
    map.insert(
        "MedicationDispense.substitution.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationDispense.substitution.reason".to_string(), "*".to_string());
    map.insert(
        "MedicationDispense.substitution.responsibleParty".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationDispense.substitution.type".to_string(), "1".to_string());
    map.insert(
        "MedicationDispense.substitution.wasSubstituted".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationDispense.supportingInformation".to_string(), "*".to_string());
    map.insert("MedicationDispense.text".to_string(), "1".to_string());
    map.insert("MedicationDispense.type".to_string(), "1".to_string());
    map.insert("MedicationDispense.whenHandedOver".to_string(), "1".to_string());
    map.insert("MedicationDispense.whenPrepared".to_string(), "1".to_string());
    map.insert(
        "MedicationKnowledge.administrationGuidelines".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.dosage".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.dosage.dosage".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.dosage.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.dosage.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.dosage.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.dosage.type".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.indicationCodeableConcept"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.indicationReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.patientCharacteristics"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.patientCharacteristics.characteristicCodeableConcept"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.patientCharacteristics.characteristicQuantity"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.patientCharacteristics.extension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.patientCharacteristics.id"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.patientCharacteristics.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.administrationGuidelines.patientCharacteristics.value"
            .to_string(),
        "*".to_string(),
    );
    map.insert("MedicationKnowledge.amount".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.associatedMedication".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.code".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.contained".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.contraindication".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.cost".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.cost.cost".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.cost.extension".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.cost.id".to_string(), "1".to_string());
    map.insert(
        "MedicationKnowledge.cost.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationKnowledge.cost.source".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.cost.type".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.doseForm".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.drugCharacteristic".to_string(), "*".to_string());
    map.insert(
        "MedicationKnowledge.drugCharacteristic.extension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationKnowledge.drugCharacteristic.id".to_string(), "1".to_string());
    map.insert(
        "MedicationKnowledge.drugCharacteristic.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.drugCharacteristic.type".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.drugCharacteristic.valueBase64Binary".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.drugCharacteristic.valueCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.drugCharacteristic.valueQuantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.drugCharacteristic.valueString".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationKnowledge.extension".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.id".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.implicitRules".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.ingredient".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.ingredient.extension".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.ingredient.id".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.ingredient.isActive".to_string(), "1".to_string());
    map.insert(
        "MedicationKnowledge.ingredient.itemCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.ingredient.itemReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.ingredient.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationKnowledge.ingredient.strength".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.intendedRoute".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.kinetics".to_string(), "*".to_string());
    map.insert(
        "MedicationKnowledge.kinetics.areaUnderCurve".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationKnowledge.kinetics.extension".to_string(), "*".to_string());
    map.insert(
        "MedicationKnowledge.kinetics.halfLifePeriod".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationKnowledge.kinetics.id".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.kinetics.lethalDose50".to_string(), "*".to_string());
    map.insert(
        "MedicationKnowledge.kinetics.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationKnowledge.language".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.manufacturer".to_string(), "1".to_string());
    map.insert(
        "MedicationKnowledge.medicineClassification".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.medicineClassification.classification".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.medicineClassification.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.medicineClassification.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.medicineClassification.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.medicineClassification.type".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationKnowledge.meta".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.modifierExtension".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.monitoringProgram".to_string(), "*".to_string());
    map.insert(
        "MedicationKnowledge.monitoringProgram.extension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationKnowledge.monitoringProgram.id".to_string(), "1".to_string());
    map.insert(
        "MedicationKnowledge.monitoringProgram.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.monitoringProgram.name".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.monitoringProgram.type".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationKnowledge.monograph".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.monograph.extension".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.monograph.id".to_string(), "1".to_string());
    map.insert(
        "MedicationKnowledge.monograph.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationKnowledge.monograph.source".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.monograph.type".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.packaging".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.packaging.extension".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.packaging.id".to_string(), "1".to_string());
    map.insert(
        "MedicationKnowledge.packaging.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationKnowledge.packaging.quantity".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.packaging.type".to_string(), "1".to_string());
    map.insert(
        "MedicationKnowledge.preparationInstruction".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationKnowledge.productType".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.regulatory".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.regulatory.extension".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.regulatory.id".to_string(), "1".to_string());
    map.insert(
        "MedicationKnowledge.regulatory.maxDispense".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.maxDispense.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.maxDispense.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.maxDispense.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.maxDispense.period".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.maxDispense.quantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.regulatoryAuthority".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationKnowledge.regulatory.schedule".to_string(), "*".to_string());
    map.insert(
        "MedicationKnowledge.regulatory.schedule.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.schedule.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.schedule.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.schedule.schedule".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.substitution".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.substitution.allowed".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.substitution.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.substitution.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.substitution.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.regulatory.substitution.type".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.relatedMedicationKnowledge".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.relatedMedicationKnowledge.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.relatedMedicationKnowledge.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationKnowledge.relatedMedicationKnowledge.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.relatedMedicationKnowledge.reference".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationKnowledge.relatedMedicationKnowledge.type".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationKnowledge.status".to_string(), "1".to_string());
    map.insert("MedicationKnowledge.synonym".to_string(), "*".to_string());
    map.insert("MedicationKnowledge.text".to_string(), "1".to_string());
    map.insert("MedicationRequest.authoredOn".to_string(), "1".to_string());
    map.insert("MedicationRequest.basedOn".to_string(), "*".to_string());
    map.insert("MedicationRequest.category".to_string(), "*".to_string());
    map.insert("MedicationRequest.contained".to_string(), "*".to_string());
    map.insert("MedicationRequest.courseOfTherapyType".to_string(), "1".to_string());
    map.insert("MedicationRequest.detectedIssue".to_string(), "*".to_string());
    map.insert("MedicationRequest.dispenseRequest".to_string(), "1".to_string());
    map.insert(
        "MedicationRequest.dispenseRequest.dispenseInterval".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationRequest.dispenseRequest.expectedSupplyDuration".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationRequest.dispenseRequest.extension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationRequest.dispenseRequest.id".to_string(), "1".to_string());
    map.insert(
        "MedicationRequest.dispenseRequest.initialFill".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationRequest.dispenseRequest.initialFill.duration".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationRequest.dispenseRequest.initialFill.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationRequest.dispenseRequest.initialFill.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationRequest.dispenseRequest.initialFill.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationRequest.dispenseRequest.initialFill.quantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationRequest.dispenseRequest.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationRequest.dispenseRequest.numberOfRepeatsAllowed".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationRequest.dispenseRequest.performer".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationRequest.dispenseRequest.quantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationRequest.dispenseRequest.validityPeriod".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationRequest.doNotPerform".to_string(), "1".to_string());
    map.insert("MedicationRequest.dosageInstruction".to_string(), "*".to_string());
    map.insert("MedicationRequest.encounter".to_string(), "1".to_string());
    map.insert("MedicationRequest.eventHistory".to_string(), "*".to_string());
    map.insert("MedicationRequest.extension".to_string(), "*".to_string());
    map.insert("MedicationRequest.groupIdentifier".to_string(), "1".to_string());
    map.insert("MedicationRequest.id".to_string(), "1".to_string());
    map.insert("MedicationRequest.identifier".to_string(), "*".to_string());
    map.insert("MedicationRequest.implicitRules".to_string(), "1".to_string());
    map.insert("MedicationRequest.instantiatesCanonical".to_string(), "*".to_string());
    map.insert("MedicationRequest.instantiatesUri".to_string(), "*".to_string());
    map.insert("MedicationRequest.insurance".to_string(), "*".to_string());
    map.insert("MedicationRequest.intent".to_string(), "1".to_string());
    map.insert("MedicationRequest.language".to_string(), "1".to_string());
    map.insert(
        "MedicationRequest.medicationCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationRequest.medicationReference".to_string(), "1".to_string());
    map.insert("MedicationRequest.meta".to_string(), "1".to_string());
    map.insert("MedicationRequest.modifierExtension".to_string(), "*".to_string());
    map.insert("MedicationRequest.note".to_string(), "*".to_string());
    map.insert("MedicationRequest.performer".to_string(), "1".to_string());
    map.insert("MedicationRequest.performerType".to_string(), "1".to_string());
    map.insert("MedicationRequest.priorPrescription".to_string(), "1".to_string());
    map.insert("MedicationRequest.priority".to_string(), "1".to_string());
    map.insert("MedicationRequest.reasonCode".to_string(), "*".to_string());
    map.insert("MedicationRequest.reasonReference".to_string(), "*".to_string());
    map.insert("MedicationRequest.recorder".to_string(), "1".to_string());
    map.insert("MedicationRequest.reportedBoolean".to_string(), "1".to_string());
    map.insert("MedicationRequest.reportedReference".to_string(), "1".to_string());
    map.insert("MedicationRequest.requester".to_string(), "1".to_string());
    map.insert("MedicationRequest.status".to_string(), "1".to_string());
    map.insert("MedicationRequest.statusReason".to_string(), "1".to_string());
    map.insert("MedicationRequest.subject".to_string(), "1".to_string());
    map.insert("MedicationRequest.substitution".to_string(), "1".to_string());
    map.insert(
        "MedicationRequest.substitution.allowedBoolean".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicationRequest.substitution.allowedCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationRequest.substitution.extension".to_string(), "*".to_string());
    map.insert("MedicationRequest.substitution.id".to_string(), "1".to_string());
    map.insert(
        "MedicationRequest.substitution.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationRequest.substitution.reason".to_string(), "1".to_string());
    map.insert("MedicationRequest.supportingInformation".to_string(), "*".to_string());
    map.insert("MedicationRequest.text".to_string(), "1".to_string());
    map.insert("MedicationStatement.basedOn".to_string(), "*".to_string());
    map.insert("MedicationStatement.category".to_string(), "1".to_string());
    map.insert("MedicationStatement.contained".to_string(), "*".to_string());
    map.insert("MedicationStatement.context".to_string(), "1".to_string());
    map.insert("MedicationStatement.dateAsserted".to_string(), "1".to_string());
    map.insert("MedicationStatement.derivedFrom".to_string(), "*".to_string());
    map.insert("MedicationStatement.dosage".to_string(), "*".to_string());
    map.insert("MedicationStatement.effectiveDateTime".to_string(), "1".to_string());
    map.insert("MedicationStatement.effectivePeriod".to_string(), "1".to_string());
    map.insert("MedicationStatement.extension".to_string(), "*".to_string());
    map.insert("MedicationStatement.id".to_string(), "1".to_string());
    map.insert("MedicationStatement.identifier".to_string(), "*".to_string());
    map.insert("MedicationStatement.implicitRules".to_string(), "1".to_string());
    map.insert("MedicationStatement.informationSource".to_string(), "1".to_string());
    map.insert("MedicationStatement.language".to_string(), "1".to_string());
    map.insert(
        "MedicationStatement.medicationCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationStatement.medicationReference".to_string(), "1".to_string());
    map.insert("MedicationStatement.meta".to_string(), "1".to_string());
    map.insert("MedicationStatement.modifierExtension".to_string(), "*".to_string());
    map.insert("MedicationStatement.note".to_string(), "*".to_string());
    map.insert("MedicationStatement.partOf".to_string(), "*".to_string());
    map.insert("MedicationStatement.reasonCode".to_string(), "*".to_string());
    map.insert("MedicationStatement.reasonReference".to_string(), "*".to_string());
    map.insert("MedicationStatement.status".to_string(), "1".to_string());
    map.insert("MedicationStatement.statusReason".to_string(), "*".to_string());
    map.insert("MedicationStatement.subject".to_string(), "1".to_string());
    map.insert("MedicationStatement.text".to_string(), "1".to_string());
    map.insert(
        "MedicinalProduct.additionalMonitoringIndicator".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProduct.attachedDocument".to_string(), "*".to_string());
    map.insert("MedicinalProduct.clinicalTrial".to_string(), "*".to_string());
    map.insert(
        "MedicinalProduct.combinedPharmaceuticalDoseForm".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProduct.contact".to_string(), "*".to_string());
    map.insert("MedicinalProduct.contained".to_string(), "*".to_string());
    map.insert("MedicinalProduct.crossReference".to_string(), "*".to_string());
    map.insert("MedicinalProduct.domain".to_string(), "1".to_string());
    map.insert("MedicinalProduct.extension".to_string(), "*".to_string());
    map.insert("MedicinalProduct.id".to_string(), "1".to_string());
    map.insert("MedicinalProduct.identifier".to_string(), "*".to_string());
    map.insert("MedicinalProduct.implicitRules".to_string(), "1".to_string());
    map.insert("MedicinalProduct.language".to_string(), "1".to_string());
    map.insert("MedicinalProduct.legalStatusOfSupply".to_string(), "1".to_string());
    map.insert(
        "MedicinalProduct.manufacturingBusinessOperation".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProduct.manufacturingBusinessOperation.authorisationReferenceNumber"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProduct.manufacturingBusinessOperation.confidentialityIndicator"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProduct.manufacturingBusinessOperation.effectiveDate".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProduct.manufacturingBusinessOperation.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProduct.manufacturingBusinessOperation.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProduct.manufacturingBusinessOperation.manufacturer".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProduct.manufacturingBusinessOperation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProduct.manufacturingBusinessOperation.operationType".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProduct.manufacturingBusinessOperation.regulator".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProduct.marketingStatus".to_string(), "*".to_string());
    map.insert("MedicinalProduct.masterFile".to_string(), "*".to_string());
    map.insert("MedicinalProduct.meta".to_string(), "1".to_string());
    map.insert("MedicinalProduct.modifierExtension".to_string(), "*".to_string());
    map.insert("MedicinalProduct.name".to_string(), "*".to_string());
    map.insert("MedicinalProduct.name.countryLanguage".to_string(), "*".to_string());
    map.insert(
        "MedicinalProduct.name.countryLanguage.country".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProduct.name.countryLanguage.extension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProduct.name.countryLanguage.id".to_string(), "1".to_string());
    map.insert(
        "MedicinalProduct.name.countryLanguage.jurisdiction".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProduct.name.countryLanguage.language".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProduct.name.countryLanguage.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProduct.name.extension".to_string(), "*".to_string());
    map.insert("MedicinalProduct.name.id".to_string(), "1".to_string());
    map.insert("MedicinalProduct.name.modifierExtension".to_string(), "*".to_string());
    map.insert("MedicinalProduct.name.namePart".to_string(), "*".to_string());
    map.insert("MedicinalProduct.name.namePart.extension".to_string(), "*".to_string());
    map.insert("MedicinalProduct.name.namePart.id".to_string(), "1".to_string());
    map.insert(
        "MedicinalProduct.name.namePart.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProduct.name.namePart.part".to_string(), "1".to_string());
    map.insert("MedicinalProduct.name.namePart.type".to_string(), "1".to_string());
    map.insert("MedicinalProduct.name.productName".to_string(), "1".to_string());
    map.insert("MedicinalProduct.packagedMedicinalProduct".to_string(), "*".to_string());
    map.insert("MedicinalProduct.paediatricUseIndicator".to_string(), "1".to_string());
    map.insert("MedicinalProduct.pharmaceuticalProduct".to_string(), "*".to_string());
    map.insert("MedicinalProduct.productClassification".to_string(), "*".to_string());
    map.insert("MedicinalProduct.specialDesignation".to_string(), "*".to_string());
    map.insert("MedicinalProduct.specialDesignation.date".to_string(), "1".to_string());
    map.insert(
        "MedicinalProduct.specialDesignation.extension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProduct.specialDesignation.id".to_string(), "1".to_string());
    map.insert(
        "MedicinalProduct.specialDesignation.identifier".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProduct.specialDesignation.indicationCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProduct.specialDesignation.indicationReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProduct.specialDesignation.intendedUse".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProduct.specialDesignation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProduct.specialDesignation.species".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProduct.specialDesignation.status".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProduct.specialDesignation.type".to_string(), "1".to_string());
    map.insert("MedicinalProduct.specialMeasures".to_string(), "*".to_string());
    map.insert("MedicinalProduct.text".to_string(), "1".to_string());
    map.insert("MedicinalProduct.type".to_string(), "1".to_string());
    map.insert("MedicinalProductAuthorization.contained".to_string(), "*".to_string());
    map.insert("MedicinalProductAuthorization.country".to_string(), "*".to_string());
    map.insert(
        "MedicinalProductAuthorization.dataExclusivityPeriod".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.dateOfFirstAuthorization".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductAuthorization.extension".to_string(), "*".to_string());
    map.insert("MedicinalProductAuthorization.holder".to_string(), "1".to_string());
    map.insert("MedicinalProductAuthorization.id".to_string(), "1".to_string());
    map.insert("MedicinalProductAuthorization.identifier".to_string(), "*".to_string());
    map.insert(
        "MedicinalProductAuthorization.implicitRules".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.internationalBirthDate".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.jurisdiction".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.jurisdictionalAuthorization".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.jurisdictionalAuthorization.country".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.jurisdictionalAuthorization.extension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.jurisdictionalAuthorization.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.jurisdictionalAuthorization.identifier"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.jurisdictionalAuthorization.jurisdiction"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.jurisdictionalAuthorization.legalStatusOfSupply"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.jurisdictionalAuthorization.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.jurisdictionalAuthorization.validityPeriod"
            .to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductAuthorization.language".to_string(), "1".to_string());
    map.insert("MedicinalProductAuthorization.legalBasis".to_string(), "1".to_string());
    map.insert("MedicinalProductAuthorization.meta".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductAuthorization.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProductAuthorization.procedure".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductAuthorization.procedure.application".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.procedure.dateDateTime".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.procedure.datePeriod".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.procedure.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.procedure.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.procedure.identifier".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.procedure.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductAuthorization.procedure.type".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductAuthorization.regulator".to_string(), "1".to_string());
    map.insert("MedicinalProductAuthorization.restoreDate".to_string(), "1".to_string());
    map.insert("MedicinalProductAuthorization.status".to_string(), "1".to_string());
    map.insert("MedicinalProductAuthorization.statusDate".to_string(), "1".to_string());
    map.insert("MedicinalProductAuthorization.subject".to_string(), "1".to_string());
    map.insert("MedicinalProductAuthorization.text".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductAuthorization.validityPeriod".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductContraindication.comorbidity".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductContraindication.contained".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProductContraindication.disease".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductContraindication.diseaseStatus".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductContraindication.extension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProductContraindication.id".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductContraindication.implicitRules".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductContraindication.language".to_string(), "1".to_string());
    map.insert("MedicinalProductContraindication.meta".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductContraindication.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductContraindication.otherTherapy".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductContraindication.otherTherapy.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductContraindication.otherTherapy.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductContraindication.otherTherapy.medicationCodeableConcept"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductContraindication.otherTherapy.medicationReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductContraindication.otherTherapy.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductContraindication.otherTherapy.therapyRelationshipType"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductContraindication.population".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProductContraindication.subject".to_string(), "*".to_string());
    map.insert("MedicinalProductContraindication.text".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductContraindication.therapeuticIndication".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProductIndication.comorbidity".to_string(), "*".to_string());
    map.insert("MedicinalProductIndication.contained".to_string(), "*".to_string());
    map.insert("MedicinalProductIndication.diseaseStatus".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductIndication.diseaseSymptomProcedure".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductIndication.duration".to_string(), "1".to_string());
    map.insert("MedicinalProductIndication.extension".to_string(), "*".to_string());
    map.insert("MedicinalProductIndication.id".to_string(), "1".to_string());
    map.insert("MedicinalProductIndication.implicitRules".to_string(), "1".to_string());
    map.insert("MedicinalProductIndication.intendedEffect".to_string(), "1".to_string());
    map.insert("MedicinalProductIndication.language".to_string(), "1".to_string());
    map.insert("MedicinalProductIndication.meta".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductIndication.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProductIndication.otherTherapy".to_string(), "*".to_string());
    map.insert(
        "MedicinalProductIndication.otherTherapy.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductIndication.otherTherapy.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIndication.otherTherapy.medicationCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIndication.otherTherapy.medicationReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIndication.otherTherapy.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductIndication.otherTherapy.therapyRelationshipType".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductIndication.population".to_string(), "*".to_string());
    map.insert("MedicinalProductIndication.subject".to_string(), "*".to_string());
    map.insert("MedicinalProductIndication.text".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductIndication.undesirableEffect".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.allergenicIndicator".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductIngredient.contained".to_string(), "*".to_string());
    map.insert("MedicinalProductIngredient.extension".to_string(), "*".to_string());
    map.insert("MedicinalProductIngredient.id".to_string(), "1".to_string());
    map.insert("MedicinalProductIngredient.identifier".to_string(), "1".to_string());
    map.insert("MedicinalProductIngredient.implicitRules".to_string(), "1".to_string());
    map.insert("MedicinalProductIngredient.language".to_string(), "1".to_string());
    map.insert("MedicinalProductIngredient.manufacturer".to_string(), "*".to_string());
    map.insert("MedicinalProductIngredient.meta".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductIngredient.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProductIngredient.role".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.code".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.confidentiality".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.group".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.concentration"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.concentrationLowLimit"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.country".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.measurementPoint"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.presentation"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.presentationLowLimit"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.referenceStrength"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.referenceStrength.country"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.referenceStrength.extension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.referenceStrength.id"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.referenceStrength.measurementPoint"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.referenceStrength.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.referenceStrength.strength"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.referenceStrength.strengthLowLimit"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.specifiedSubstance.strength.referenceStrength.substance"
            .to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductIngredient.substance".to_string(), "1".to_string());
    map.insert("MedicinalProductIngredient.substance.code".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductIngredient.substance.extension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProductIngredient.substance.id".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductIngredient.substance.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductIngredient.substance.strength".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProductIngredient.text".to_string(), "1".to_string());
    map.insert("MedicinalProductInteraction.contained".to_string(), "*".to_string());
    map.insert("MedicinalProductInteraction.description".to_string(), "1".to_string());
    map.insert("MedicinalProductInteraction.effect".to_string(), "1".to_string());
    map.insert("MedicinalProductInteraction.extension".to_string(), "*".to_string());
    map.insert("MedicinalProductInteraction.id".to_string(), "1".to_string());
    map.insert("MedicinalProductInteraction.implicitRules".to_string(), "1".to_string());
    map.insert("MedicinalProductInteraction.incidence".to_string(), "1".to_string());
    map.insert("MedicinalProductInteraction.interactant".to_string(), "*".to_string());
    map.insert(
        "MedicinalProductInteraction.interactant.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductInteraction.interactant.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductInteraction.interactant.itemCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductInteraction.interactant.itemReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductInteraction.interactant.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProductInteraction.language".to_string(), "1".to_string());
    map.insert("MedicinalProductInteraction.management".to_string(), "1".to_string());
    map.insert("MedicinalProductInteraction.meta".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductInteraction.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProductInteraction.subject".to_string(), "*".to_string());
    map.insert("MedicinalProductInteraction.text".to_string(), "1".to_string());
    map.insert("MedicinalProductInteraction.type".to_string(), "1".to_string());
    map.insert("MedicinalProductManufactured.contained".to_string(), "*".to_string());
    map.insert("MedicinalProductManufactured.extension".to_string(), "*".to_string());
    map.insert("MedicinalProductManufactured.id".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductManufactured.implicitRules".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductManufactured.ingredient".to_string(), "*".to_string());
    map.insert("MedicinalProductManufactured.language".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductManufactured.manufacturedDoseForm".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductManufactured.manufacturer".to_string(), "*".to_string());
    map.insert("MedicinalProductManufactured.meta".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductManufactured.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductManufactured.otherCharacteristics".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductManufactured.physicalCharacteristics".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductManufactured.quantity".to_string(), "1".to_string());
    map.insert("MedicinalProductManufactured.text".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductManufactured.unitOfPresentation".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductPackaged.batchIdentifier".to_string(), "*".to_string());
    map.insert(
        "MedicinalProductPackaged.batchIdentifier.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPackaged.batchIdentifier.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPackaged.batchIdentifier.immediatePackaging".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPackaged.batchIdentifier.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPackaged.batchIdentifier.outerPackaging".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductPackaged.contained".to_string(), "*".to_string());
    map.insert("MedicinalProductPackaged.description".to_string(), "1".to_string());
    map.insert("MedicinalProductPackaged.extension".to_string(), "*".to_string());
    map.insert("MedicinalProductPackaged.id".to_string(), "1".to_string());
    map.insert("MedicinalProductPackaged.identifier".to_string(), "*".to_string());
    map.insert("MedicinalProductPackaged.implicitRules".to_string(), "1".to_string());
    map.insert("MedicinalProductPackaged.language".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductPackaged.legalStatusOfSupply".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductPackaged.manufacturer".to_string(), "*".to_string());
    map.insert(
        "MedicinalProductPackaged.marketingAuthorization".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductPackaged.marketingStatus".to_string(), "*".to_string());
    map.insert("MedicinalProductPackaged.meta".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductPackaged.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProductPackaged.packageItem".to_string(), "*".to_string());
    map.insert(
        "MedicinalProductPackaged.packageItem.alternateMaterial".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPackaged.packageItem.device".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPackaged.packageItem.extension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProductPackaged.packageItem.id".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductPackaged.packageItem.identifier".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPackaged.packageItem.manufacturedItem".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPackaged.packageItem.manufacturer".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPackaged.packageItem.material".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPackaged.packageItem.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPackaged.packageItem.otherCharacteristics".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPackaged.packageItem.packageItem".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPackaged.packageItem.physicalCharacteristics".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPackaged.packageItem.quantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPackaged.packageItem.shelfLifeStorage".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProductPackaged.packageItem.type".to_string(), "1".to_string());
    map.insert("MedicinalProductPackaged.subject".to_string(), "*".to_string());
    map.insert("MedicinalProductPackaged.text".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductPharmaceutical.administrableDoseForm".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.characteristics".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.characteristics.code".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.characteristics.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.characteristics.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.characteristics.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.characteristics.status".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductPharmaceutical.contained".to_string(), "*".to_string());
    map.insert("MedicinalProductPharmaceutical.device".to_string(), "*".to_string());
    map.insert("MedicinalProductPharmaceutical.extension".to_string(), "*".to_string());
    map.insert("MedicinalProductPharmaceutical.id".to_string(), "1".to_string());
    map.insert("MedicinalProductPharmaceutical.identifier".to_string(), "*".to_string());
    map.insert(
        "MedicinalProductPharmaceutical.implicitRules".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductPharmaceutical.ingredient".to_string(), "*".to_string());
    map.insert("MedicinalProductPharmaceutical.language".to_string(), "1".to_string());
    map.insert("MedicinalProductPharmaceutical.meta".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductPharmaceutical.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.code".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.firstDose".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.maxDosePerDay".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.maxDosePerTreatmentPeriod"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.maxSingleDose".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.maxTreatmentPeriod"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.targetSpecies".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.targetSpecies.code"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.targetSpecies.extension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.targetSpecies.id"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.targetSpecies.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.targetSpecies.withdrawalPeriod"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.targetSpecies.withdrawalPeriod.extension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.targetSpecies.withdrawalPeriod.id"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.targetSpecies.withdrawalPeriod.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.targetSpecies.withdrawalPeriod.supportingInformation"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.targetSpecies.withdrawalPeriod.tissue"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductPharmaceutical.routeOfAdministration.targetSpecies.withdrawalPeriod.value"
            .to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductPharmaceutical.text".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductPharmaceutical.unitOfPresentation".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductUndesirableEffect.classification".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductUndesirableEffect.contained".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductUndesirableEffect.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductUndesirableEffect.frequencyOfOccurrence".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductUndesirableEffect.id".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductUndesirableEffect.implicitRules".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MedicinalProductUndesirableEffect.language".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductUndesirableEffect.meta".to_string(), "1".to_string());
    map.insert(
        "MedicinalProductUndesirableEffect.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicinalProductUndesirableEffect.population".to_string(),
        "*".to_string(),
    );
    map.insert("MedicinalProductUndesirableEffect.subject".to_string(), "*".to_string());
    map.insert(
        "MedicinalProductUndesirableEffect.symptomConditionEffect".to_string(),
        "1".to_string(),
    );
    map.insert("MedicinalProductUndesirableEffect.text".to_string(), "1".to_string());
    map.insert("MessageDefinition.allowedResponse".to_string(), "*".to_string());
    map.insert(
        "MessageDefinition.allowedResponse.extension".to_string(),
        "*".to_string(),
    );
    map.insert("MessageDefinition.allowedResponse.id".to_string(), "1".to_string());
    map.insert("MessageDefinition.allowedResponse.message".to_string(), "1".to_string());
    map.insert(
        "MessageDefinition.allowedResponse.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MessageDefinition.allowedResponse.situation".to_string(),
        "1".to_string(),
    );
    map.insert("MessageDefinition.base".to_string(), "1".to_string());
    map.insert("MessageDefinition.category".to_string(), "1".to_string());
    map.insert("MessageDefinition.contact".to_string(), "*".to_string());
    map.insert("MessageDefinition.contained".to_string(), "*".to_string());
    map.insert("MessageDefinition.copyright".to_string(), "1".to_string());
    map.insert("MessageDefinition.date".to_string(), "1".to_string());
    map.insert("MessageDefinition.description".to_string(), "1".to_string());
    map.insert("MessageDefinition.eventCoding".to_string(), "1".to_string());
    map.insert("MessageDefinition.eventUri".to_string(), "1".to_string());
    map.insert("MessageDefinition.experimental".to_string(), "1".to_string());
    map.insert("MessageDefinition.extension".to_string(), "*".to_string());
    map.insert("MessageDefinition.focus".to_string(), "*".to_string());
    map.insert("MessageDefinition.focus.code".to_string(), "1".to_string());
    map.insert("MessageDefinition.focus.extension".to_string(), "*".to_string());
    map.insert("MessageDefinition.focus.id".to_string(), "1".to_string());
    map.insert("MessageDefinition.focus.max".to_string(), "1".to_string());
    map.insert("MessageDefinition.focus.min".to_string(), "1".to_string());
    map.insert("MessageDefinition.focus.modifierExtension".to_string(), "*".to_string());
    map.insert("MessageDefinition.focus.profile".to_string(), "1".to_string());
    map.insert("MessageDefinition.graph".to_string(), "*".to_string());
    map.insert("MessageDefinition.id".to_string(), "1".to_string());
    map.insert("MessageDefinition.identifier".to_string(), "*".to_string());
    map.insert("MessageDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("MessageDefinition.jurisdiction".to_string(), "*".to_string());
    map.insert("MessageDefinition.language".to_string(), "1".to_string());
    map.insert("MessageDefinition.meta".to_string(), "1".to_string());
    map.insert("MessageDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("MessageDefinition.name".to_string(), "1".to_string());
    map.insert("MessageDefinition.parent".to_string(), "*".to_string());
    map.insert("MessageDefinition.publisher".to_string(), "1".to_string());
    map.insert("MessageDefinition.purpose".to_string(), "1".to_string());
    map.insert("MessageDefinition.replaces".to_string(), "*".to_string());
    map.insert("MessageDefinition.responseRequired".to_string(), "1".to_string());
    map.insert("MessageDefinition.status".to_string(), "1".to_string());
    map.insert("MessageDefinition.text".to_string(), "1".to_string());
    map.insert("MessageDefinition.title".to_string(), "1".to_string());
    map.insert("MessageDefinition.url".to_string(), "1".to_string());
    map.insert("MessageDefinition.useContext".to_string(), "*".to_string());
    map.insert("MessageDefinition.version".to_string(), "1".to_string());
    map.insert("MessageHeader.author".to_string(), "1".to_string());
    map.insert("MessageHeader.contained".to_string(), "*".to_string());
    map.insert("MessageHeader.definition".to_string(), "1".to_string());
    map.insert("MessageHeader.destination".to_string(), "*".to_string());
    map.insert("MessageHeader.destination.endpoint".to_string(), "1".to_string());
    map.insert("MessageHeader.destination.extension".to_string(), "*".to_string());
    map.insert("MessageHeader.destination.id".to_string(), "1".to_string());
    map.insert(
        "MessageHeader.destination.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MessageHeader.destination.name".to_string(), "1".to_string());
    map.insert("MessageHeader.destination.receiver".to_string(), "1".to_string());
    map.insert("MessageHeader.destination.target".to_string(), "1".to_string());
    map.insert("MessageHeader.enterer".to_string(), "1".to_string());
    map.insert("MessageHeader.eventCoding".to_string(), "1".to_string());
    map.insert("MessageHeader.eventUri".to_string(), "1".to_string());
    map.insert("MessageHeader.extension".to_string(), "*".to_string());
    map.insert("MessageHeader.focus".to_string(), "*".to_string());
    map.insert("MessageHeader.id".to_string(), "1".to_string());
    map.insert("MessageHeader.implicitRules".to_string(), "1".to_string());
    map.insert("MessageHeader.language".to_string(), "1".to_string());
    map.insert("MessageHeader.meta".to_string(), "1".to_string());
    map.insert("MessageHeader.modifierExtension".to_string(), "*".to_string());
    map.insert("MessageHeader.reason".to_string(), "1".to_string());
    map.insert("MessageHeader.response".to_string(), "1".to_string());
    map.insert("MessageHeader.response.code".to_string(), "1".to_string());
    map.insert("MessageHeader.response.details".to_string(), "1".to_string());
    map.insert("MessageHeader.response.extension".to_string(), "*".to_string());
    map.insert("MessageHeader.response.id".to_string(), "1".to_string());
    map.insert("MessageHeader.response.identifier".to_string(), "1".to_string());
    map.insert("MessageHeader.response.modifierExtension".to_string(), "*".to_string());
    map.insert("MessageHeader.responsible".to_string(), "1".to_string());
    map.insert("MessageHeader.sender".to_string(), "1".to_string());
    map.insert("MessageHeader.source".to_string(), "1".to_string());
    map.insert("MessageHeader.source.contact".to_string(), "1".to_string());
    map.insert("MessageHeader.source.endpoint".to_string(), "1".to_string());
    map.insert("MessageHeader.source.extension".to_string(), "*".to_string());
    map.insert("MessageHeader.source.id".to_string(), "1".to_string());
    map.insert("MessageHeader.source.modifierExtension".to_string(), "*".to_string());
    map.insert("MessageHeader.source.name".to_string(), "1".to_string());
    map.insert("MessageHeader.source.software".to_string(), "1".to_string());
    map.insert("MessageHeader.source.version".to_string(), "1".to_string());
    map.insert("MessageHeader.text".to_string(), "1".to_string());
    map.insert("Meta.extension".to_string(), "*".to_string());
    map.insert("Meta.id".to_string(), "1".to_string());
    map.insert("Meta.lastUpdated".to_string(), "1".to_string());
    map.insert("Meta.profile".to_string(), "*".to_string());
    map.insert("Meta.security".to_string(), "*".to_string());
    map.insert("Meta.source".to_string(), "1".to_string());
    map.insert("Meta.tag".to_string(), "*".to_string());
    map.insert("Meta.versionId".to_string(), "1".to_string());
    map.insert("MetadataResource.contact".to_string(), "*".to_string());
    map.insert("MetadataResource.contained".to_string(), "*".to_string());
    map.insert("MetadataResource.date".to_string(), "1".to_string());
    map.insert("MetadataResource.description".to_string(), "1".to_string());
    map.insert("MetadataResource.experimental".to_string(), "1".to_string());
    map.insert("MetadataResource.extension".to_string(), "*".to_string());
    map.insert("MetadataResource.id".to_string(), "1".to_string());
    map.insert("MetadataResource.implicitRules".to_string(), "1".to_string());
    map.insert("MetadataResource.jurisdiction".to_string(), "*".to_string());
    map.insert("MetadataResource.language".to_string(), "1".to_string());
    map.insert("MetadataResource.meta".to_string(), "1".to_string());
    map.insert("MetadataResource.modifierExtension".to_string(), "*".to_string());
    map.insert("MetadataResource.name".to_string(), "1".to_string());
    map.insert("MetadataResource.publisher".to_string(), "1".to_string());
    map.insert("MetadataResource.status".to_string(), "1".to_string());
    map.insert("MetadataResource.text".to_string(), "1".to_string());
    map.insert("MetadataResource.title".to_string(), "1".to_string());
    map.insert("MetadataResource.url".to_string(), "1".to_string());
    map.insert("MetadataResource.useContext".to_string(), "*".to_string());
    map.insert("MetadataResource.version".to_string(), "1".to_string());
    map.insert("MolecularSequence.contained".to_string(), "*".to_string());
    map.insert("MolecularSequence.coordinateSystem".to_string(), "1".to_string());
    map.insert("MolecularSequence.device".to_string(), "1".to_string());
    map.insert("MolecularSequence.extension".to_string(), "*".to_string());
    map.insert("MolecularSequence.id".to_string(), "1".to_string());
    map.insert("MolecularSequence.identifier".to_string(), "*".to_string());
    map.insert("MolecularSequence.implicitRules".to_string(), "1".to_string());
    map.insert("MolecularSequence.language".to_string(), "1".to_string());
    map.insert("MolecularSequence.meta".to_string(), "1".to_string());
    map.insert("MolecularSequence.modifierExtension".to_string(), "*".to_string());
    map.insert("MolecularSequence.observedSeq".to_string(), "1".to_string());
    map.insert("MolecularSequence.patient".to_string(), "1".to_string());
    map.insert("MolecularSequence.performer".to_string(), "1".to_string());
    map.insert("MolecularSequence.pointer".to_string(), "*".to_string());
    map.insert("MolecularSequence.quality".to_string(), "*".to_string());
    map.insert("MolecularSequence.quality.end".to_string(), "1".to_string());
    map.insert("MolecularSequence.quality.extension".to_string(), "*".to_string());
    map.insert("MolecularSequence.quality.fScore".to_string(), "1".to_string());
    map.insert("MolecularSequence.quality.gtFP".to_string(), "1".to_string());
    map.insert("MolecularSequence.quality.id".to_string(), "1".to_string());
    map.insert("MolecularSequence.quality.method".to_string(), "1".to_string());
    map.insert(
        "MolecularSequence.quality.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MolecularSequence.quality.precision".to_string(), "1".to_string());
    map.insert("MolecularSequence.quality.queryFP".to_string(), "1".to_string());
    map.insert("MolecularSequence.quality.queryTP".to_string(), "1".to_string());
    map.insert("MolecularSequence.quality.recall".to_string(), "1".to_string());
    map.insert("MolecularSequence.quality.roc".to_string(), "1".to_string());
    map.insert("MolecularSequence.quality.roc.extension".to_string(), "*".to_string());
    map.insert("MolecularSequence.quality.roc.fMeasure".to_string(), "*".to_string());
    map.insert("MolecularSequence.quality.roc.id".to_string(), "1".to_string());
    map.insert(
        "MolecularSequence.quality.roc.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MolecularSequence.quality.roc.numFN".to_string(), "*".to_string());
    map.insert("MolecularSequence.quality.roc.numFP".to_string(), "*".to_string());
    map.insert("MolecularSequence.quality.roc.numTP".to_string(), "*".to_string());
    map.insert("MolecularSequence.quality.roc.precision".to_string(), "*".to_string());
    map.insert("MolecularSequence.quality.roc.score".to_string(), "*".to_string());
    map.insert("MolecularSequence.quality.roc.sensitivity".to_string(), "*".to_string());
    map.insert("MolecularSequence.quality.score".to_string(), "1".to_string());
    map.insert(
        "MolecularSequence.quality.standardSequence".to_string(),
        "1".to_string(),
    );
    map.insert("MolecularSequence.quality.start".to_string(), "1".to_string());
    map.insert("MolecularSequence.quality.truthFN".to_string(), "1".to_string());
    map.insert("MolecularSequence.quality.truthTP".to_string(), "1".to_string());
    map.insert("MolecularSequence.quality.type".to_string(), "1".to_string());
    map.insert("MolecularSequence.quantity".to_string(), "1".to_string());
    map.insert("MolecularSequence.readCoverage".to_string(), "1".to_string());
    map.insert("MolecularSequence.referenceSeq".to_string(), "1".to_string());
    map.insert("MolecularSequence.referenceSeq.chromosome".to_string(), "1".to_string());
    map.insert("MolecularSequence.referenceSeq.extension".to_string(), "*".to_string());
    map.insert(
        "MolecularSequence.referenceSeq.genomeBuild".to_string(),
        "1".to_string(),
    );
    map.insert("MolecularSequence.referenceSeq.id".to_string(), "1".to_string());
    map.insert(
        "MolecularSequence.referenceSeq.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MolecularSequence.referenceSeq.orientation".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MolecularSequence.referenceSeq.referenceSeqId".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MolecularSequence.referenceSeq.referenceSeqPointer".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MolecularSequence.referenceSeq.referenceSeqString".to_string(),
        "1".to_string(),
    );
    map.insert("MolecularSequence.referenceSeq.strand".to_string(), "1".to_string());
    map.insert("MolecularSequence.referenceSeq.windowEnd".to_string(), "1".to_string());
    map.insert(
        "MolecularSequence.referenceSeq.windowStart".to_string(),
        "1".to_string(),
    );
    map.insert("MolecularSequence.repository".to_string(), "*".to_string());
    map.insert("MolecularSequence.repository.datasetId".to_string(), "1".to_string());
    map.insert("MolecularSequence.repository.extension".to_string(), "*".to_string());
    map.insert("MolecularSequence.repository.id".to_string(), "1".to_string());
    map.insert(
        "MolecularSequence.repository.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MolecularSequence.repository.name".to_string(), "1".to_string());
    map.insert("MolecularSequence.repository.readsetId".to_string(), "1".to_string());
    map.insert("MolecularSequence.repository.type".to_string(), "1".to_string());
    map.insert("MolecularSequence.repository.url".to_string(), "1".to_string());
    map.insert("MolecularSequence.repository.variantsetId".to_string(), "1".to_string());
    map.insert("MolecularSequence.specimen".to_string(), "1".to_string());
    map.insert("MolecularSequence.structureVariant".to_string(), "*".to_string());
    map.insert("MolecularSequence.structureVariant.exact".to_string(), "1".to_string());
    map.insert(
        "MolecularSequence.structureVariant.extension".to_string(),
        "*".to_string(),
    );
    map.insert("MolecularSequence.structureVariant.id".to_string(), "1".to_string());
    map.insert("MolecularSequence.structureVariant.inner".to_string(), "1".to_string());
    map.insert(
        "MolecularSequence.structureVariant.inner.end".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MolecularSequence.structureVariant.inner.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MolecularSequence.structureVariant.inner.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MolecularSequence.structureVariant.inner.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MolecularSequence.structureVariant.inner.start".to_string(),
        "1".to_string(),
    );
    map.insert("MolecularSequence.structureVariant.length".to_string(), "1".to_string());
    map.insert(
        "MolecularSequence.structureVariant.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MolecularSequence.structureVariant.outer".to_string(), "1".to_string());
    map.insert(
        "MolecularSequence.structureVariant.outer.end".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MolecularSequence.structureVariant.outer.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MolecularSequence.structureVariant.outer.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MolecularSequence.structureVariant.outer.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MolecularSequence.structureVariant.outer.start".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MolecularSequence.structureVariant.variantType".to_string(),
        "1".to_string(),
    );
    map.insert("MolecularSequence.text".to_string(), "1".to_string());
    map.insert("MolecularSequence.type".to_string(), "1".to_string());
    map.insert("MolecularSequence.variant".to_string(), "*".to_string());
    map.insert("MolecularSequence.variant.cigar".to_string(), "1".to_string());
    map.insert("MolecularSequence.variant.end".to_string(), "1".to_string());
    map.insert("MolecularSequence.variant.extension".to_string(), "*".to_string());
    map.insert("MolecularSequence.variant.id".to_string(), "1".to_string());
    map.insert(
        "MolecularSequence.variant.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MolecularSequence.variant.observedAllele".to_string(), "1".to_string());
    map.insert("MolecularSequence.variant.referenceAllele".to_string(), "1".to_string());
    map.insert("MolecularSequence.variant.start".to_string(), "1".to_string());
    map.insert("MolecularSequence.variant.variantPointer".to_string(), "1".to_string());
    map.insert("Money.currency".to_string(), "1".to_string());
    map.insert("Money.extension".to_string(), "*".to_string());
    map.insert("Money.id".to_string(), "1".to_string());
    map.insert("Money.value".to_string(), "1".to_string());
    map.insert("NamingSystem.contact".to_string(), "*".to_string());
    map.insert("NamingSystem.contained".to_string(), "*".to_string());
    map.insert("NamingSystem.date".to_string(), "1".to_string());
    map.insert("NamingSystem.description".to_string(), "1".to_string());
    map.insert("NamingSystem.extension".to_string(), "*".to_string());
    map.insert("NamingSystem.id".to_string(), "1".to_string());
    map.insert("NamingSystem.implicitRules".to_string(), "1".to_string());
    map.insert("NamingSystem.jurisdiction".to_string(), "*".to_string());
    map.insert("NamingSystem.kind".to_string(), "1".to_string());
    map.insert("NamingSystem.language".to_string(), "1".to_string());
    map.insert("NamingSystem.meta".to_string(), "1".to_string());
    map.insert("NamingSystem.modifierExtension".to_string(), "*".to_string());
    map.insert("NamingSystem.name".to_string(), "1".to_string());
    map.insert("NamingSystem.publisher".to_string(), "1".to_string());
    map.insert("NamingSystem.responsible".to_string(), "1".to_string());
    map.insert("NamingSystem.status".to_string(), "1".to_string());
    map.insert("NamingSystem.text".to_string(), "1".to_string());
    map.insert("NamingSystem.type".to_string(), "1".to_string());
    map.insert("NamingSystem.uniqueId".to_string(), "*".to_string());
    map.insert("NamingSystem.uniqueId.comment".to_string(), "1".to_string());
    map.insert("NamingSystem.uniqueId.extension".to_string(), "*".to_string());
    map.insert("NamingSystem.uniqueId.id".to_string(), "1".to_string());
    map.insert("NamingSystem.uniqueId.modifierExtension".to_string(), "*".to_string());
    map.insert("NamingSystem.uniqueId.period".to_string(), "1".to_string());
    map.insert("NamingSystem.uniqueId.preferred".to_string(), "1".to_string());
    map.insert("NamingSystem.uniqueId.type".to_string(), "1".to_string());
    map.insert("NamingSystem.uniqueId.value".to_string(), "1".to_string());
    map.insert("NamingSystem.usage".to_string(), "1".to_string());
    map.insert("NamingSystem.useContext".to_string(), "*".to_string());
    map.insert("Narrative.div".to_string(), "1".to_string());
    map.insert("Narrative.extension".to_string(), "*".to_string());
    map.insert("Narrative.id".to_string(), "1".to_string());
    map.insert("Narrative.status".to_string(), "1".to_string());
    map.insert("NutritionOrder.allergyIntolerance".to_string(), "*".to_string());
    map.insert("NutritionOrder.contained".to_string(), "*".to_string());
    map.insert("NutritionOrder.dateTime".to_string(), "1".to_string());
    map.insert("NutritionOrder.encounter".to_string(), "1".to_string());
    map.insert("NutritionOrder.enteralFormula".to_string(), "1".to_string());
    map.insert(
        "NutritionOrder.enteralFormula.additiveProductName".to_string(),
        "1".to_string(),
    );
    map.insert(
        "NutritionOrder.enteralFormula.additiveType".to_string(),
        "1".to_string(),
    );
    map.insert(
        "NutritionOrder.enteralFormula.administration".to_string(),
        "*".to_string(),
    );
    map.insert(
        "NutritionOrder.enteralFormula.administration.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "NutritionOrder.enteralFormula.administration.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "NutritionOrder.enteralFormula.administration.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "NutritionOrder.enteralFormula.administration.quantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "NutritionOrder.enteralFormula.administration.rateQuantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "NutritionOrder.enteralFormula.administration.rateRatio".to_string(),
        "1".to_string(),
    );
    map.insert(
        "NutritionOrder.enteralFormula.administration.schedule".to_string(),
        "1".to_string(),
    );
    map.insert(
        "NutritionOrder.enteralFormula.administrationInstruction".to_string(),
        "1".to_string(),
    );
    map.insert(
        "NutritionOrder.enteralFormula.baseFormulaProductName".to_string(),
        "1".to_string(),
    );
    map.insert(
        "NutritionOrder.enteralFormula.baseFormulaType".to_string(),
        "1".to_string(),
    );
    map.insert(
        "NutritionOrder.enteralFormula.caloricDensity".to_string(),
        "1".to_string(),
    );
    map.insert("NutritionOrder.enteralFormula.extension".to_string(), "*".to_string());
    map.insert("NutritionOrder.enteralFormula.id".to_string(), "1".to_string());
    map.insert(
        "NutritionOrder.enteralFormula.maxVolumeToDeliver".to_string(),
        "1".to_string(),
    );
    map.insert(
        "NutritionOrder.enteralFormula.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "NutritionOrder.enteralFormula.routeofAdministration".to_string(),
        "1".to_string(),
    );
    map.insert("NutritionOrder.excludeFoodModifier".to_string(), "*".to_string());
    map.insert("NutritionOrder.extension".to_string(), "*".to_string());
    map.insert("NutritionOrder.foodPreferenceModifier".to_string(), "*".to_string());
    map.insert("NutritionOrder.id".to_string(), "1".to_string());
    map.insert("NutritionOrder.identifier".to_string(), "*".to_string());
    map.insert("NutritionOrder.implicitRules".to_string(), "1".to_string());
    map.insert("NutritionOrder.instantiates".to_string(), "*".to_string());
    map.insert("NutritionOrder.instantiatesCanonical".to_string(), "*".to_string());
    map.insert("NutritionOrder.instantiatesUri".to_string(), "*".to_string());
    map.insert("NutritionOrder.intent".to_string(), "1".to_string());
    map.insert("NutritionOrder.language".to_string(), "1".to_string());
    map.insert("NutritionOrder.meta".to_string(), "1".to_string());
    map.insert("NutritionOrder.modifierExtension".to_string(), "*".to_string());
    map.insert("NutritionOrder.note".to_string(), "*".to_string());
    map.insert("NutritionOrder.oralDiet".to_string(), "1".to_string());
    map.insert("NutritionOrder.oralDiet.extension".to_string(), "*".to_string());
    map.insert(
        "NutritionOrder.oralDiet.fluidConsistencyType".to_string(),
        "*".to_string(),
    );
    map.insert("NutritionOrder.oralDiet.id".to_string(), "1".to_string());
    map.insert("NutritionOrder.oralDiet.instruction".to_string(), "1".to_string());
    map.insert("NutritionOrder.oralDiet.modifierExtension".to_string(), "*".to_string());
    map.insert("NutritionOrder.oralDiet.nutrient".to_string(), "*".to_string());
    map.insert("NutritionOrder.oralDiet.nutrient.amount".to_string(), "1".to_string());
    map.insert(
        "NutritionOrder.oralDiet.nutrient.extension".to_string(),
        "*".to_string(),
    );
    map.insert("NutritionOrder.oralDiet.nutrient.id".to_string(), "1".to_string());
    map.insert("NutritionOrder.oralDiet.nutrient.modifier".to_string(), "1".to_string());
    map.insert(
        "NutritionOrder.oralDiet.nutrient.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("NutritionOrder.oralDiet.schedule".to_string(), "*".to_string());
    map.insert("NutritionOrder.oralDiet.texture".to_string(), "*".to_string());
    map.insert("NutritionOrder.oralDiet.texture.extension".to_string(), "*".to_string());
    map.insert("NutritionOrder.oralDiet.texture.foodType".to_string(), "1".to_string());
    map.insert("NutritionOrder.oralDiet.texture.id".to_string(), "1".to_string());
    map.insert("NutritionOrder.oralDiet.texture.modifier".to_string(), "1".to_string());
    map.insert(
        "NutritionOrder.oralDiet.texture.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("NutritionOrder.oralDiet.type".to_string(), "*".to_string());
    map.insert("NutritionOrder.orderer".to_string(), "1".to_string());
    map.insert("NutritionOrder.patient".to_string(), "1".to_string());
    map.insert("NutritionOrder.status".to_string(), "1".to_string());
    map.insert("NutritionOrder.supplement".to_string(), "*".to_string());
    map.insert("NutritionOrder.supplement.extension".to_string(), "*".to_string());
    map.insert("NutritionOrder.supplement.id".to_string(), "1".to_string());
    map.insert("NutritionOrder.supplement.instruction".to_string(), "1".to_string());
    map.insert(
        "NutritionOrder.supplement.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("NutritionOrder.supplement.productName".to_string(), "1".to_string());
    map.insert("NutritionOrder.supplement.quantity".to_string(), "1".to_string());
    map.insert("NutritionOrder.supplement.schedule".to_string(), "*".to_string());
    map.insert("NutritionOrder.supplement.type".to_string(), "1".to_string());
    map.insert("NutritionOrder.text".to_string(), "1".to_string());
    map.insert("Observation.basedOn".to_string(), "*".to_string());
    map.insert("Observation.bodySite".to_string(), "1".to_string());
    map.insert("Observation.category".to_string(), "*".to_string());
    map.insert("Observation.code".to_string(), "1".to_string());
    map.insert("Observation.component".to_string(), "*".to_string());
    map.insert("Observation.component.code".to_string(), "1".to_string());
    map.insert("Observation.component.dataAbsentReason".to_string(), "1".to_string());
    map.insert("Observation.component.extension".to_string(), "*".to_string());
    map.insert("Observation.component.id".to_string(), "1".to_string());
    map.insert("Observation.component.interpretation".to_string(), "*".to_string());
    map.insert("Observation.component.modifierExtension".to_string(), "*".to_string());
    map.insert("Observation.component.referenceRange".to_string(), "*".to_string());
    map.insert("Observation.component.valueBoolean".to_string(), "1".to_string());
    map.insert(
        "Observation.component.valueCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("Observation.component.valueDateTime".to_string(), "1".to_string());
    map.insert("Observation.component.valueInteger".to_string(), "1".to_string());
    map.insert("Observation.component.valuePeriod".to_string(), "1".to_string());
    map.insert("Observation.component.valueQuantity".to_string(), "1".to_string());
    map.insert("Observation.component.valueRange".to_string(), "1".to_string());
    map.insert("Observation.component.valueRatio".to_string(), "1".to_string());
    map.insert("Observation.component.valueSampledData".to_string(), "1".to_string());
    map.insert("Observation.component.valueString".to_string(), "1".to_string());
    map.insert("Observation.component.valueTime".to_string(), "1".to_string());
    map.insert("Observation.contained".to_string(), "*".to_string());
    map.insert("Observation.dataAbsentReason".to_string(), "1".to_string());
    map.insert("Observation.derivedFrom".to_string(), "*".to_string());
    map.insert("Observation.device".to_string(), "1".to_string());
    map.insert("Observation.effectiveDateTime".to_string(), "1".to_string());
    map.insert("Observation.effectiveInstant".to_string(), "1".to_string());
    map.insert("Observation.effectivePeriod".to_string(), "1".to_string());
    map.insert("Observation.effectiveTiming".to_string(), "1".to_string());
    map.insert("Observation.encounter".to_string(), "1".to_string());
    map.insert("Observation.extension".to_string(), "*".to_string());
    map.insert("Observation.focus".to_string(), "*".to_string());
    map.insert("Observation.hasMember".to_string(), "*".to_string());
    map.insert("Observation.id".to_string(), "1".to_string());
    map.insert("Observation.identifier".to_string(), "*".to_string());
    map.insert("Observation.implicitRules".to_string(), "1".to_string());
    map.insert("Observation.interpretation".to_string(), "*".to_string());
    map.insert("Observation.issued".to_string(), "1".to_string());
    map.insert("Observation.language".to_string(), "1".to_string());
    map.insert("Observation.meta".to_string(), "1".to_string());
    map.insert("Observation.method".to_string(), "1".to_string());
    map.insert("Observation.modifierExtension".to_string(), "*".to_string());
    map.insert("Observation.note".to_string(), "*".to_string());
    map.insert("Observation.partOf".to_string(), "*".to_string());
    map.insert("Observation.performer".to_string(), "*".to_string());
    map.insert("Observation.referenceRange".to_string(), "*".to_string());
    map.insert("Observation.referenceRange.age".to_string(), "1".to_string());
    map.insert("Observation.referenceRange.appliesTo".to_string(), "*".to_string());
    map.insert("Observation.referenceRange.extension".to_string(), "*".to_string());
    map.insert("Observation.referenceRange.high".to_string(), "1".to_string());
    map.insert("Observation.referenceRange.id".to_string(), "1".to_string());
    map.insert("Observation.referenceRange.low".to_string(), "1".to_string());
    map.insert(
        "Observation.referenceRange.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Observation.referenceRange.text".to_string(), "1".to_string());
    map.insert("Observation.referenceRange.type".to_string(), "1".to_string());
    map.insert("Observation.specimen".to_string(), "1".to_string());
    map.insert("Observation.status".to_string(), "1".to_string());
    map.insert("Observation.subject".to_string(), "1".to_string());
    map.insert("Observation.text".to_string(), "1".to_string());
    map.insert("Observation.valueBoolean".to_string(), "1".to_string());
    map.insert("Observation.valueCodeableConcept".to_string(), "1".to_string());
    map.insert("Observation.valueDateTime".to_string(), "1".to_string());
    map.insert("Observation.valueInteger".to_string(), "1".to_string());
    map.insert("Observation.valuePeriod".to_string(), "1".to_string());
    map.insert("Observation.valueQuantity".to_string(), "1".to_string());
    map.insert("Observation.valueRange".to_string(), "1".to_string());
    map.insert("Observation.valueRatio".to_string(), "1".to_string());
    map.insert("Observation.valueSampledData".to_string(), "1".to_string());
    map.insert("Observation.valueString".to_string(), "1".to_string());
    map.insert("Observation.valueTime".to_string(), "1".to_string());
    map.insert(
        "ObservationDefinition.abnormalCodedValueSet".to_string(),
        "1".to_string(),
    );
    map.insert("ObservationDefinition.category".to_string(), "*".to_string());
    map.insert("ObservationDefinition.code".to_string(), "1".to_string());
    map.insert("ObservationDefinition.contained".to_string(), "*".to_string());
    map.insert(
        "ObservationDefinition.criticalCodedValueSet".to_string(),
        "1".to_string(),
    );
    map.insert("ObservationDefinition.extension".to_string(), "*".to_string());
    map.insert("ObservationDefinition.id".to_string(), "1".to_string());
    map.insert("ObservationDefinition.identifier".to_string(), "*".to_string());
    map.insert("ObservationDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("ObservationDefinition.language".to_string(), "1".to_string());
    map.insert("ObservationDefinition.meta".to_string(), "1".to_string());
    map.insert("ObservationDefinition.method".to_string(), "1".to_string());
    map.insert("ObservationDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert(
        "ObservationDefinition.multipleResultsAllowed".to_string(),
        "1".to_string(),
    );
    map.insert("ObservationDefinition.normalCodedValueSet".to_string(), "1".to_string());
    map.insert("ObservationDefinition.permittedDataType".to_string(), "*".to_string());
    map.insert("ObservationDefinition.preferredReportName".to_string(), "1".to_string());
    map.insert("ObservationDefinition.qualifiedInterval".to_string(), "*".to_string());
    map.insert(
        "ObservationDefinition.qualifiedInterval.age".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ObservationDefinition.qualifiedInterval.appliesTo".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ObservationDefinition.qualifiedInterval.category".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ObservationDefinition.qualifiedInterval.condition".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ObservationDefinition.qualifiedInterval.context".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ObservationDefinition.qualifiedInterval.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ObservationDefinition.qualifiedInterval.gender".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ObservationDefinition.qualifiedInterval.gestationalAge".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ObservationDefinition.qualifiedInterval.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ObservationDefinition.qualifiedInterval.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ObservationDefinition.qualifiedInterval.range".to_string(),
        "1".to_string(),
    );
    map.insert("ObservationDefinition.quantitativeDetails".to_string(), "1".to_string());
    map.insert(
        "ObservationDefinition.quantitativeDetails.conversionFactor".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ObservationDefinition.quantitativeDetails.customaryUnit".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ObservationDefinition.quantitativeDetails.decimalPrecision".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ObservationDefinition.quantitativeDetails.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ObservationDefinition.quantitativeDetails.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ObservationDefinition.quantitativeDetails.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ObservationDefinition.quantitativeDetails.unit".to_string(),
        "1".to_string(),
    );
    map.insert("ObservationDefinition.text".to_string(), "1".to_string());
    map.insert("ObservationDefinition.validCodedValueSet".to_string(), "1".to_string());
    map.insert("OperationDefinition.affectsState".to_string(), "1".to_string());
    map.insert("OperationDefinition.base".to_string(), "1".to_string());
    map.insert("OperationDefinition.code".to_string(), "1".to_string());
    map.insert("OperationDefinition.comment".to_string(), "1".to_string());
    map.insert("OperationDefinition.contact".to_string(), "*".to_string());
    map.insert("OperationDefinition.contained".to_string(), "*".to_string());
    map.insert("OperationDefinition.date".to_string(), "1".to_string());
    map.insert("OperationDefinition.description".to_string(), "1".to_string());
    map.insert("OperationDefinition.experimental".to_string(), "1".to_string());
    map.insert("OperationDefinition.extension".to_string(), "*".to_string());
    map.insert("OperationDefinition.id".to_string(), "1".to_string());
    map.insert("OperationDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("OperationDefinition.inputProfile".to_string(), "1".to_string());
    map.insert("OperationDefinition.instance".to_string(), "1".to_string());
    map.insert("OperationDefinition.jurisdiction".to_string(), "*".to_string());
    map.insert("OperationDefinition.kind".to_string(), "1".to_string());
    map.insert("OperationDefinition.language".to_string(), "1".to_string());
    map.insert("OperationDefinition.meta".to_string(), "1".to_string());
    map.insert("OperationDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("OperationDefinition.name".to_string(), "1".to_string());
    map.insert("OperationDefinition.outputProfile".to_string(), "1".to_string());
    map.insert("OperationDefinition.overload".to_string(), "*".to_string());
    map.insert("OperationDefinition.overload.comment".to_string(), "1".to_string());
    map.insert("OperationDefinition.overload.extension".to_string(), "*".to_string());
    map.insert("OperationDefinition.overload.id".to_string(), "1".to_string());
    map.insert(
        "OperationDefinition.overload.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "OperationDefinition.overload.parameterName".to_string(),
        "*".to_string(),
    );
    map.insert("OperationDefinition.parameter".to_string(), "*".to_string());
    map.insert("OperationDefinition.parameter.binding".to_string(), "1".to_string());
    map.insert(
        "OperationDefinition.parameter.binding.extension".to_string(),
        "*".to_string(),
    );
    map.insert("OperationDefinition.parameter.binding.id".to_string(), "1".to_string());
    map.insert(
        "OperationDefinition.parameter.binding.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "OperationDefinition.parameter.binding.strength".to_string(),
        "1".to_string(),
    );
    map.insert(
        "OperationDefinition.parameter.binding.valueSet".to_string(),
        "1".to_string(),
    );
    map.insert(
        "OperationDefinition.parameter.documentation".to_string(),
        "1".to_string(),
    );
    map.insert("OperationDefinition.parameter.extension".to_string(), "*".to_string());
    map.insert("OperationDefinition.parameter.id".to_string(), "1".to_string());
    map.insert("OperationDefinition.parameter.max".to_string(), "1".to_string());
    map.insert("OperationDefinition.parameter.min".to_string(), "1".to_string());
    map.insert(
        "OperationDefinition.parameter.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("OperationDefinition.parameter.name".to_string(), "1".to_string());
    map.insert("OperationDefinition.parameter.part".to_string(), "*".to_string());
    map.insert(
        "OperationDefinition.parameter.referencedFrom".to_string(),
        "*".to_string(),
    );
    map.insert(
        "OperationDefinition.parameter.referencedFrom.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "OperationDefinition.parameter.referencedFrom.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "OperationDefinition.parameter.referencedFrom.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "OperationDefinition.parameter.referencedFrom.source".to_string(),
        "1".to_string(),
    );
    map.insert(
        "OperationDefinition.parameter.referencedFrom.sourceId".to_string(),
        "1".to_string(),
    );
    map.insert("OperationDefinition.parameter.searchType".to_string(), "1".to_string());
    map.insert(
        "OperationDefinition.parameter.targetProfile".to_string(),
        "*".to_string(),
    );
    map.insert("OperationDefinition.parameter.type".to_string(), "1".to_string());
    map.insert("OperationDefinition.parameter.use".to_string(), "1".to_string());
    map.insert("OperationDefinition.publisher".to_string(), "1".to_string());
    map.insert("OperationDefinition.purpose".to_string(), "1".to_string());
    map.insert("OperationDefinition.resource".to_string(), "*".to_string());
    map.insert("OperationDefinition.status".to_string(), "1".to_string());
    map.insert("OperationDefinition.system".to_string(), "1".to_string());
    map.insert("OperationDefinition.text".to_string(), "1".to_string());
    map.insert("OperationDefinition.title".to_string(), "1".to_string());
    map.insert("OperationDefinition.type".to_string(), "1".to_string());
    map.insert("OperationDefinition.url".to_string(), "1".to_string());
    map.insert("OperationDefinition.useContext".to_string(), "*".to_string());
    map.insert("OperationDefinition.version".to_string(), "1".to_string());
    map.insert("OperationOutcome.contained".to_string(), "*".to_string());
    map.insert("OperationOutcome.extension".to_string(), "*".to_string());
    map.insert("OperationOutcome.id".to_string(), "1".to_string());
    map.insert("OperationOutcome.implicitRules".to_string(), "1".to_string());
    map.insert("OperationOutcome.issue".to_string(), "*".to_string());
    map.insert("OperationOutcome.issue.code".to_string(), "1".to_string());
    map.insert("OperationOutcome.issue.details".to_string(), "1".to_string());
    map.insert("OperationOutcome.issue.diagnostics".to_string(), "1".to_string());
    map.insert("OperationOutcome.issue.expression".to_string(), "*".to_string());
    map.insert("OperationOutcome.issue.extension".to_string(), "*".to_string());
    map.insert("OperationOutcome.issue.id".to_string(), "1".to_string());
    map.insert("OperationOutcome.issue.location".to_string(), "*".to_string());
    map.insert("OperationOutcome.issue.modifierExtension".to_string(), "*".to_string());
    map.insert("OperationOutcome.issue.severity".to_string(), "1".to_string());
    map.insert("OperationOutcome.language".to_string(), "1".to_string());
    map.insert("OperationOutcome.meta".to_string(), "1".to_string());
    map.insert("OperationOutcome.modifierExtension".to_string(), "*".to_string());
    map.insert("OperationOutcome.text".to_string(), "1".to_string());
    map.insert("Organization.active".to_string(), "1".to_string());
    map.insert("Organization.address".to_string(), "*".to_string());
    map.insert("Organization.alias".to_string(), "*".to_string());
    map.insert("Organization.contact".to_string(), "*".to_string());
    map.insert("Organization.contact.address".to_string(), "1".to_string());
    map.insert("Organization.contact.extension".to_string(), "*".to_string());
    map.insert("Organization.contact.id".to_string(), "1".to_string());
    map.insert("Organization.contact.modifierExtension".to_string(), "*".to_string());
    map.insert("Organization.contact.name".to_string(), "1".to_string());
    map.insert("Organization.contact.purpose".to_string(), "1".to_string());
    map.insert("Organization.contact.telecom".to_string(), "*".to_string());
    map.insert("Organization.contained".to_string(), "*".to_string());
    map.insert("Organization.endpoint".to_string(), "*".to_string());
    map.insert("Organization.extension".to_string(), "*".to_string());
    map.insert("Organization.id".to_string(), "1".to_string());
    map.insert("Organization.identifier".to_string(), "*".to_string());
    map.insert("Organization.implicitRules".to_string(), "1".to_string());
    map.insert("Organization.language".to_string(), "1".to_string());
    map.insert("Organization.meta".to_string(), "1".to_string());
    map.insert("Organization.modifierExtension".to_string(), "*".to_string());
    map.insert("Organization.name".to_string(), "1".to_string());
    map.insert("Organization.partOf".to_string(), "1".to_string());
    map.insert("Organization.telecom".to_string(), "*".to_string());
    map.insert("Organization.text".to_string(), "1".to_string());
    map.insert("Organization.type".to_string(), "*".to_string());
    map.insert("OrganizationAffiliation.active".to_string(), "1".to_string());
    map.insert("OrganizationAffiliation.code".to_string(), "*".to_string());
    map.insert("OrganizationAffiliation.contained".to_string(), "*".to_string());
    map.insert("OrganizationAffiliation.endpoint".to_string(), "*".to_string());
    map.insert("OrganizationAffiliation.extension".to_string(), "*".to_string());
    map.insert("OrganizationAffiliation.healthcareService".to_string(), "*".to_string());
    map.insert("OrganizationAffiliation.id".to_string(), "1".to_string());
    map.insert("OrganizationAffiliation.identifier".to_string(), "*".to_string());
    map.insert("OrganizationAffiliation.implicitRules".to_string(), "1".to_string());
    map.insert("OrganizationAffiliation.language".to_string(), "1".to_string());
    map.insert("OrganizationAffiliation.location".to_string(), "*".to_string());
    map.insert("OrganizationAffiliation.meta".to_string(), "1".to_string());
    map.insert("OrganizationAffiliation.modifierExtension".to_string(), "*".to_string());
    map.insert("OrganizationAffiliation.network".to_string(), "*".to_string());
    map.insert("OrganizationAffiliation.organization".to_string(), "1".to_string());
    map.insert(
        "OrganizationAffiliation.participatingOrganization".to_string(),
        "1".to_string(),
    );
    map.insert("OrganizationAffiliation.period".to_string(), "1".to_string());
    map.insert("OrganizationAffiliation.specialty".to_string(), "*".to_string());
    map.insert("OrganizationAffiliation.telecom".to_string(), "*".to_string());
    map.insert("OrganizationAffiliation.text".to_string(), "1".to_string());
    map.insert("ParameterDefinition.documentation".to_string(), "1".to_string());
    map.insert("ParameterDefinition.extension".to_string(), "*".to_string());
    map.insert("ParameterDefinition.id".to_string(), "1".to_string());
    map.insert("ParameterDefinition.max".to_string(), "1".to_string());
    map.insert("ParameterDefinition.min".to_string(), "1".to_string());
    map.insert("ParameterDefinition.name".to_string(), "1".to_string());
    map.insert("ParameterDefinition.profile".to_string(), "1".to_string());
    map.insert("ParameterDefinition.type".to_string(), "1".to_string());
    map.insert("ParameterDefinition.use".to_string(), "1".to_string());
    map.insert("Parameters.id".to_string(), "1".to_string());
    map.insert("Parameters.implicitRules".to_string(), "1".to_string());
    map.insert("Parameters.language".to_string(), "1".to_string());
    map.insert("Parameters.meta".to_string(), "1".to_string());
    map.insert("Parameters.parameter".to_string(), "*".to_string());
    map.insert("Parameters.parameter.extension".to_string(), "*".to_string());
    map.insert("Parameters.parameter.id".to_string(), "1".to_string());
    map.insert("Parameters.parameter.modifierExtension".to_string(), "*".to_string());
    map.insert("Parameters.parameter.name".to_string(), "1".to_string());
    map.insert("Parameters.parameter.part".to_string(), "*".to_string());
    map.insert("Parameters.parameter.resource".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueAddress".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueAge".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueAnnotation".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueAttachment".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueBase64Binary".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueBoolean".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueCanonical".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueCode".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueCodeableConcept".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueCoding".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueContactDetail".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueContactPoint".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueContributor".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueCount".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueDataRequirement".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueDate".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueDateTime".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueDecimal".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueDistance".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueDosage".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueDuration".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueExpression".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueHumanName".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueId".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueIdentifier".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueInstant".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueInteger".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueMarkdown".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueMeta".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueMoney".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueOid".to_string(), "1".to_string());
    map.insert(
        "Parameters.parameter.valueParameterDefinition".to_string(),
        "1".to_string(),
    );
    map.insert("Parameters.parameter.valuePeriod".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valuePositiveInt".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueQuantity".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueRange".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueRatio".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueReference".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueRelatedArtifact".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueSampledData".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueSignature".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueString".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueTime".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueTiming".to_string(), "1".to_string());
    map.insert(
        "Parameters.parameter.valueTriggerDefinition".to_string(),
        "1".to_string(),
    );
    map.insert("Parameters.parameter.valueUnsignedInt".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueUri".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueUrl".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueUsageContext".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueUuid".to_string(), "1".to_string());
    map.insert("Patient.active".to_string(), "1".to_string());
    map.insert("Patient.address".to_string(), "*".to_string());
    map.insert("Patient.birthDate".to_string(), "1".to_string());
    map.insert("Patient.communication".to_string(), "*".to_string());
    map.insert("Patient.communication.extension".to_string(), "*".to_string());
    map.insert("Patient.communication.id".to_string(), "1".to_string());
    map.insert("Patient.communication.language".to_string(), "1".to_string());
    map.insert("Patient.communication.modifierExtension".to_string(), "*".to_string());
    map.insert("Patient.communication.preferred".to_string(), "1".to_string());
    map.insert("Patient.contact".to_string(), "*".to_string());
    map.insert("Patient.contact.address".to_string(), "1".to_string());
    map.insert("Patient.contact.extension".to_string(), "*".to_string());
    map.insert("Patient.contact.gender".to_string(), "1".to_string());
    map.insert("Patient.contact.id".to_string(), "1".to_string());
    map.insert("Patient.contact.modifierExtension".to_string(), "*".to_string());
    map.insert("Patient.contact.name".to_string(), "1".to_string());
    map.insert("Patient.contact.organization".to_string(), "1".to_string());
    map.insert("Patient.contact.period".to_string(), "1".to_string());
    map.insert("Patient.contact.relationship".to_string(), "*".to_string());
    map.insert("Patient.contact.telecom".to_string(), "*".to_string());
    map.insert("Patient.contained".to_string(), "*".to_string());
    map.insert("Patient.deceasedBoolean".to_string(), "1".to_string());
    map.insert("Patient.deceasedDateTime".to_string(), "1".to_string());
    map.insert("Patient.extension".to_string(), "*".to_string());
    map.insert("Patient.gender".to_string(), "1".to_string());
    map.insert("Patient.generalPractitioner".to_string(), "*".to_string());
    map.insert("Patient.id".to_string(), "1".to_string());
    map.insert("Patient.identifier".to_string(), "*".to_string());
    map.insert("Patient.implicitRules".to_string(), "1".to_string());
    map.insert("Patient.language".to_string(), "1".to_string());
    map.insert("Patient.link".to_string(), "*".to_string());
    map.insert("Patient.link.extension".to_string(), "*".to_string());
    map.insert("Patient.link.id".to_string(), "1".to_string());
    map.insert("Patient.link.modifierExtension".to_string(), "*".to_string());
    map.insert("Patient.link.other".to_string(), "1".to_string());
    map.insert("Patient.link.type".to_string(), "1".to_string());
    map.insert("Patient.managingOrganization".to_string(), "1".to_string());
    map.insert("Patient.maritalStatus".to_string(), "1".to_string());
    map.insert("Patient.meta".to_string(), "1".to_string());
    map.insert("Patient.modifierExtension".to_string(), "*".to_string());
    map.insert("Patient.multipleBirthBoolean".to_string(), "1".to_string());
    map.insert("Patient.multipleBirthInteger".to_string(), "1".to_string());
    map.insert("Patient.name".to_string(), "*".to_string());
    map.insert("Patient.photo".to_string(), "*".to_string());
    map.insert("Patient.telecom".to_string(), "*".to_string());
    map.insert("Patient.text".to_string(), "1".to_string());
    map.insert("PaymentNotice.amount".to_string(), "1".to_string());
    map.insert("PaymentNotice.contained".to_string(), "*".to_string());
    map.insert("PaymentNotice.created".to_string(), "1".to_string());
    map.insert("PaymentNotice.extension".to_string(), "*".to_string());
    map.insert("PaymentNotice.id".to_string(), "1".to_string());
    map.insert("PaymentNotice.identifier".to_string(), "*".to_string());
    map.insert("PaymentNotice.implicitRules".to_string(), "1".to_string());
    map.insert("PaymentNotice.language".to_string(), "1".to_string());
    map.insert("PaymentNotice.meta".to_string(), "1".to_string());
    map.insert("PaymentNotice.modifierExtension".to_string(), "*".to_string());
    map.insert("PaymentNotice.payee".to_string(), "1".to_string());
    map.insert("PaymentNotice.payment".to_string(), "1".to_string());
    map.insert("PaymentNotice.paymentDate".to_string(), "1".to_string());
    map.insert("PaymentNotice.paymentStatus".to_string(), "1".to_string());
    map.insert("PaymentNotice.provider".to_string(), "1".to_string());
    map.insert("PaymentNotice.recipient".to_string(), "1".to_string());
    map.insert("PaymentNotice.request".to_string(), "1".to_string());
    map.insert("PaymentNotice.response".to_string(), "1".to_string());
    map.insert("PaymentNotice.status".to_string(), "1".to_string());
    map.insert("PaymentNotice.text".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.contained".to_string(), "*".to_string());
    map.insert("PaymentReconciliation.created".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail".to_string(), "*".to_string());
    map.insert("PaymentReconciliation.detail.amount".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail.date".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail.extension".to_string(), "*".to_string());
    map.insert("PaymentReconciliation.detail.id".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail.identifier".to_string(), "1".to_string());
    map.insert(
        "PaymentReconciliation.detail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("PaymentReconciliation.detail.payee".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail.predecessor".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail.request".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail.response".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail.responsible".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail.submitter".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail.type".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.disposition".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.extension".to_string(), "*".to_string());
    map.insert("PaymentReconciliation.formCode".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.id".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.identifier".to_string(), "*".to_string());
    map.insert("PaymentReconciliation.implicitRules".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.language".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.meta".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.modifierExtension".to_string(), "*".to_string());
    map.insert("PaymentReconciliation.outcome".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.paymentAmount".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.paymentDate".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.paymentIdentifier".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.paymentIssuer".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.period".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.processNote".to_string(), "*".to_string());
    map.insert(
        "PaymentReconciliation.processNote.extension".to_string(),
        "*".to_string(),
    );
    map.insert("PaymentReconciliation.processNote.id".to_string(), "1".to_string());
    map.insert(
        "PaymentReconciliation.processNote.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("PaymentReconciliation.processNote.text".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.processNote.type".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.request".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.requestor".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.status".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.text".to_string(), "1".to_string());
    map.insert("Period.end".to_string(), "1".to_string());
    map.insert("Period.extension".to_string(), "*".to_string());
    map.insert("Period.id".to_string(), "1".to_string());
    map.insert("Period.start".to_string(), "1".to_string());
    map.insert("Person.active".to_string(), "1".to_string());
    map.insert("Person.address".to_string(), "*".to_string());
    map.insert("Person.birthDate".to_string(), "1".to_string());
    map.insert("Person.contained".to_string(), "*".to_string());
    map.insert("Person.extension".to_string(), "*".to_string());
    map.insert("Person.gender".to_string(), "1".to_string());
    map.insert("Person.id".to_string(), "1".to_string());
    map.insert("Person.identifier".to_string(), "*".to_string());
    map.insert("Person.implicitRules".to_string(), "1".to_string());
    map.insert("Person.language".to_string(), "1".to_string());
    map.insert("Person.link".to_string(), "*".to_string());
    map.insert("Person.link.assurance".to_string(), "1".to_string());
    map.insert("Person.link.extension".to_string(), "*".to_string());
    map.insert("Person.link.id".to_string(), "1".to_string());
    map.insert("Person.link.modifierExtension".to_string(), "*".to_string());
    map.insert("Person.link.target".to_string(), "1".to_string());
    map.insert("Person.managingOrganization".to_string(), "1".to_string());
    map.insert("Person.meta".to_string(), "1".to_string());
    map.insert("Person.modifierExtension".to_string(), "*".to_string());
    map.insert("Person.name".to_string(), "*".to_string());
    map.insert("Person.photo".to_string(), "1".to_string());
    map.insert("Person.telecom".to_string(), "*".to_string());
    map.insert("Person.text".to_string(), "1".to_string());
    map.insert("PlanDefinition.action".to_string(), "*".to_string());
    map.insert("PlanDefinition.action.action".to_string(), "*".to_string());
    map.insert("PlanDefinition.action.cardinalityBehavior".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.code".to_string(), "*".to_string());
    map.insert("PlanDefinition.action.condition".to_string(), "*".to_string());
    map.insert(
        "PlanDefinition.action.condition.expression".to_string(),
        "1".to_string(),
    );
    map.insert("PlanDefinition.action.condition.extension".to_string(), "*".to_string());
    map.insert("PlanDefinition.action.condition.id".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.condition.kind".to_string(), "1".to_string());
    map.insert(
        "PlanDefinition.action.condition.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("PlanDefinition.action.definitionCanonical".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.definitionUri".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.description".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.documentation".to_string(), "*".to_string());
    map.insert("PlanDefinition.action.dynamicValue".to_string(), "*".to_string());
    map.insert(
        "PlanDefinition.action.dynamicValue.expression".to_string(),
        "1".to_string(),
    );
    map.insert(
        "PlanDefinition.action.dynamicValue.extension".to_string(),
        "*".to_string(),
    );
    map.insert("PlanDefinition.action.dynamicValue.id".to_string(), "1".to_string());
    map.insert(
        "PlanDefinition.action.dynamicValue.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("PlanDefinition.action.dynamicValue.path".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.extension".to_string(), "*".to_string());
    map.insert("PlanDefinition.action.goalId".to_string(), "*".to_string());
    map.insert("PlanDefinition.action.groupingBehavior".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.id".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.input".to_string(), "*".to_string());
    map.insert("PlanDefinition.action.modifierExtension".to_string(), "*".to_string());
    map.insert("PlanDefinition.action.output".to_string(), "*".to_string());
    map.insert("PlanDefinition.action.participant".to_string(), "*".to_string());
    map.insert(
        "PlanDefinition.action.participant.extension".to_string(),
        "*".to_string(),
    );
    map.insert("PlanDefinition.action.participant.id".to_string(), "1".to_string());
    map.insert(
        "PlanDefinition.action.participant.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("PlanDefinition.action.participant.role".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.participant.type".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.precheckBehavior".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.prefix".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.priority".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.reason".to_string(), "*".to_string());
    map.insert("PlanDefinition.action.relatedAction".to_string(), "*".to_string());
    map.insert(
        "PlanDefinition.action.relatedAction.actionId".to_string(),
        "1".to_string(),
    );
    map.insert(
        "PlanDefinition.action.relatedAction.extension".to_string(),
        "*".to_string(),
    );
    map.insert("PlanDefinition.action.relatedAction.id".to_string(), "1".to_string());
    map.insert(
        "PlanDefinition.action.relatedAction.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "PlanDefinition.action.relatedAction.offsetDuration".to_string(),
        "1".to_string(),
    );
    map.insert(
        "PlanDefinition.action.relatedAction.offsetRange".to_string(),
        "1".to_string(),
    );
    map.insert(
        "PlanDefinition.action.relatedAction.relationship".to_string(),
        "1".to_string(),
    );
    map.insert("PlanDefinition.action.requiredBehavior".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.selectionBehavior".to_string(), "1".to_string());
    map.insert(
        "PlanDefinition.action.subjectCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("PlanDefinition.action.subjectReference".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.textEquivalent".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.timingAge".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.timingDateTime".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.timingDuration".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.timingPeriod".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.timingRange".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.timingTiming".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.title".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.transform".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.trigger".to_string(), "*".to_string());
    map.insert("PlanDefinition.action.type".to_string(), "1".to_string());
    map.insert("PlanDefinition.approvalDate".to_string(), "1".to_string());
    map.insert("PlanDefinition.author".to_string(), "*".to_string());
    map.insert("PlanDefinition.contact".to_string(), "*".to_string());
    map.insert("PlanDefinition.contained".to_string(), "*".to_string());
    map.insert("PlanDefinition.copyright".to_string(), "1".to_string());
    map.insert("PlanDefinition.date".to_string(), "1".to_string());
    map.insert("PlanDefinition.description".to_string(), "1".to_string());
    map.insert("PlanDefinition.editor".to_string(), "*".to_string());
    map.insert("PlanDefinition.effectivePeriod".to_string(), "1".to_string());
    map.insert("PlanDefinition.endorser".to_string(), "*".to_string());
    map.insert("PlanDefinition.experimental".to_string(), "1".to_string());
    map.insert("PlanDefinition.extension".to_string(), "*".to_string());
    map.insert("PlanDefinition.goal".to_string(), "*".to_string());
    map.insert("PlanDefinition.goal.addresses".to_string(), "*".to_string());
    map.insert("PlanDefinition.goal.category".to_string(), "1".to_string());
    map.insert("PlanDefinition.goal.description".to_string(), "1".to_string());
    map.insert("PlanDefinition.goal.documentation".to_string(), "*".to_string());
    map.insert("PlanDefinition.goal.extension".to_string(), "*".to_string());
    map.insert("PlanDefinition.goal.id".to_string(), "1".to_string());
    map.insert("PlanDefinition.goal.modifierExtension".to_string(), "*".to_string());
    map.insert("PlanDefinition.goal.priority".to_string(), "1".to_string());
    map.insert("PlanDefinition.goal.start".to_string(), "1".to_string());
    map.insert("PlanDefinition.goal.target".to_string(), "*".to_string());
    map.insert(
        "PlanDefinition.goal.target.detailCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("PlanDefinition.goal.target.detailQuantity".to_string(), "1".to_string());
    map.insert("PlanDefinition.goal.target.detailRange".to_string(), "1".to_string());
    map.insert("PlanDefinition.goal.target.due".to_string(), "1".to_string());
    map.insert("PlanDefinition.goal.target.extension".to_string(), "*".to_string());
    map.insert("PlanDefinition.goal.target.id".to_string(), "1".to_string());
    map.insert("PlanDefinition.goal.target.measure".to_string(), "1".to_string());
    map.insert(
        "PlanDefinition.goal.target.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("PlanDefinition.id".to_string(), "1".to_string());
    map.insert("PlanDefinition.identifier".to_string(), "*".to_string());
    map.insert("PlanDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("PlanDefinition.jurisdiction".to_string(), "*".to_string());
    map.insert("PlanDefinition.language".to_string(), "1".to_string());
    map.insert("PlanDefinition.lastReviewDate".to_string(), "1".to_string());
    map.insert("PlanDefinition.library".to_string(), "*".to_string());
    map.insert("PlanDefinition.meta".to_string(), "1".to_string());
    map.insert("PlanDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("PlanDefinition.name".to_string(), "1".to_string());
    map.insert("PlanDefinition.publisher".to_string(), "1".to_string());
    map.insert("PlanDefinition.purpose".to_string(), "1".to_string());
    map.insert("PlanDefinition.relatedArtifact".to_string(), "*".to_string());
    map.insert("PlanDefinition.reviewer".to_string(), "*".to_string());
    map.insert("PlanDefinition.status".to_string(), "1".to_string());
    map.insert("PlanDefinition.subjectCodeableConcept".to_string(), "1".to_string());
    map.insert("PlanDefinition.subjectReference".to_string(), "1".to_string());
    map.insert("PlanDefinition.subtitle".to_string(), "1".to_string());
    map.insert("PlanDefinition.text".to_string(), "1".to_string());
    map.insert("PlanDefinition.title".to_string(), "1".to_string());
    map.insert("PlanDefinition.topic".to_string(), "*".to_string());
    map.insert("PlanDefinition.type".to_string(), "1".to_string());
    map.insert("PlanDefinition.url".to_string(), "1".to_string());
    map.insert("PlanDefinition.usage".to_string(), "1".to_string());
    map.insert("PlanDefinition.useContext".to_string(), "*".to_string());
    map.insert("PlanDefinition.version".to_string(), "1".to_string());
    map.insert("Population.ageCodeableConcept".to_string(), "1".to_string());
    map.insert("Population.ageRange".to_string(), "1".to_string());
    map.insert("Population.extension".to_string(), "*".to_string());
    map.insert("Population.gender".to_string(), "1".to_string());
    map.insert("Population.id".to_string(), "1".to_string());
    map.insert("Population.modifierExtension".to_string(), "*".to_string());
    map.insert("Population.physiologicalCondition".to_string(), "1".to_string());
    map.insert("Population.race".to_string(), "1".to_string());
    map.insert("Practitioner.active".to_string(), "1".to_string());
    map.insert("Practitioner.address".to_string(), "*".to_string());
    map.insert("Practitioner.birthDate".to_string(), "1".to_string());
    map.insert("Practitioner.communication".to_string(), "*".to_string());
    map.insert("Practitioner.contained".to_string(), "*".to_string());
    map.insert("Practitioner.extension".to_string(), "*".to_string());
    map.insert("Practitioner.gender".to_string(), "1".to_string());
    map.insert("Practitioner.id".to_string(), "1".to_string());
    map.insert("Practitioner.identifier".to_string(), "*".to_string());
    map.insert("Practitioner.implicitRules".to_string(), "1".to_string());
    map.insert("Practitioner.language".to_string(), "1".to_string());
    map.insert("Practitioner.meta".to_string(), "1".to_string());
    map.insert("Practitioner.modifierExtension".to_string(), "*".to_string());
    map.insert("Practitioner.name".to_string(), "*".to_string());
    map.insert("Practitioner.photo".to_string(), "*".to_string());
    map.insert("Practitioner.qualification".to_string(), "*".to_string());
    map.insert("Practitioner.qualification.code".to_string(), "1".to_string());
    map.insert("Practitioner.qualification.extension".to_string(), "*".to_string());
    map.insert("Practitioner.qualification.id".to_string(), "1".to_string());
    map.insert("Practitioner.qualification.identifier".to_string(), "*".to_string());
    map.insert("Practitioner.qualification.issuer".to_string(), "1".to_string());
    map.insert(
        "Practitioner.qualification.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Practitioner.qualification.period".to_string(), "1".to_string());
    map.insert("Practitioner.telecom".to_string(), "*".to_string());
    map.insert("Practitioner.text".to_string(), "1".to_string());
    map.insert("PractitionerRole.active".to_string(), "1".to_string());
    map.insert("PractitionerRole.availabilityExceptions".to_string(), "1".to_string());
    map.insert("PractitionerRole.availableTime".to_string(), "*".to_string());
    map.insert("PractitionerRole.availableTime.allDay".to_string(), "1".to_string());
    map.insert(
        "PractitionerRole.availableTime.availableEndTime".to_string(),
        "1".to_string(),
    );
    map.insert(
        "PractitionerRole.availableTime.availableStartTime".to_string(),
        "1".to_string(),
    );
    map.insert("PractitionerRole.availableTime.daysOfWeek".to_string(), "*".to_string());
    map.insert("PractitionerRole.availableTime.extension".to_string(), "*".to_string());
    map.insert("PractitionerRole.availableTime.id".to_string(), "1".to_string());
    map.insert(
        "PractitionerRole.availableTime.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("PractitionerRole.code".to_string(), "*".to_string());
    map.insert("PractitionerRole.contained".to_string(), "*".to_string());
    map.insert("PractitionerRole.endpoint".to_string(), "*".to_string());
    map.insert("PractitionerRole.extension".to_string(), "*".to_string());
    map.insert("PractitionerRole.healthcareService".to_string(), "*".to_string());
    map.insert("PractitionerRole.id".to_string(), "1".to_string());
    map.insert("PractitionerRole.identifier".to_string(), "*".to_string());
    map.insert("PractitionerRole.implicitRules".to_string(), "1".to_string());
    map.insert("PractitionerRole.language".to_string(), "1".to_string());
    map.insert("PractitionerRole.location".to_string(), "*".to_string());
    map.insert("PractitionerRole.meta".to_string(), "1".to_string());
    map.insert("PractitionerRole.modifierExtension".to_string(), "*".to_string());
    map.insert("PractitionerRole.notAvailable".to_string(), "*".to_string());
    map.insert("PractitionerRole.notAvailable.description".to_string(), "1".to_string());
    map.insert("PractitionerRole.notAvailable.during".to_string(), "1".to_string());
    map.insert("PractitionerRole.notAvailable.extension".to_string(), "*".to_string());
    map.insert("PractitionerRole.notAvailable.id".to_string(), "1".to_string());
    map.insert(
        "PractitionerRole.notAvailable.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("PractitionerRole.organization".to_string(), "1".to_string());
    map.insert("PractitionerRole.period".to_string(), "1".to_string());
    map.insert("PractitionerRole.practitioner".to_string(), "1".to_string());
    map.insert("PractitionerRole.specialty".to_string(), "*".to_string());
    map.insert("PractitionerRole.telecom".to_string(), "*".to_string());
    map.insert("PractitionerRole.text".to_string(), "1".to_string());
    map.insert("Procedure.asserter".to_string(), "1".to_string());
    map.insert("Procedure.basedOn".to_string(), "*".to_string());
    map.insert("Procedure.bodySite".to_string(), "*".to_string());
    map.insert("Procedure.category".to_string(), "1".to_string());
    map.insert("Procedure.code".to_string(), "1".to_string());
    map.insert("Procedure.complication".to_string(), "*".to_string());
    map.insert("Procedure.complicationDetail".to_string(), "*".to_string());
    map.insert("Procedure.contained".to_string(), "*".to_string());
    map.insert("Procedure.encounter".to_string(), "1".to_string());
    map.insert("Procedure.extension".to_string(), "*".to_string());
    map.insert("Procedure.focalDevice".to_string(), "*".to_string());
    map.insert("Procedure.focalDevice.action".to_string(), "1".to_string());
    map.insert("Procedure.focalDevice.extension".to_string(), "*".to_string());
    map.insert("Procedure.focalDevice.id".to_string(), "1".to_string());
    map.insert("Procedure.focalDevice.manipulated".to_string(), "1".to_string());
    map.insert("Procedure.focalDevice.modifierExtension".to_string(), "*".to_string());
    map.insert("Procedure.followUp".to_string(), "*".to_string());
    map.insert("Procedure.id".to_string(), "1".to_string());
    map.insert("Procedure.identifier".to_string(), "*".to_string());
    map.insert("Procedure.implicitRules".to_string(), "1".to_string());
    map.insert("Procedure.instantiatesCanonical".to_string(), "*".to_string());
    map.insert("Procedure.instantiatesUri".to_string(), "*".to_string());
    map.insert("Procedure.language".to_string(), "1".to_string());
    map.insert("Procedure.location".to_string(), "1".to_string());
    map.insert("Procedure.meta".to_string(), "1".to_string());
    map.insert("Procedure.modifierExtension".to_string(), "*".to_string());
    map.insert("Procedure.note".to_string(), "*".to_string());
    map.insert("Procedure.outcome".to_string(), "1".to_string());
    map.insert("Procedure.partOf".to_string(), "*".to_string());
    map.insert("Procedure.performedAge".to_string(), "1".to_string());
    map.insert("Procedure.performedDateTime".to_string(), "1".to_string());
    map.insert("Procedure.performedPeriod".to_string(), "1".to_string());
    map.insert("Procedure.performedRange".to_string(), "1".to_string());
    map.insert("Procedure.performedString".to_string(), "1".to_string());
    map.insert("Procedure.performer".to_string(), "*".to_string());
    map.insert("Procedure.performer.actor".to_string(), "1".to_string());
    map.insert("Procedure.performer.extension".to_string(), "*".to_string());
    map.insert("Procedure.performer.function".to_string(), "1".to_string());
    map.insert("Procedure.performer.id".to_string(), "1".to_string());
    map.insert("Procedure.performer.modifierExtension".to_string(), "*".to_string());
    map.insert("Procedure.performer.onBehalfOf".to_string(), "1".to_string());
    map.insert("Procedure.reasonCode".to_string(), "*".to_string());
    map.insert("Procedure.reasonReference".to_string(), "*".to_string());
    map.insert("Procedure.recorder".to_string(), "1".to_string());
    map.insert("Procedure.report".to_string(), "*".to_string());
    map.insert("Procedure.status".to_string(), "1".to_string());
    map.insert("Procedure.statusReason".to_string(), "1".to_string());
    map.insert("Procedure.subject".to_string(), "1".to_string());
    map.insert("Procedure.text".to_string(), "1".to_string());
    map.insert("Procedure.usedCode".to_string(), "*".to_string());
    map.insert("Procedure.usedReference".to_string(), "*".to_string());
    map.insert("ProdCharacteristic.color".to_string(), "*".to_string());
    map.insert("ProdCharacteristic.depth".to_string(), "1".to_string());
    map.insert("ProdCharacteristic.extension".to_string(), "*".to_string());
    map.insert("ProdCharacteristic.externalDiameter".to_string(), "1".to_string());
    map.insert("ProdCharacteristic.height".to_string(), "1".to_string());
    map.insert("ProdCharacteristic.id".to_string(), "1".to_string());
    map.insert("ProdCharacteristic.image".to_string(), "*".to_string());
    map.insert("ProdCharacteristic.imprint".to_string(), "*".to_string());
    map.insert("ProdCharacteristic.modifierExtension".to_string(), "*".to_string());
    map.insert("ProdCharacteristic.nominalVolume".to_string(), "1".to_string());
    map.insert("ProdCharacteristic.scoring".to_string(), "1".to_string());
    map.insert("ProdCharacteristic.shape".to_string(), "1".to_string());
    map.insert("ProdCharacteristic.weight".to_string(), "1".to_string());
    map.insert("ProdCharacteristic.width".to_string(), "1".to_string());
    map.insert("ProductShelfLife.extension".to_string(), "*".to_string());
    map.insert("ProductShelfLife.id".to_string(), "1".to_string());
    map.insert("ProductShelfLife.identifier".to_string(), "1".to_string());
    map.insert("ProductShelfLife.modifierExtension".to_string(), "*".to_string());
    map.insert("ProductShelfLife.period".to_string(), "1".to_string());
    map.insert(
        "ProductShelfLife.specialPrecautionsForStorage".to_string(),
        "*".to_string(),
    );
    map.insert("ProductShelfLife.type".to_string(), "1".to_string());
    map.insert("Provenance.activity".to_string(), "1".to_string());
    map.insert("Provenance.agent".to_string(), "*".to_string());
    map.insert("Provenance.agent.extension".to_string(), "*".to_string());
    map.insert("Provenance.agent.id".to_string(), "1".to_string());
    map.insert("Provenance.agent.modifierExtension".to_string(), "*".to_string());
    map.insert("Provenance.agent.onBehalfOf".to_string(), "1".to_string());
    map.insert("Provenance.agent.role".to_string(), "*".to_string());
    map.insert("Provenance.agent.type".to_string(), "1".to_string());
    map.insert("Provenance.agent.who".to_string(), "1".to_string());
    map.insert("Provenance.contained".to_string(), "*".to_string());
    map.insert("Provenance.entity".to_string(), "*".to_string());
    map.insert("Provenance.entity.agent".to_string(), "*".to_string());
    map.insert("Provenance.entity.extension".to_string(), "*".to_string());
    map.insert("Provenance.entity.id".to_string(), "1".to_string());
    map.insert("Provenance.entity.modifierExtension".to_string(), "*".to_string());
    map.insert("Provenance.entity.role".to_string(), "1".to_string());
    map.insert("Provenance.entity.what".to_string(), "1".to_string());
    map.insert("Provenance.extension".to_string(), "*".to_string());
    map.insert("Provenance.id".to_string(), "1".to_string());
    map.insert("Provenance.implicitRules".to_string(), "1".to_string());
    map.insert("Provenance.language".to_string(), "1".to_string());
    map.insert("Provenance.location".to_string(), "1".to_string());
    map.insert("Provenance.meta".to_string(), "1".to_string());
    map.insert("Provenance.modifierExtension".to_string(), "*".to_string());
    map.insert("Provenance.occurredDateTime".to_string(), "1".to_string());
    map.insert("Provenance.occurredPeriod".to_string(), "1".to_string());
    map.insert("Provenance.policy".to_string(), "*".to_string());
    map.insert("Provenance.reason".to_string(), "*".to_string());
    map.insert("Provenance.recorded".to_string(), "1".to_string());
    map.insert("Provenance.signature".to_string(), "*".to_string());
    map.insert("Provenance.target".to_string(), "*".to_string());
    map.insert("Provenance.text".to_string(), "1".to_string());
    map.insert("Quantity.code".to_string(), "1".to_string());
    map.insert("Quantity.comparator".to_string(), "1".to_string());
    map.insert("Quantity.extension".to_string(), "*".to_string());
    map.insert("Quantity.id".to_string(), "1".to_string());
    map.insert("Quantity.system".to_string(), "1".to_string());
    map.insert("Quantity.unit".to_string(), "1".to_string());
    map.insert("Quantity.value".to_string(), "1".to_string());
    map.insert("Questionnaire.approvalDate".to_string(), "1".to_string());
    map.insert("Questionnaire.code".to_string(), "*".to_string());
    map.insert("Questionnaire.contact".to_string(), "*".to_string());
    map.insert("Questionnaire.contained".to_string(), "*".to_string());
    map.insert("Questionnaire.copyright".to_string(), "1".to_string());
    map.insert("Questionnaire.date".to_string(), "1".to_string());
    map.insert("Questionnaire.derivedFrom".to_string(), "*".to_string());
    map.insert("Questionnaire.description".to_string(), "1".to_string());
    map.insert("Questionnaire.effectivePeriod".to_string(), "1".to_string());
    map.insert("Questionnaire.experimental".to_string(), "1".to_string());
    map.insert("Questionnaire.extension".to_string(), "*".to_string());
    map.insert("Questionnaire.id".to_string(), "1".to_string());
    map.insert("Questionnaire.identifier".to_string(), "*".to_string());
    map.insert("Questionnaire.implicitRules".to_string(), "1".to_string());
    map.insert("Questionnaire.item".to_string(), "*".to_string());
    map.insert("Questionnaire.item.answerOption".to_string(), "*".to_string());
    map.insert("Questionnaire.item.answerOption.extension".to_string(), "*".to_string());
    map.insert("Questionnaire.item.answerOption.id".to_string(), "1".to_string());
    map.insert(
        "Questionnaire.item.answerOption.initialSelected".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Questionnaire.item.answerOption.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "Questionnaire.item.answerOption.valueCoding".to_string(),
        "1".to_string(),
    );
    map.insert("Questionnaire.item.answerOption.valueDate".to_string(), "1".to_string());
    map.insert(
        "Questionnaire.item.answerOption.valueInteger".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Questionnaire.item.answerOption.valueReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Questionnaire.item.answerOption.valueString".to_string(),
        "1".to_string(),
    );
    map.insert("Questionnaire.item.answerOption.valueTime".to_string(), "1".to_string());
    map.insert("Questionnaire.item.answerValueSet".to_string(), "1".to_string());
    map.insert("Questionnaire.item.code".to_string(), "*".to_string());
    map.insert("Questionnaire.item.definition".to_string(), "1".to_string());
    map.insert("Questionnaire.item.enableBehavior".to_string(), "1".to_string());
    map.insert("Questionnaire.item.enableWhen".to_string(), "*".to_string());
    map.insert(
        "Questionnaire.item.enableWhen.answerBoolean".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Questionnaire.item.enableWhen.answerCoding".to_string(),
        "1".to_string(),
    );
    map.insert("Questionnaire.item.enableWhen.answerDate".to_string(), "1".to_string());
    map.insert(
        "Questionnaire.item.enableWhen.answerDateTime".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Questionnaire.item.enableWhen.answerDecimal".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Questionnaire.item.enableWhen.answerInteger".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Questionnaire.item.enableWhen.answerQuantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Questionnaire.item.enableWhen.answerReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Questionnaire.item.enableWhen.answerString".to_string(),
        "1".to_string(),
    );
    map.insert("Questionnaire.item.enableWhen.answerTime".to_string(), "1".to_string());
    map.insert("Questionnaire.item.enableWhen.extension".to_string(), "*".to_string());
    map.insert("Questionnaire.item.enableWhen.id".to_string(), "1".to_string());
    map.insert(
        "Questionnaire.item.enableWhen.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Questionnaire.item.enableWhen.operator".to_string(), "1".to_string());
    map.insert("Questionnaire.item.enableWhen.question".to_string(), "1".to_string());
    map.insert("Questionnaire.item.extension".to_string(), "*".to_string());
    map.insert("Questionnaire.item.id".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initial".to_string(), "*".to_string());
    map.insert("Questionnaire.item.initial.extension".to_string(), "*".to_string());
    map.insert("Questionnaire.item.initial.id".to_string(), "1".to_string());
    map.insert(
        "Questionnaire.item.initial.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "Questionnaire.item.initial.valueAttachment".to_string(),
        "1".to_string(),
    );
    map.insert("Questionnaire.item.initial.valueBoolean".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initial.valueCoding".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initial.valueDate".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initial.valueDateTime".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initial.valueDecimal".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initial.valueInteger".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initial.valueQuantity".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initial.valueReference".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initial.valueString".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initial.valueTime".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initial.valueUri".to_string(), "1".to_string());
    map.insert("Questionnaire.item.item".to_string(), "*".to_string());
    map.insert("Questionnaire.item.linkId".to_string(), "1".to_string());
    map.insert("Questionnaire.item.maxLength".to_string(), "1".to_string());
    map.insert("Questionnaire.item.modifierExtension".to_string(), "*".to_string());
    map.insert("Questionnaire.item.prefix".to_string(), "1".to_string());
    map.insert("Questionnaire.item.readOnly".to_string(), "1".to_string());
    map.insert("Questionnaire.item.repeats".to_string(), "1".to_string());
    map.insert("Questionnaire.item.required".to_string(), "1".to_string());
    map.insert("Questionnaire.item.text".to_string(), "1".to_string());
    map.insert("Questionnaire.item.type".to_string(), "1".to_string());
    map.insert("Questionnaire.jurisdiction".to_string(), "*".to_string());
    map.insert("Questionnaire.language".to_string(), "1".to_string());
    map.insert("Questionnaire.lastReviewDate".to_string(), "1".to_string());
    map.insert("Questionnaire.meta".to_string(), "1".to_string());
    map.insert("Questionnaire.modifierExtension".to_string(), "*".to_string());
    map.insert("Questionnaire.name".to_string(), "1".to_string());
    map.insert("Questionnaire.publisher".to_string(), "1".to_string());
    map.insert("Questionnaire.purpose".to_string(), "1".to_string());
    map.insert("Questionnaire.status".to_string(), "1".to_string());
    map.insert("Questionnaire.subjectType".to_string(), "*".to_string());
    map.insert("Questionnaire.text".to_string(), "1".to_string());
    map.insert("Questionnaire.title".to_string(), "1".to_string());
    map.insert("Questionnaire.url".to_string(), "1".to_string());
    map.insert("Questionnaire.useContext".to_string(), "*".to_string());
    map.insert("Questionnaire.version".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.author".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.authored".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.basedOn".to_string(), "*".to_string());
    map.insert("QuestionnaireResponse.contained".to_string(), "*".to_string());
    map.insert("QuestionnaireResponse.encounter".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.extension".to_string(), "*".to_string());
    map.insert("QuestionnaireResponse.id".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.identifier".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.implicitRules".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.item".to_string(), "*".to_string());
    map.insert("QuestionnaireResponse.item.answer".to_string(), "*".to_string());
    map.insert(
        "QuestionnaireResponse.item.answer.extension".to_string(),
        "*".to_string(),
    );
    map.insert("QuestionnaireResponse.item.answer.id".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.item.answer.item".to_string(), "*".to_string());
    map.insert(
        "QuestionnaireResponse.item.answer.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "QuestionnaireResponse.item.answer.valueAttachment".to_string(),
        "1".to_string(),
    );
    map.insert(
        "QuestionnaireResponse.item.answer.valueBoolean".to_string(),
        "1".to_string(),
    );
    map.insert(
        "QuestionnaireResponse.item.answer.valueCoding".to_string(),
        "1".to_string(),
    );
    map.insert(
        "QuestionnaireResponse.item.answer.valueDate".to_string(),
        "1".to_string(),
    );
    map.insert(
        "QuestionnaireResponse.item.answer.valueDateTime".to_string(),
        "1".to_string(),
    );
    map.insert(
        "QuestionnaireResponse.item.answer.valueDecimal".to_string(),
        "1".to_string(),
    );
    map.insert(
        "QuestionnaireResponse.item.answer.valueInteger".to_string(),
        "1".to_string(),
    );
    map.insert(
        "QuestionnaireResponse.item.answer.valueQuantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "QuestionnaireResponse.item.answer.valueReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "QuestionnaireResponse.item.answer.valueString".to_string(),
        "1".to_string(),
    );
    map.insert(
        "QuestionnaireResponse.item.answer.valueTime".to_string(),
        "1".to_string(),
    );
    map.insert(
        "QuestionnaireResponse.item.answer.valueUri".to_string(),
        "1".to_string(),
    );
    map.insert("QuestionnaireResponse.item.definition".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.item.extension".to_string(), "*".to_string());
    map.insert("QuestionnaireResponse.item.id".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.item.item".to_string(), "*".to_string());
    map.insert("QuestionnaireResponse.item.linkId".to_string(), "1".to_string());
    map.insert(
        "QuestionnaireResponse.item.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("QuestionnaireResponse.item.text".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.language".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.meta".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.modifierExtension".to_string(), "*".to_string());
    map.insert("QuestionnaireResponse.partOf".to_string(), "*".to_string());
    map.insert("QuestionnaireResponse.questionnaire".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.source".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.status".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.subject".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.text".to_string(), "1".to_string());
    map.insert("Range.extension".to_string(), "*".to_string());
    map.insert("Range.high".to_string(), "1".to_string());
    map.insert("Range.id".to_string(), "1".to_string());
    map.insert("Range.low".to_string(), "1".to_string());
    map.insert("Ratio.denominator".to_string(), "1".to_string());
    map.insert("Ratio.extension".to_string(), "*".to_string());
    map.insert("Ratio.id".to_string(), "1".to_string());
    map.insert("Ratio.numerator".to_string(), "1".to_string());
    map.insert("Reference.display".to_string(), "1".to_string());
    map.insert("Reference.extension".to_string(), "*".to_string());
    map.insert("Reference.id".to_string(), "1".to_string());
    map.insert("Reference.identifier".to_string(), "1".to_string());
    map.insert("Reference.reference".to_string(), "1".to_string());
    map.insert("Reference.type".to_string(), "1".to_string());
    map.insert("RelatedArtifact.citation".to_string(), "1".to_string());
    map.insert("RelatedArtifact.display".to_string(), "1".to_string());
    map.insert("RelatedArtifact.document".to_string(), "1".to_string());
    map.insert("RelatedArtifact.extension".to_string(), "*".to_string());
    map.insert("RelatedArtifact.id".to_string(), "1".to_string());
    map.insert("RelatedArtifact.label".to_string(), "1".to_string());
    map.insert("RelatedArtifact.resource".to_string(), "1".to_string());
    map.insert("RelatedArtifact.type".to_string(), "1".to_string());
    map.insert("RelatedArtifact.url".to_string(), "1".to_string());
    map.insert("RelatedPerson.active".to_string(), "1".to_string());
    map.insert("RelatedPerson.address".to_string(), "*".to_string());
    map.insert("RelatedPerson.birthDate".to_string(), "1".to_string());
    map.insert("RelatedPerson.communication".to_string(), "*".to_string());
    map.insert("RelatedPerson.communication.extension".to_string(), "*".to_string());
    map.insert("RelatedPerson.communication.id".to_string(), "1".to_string());
    map.insert("RelatedPerson.communication.language".to_string(), "1".to_string());
    map.insert(
        "RelatedPerson.communication.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("RelatedPerson.communication.preferred".to_string(), "1".to_string());
    map.insert("RelatedPerson.contained".to_string(), "*".to_string());
    map.insert("RelatedPerson.extension".to_string(), "*".to_string());
    map.insert("RelatedPerson.gender".to_string(), "1".to_string());
    map.insert("RelatedPerson.id".to_string(), "1".to_string());
    map.insert("RelatedPerson.identifier".to_string(), "*".to_string());
    map.insert("RelatedPerson.implicitRules".to_string(), "1".to_string());
    map.insert("RelatedPerson.language".to_string(), "1".to_string());
    map.insert("RelatedPerson.meta".to_string(), "1".to_string());
    map.insert("RelatedPerson.modifierExtension".to_string(), "*".to_string());
    map.insert("RelatedPerson.name".to_string(), "*".to_string());
    map.insert("RelatedPerson.patient".to_string(), "1".to_string());
    map.insert("RelatedPerson.period".to_string(), "1".to_string());
    map.insert("RelatedPerson.photo".to_string(), "*".to_string());
    map.insert("RelatedPerson.relationship".to_string(), "*".to_string());
    map.insert("RelatedPerson.telecom".to_string(), "*".to_string());
    map.insert("RelatedPerson.text".to_string(), "1".to_string());
    map.insert("RequestGroup.action".to_string(), "*".to_string());
    map.insert("RequestGroup.action.action".to_string(), "*".to_string());
    map.insert("RequestGroup.action.cardinalityBehavior".to_string(), "1".to_string());
    map.insert("RequestGroup.action.code".to_string(), "*".to_string());
    map.insert("RequestGroup.action.condition".to_string(), "*".to_string());
    map.insert("RequestGroup.action.condition.expression".to_string(), "1".to_string());
    map.insert("RequestGroup.action.condition.extension".to_string(), "*".to_string());
    map.insert("RequestGroup.action.condition.id".to_string(), "1".to_string());
    map.insert("RequestGroup.action.condition.kind".to_string(), "1".to_string());
    map.insert(
        "RequestGroup.action.condition.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("RequestGroup.action.description".to_string(), "1".to_string());
    map.insert("RequestGroup.action.documentation".to_string(), "*".to_string());
    map.insert("RequestGroup.action.extension".to_string(), "*".to_string());
    map.insert("RequestGroup.action.groupingBehavior".to_string(), "1".to_string());
    map.insert("RequestGroup.action.id".to_string(), "1".to_string());
    map.insert("RequestGroup.action.modifierExtension".to_string(), "*".to_string());
    map.insert("RequestGroup.action.participant".to_string(), "*".to_string());
    map.insert("RequestGroup.action.precheckBehavior".to_string(), "1".to_string());
    map.insert("RequestGroup.action.prefix".to_string(), "1".to_string());
    map.insert("RequestGroup.action.priority".to_string(), "1".to_string());
    map.insert("RequestGroup.action.relatedAction".to_string(), "*".to_string());
    map.insert(
        "RequestGroup.action.relatedAction.actionId".to_string(),
        "1".to_string(),
    );
    map.insert(
        "RequestGroup.action.relatedAction.extension".to_string(),
        "*".to_string(),
    );
    map.insert("RequestGroup.action.relatedAction.id".to_string(), "1".to_string());
    map.insert(
        "RequestGroup.action.relatedAction.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "RequestGroup.action.relatedAction.offsetDuration".to_string(),
        "1".to_string(),
    );
    map.insert(
        "RequestGroup.action.relatedAction.offsetRange".to_string(),
        "1".to_string(),
    );
    map.insert(
        "RequestGroup.action.relatedAction.relationship".to_string(),
        "1".to_string(),
    );
    map.insert("RequestGroup.action.requiredBehavior".to_string(), "1".to_string());
    map.insert("RequestGroup.action.resource".to_string(), "1".to_string());
    map.insert("RequestGroup.action.selectionBehavior".to_string(), "1".to_string());
    map.insert("RequestGroup.action.textEquivalent".to_string(), "1".to_string());
    map.insert("RequestGroup.action.timingAge".to_string(), "1".to_string());
    map.insert("RequestGroup.action.timingDateTime".to_string(), "1".to_string());
    map.insert("RequestGroup.action.timingDuration".to_string(), "1".to_string());
    map.insert("RequestGroup.action.timingPeriod".to_string(), "1".to_string());
    map.insert("RequestGroup.action.timingRange".to_string(), "1".to_string());
    map.insert("RequestGroup.action.timingTiming".to_string(), "1".to_string());
    map.insert("RequestGroup.action.title".to_string(), "1".to_string());
    map.insert("RequestGroup.action.type".to_string(), "1".to_string());
    map.insert("RequestGroup.author".to_string(), "1".to_string());
    map.insert("RequestGroup.authoredOn".to_string(), "1".to_string());
    map.insert("RequestGroup.basedOn".to_string(), "*".to_string());
    map.insert("RequestGroup.code".to_string(), "1".to_string());
    map.insert("RequestGroup.contained".to_string(), "*".to_string());
    map.insert("RequestGroup.encounter".to_string(), "1".to_string());
    map.insert("RequestGroup.extension".to_string(), "*".to_string());
    map.insert("RequestGroup.groupIdentifier".to_string(), "1".to_string());
    map.insert("RequestGroup.id".to_string(), "1".to_string());
    map.insert("RequestGroup.identifier".to_string(), "*".to_string());
    map.insert("RequestGroup.implicitRules".to_string(), "1".to_string());
    map.insert("RequestGroup.instantiatesCanonical".to_string(), "*".to_string());
    map.insert("RequestGroup.instantiatesUri".to_string(), "*".to_string());
    map.insert("RequestGroup.intent".to_string(), "1".to_string());
    map.insert("RequestGroup.language".to_string(), "1".to_string());
    map.insert("RequestGroup.meta".to_string(), "1".to_string());
    map.insert("RequestGroup.modifierExtension".to_string(), "*".to_string());
    map.insert("RequestGroup.note".to_string(), "*".to_string());
    map.insert("RequestGroup.priority".to_string(), "1".to_string());
    map.insert("RequestGroup.reasonCode".to_string(), "*".to_string());
    map.insert("RequestGroup.reasonReference".to_string(), "*".to_string());
    map.insert("RequestGroup.replaces".to_string(), "*".to_string());
    map.insert("RequestGroup.status".to_string(), "1".to_string());
    map.insert("RequestGroup.subject".to_string(), "1".to_string());
    map.insert("RequestGroup.text".to_string(), "1".to_string());
    map.insert("ResearchDefinition.approvalDate".to_string(), "1".to_string());
    map.insert("ResearchDefinition.author".to_string(), "*".to_string());
    map.insert("ResearchDefinition.comment".to_string(), "*".to_string());
    map.insert("ResearchDefinition.contact".to_string(), "*".to_string());
    map.insert("ResearchDefinition.contained".to_string(), "*".to_string());
    map.insert("ResearchDefinition.copyright".to_string(), "1".to_string());
    map.insert("ResearchDefinition.date".to_string(), "1".to_string());
    map.insert("ResearchDefinition.description".to_string(), "1".to_string());
    map.insert("ResearchDefinition.editor".to_string(), "*".to_string());
    map.insert("ResearchDefinition.effectivePeriod".to_string(), "1".to_string());
    map.insert("ResearchDefinition.endorser".to_string(), "*".to_string());
    map.insert("ResearchDefinition.experimental".to_string(), "1".to_string());
    map.insert("ResearchDefinition.exposure".to_string(), "1".to_string());
    map.insert("ResearchDefinition.exposureAlternative".to_string(), "1".to_string());
    map.insert("ResearchDefinition.extension".to_string(), "*".to_string());
    map.insert("ResearchDefinition.id".to_string(), "1".to_string());
    map.insert("ResearchDefinition.identifier".to_string(), "*".to_string());
    map.insert("ResearchDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("ResearchDefinition.jurisdiction".to_string(), "*".to_string());
    map.insert("ResearchDefinition.language".to_string(), "1".to_string());
    map.insert("ResearchDefinition.lastReviewDate".to_string(), "1".to_string());
    map.insert("ResearchDefinition.library".to_string(), "*".to_string());
    map.insert("ResearchDefinition.meta".to_string(), "1".to_string());
    map.insert("ResearchDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("ResearchDefinition.name".to_string(), "1".to_string());
    map.insert("ResearchDefinition.outcome".to_string(), "1".to_string());
    map.insert("ResearchDefinition.population".to_string(), "1".to_string());
    map.insert("ResearchDefinition.publisher".to_string(), "1".to_string());
    map.insert("ResearchDefinition.purpose".to_string(), "1".to_string());
    map.insert("ResearchDefinition.relatedArtifact".to_string(), "*".to_string());
    map.insert("ResearchDefinition.reviewer".to_string(), "*".to_string());
    map.insert("ResearchDefinition.shortTitle".to_string(), "1".to_string());
    map.insert("ResearchDefinition.status".to_string(), "1".to_string());
    map.insert("ResearchDefinition.subjectCodeableConcept".to_string(), "1".to_string());
    map.insert("ResearchDefinition.subjectReference".to_string(), "1".to_string());
    map.insert("ResearchDefinition.subtitle".to_string(), "1".to_string());
    map.insert("ResearchDefinition.text".to_string(), "1".to_string());
    map.insert("ResearchDefinition.title".to_string(), "1".to_string());
    map.insert("ResearchDefinition.topic".to_string(), "*".to_string());
    map.insert("ResearchDefinition.url".to_string(), "1".to_string());
    map.insert("ResearchDefinition.usage".to_string(), "1".to_string());
    map.insert("ResearchDefinition.useContext".to_string(), "*".to_string());
    map.insert("ResearchDefinition.version".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.approvalDate".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.author".to_string(), "*".to_string());
    map.insert("ResearchElementDefinition.characteristic".to_string(), "*".to_string());
    map.insert(
        "ResearchElementDefinition.characteristic.definitionCanonical".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.definitionCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.definitionDataRequirement".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.definitionExpression".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.exclude".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.participantEffectiveDateTime"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.participantEffectiveDescription"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.participantEffectiveDuration"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.participantEffectiveGroupMeasure"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.participantEffectivePeriod"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.participantEffectiveTimeFromStart"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.participantEffectiveTiming"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.studyEffectiveDateTime".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.studyEffectiveDescription".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.studyEffectiveDuration".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.studyEffectiveGroupMeasure"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.studyEffectivePeriod".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.studyEffectiveTimeFromStart"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.studyEffectiveTiming".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.unitOfMeasure".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.characteristic.usageContext".to_string(),
        "*".to_string(),
    );
    map.insert("ResearchElementDefinition.comment".to_string(), "*".to_string());
    map.insert("ResearchElementDefinition.contact".to_string(), "*".to_string());
    map.insert("ResearchElementDefinition.contained".to_string(), "*".to_string());
    map.insert("ResearchElementDefinition.copyright".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.date".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.description".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.editor".to_string(), "*".to_string());
    map.insert("ResearchElementDefinition.effectivePeriod".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.endorser".to_string(), "*".to_string());
    map.insert("ResearchElementDefinition.experimental".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.extension".to_string(), "*".to_string());
    map.insert("ResearchElementDefinition.id".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.identifier".to_string(), "*".to_string());
    map.insert("ResearchElementDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.jurisdiction".to_string(), "*".to_string());
    map.insert("ResearchElementDefinition.language".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.lastReviewDate".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.library".to_string(), "*".to_string());
    map.insert("ResearchElementDefinition.meta".to_string(), "1".to_string());
    map.insert(
        "ResearchElementDefinition.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ResearchElementDefinition.name".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.publisher".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.purpose".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.relatedArtifact".to_string(), "*".to_string());
    map.insert("ResearchElementDefinition.reviewer".to_string(), "*".to_string());
    map.insert("ResearchElementDefinition.shortTitle".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.status".to_string(), "1".to_string());
    map.insert(
        "ResearchElementDefinition.subjectCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ResearchElementDefinition.subjectReference".to_string(),
        "1".to_string(),
    );
    map.insert("ResearchElementDefinition.subtitle".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.text".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.title".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.topic".to_string(), "*".to_string());
    map.insert("ResearchElementDefinition.type".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.url".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.usage".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.useContext".to_string(), "*".to_string());
    map.insert("ResearchElementDefinition.variableType".to_string(), "1".to_string());
    map.insert("ResearchElementDefinition.version".to_string(), "1".to_string());
    map.insert("ResearchStudy.arm".to_string(), "*".to_string());
    map.insert("ResearchStudy.arm.description".to_string(), "1".to_string());
    map.insert("ResearchStudy.arm.extension".to_string(), "*".to_string());
    map.insert("ResearchStudy.arm.id".to_string(), "1".to_string());
    map.insert("ResearchStudy.arm.modifierExtension".to_string(), "*".to_string());
    map.insert("ResearchStudy.arm.name".to_string(), "1".to_string());
    map.insert("ResearchStudy.arm.type".to_string(), "1".to_string());
    map.insert("ResearchStudy.category".to_string(), "*".to_string());
    map.insert("ResearchStudy.condition".to_string(), "*".to_string());
    map.insert("ResearchStudy.contact".to_string(), "*".to_string());
    map.insert("ResearchStudy.contained".to_string(), "*".to_string());
    map.insert("ResearchStudy.description".to_string(), "1".to_string());
    map.insert("ResearchStudy.enrollment".to_string(), "*".to_string());
    map.insert("ResearchStudy.extension".to_string(), "*".to_string());
    map.insert("ResearchStudy.focus".to_string(), "*".to_string());
    map.insert("ResearchStudy.id".to_string(), "1".to_string());
    map.insert("ResearchStudy.identifier".to_string(), "*".to_string());
    map.insert("ResearchStudy.implicitRules".to_string(), "1".to_string());
    map.insert("ResearchStudy.keyword".to_string(), "*".to_string());
    map.insert("ResearchStudy.language".to_string(), "1".to_string());
    map.insert("ResearchStudy.location".to_string(), "*".to_string());
    map.insert("ResearchStudy.meta".to_string(), "1".to_string());
    map.insert("ResearchStudy.modifierExtension".to_string(), "*".to_string());
    map.insert("ResearchStudy.note".to_string(), "*".to_string());
    map.insert("ResearchStudy.objective".to_string(), "*".to_string());
    map.insert("ResearchStudy.objective.extension".to_string(), "*".to_string());
    map.insert("ResearchStudy.objective.id".to_string(), "1".to_string());
    map.insert("ResearchStudy.objective.modifierExtension".to_string(), "*".to_string());
    map.insert("ResearchStudy.objective.name".to_string(), "1".to_string());
    map.insert("ResearchStudy.objective.type".to_string(), "1".to_string());
    map.insert("ResearchStudy.partOf".to_string(), "*".to_string());
    map.insert("ResearchStudy.period".to_string(), "1".to_string());
    map.insert("ResearchStudy.phase".to_string(), "1".to_string());
    map.insert("ResearchStudy.primaryPurposeType".to_string(), "1".to_string());
    map.insert("ResearchStudy.principalInvestigator".to_string(), "1".to_string());
    map.insert("ResearchStudy.protocol".to_string(), "*".to_string());
    map.insert("ResearchStudy.reasonStopped".to_string(), "1".to_string());
    map.insert("ResearchStudy.relatedArtifact".to_string(), "*".to_string());
    map.insert("ResearchStudy.site".to_string(), "*".to_string());
    map.insert("ResearchStudy.sponsor".to_string(), "1".to_string());
    map.insert("ResearchStudy.status".to_string(), "1".to_string());
    map.insert("ResearchStudy.text".to_string(), "1".to_string());
    map.insert("ResearchStudy.title".to_string(), "1".to_string());
    map.insert("ResearchSubject.actualArm".to_string(), "1".to_string());
    map.insert("ResearchSubject.assignedArm".to_string(), "1".to_string());
    map.insert("ResearchSubject.consent".to_string(), "1".to_string());
    map.insert("ResearchSubject.contained".to_string(), "*".to_string());
    map.insert("ResearchSubject.extension".to_string(), "*".to_string());
    map.insert("ResearchSubject.id".to_string(), "1".to_string());
    map.insert("ResearchSubject.identifier".to_string(), "*".to_string());
    map.insert("ResearchSubject.implicitRules".to_string(), "1".to_string());
    map.insert("ResearchSubject.individual".to_string(), "1".to_string());
    map.insert("ResearchSubject.language".to_string(), "1".to_string());
    map.insert("ResearchSubject.meta".to_string(), "1".to_string());
    map.insert("ResearchSubject.modifierExtension".to_string(), "*".to_string());
    map.insert("ResearchSubject.period".to_string(), "1".to_string());
    map.insert("ResearchSubject.status".to_string(), "1".to_string());
    map.insert("ResearchSubject.study".to_string(), "1".to_string());
    map.insert("ResearchSubject.text".to_string(), "1".to_string());
    map.insert("RiskAssessment.basedOn".to_string(), "1".to_string());
    map.insert("RiskAssessment.basis".to_string(), "*".to_string());
    map.insert("RiskAssessment.code".to_string(), "1".to_string());
    map.insert("RiskAssessment.condition".to_string(), "1".to_string());
    map.insert("RiskAssessment.contained".to_string(), "*".to_string());
    map.insert("RiskAssessment.encounter".to_string(), "1".to_string());
    map.insert("RiskAssessment.extension".to_string(), "*".to_string());
    map.insert("RiskAssessment.id".to_string(), "1".to_string());
    map.insert("RiskAssessment.identifier".to_string(), "*".to_string());
    map.insert("RiskAssessment.implicitRules".to_string(), "1".to_string());
    map.insert("RiskAssessment.language".to_string(), "1".to_string());
    map.insert("RiskAssessment.meta".to_string(), "1".to_string());
    map.insert("RiskAssessment.method".to_string(), "1".to_string());
    map.insert("RiskAssessment.mitigation".to_string(), "1".to_string());
    map.insert("RiskAssessment.modifierExtension".to_string(), "*".to_string());
    map.insert("RiskAssessment.note".to_string(), "*".to_string());
    map.insert("RiskAssessment.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("RiskAssessment.occurrencePeriod".to_string(), "1".to_string());
    map.insert("RiskAssessment.parent".to_string(), "1".to_string());
    map.insert("RiskAssessment.performer".to_string(), "1".to_string());
    map.insert("RiskAssessment.prediction".to_string(), "*".to_string());
    map.insert("RiskAssessment.prediction.extension".to_string(), "*".to_string());
    map.insert("RiskAssessment.prediction.id".to_string(), "1".to_string());
    map.insert(
        "RiskAssessment.prediction.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("RiskAssessment.prediction.outcome".to_string(), "1".to_string());
    map.insert(
        "RiskAssessment.prediction.probabilityDecimal".to_string(),
        "1".to_string(),
    );
    map.insert(
        "RiskAssessment.prediction.probabilityRange".to_string(),
        "1".to_string(),
    );
    map.insert("RiskAssessment.prediction.qualitativeRisk".to_string(), "1".to_string());
    map.insert("RiskAssessment.prediction.rationale".to_string(), "1".to_string());
    map.insert("RiskAssessment.prediction.relativeRisk".to_string(), "1".to_string());
    map.insert("RiskAssessment.prediction.whenPeriod".to_string(), "1".to_string());
    map.insert("RiskAssessment.prediction.whenRange".to_string(), "1".to_string());
    map.insert("RiskAssessment.reasonCode".to_string(), "*".to_string());
    map.insert("RiskAssessment.reasonReference".to_string(), "*".to_string());
    map.insert("RiskAssessment.status".to_string(), "1".to_string());
    map.insert("RiskAssessment.subject".to_string(), "1".to_string());
    map.insert("RiskAssessment.text".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.approvalDate".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.author".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.certainty".to_string(), "*".to_string());
    map.insert(
        "RiskEvidenceSynthesis.certainty.certaintySubcomponent".to_string(),
        "*".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.certainty.certaintySubcomponent.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.certainty.certaintySubcomponent.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.certainty.certaintySubcomponent.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.certainty.certaintySubcomponent.note".to_string(),
        "*".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.certainty.certaintySubcomponent.rating".to_string(),
        "*".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.certainty.certaintySubcomponent.type".to_string(),
        "1".to_string(),
    );
    map.insert("RiskEvidenceSynthesis.certainty.extension".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.certainty.id".to_string(), "1".to_string());
    map.insert(
        "RiskEvidenceSynthesis.certainty.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("RiskEvidenceSynthesis.certainty.note".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.certainty.rating".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.contact".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.contained".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.copyright".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.date".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.description".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.editor".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.effectivePeriod".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.endorser".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.exposure".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.extension".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.id".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.identifier".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.implicitRules".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.jurisdiction".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.language".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.lastReviewDate".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.meta".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.modifierExtension".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.name".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.note".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.outcome".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.population".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.publisher".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.relatedArtifact".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.reviewer".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.riskEstimate".to_string(), "1".to_string());
    map.insert(
        "RiskEvidenceSynthesis.riskEstimate.denominatorCount".to_string(),
        "1".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.riskEstimate.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.riskEstimate.extension".to_string(),
        "*".to_string(),
    );
    map.insert("RiskEvidenceSynthesis.riskEstimate.id".to_string(), "1".to_string());
    map.insert(
        "RiskEvidenceSynthesis.riskEstimate.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.riskEstimate.numeratorCount".to_string(),
        "1".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.riskEstimate.precisionEstimate".to_string(),
        "*".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.riskEstimate.precisionEstimate.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.riskEstimate.precisionEstimate.from".to_string(),
        "1".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.riskEstimate.precisionEstimate.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.riskEstimate.precisionEstimate.level".to_string(),
        "1".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.riskEstimate.precisionEstimate.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.riskEstimate.precisionEstimate.to".to_string(),
        "1".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.riskEstimate.precisionEstimate.type".to_string(),
        "1".to_string(),
    );
    map.insert("RiskEvidenceSynthesis.riskEstimate.type".to_string(), "1".to_string());
    map.insert(
        "RiskEvidenceSynthesis.riskEstimate.unitOfMeasure".to_string(),
        "1".to_string(),
    );
    map.insert("RiskEvidenceSynthesis.riskEstimate.value".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.sampleSize".to_string(), "1".to_string());
    map.insert(
        "RiskEvidenceSynthesis.sampleSize.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.sampleSize.extension".to_string(),
        "*".to_string(),
    );
    map.insert("RiskEvidenceSynthesis.sampleSize.id".to_string(), "1".to_string());
    map.insert(
        "RiskEvidenceSynthesis.sampleSize.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.sampleSize.numberOfParticipants".to_string(),
        "1".to_string(),
    );
    map.insert(
        "RiskEvidenceSynthesis.sampleSize.numberOfStudies".to_string(),
        "1".to_string(),
    );
    map.insert("RiskEvidenceSynthesis.status".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.studyType".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.synthesisType".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.text".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.title".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.topic".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.url".to_string(), "1".to_string());
    map.insert("RiskEvidenceSynthesis.useContext".to_string(), "*".to_string());
    map.insert("RiskEvidenceSynthesis.version".to_string(), "1".to_string());
    map.insert("SampledData.data".to_string(), "1".to_string());
    map.insert("SampledData.dimensions".to_string(), "1".to_string());
    map.insert("SampledData.extension".to_string(), "*".to_string());
    map.insert("SampledData.factor".to_string(), "1".to_string());
    map.insert("SampledData.id".to_string(), "1".to_string());
    map.insert("SampledData.lowerLimit".to_string(), "1".to_string());
    map.insert("SampledData.origin".to_string(), "1".to_string());
    map.insert("SampledData.period".to_string(), "1".to_string());
    map.insert("SampledData.upperLimit".to_string(), "1".to_string());
    map.insert("Schedule.active".to_string(), "1".to_string());
    map.insert("Schedule.actor".to_string(), "*".to_string());
    map.insert("Schedule.comment".to_string(), "1".to_string());
    map.insert("Schedule.contained".to_string(), "*".to_string());
    map.insert("Schedule.extension".to_string(), "*".to_string());
    map.insert("Schedule.id".to_string(), "1".to_string());
    map.insert("Schedule.identifier".to_string(), "*".to_string());
    map.insert("Schedule.implicitRules".to_string(), "1".to_string());
    map.insert("Schedule.language".to_string(), "1".to_string());
    map.insert("Schedule.meta".to_string(), "1".to_string());
    map.insert("Schedule.modifierExtension".to_string(), "*".to_string());
    map.insert("Schedule.planningHorizon".to_string(), "1".to_string());
    map.insert("Schedule.serviceCategory".to_string(), "*".to_string());
    map.insert("Schedule.serviceType".to_string(), "*".to_string());
    map.insert("Schedule.specialty".to_string(), "*".to_string());
    map.insert("Schedule.text".to_string(), "1".to_string());
    map.insert("SearchParameter.base".to_string(), "*".to_string());
    map.insert("SearchParameter.chain".to_string(), "*".to_string());
    map.insert("SearchParameter.code".to_string(), "1".to_string());
    map.insert("SearchParameter.comparator".to_string(), "*".to_string());
    map.insert("SearchParameter.component".to_string(), "*".to_string());
    map.insert("SearchParameter.component.definition".to_string(), "1".to_string());
    map.insert("SearchParameter.component.expression".to_string(), "1".to_string());
    map.insert("SearchParameter.component.extension".to_string(), "*".to_string());
    map.insert("SearchParameter.component.id".to_string(), "1".to_string());
    map.insert(
        "SearchParameter.component.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("SearchParameter.contact".to_string(), "*".to_string());
    map.insert("SearchParameter.contained".to_string(), "*".to_string());
    map.insert("SearchParameter.date".to_string(), "1".to_string());
    map.insert("SearchParameter.derivedFrom".to_string(), "1".to_string());
    map.insert("SearchParameter.description".to_string(), "1".to_string());
    map.insert("SearchParameter.experimental".to_string(), "1".to_string());
    map.insert("SearchParameter.expression".to_string(), "1".to_string());
    map.insert("SearchParameter.extension".to_string(), "*".to_string());
    map.insert("SearchParameter.id".to_string(), "1".to_string());
    map.insert("SearchParameter.implicitRules".to_string(), "1".to_string());
    map.insert("SearchParameter.jurisdiction".to_string(), "*".to_string());
    map.insert("SearchParameter.language".to_string(), "1".to_string());
    map.insert("SearchParameter.meta".to_string(), "1".to_string());
    map.insert("SearchParameter.modifier".to_string(), "*".to_string());
    map.insert("SearchParameter.modifierExtension".to_string(), "*".to_string());
    map.insert("SearchParameter.multipleAnd".to_string(), "1".to_string());
    map.insert("SearchParameter.multipleOr".to_string(), "1".to_string());
    map.insert("SearchParameter.name".to_string(), "1".to_string());
    map.insert("SearchParameter.publisher".to_string(), "1".to_string());
    map.insert("SearchParameter.purpose".to_string(), "1".to_string());
    map.insert("SearchParameter.status".to_string(), "1".to_string());
    map.insert("SearchParameter.target".to_string(), "*".to_string());
    map.insert("SearchParameter.text".to_string(), "1".to_string());
    map.insert("SearchParameter.type".to_string(), "1".to_string());
    map.insert("SearchParameter.url".to_string(), "1".to_string());
    map.insert("SearchParameter.useContext".to_string(), "*".to_string());
    map.insert("SearchParameter.version".to_string(), "1".to_string());
    map.insert("SearchParameter.xpath".to_string(), "1".to_string());
    map.insert("SearchParameter.xpathUsage".to_string(), "1".to_string());
    map.insert("ServiceRequest.asNeededBoolean".to_string(), "1".to_string());
    map.insert("ServiceRequest.asNeededCodeableConcept".to_string(), "1".to_string());
    map.insert("ServiceRequest.authoredOn".to_string(), "1".to_string());
    map.insert("ServiceRequest.basedOn".to_string(), "*".to_string());
    map.insert("ServiceRequest.bodySite".to_string(), "*".to_string());
    map.insert("ServiceRequest.category".to_string(), "*".to_string());
    map.insert("ServiceRequest.code".to_string(), "1".to_string());
    map.insert("ServiceRequest.contained".to_string(), "*".to_string());
    map.insert("ServiceRequest.doNotPerform".to_string(), "1".to_string());
    map.insert("ServiceRequest.encounter".to_string(), "1".to_string());
    map.insert("ServiceRequest.extension".to_string(), "*".to_string());
    map.insert("ServiceRequest.id".to_string(), "1".to_string());
    map.insert("ServiceRequest.identifier".to_string(), "*".to_string());
    map.insert("ServiceRequest.implicitRules".to_string(), "1".to_string());
    map.insert("ServiceRequest.instantiatesCanonical".to_string(), "*".to_string());
    map.insert("ServiceRequest.instantiatesUri".to_string(), "*".to_string());
    map.insert("ServiceRequest.insurance".to_string(), "*".to_string());
    map.insert("ServiceRequest.intent".to_string(), "1".to_string());
    map.insert("ServiceRequest.language".to_string(), "1".to_string());
    map.insert("ServiceRequest.locationCode".to_string(), "*".to_string());
    map.insert("ServiceRequest.locationReference".to_string(), "*".to_string());
    map.insert("ServiceRequest.meta".to_string(), "1".to_string());
    map.insert("ServiceRequest.modifierExtension".to_string(), "*".to_string());
    map.insert("ServiceRequest.note".to_string(), "*".to_string());
    map.insert("ServiceRequest.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("ServiceRequest.occurrencePeriod".to_string(), "1".to_string());
    map.insert("ServiceRequest.occurrenceTiming".to_string(), "1".to_string());
    map.insert("ServiceRequest.orderDetail".to_string(), "*".to_string());
    map.insert("ServiceRequest.patientInstruction".to_string(), "1".to_string());
    map.insert("ServiceRequest.performer".to_string(), "*".to_string());
    map.insert("ServiceRequest.performerType".to_string(), "1".to_string());
    map.insert("ServiceRequest.priority".to_string(), "1".to_string());
    map.insert("ServiceRequest.quantityQuantity".to_string(), "1".to_string());
    map.insert("ServiceRequest.quantityRange".to_string(), "1".to_string());
    map.insert("ServiceRequest.quantityRatio".to_string(), "1".to_string());
    map.insert("ServiceRequest.reasonCode".to_string(), "*".to_string());
    map.insert("ServiceRequest.reasonReference".to_string(), "*".to_string());
    map.insert("ServiceRequest.relevantHistory".to_string(), "*".to_string());
    map.insert("ServiceRequest.replaces".to_string(), "*".to_string());
    map.insert("ServiceRequest.requester".to_string(), "1".to_string());
    map.insert("ServiceRequest.requisition".to_string(), "1".to_string());
    map.insert("ServiceRequest.specimen".to_string(), "*".to_string());
    map.insert("ServiceRequest.status".to_string(), "1".to_string());
    map.insert("ServiceRequest.subject".to_string(), "1".to_string());
    map.insert("ServiceRequest.supportingInfo".to_string(), "*".to_string());
    map.insert("ServiceRequest.text".to_string(), "1".to_string());
    map.insert("Signature.data".to_string(), "1".to_string());
    map.insert("Signature.extension".to_string(), "*".to_string());
    map.insert("Signature.id".to_string(), "1".to_string());
    map.insert("Signature.onBehalfOf".to_string(), "1".to_string());
    map.insert("Signature.sigFormat".to_string(), "1".to_string());
    map.insert("Signature.targetFormat".to_string(), "1".to_string());
    map.insert("Signature.type".to_string(), "*".to_string());
    map.insert("Signature.when".to_string(), "1".to_string());
    map.insert("Signature.who".to_string(), "1".to_string());
    map.insert("Slot.appointmentType".to_string(), "1".to_string());
    map.insert("Slot.comment".to_string(), "1".to_string());
    map.insert("Slot.contained".to_string(), "*".to_string());
    map.insert("Slot.end".to_string(), "1".to_string());
    map.insert("Slot.extension".to_string(), "*".to_string());
    map.insert("Slot.id".to_string(), "1".to_string());
    map.insert("Slot.identifier".to_string(), "*".to_string());
    map.insert("Slot.implicitRules".to_string(), "1".to_string());
    map.insert("Slot.language".to_string(), "1".to_string());
    map.insert("Slot.meta".to_string(), "1".to_string());
    map.insert("Slot.modifierExtension".to_string(), "*".to_string());
    map.insert("Slot.overbooked".to_string(), "1".to_string());
    map.insert("Slot.schedule".to_string(), "1".to_string());
    map.insert("Slot.serviceCategory".to_string(), "*".to_string());
    map.insert("Slot.serviceType".to_string(), "*".to_string());
    map.insert("Slot.specialty".to_string(), "*".to_string());
    map.insert("Slot.start".to_string(), "1".to_string());
    map.insert("Slot.status".to_string(), "1".to_string());
    map.insert("Slot.text".to_string(), "1".to_string());
    map.insert("Specimen.accessionIdentifier".to_string(), "1".to_string());
    map.insert("Specimen.collection".to_string(), "1".to_string());
    map.insert("Specimen.collection.bodySite".to_string(), "1".to_string());
    map.insert("Specimen.collection.collectedDateTime".to_string(), "1".to_string());
    map.insert("Specimen.collection.collectedPeriod".to_string(), "1".to_string());
    map.insert("Specimen.collection.collector".to_string(), "1".to_string());
    map.insert("Specimen.collection.duration".to_string(), "1".to_string());
    map.insert("Specimen.collection.extension".to_string(), "*".to_string());
    map.insert(
        "Specimen.collection.fastingStatusCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("Specimen.collection.fastingStatusDuration".to_string(), "1".to_string());
    map.insert("Specimen.collection.id".to_string(), "1".to_string());
    map.insert("Specimen.collection.method".to_string(), "1".to_string());
    map.insert("Specimen.collection.modifierExtension".to_string(), "*".to_string());
    map.insert("Specimen.collection.quantity".to_string(), "1".to_string());
    map.insert("Specimen.condition".to_string(), "*".to_string());
    map.insert("Specimen.contained".to_string(), "*".to_string());
    map.insert("Specimen.container".to_string(), "*".to_string());
    map.insert(
        "Specimen.container.additiveCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("Specimen.container.additiveReference".to_string(), "1".to_string());
    map.insert("Specimen.container.capacity".to_string(), "1".to_string());
    map.insert("Specimen.container.description".to_string(), "1".to_string());
    map.insert("Specimen.container.extension".to_string(), "*".to_string());
    map.insert("Specimen.container.id".to_string(), "1".to_string());
    map.insert("Specimen.container.identifier".to_string(), "*".to_string());
    map.insert("Specimen.container.modifierExtension".to_string(), "*".to_string());
    map.insert("Specimen.container.specimenQuantity".to_string(), "1".to_string());
    map.insert("Specimen.container.type".to_string(), "1".to_string());
    map.insert("Specimen.extension".to_string(), "*".to_string());
    map.insert("Specimen.id".to_string(), "1".to_string());
    map.insert("Specimen.identifier".to_string(), "*".to_string());
    map.insert("Specimen.implicitRules".to_string(), "1".to_string());
    map.insert("Specimen.language".to_string(), "1".to_string());
    map.insert("Specimen.meta".to_string(), "1".to_string());
    map.insert("Specimen.modifierExtension".to_string(), "*".to_string());
    map.insert("Specimen.note".to_string(), "*".to_string());
    map.insert("Specimen.parent".to_string(), "*".to_string());
    map.insert("Specimen.processing".to_string(), "*".to_string());
    map.insert("Specimen.processing.additive".to_string(), "*".to_string());
    map.insert("Specimen.processing.description".to_string(), "1".to_string());
    map.insert("Specimen.processing.extension".to_string(), "*".to_string());
    map.insert("Specimen.processing.id".to_string(), "1".to_string());
    map.insert("Specimen.processing.modifierExtension".to_string(), "*".to_string());
    map.insert("Specimen.processing.procedure".to_string(), "1".to_string());
    map.insert("Specimen.processing.timeDateTime".to_string(), "1".to_string());
    map.insert("Specimen.processing.timePeriod".to_string(), "1".to_string());
    map.insert("Specimen.receivedTime".to_string(), "1".to_string());
    map.insert("Specimen.request".to_string(), "*".to_string());
    map.insert("Specimen.status".to_string(), "1".to_string());
    map.insert("Specimen.subject".to_string(), "1".to_string());
    map.insert("Specimen.text".to_string(), "1".to_string());
    map.insert("Specimen.type".to_string(), "1".to_string());
    map.insert("SpecimenDefinition.collection".to_string(), "*".to_string());
    map.insert("SpecimenDefinition.contained".to_string(), "*".to_string());
    map.insert("SpecimenDefinition.extension".to_string(), "*".to_string());
    map.insert("SpecimenDefinition.id".to_string(), "1".to_string());
    map.insert("SpecimenDefinition.identifier".to_string(), "1".to_string());
    map.insert("SpecimenDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("SpecimenDefinition.language".to_string(), "1".to_string());
    map.insert("SpecimenDefinition.meta".to_string(), "1".to_string());
    map.insert("SpecimenDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("SpecimenDefinition.patientPreparation".to_string(), "*".to_string());
    map.insert("SpecimenDefinition.text".to_string(), "1".to_string());
    map.insert("SpecimenDefinition.timeAspect".to_string(), "1".to_string());
    map.insert("SpecimenDefinition.typeCollected".to_string(), "1".to_string());
    map.insert("SpecimenDefinition.typeTested".to_string(), "*".to_string());
    map.insert("SpecimenDefinition.typeTested.container".to_string(), "1".to_string());
    map.insert(
        "SpecimenDefinition.typeTested.container.additive".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.additive.additiveCodeableConcept"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.additive.additiveReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.additive.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.additive.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.additive.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.cap".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.capacity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.material".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.minimumVolumeQuantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.minimumVolumeString".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.preparation".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.container.type".to_string(),
        "1".to_string(),
    );
    map.insert("SpecimenDefinition.typeTested.extension".to_string(), "*".to_string());
    map.insert("SpecimenDefinition.typeTested.handling".to_string(), "*".to_string());
    map.insert(
        "SpecimenDefinition.typeTested.handling.extension".to_string(),
        "*".to_string(),
    );
    map.insert("SpecimenDefinition.typeTested.handling.id".to_string(), "1".to_string());
    map.insert(
        "SpecimenDefinition.typeTested.handling.instruction".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.handling.maxDuration".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.handling.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.handling.temperatureQualifier".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SpecimenDefinition.typeTested.handling.temperatureRange".to_string(),
        "1".to_string(),
    );
    map.insert("SpecimenDefinition.typeTested.id".to_string(), "1".to_string());
    map.insert("SpecimenDefinition.typeTested.isDerived".to_string(), "1".to_string());
    map.insert(
        "SpecimenDefinition.typeTested.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("SpecimenDefinition.typeTested.preference".to_string(), "1".to_string());
    map.insert(
        "SpecimenDefinition.typeTested.rejectionCriterion".to_string(),
        "*".to_string(),
    );
    map.insert("SpecimenDefinition.typeTested.requirement".to_string(), "1".to_string());
    map.insert(
        "SpecimenDefinition.typeTested.retentionTime".to_string(),
        "1".to_string(),
    );
    map.insert("SpecimenDefinition.typeTested.type".to_string(), "1".to_string());
    map.insert("StructureDefinition.abstract".to_string(), "1".to_string());
    map.insert("StructureDefinition.baseDefinition".to_string(), "1".to_string());
    map.insert("StructureDefinition.contact".to_string(), "*".to_string());
    map.insert("StructureDefinition.contained".to_string(), "*".to_string());
    map.insert("StructureDefinition.context".to_string(), "*".to_string());
    map.insert("StructureDefinition.context.expression".to_string(), "1".to_string());
    map.insert("StructureDefinition.context.extension".to_string(), "*".to_string());
    map.insert("StructureDefinition.context.id".to_string(), "1".to_string());
    map.insert(
        "StructureDefinition.context.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("StructureDefinition.context.type".to_string(), "1".to_string());
    map.insert("StructureDefinition.contextInvariant".to_string(), "*".to_string());
    map.insert("StructureDefinition.copyright".to_string(), "1".to_string());
    map.insert("StructureDefinition.date".to_string(), "1".to_string());
    map.insert("StructureDefinition.derivation".to_string(), "1".to_string());
    map.insert("StructureDefinition.description".to_string(), "1".to_string());
    map.insert("StructureDefinition.differential".to_string(), "1".to_string());
    map.insert("StructureDefinition.differential.element".to_string(), "*".to_string());
    map.insert(
        "StructureDefinition.differential.extension".to_string(),
        "*".to_string(),
    );
    map.insert("StructureDefinition.differential.id".to_string(), "1".to_string());
    map.insert(
        "StructureDefinition.differential.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("StructureDefinition.experimental".to_string(), "1".to_string());
    map.insert("StructureDefinition.extension".to_string(), "*".to_string());
    map.insert("StructureDefinition.fhirVersion".to_string(), "1".to_string());
    map.insert("StructureDefinition.id".to_string(), "1".to_string());
    map.insert("StructureDefinition.identifier".to_string(), "*".to_string());
    map.insert("StructureDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("StructureDefinition.jurisdiction".to_string(), "*".to_string());
    map.insert("StructureDefinition.keyword".to_string(), "*".to_string());
    map.insert("StructureDefinition.kind".to_string(), "1".to_string());
    map.insert("StructureDefinition.language".to_string(), "1".to_string());
    map.insert("StructureDefinition.mapping".to_string(), "*".to_string());
    map.insert("StructureDefinition.mapping.comment".to_string(), "1".to_string());
    map.insert("StructureDefinition.mapping.extension".to_string(), "*".to_string());
    map.insert("StructureDefinition.mapping.id".to_string(), "1".to_string());
    map.insert("StructureDefinition.mapping.identity".to_string(), "1".to_string());
    map.insert(
        "StructureDefinition.mapping.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("StructureDefinition.mapping.name".to_string(), "1".to_string());
    map.insert("StructureDefinition.mapping.uri".to_string(), "1".to_string());
    map.insert("StructureDefinition.meta".to_string(), "1".to_string());
    map.insert("StructureDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("StructureDefinition.name".to_string(), "1".to_string());
    map.insert("StructureDefinition.publisher".to_string(), "1".to_string());
    map.insert("StructureDefinition.purpose".to_string(), "1".to_string());
    map.insert("StructureDefinition.snapshot".to_string(), "1".to_string());
    map.insert("StructureDefinition.snapshot.element".to_string(), "*".to_string());
    map.insert("StructureDefinition.snapshot.extension".to_string(), "*".to_string());
    map.insert("StructureDefinition.snapshot.id".to_string(), "1".to_string());
    map.insert(
        "StructureDefinition.snapshot.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("StructureDefinition.status".to_string(), "1".to_string());
    map.insert("StructureDefinition.text".to_string(), "1".to_string());
    map.insert("StructureDefinition.title".to_string(), "1".to_string());
    map.insert("StructureDefinition.type".to_string(), "1".to_string());
    map.insert("StructureDefinition.url".to_string(), "1".to_string());
    map.insert("StructureDefinition.useContext".to_string(), "*".to_string());
    map.insert("StructureDefinition.version".to_string(), "1".to_string());
    map.insert("StructureMap.contact".to_string(), "*".to_string());
    map.insert("StructureMap.contained".to_string(), "*".to_string());
    map.insert("StructureMap.copyright".to_string(), "1".to_string());
    map.insert("StructureMap.date".to_string(), "1".to_string());
    map.insert("StructureMap.description".to_string(), "1".to_string());
    map.insert("StructureMap.experimental".to_string(), "1".to_string());
    map.insert("StructureMap.extension".to_string(), "*".to_string());
    map.insert("StructureMap.group".to_string(), "*".to_string());
    map.insert("StructureMap.group.documentation".to_string(), "1".to_string());
    map.insert("StructureMap.group.extends".to_string(), "1".to_string());
    map.insert("StructureMap.group.extension".to_string(), "*".to_string());
    map.insert("StructureMap.group.id".to_string(), "1".to_string());
    map.insert("StructureMap.group.input".to_string(), "*".to_string());
    map.insert("StructureMap.group.input.documentation".to_string(), "1".to_string());
    map.insert("StructureMap.group.input.extension".to_string(), "*".to_string());
    map.insert("StructureMap.group.input.id".to_string(), "1".to_string());
    map.insert("StructureMap.group.input.mode".to_string(), "1".to_string());
    map.insert(
        "StructureMap.group.input.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("StructureMap.group.input.name".to_string(), "1".to_string());
    map.insert("StructureMap.group.input.type".to_string(), "1".to_string());
    map.insert("StructureMap.group.modifierExtension".to_string(), "*".to_string());
    map.insert("StructureMap.group.name".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule".to_string(), "*".to_string());
    map.insert("StructureMap.group.rule.dependent".to_string(), "*".to_string());
    map.insert(
        "StructureMap.group.rule.dependent.extension".to_string(),
        "*".to_string(),
    );
    map.insert("StructureMap.group.rule.dependent.id".to_string(), "1".to_string());
    map.insert(
        "StructureMap.group.rule.dependent.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("StructureMap.group.rule.dependent.name".to_string(), "1".to_string());
    map.insert(
        "StructureMap.group.rule.dependent.variable".to_string(),
        "*".to_string(),
    );
    map.insert("StructureMap.group.rule.documentation".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.extension".to_string(), "*".to_string());
    map.insert("StructureMap.group.rule.id".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.modifierExtension".to_string(), "*".to_string());
    map.insert("StructureMap.group.rule.name".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.rule".to_string(), "*".to_string());
    map.insert("StructureMap.group.rule.source".to_string(), "*".to_string());
    map.insert("StructureMap.group.rule.source.check".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.source.condition".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.source.context".to_string(), "1".to_string());
    map.insert(
        "StructureMap.group.rule.source.defaultValueAddress".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueAge".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueAnnotation".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueAttachment".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueBase64Binary".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueBoolean".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueCanonical".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueCode".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueCoding".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueContactDetail".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueContactPoint".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueContributor".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueCount".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueDataRequirement".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueDate".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueDateTime".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueDecimal".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueDistance".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueDosage".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueDuration".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueExpression".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueHumanName".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueId".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueIdentifier".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueInstant".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueInteger".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueMarkdown".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueMeta".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueMoney".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueOid".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueParameterDefinition".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValuePeriod".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValuePositiveInt".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueQuantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueRange".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueRatio".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueRelatedArtifact".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueSampledData".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueSignature".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueString".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueTime".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueTiming".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueTriggerDefinition".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueUnsignedInt".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueUri".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueUrl".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueUsageContext".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueUuid".to_string(),
        "1".to_string(),
    );
    map.insert("StructureMap.group.rule.source.element".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.source.extension".to_string(), "*".to_string());
    map.insert("StructureMap.group.rule.source.id".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.source.listMode".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.source.logMessage".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.source.max".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.source.min".to_string(), "1".to_string());
    map.insert(
        "StructureMap.group.rule.source.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("StructureMap.group.rule.source.type".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.source.variable".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.target".to_string(), "*".to_string());
    map.insert("StructureMap.group.rule.target.context".to_string(), "1".to_string());
    map.insert(
        "StructureMap.group.rule.target.contextType".to_string(),
        "1".to_string(),
    );
    map.insert("StructureMap.group.rule.target.element".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.target.extension".to_string(), "*".to_string());
    map.insert("StructureMap.group.rule.target.id".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.target.listMode".to_string(), "*".to_string());
    map.insert("StructureMap.group.rule.target.listRuleId".to_string(), "1".to_string());
    map.insert(
        "StructureMap.group.rule.target.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("StructureMap.group.rule.target.parameter".to_string(), "*".to_string());
    map.insert(
        "StructureMap.group.rule.target.parameter.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.target.parameter.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.target.parameter.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.target.parameter.valueBoolean".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.target.parameter.valueDecimal".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.target.parameter.valueId".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.target.parameter.valueInteger".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.target.parameter.valueString".to_string(),
        "1".to_string(),
    );
    map.insert("StructureMap.group.rule.target.transform".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.target.variable".to_string(), "1".to_string());
    map.insert("StructureMap.group.typeMode".to_string(), "1".to_string());
    map.insert("StructureMap.id".to_string(), "1".to_string());
    map.insert("StructureMap.identifier".to_string(), "*".to_string());
    map.insert("StructureMap.implicitRules".to_string(), "1".to_string());
    map.insert("StructureMap.import".to_string(), "*".to_string());
    map.insert("StructureMap.jurisdiction".to_string(), "*".to_string());
    map.insert("StructureMap.language".to_string(), "1".to_string());
    map.insert("StructureMap.meta".to_string(), "1".to_string());
    map.insert("StructureMap.modifierExtension".to_string(), "*".to_string());
    map.insert("StructureMap.name".to_string(), "1".to_string());
    map.insert("StructureMap.publisher".to_string(), "1".to_string());
    map.insert("StructureMap.purpose".to_string(), "1".to_string());
    map.insert("StructureMap.status".to_string(), "1".to_string());
    map.insert("StructureMap.structure".to_string(), "*".to_string());
    map.insert("StructureMap.structure.alias".to_string(), "1".to_string());
    map.insert("StructureMap.structure.documentation".to_string(), "1".to_string());
    map.insert("StructureMap.structure.extension".to_string(), "*".to_string());
    map.insert("StructureMap.structure.id".to_string(), "1".to_string());
    map.insert("StructureMap.structure.mode".to_string(), "1".to_string());
    map.insert("StructureMap.structure.modifierExtension".to_string(), "*".to_string());
    map.insert("StructureMap.structure.url".to_string(), "1".to_string());
    map.insert("StructureMap.text".to_string(), "1".to_string());
    map.insert("StructureMap.title".to_string(), "1".to_string());
    map.insert("StructureMap.url".to_string(), "1".to_string());
    map.insert("StructureMap.useContext".to_string(), "*".to_string());
    map.insert("StructureMap.version".to_string(), "1".to_string());
    map.insert("Subscription.channel".to_string(), "1".to_string());
    map.insert("Subscription.channel.endpoint".to_string(), "1".to_string());
    map.insert("Subscription.channel.extension".to_string(), "*".to_string());
    map.insert("Subscription.channel.header".to_string(), "*".to_string());
    map.insert("Subscription.channel.id".to_string(), "1".to_string());
    map.insert("Subscription.channel.modifierExtension".to_string(), "*".to_string());
    map.insert("Subscription.channel.payload".to_string(), "1".to_string());
    map.insert("Subscription.channel.type".to_string(), "1".to_string());
    map.insert("Subscription.contact".to_string(), "*".to_string());
    map.insert("Subscription.contained".to_string(), "*".to_string());
    map.insert("Subscription.criteria".to_string(), "1".to_string());
    map.insert("Subscription.end".to_string(), "1".to_string());
    map.insert("Subscription.error".to_string(), "1".to_string());
    map.insert("Subscription.extension".to_string(), "*".to_string());
    map.insert("Subscription.id".to_string(), "1".to_string());
    map.insert("Subscription.implicitRules".to_string(), "1".to_string());
    map.insert("Subscription.language".to_string(), "1".to_string());
    map.insert("Subscription.meta".to_string(), "1".to_string());
    map.insert("Subscription.modifierExtension".to_string(), "*".to_string());
    map.insert("Subscription.reason".to_string(), "1".to_string());
    map.insert("Subscription.status".to_string(), "1".to_string());
    map.insert("Subscription.text".to_string(), "1".to_string());
    map.insert("Substance.category".to_string(), "*".to_string());
    map.insert("Substance.code".to_string(), "1".to_string());
    map.insert("Substance.contained".to_string(), "*".to_string());
    map.insert("Substance.description".to_string(), "1".to_string());
    map.insert("Substance.extension".to_string(), "*".to_string());
    map.insert("Substance.id".to_string(), "1".to_string());
    map.insert("Substance.identifier".to_string(), "*".to_string());
    map.insert("Substance.implicitRules".to_string(), "1".to_string());
    map.insert("Substance.ingredient".to_string(), "*".to_string());
    map.insert("Substance.ingredient.extension".to_string(), "*".to_string());
    map.insert("Substance.ingredient.id".to_string(), "1".to_string());
    map.insert("Substance.ingredient.modifierExtension".to_string(), "*".to_string());
    map.insert("Substance.ingredient.quantity".to_string(), "1".to_string());
    map.insert(
        "Substance.ingredient.substanceCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("Substance.ingredient.substanceReference".to_string(), "1".to_string());
    map.insert("Substance.instance".to_string(), "*".to_string());
    map.insert("Substance.instance.expiry".to_string(), "1".to_string());
    map.insert("Substance.instance.extension".to_string(), "*".to_string());
    map.insert("Substance.instance.id".to_string(), "1".to_string());
    map.insert("Substance.instance.identifier".to_string(), "1".to_string());
    map.insert("Substance.instance.modifierExtension".to_string(), "*".to_string());
    map.insert("Substance.instance.quantity".to_string(), "1".to_string());
    map.insert("Substance.language".to_string(), "1".to_string());
    map.insert("Substance.meta".to_string(), "1".to_string());
    map.insert("Substance.modifierExtension".to_string(), "*".to_string());
    map.insert("Substance.status".to_string(), "1".to_string());
    map.insert("Substance.text".to_string(), "1".to_string());
    map.insert("SubstanceAmount.amountQuantity".to_string(), "1".to_string());
    map.insert("SubstanceAmount.amountRange".to_string(), "1".to_string());
    map.insert("SubstanceAmount.amountString".to_string(), "1".to_string());
    map.insert("SubstanceAmount.amountText".to_string(), "1".to_string());
    map.insert("SubstanceAmount.amountType".to_string(), "1".to_string());
    map.insert("SubstanceAmount.extension".to_string(), "*".to_string());
    map.insert("SubstanceAmount.id".to_string(), "1".to_string());
    map.insert("SubstanceAmount.modifierExtension".to_string(), "*".to_string());
    map.insert("SubstanceAmount.referenceRange".to_string(), "1".to_string());
    map.insert("SubstanceAmount.referenceRange.extension".to_string(), "*".to_string());
    map.insert("SubstanceAmount.referenceRange.highLimit".to_string(), "1".to_string());
    map.insert("SubstanceAmount.referenceRange.id".to_string(), "1".to_string());
    map.insert("SubstanceAmount.referenceRange.lowLimit".to_string(), "1".to_string());
    map.insert("SubstanceNucleicAcid.areaOfHybridisation".to_string(), "1".to_string());
    map.insert("SubstanceNucleicAcid.contained".to_string(), "*".to_string());
    map.insert("SubstanceNucleicAcid.extension".to_string(), "*".to_string());
    map.insert("SubstanceNucleicAcid.id".to_string(), "1".to_string());
    map.insert("SubstanceNucleicAcid.implicitRules".to_string(), "1".to_string());
    map.insert("SubstanceNucleicAcid.language".to_string(), "1".to_string());
    map.insert("SubstanceNucleicAcid.meta".to_string(), "1".to_string());
    map.insert("SubstanceNucleicAcid.modifierExtension".to_string(), "*".to_string());
    map.insert("SubstanceNucleicAcid.numberOfSubunits".to_string(), "1".to_string());
    map.insert("SubstanceNucleicAcid.oligoNucleotideType".to_string(), "1".to_string());
    map.insert("SubstanceNucleicAcid.sequenceType".to_string(), "1".to_string());
    map.insert("SubstanceNucleicAcid.subunit".to_string(), "*".to_string());
    map.insert("SubstanceNucleicAcid.subunit.extension".to_string(), "*".to_string());
    map.insert("SubstanceNucleicAcid.subunit.fivePrime".to_string(), "1".to_string());
    map.insert("SubstanceNucleicAcid.subunit.id".to_string(), "1".to_string());
    map.insert("SubstanceNucleicAcid.subunit.length".to_string(), "1".to_string());
    map.insert("SubstanceNucleicAcid.subunit.linkage".to_string(), "*".to_string());
    map.insert(
        "SubstanceNucleicAcid.subunit.linkage.connectivity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceNucleicAcid.subunit.linkage.extension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceNucleicAcid.subunit.linkage.id".to_string(), "1".to_string());
    map.insert(
        "SubstanceNucleicAcid.subunit.linkage.identifier".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceNucleicAcid.subunit.linkage.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceNucleicAcid.subunit.linkage.name".to_string(), "1".to_string());
    map.insert(
        "SubstanceNucleicAcid.subunit.linkage.residueSite".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceNucleicAcid.subunit.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceNucleicAcid.subunit.sequence".to_string(), "1".to_string());
    map.insert(
        "SubstanceNucleicAcid.subunit.sequenceAttachment".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceNucleicAcid.subunit.subunit".to_string(), "1".to_string());
    map.insert("SubstanceNucleicAcid.subunit.sugar".to_string(), "*".to_string());
    map.insert(
        "SubstanceNucleicAcid.subunit.sugar.extension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceNucleicAcid.subunit.sugar.id".to_string(), "1".to_string());
    map.insert(
        "SubstanceNucleicAcid.subunit.sugar.identifier".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceNucleicAcid.subunit.sugar.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceNucleicAcid.subunit.sugar.name".to_string(), "1".to_string());
    map.insert(
        "SubstanceNucleicAcid.subunit.sugar.residueSite".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceNucleicAcid.subunit.threePrime".to_string(), "1".to_string());
    map.insert("SubstanceNucleicAcid.text".to_string(), "1".to_string());
    map.insert("SubstancePolymer.class".to_string(), "1".to_string());
    map.insert("SubstancePolymer.contained".to_string(), "*".to_string());
    map.insert("SubstancePolymer.copolymerConnectivity".to_string(), "*".to_string());
    map.insert("SubstancePolymer.extension".to_string(), "*".to_string());
    map.insert("SubstancePolymer.geometry".to_string(), "1".to_string());
    map.insert("SubstancePolymer.id".to_string(), "1".to_string());
    map.insert("SubstancePolymer.implicitRules".to_string(), "1".to_string());
    map.insert("SubstancePolymer.language".to_string(), "1".to_string());
    map.insert("SubstancePolymer.meta".to_string(), "1".to_string());
    map.insert("SubstancePolymer.modification".to_string(), "*".to_string());
    map.insert("SubstancePolymer.modifierExtension".to_string(), "*".to_string());
    map.insert("SubstancePolymer.monomerSet".to_string(), "*".to_string());
    map.insert("SubstancePolymer.monomerSet.extension".to_string(), "*".to_string());
    map.insert("SubstancePolymer.monomerSet.id".to_string(), "1".to_string());
    map.insert(
        "SubstancePolymer.monomerSet.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstancePolymer.monomerSet.ratioType".to_string(), "1".to_string());
    map.insert(
        "SubstancePolymer.monomerSet.startingMaterial".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstancePolymer.monomerSet.startingMaterial.amount".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstancePolymer.monomerSet.startingMaterial.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstancePolymer.monomerSet.startingMaterial.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstancePolymer.monomerSet.startingMaterial.isDefining".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstancePolymer.monomerSet.startingMaterial.material".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstancePolymer.monomerSet.startingMaterial.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstancePolymer.monomerSet.startingMaterial.type".to_string(),
        "1".to_string(),
    );
    map.insert("SubstancePolymer.repeat".to_string(), "*".to_string());
    map.insert(
        "SubstancePolymer.repeat.averageMolecularFormula".to_string(),
        "1".to_string(),
    );
    map.insert("SubstancePolymer.repeat.extension".to_string(), "*".to_string());
    map.insert("SubstancePolymer.repeat.id".to_string(), "1".to_string());
    map.insert("SubstancePolymer.repeat.modifierExtension".to_string(), "*".to_string());
    map.insert("SubstancePolymer.repeat.numberOfUnits".to_string(), "1".to_string());
    map.insert("SubstancePolymer.repeat.repeatUnit".to_string(), "*".to_string());
    map.insert("SubstancePolymer.repeat.repeatUnit.amount".to_string(), "1".to_string());
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.degreeOfPolymerisation".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.degreeOfPolymerisation.amount".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.degreeOfPolymerisation.degree".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.degreeOfPolymerisation.extension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.degreeOfPolymerisation.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.degreeOfPolymerisation.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.extension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstancePolymer.repeat.repeatUnit.id".to_string(), "1".to_string());
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.orientationOfPolymerisation".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.repeatUnit".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.structuralRepresentation".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.structuralRepresentation.attachment"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.structuralRepresentation.extension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.structuralRepresentation.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.structuralRepresentation.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.structuralRepresentation.representation"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnit.structuralRepresentation.type".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstancePolymer.repeat.repeatUnitAmountType".to_string(),
        "1".to_string(),
    );
    map.insert("SubstancePolymer.text".to_string(), "1".to_string());
    map.insert("SubstanceProtein.contained".to_string(), "*".to_string());
    map.insert("SubstanceProtein.disulfideLinkage".to_string(), "*".to_string());
    map.insert("SubstanceProtein.extension".to_string(), "*".to_string());
    map.insert("SubstanceProtein.id".to_string(), "1".to_string());
    map.insert("SubstanceProtein.implicitRules".to_string(), "1".to_string());
    map.insert("SubstanceProtein.language".to_string(), "1".to_string());
    map.insert("SubstanceProtein.meta".to_string(), "1".to_string());
    map.insert("SubstanceProtein.modifierExtension".to_string(), "*".to_string());
    map.insert("SubstanceProtein.numberOfSubunits".to_string(), "1".to_string());
    map.insert("SubstanceProtein.sequenceType".to_string(), "1".to_string());
    map.insert("SubstanceProtein.subunit".to_string(), "*".to_string());
    map.insert(
        "SubstanceProtein.subunit.cTerminalModification".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceProtein.subunit.cTerminalModificationId".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceProtein.subunit.extension".to_string(), "*".to_string());
    map.insert("SubstanceProtein.subunit.id".to_string(), "1".to_string());
    map.insert("SubstanceProtein.subunit.length".to_string(), "1".to_string());
    map.insert(
        "SubstanceProtein.subunit.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceProtein.subunit.nTerminalModification".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceProtein.subunit.nTerminalModificationId".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceProtein.subunit.sequence".to_string(), "1".to_string());
    map.insert(
        "SubstanceProtein.subunit.sequenceAttachment".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceProtein.subunit.subunit".to_string(), "1".to_string());
    map.insert("SubstanceProtein.text".to_string(), "1".to_string());
    map.insert(
        "SubstanceReferenceInformation.classification".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.classification.classification".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.classification.domain".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.classification.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.classification.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.classification.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.classification.source".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.classification.subtype".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceReferenceInformation.comment".to_string(), "1".to_string());
    map.insert("SubstanceReferenceInformation.contained".to_string(), "*".to_string());
    map.insert("SubstanceReferenceInformation.extension".to_string(), "*".to_string());
    map.insert("SubstanceReferenceInformation.gene".to_string(), "*".to_string());
    map.insert(
        "SubstanceReferenceInformation.gene.extension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceReferenceInformation.gene.gene".to_string(), "1".to_string());
    map.insert(
        "SubstanceReferenceInformation.gene.geneSequenceOrigin".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceReferenceInformation.gene.id".to_string(), "1".to_string());
    map.insert(
        "SubstanceReferenceInformation.gene.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceReferenceInformation.gene.source".to_string(), "*".to_string());
    map.insert("SubstanceReferenceInformation.geneElement".to_string(), "*".to_string());
    map.insert(
        "SubstanceReferenceInformation.geneElement.element".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.geneElement.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.geneElement.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.geneElement.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.geneElement.source".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.geneElement.type".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceReferenceInformation.id".to_string(), "1".to_string());
    map.insert(
        "SubstanceReferenceInformation.implicitRules".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceReferenceInformation.language".to_string(), "1".to_string());
    map.insert("SubstanceReferenceInformation.meta".to_string(), "1".to_string());
    map.insert(
        "SubstanceReferenceInformation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceReferenceInformation.target".to_string(), "*".to_string());
    map.insert(
        "SubstanceReferenceInformation.target.amountQuantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.target.amountRange".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.target.amountString".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.target.amountType".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.target.extension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceReferenceInformation.target.id".to_string(), "1".to_string());
    map.insert(
        "SubstanceReferenceInformation.target.interaction".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.target.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.target.organism".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.target.organismType".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.target.source".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceReferenceInformation.target.target".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceReferenceInformation.target.type".to_string(), "1".to_string());
    map.insert("SubstanceReferenceInformation.text".to_string(), "1".to_string());
    map.insert("SubstanceSourceMaterial.contained".to_string(), "*".to_string());
    map.insert("SubstanceSourceMaterial.countryOfOrigin".to_string(), "*".to_string());
    map.insert("SubstanceSourceMaterial.developmentStage".to_string(), "1".to_string());
    map.insert("SubstanceSourceMaterial.extension".to_string(), "*".to_string());
    map.insert(
        "SubstanceSourceMaterial.fractionDescription".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.fractionDescription.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.fractionDescription.fraction".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.fractionDescription.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.fractionDescription.materialType".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.fractionDescription.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.geographicalLocation".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceSourceMaterial.id".to_string(), "1".to_string());
    map.insert("SubstanceSourceMaterial.implicitRules".to_string(), "1".to_string());
    map.insert("SubstanceSourceMaterial.language".to_string(), "1".to_string());
    map.insert("SubstanceSourceMaterial.meta".to_string(), "1".to_string());
    map.insert("SubstanceSourceMaterial.modifierExtension".to_string(), "*".to_string());
    map.insert("SubstanceSourceMaterial.organism".to_string(), "1".to_string());
    map.insert("SubstanceSourceMaterial.organism.author".to_string(), "*".to_string());
    map.insert(
        "SubstanceSourceMaterial.organism.author.authorDescription".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.author.authorType".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.author.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.author.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.author.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.extension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceSourceMaterial.organism.family".to_string(), "1".to_string());
    map.insert("SubstanceSourceMaterial.organism.genus".to_string(), "1".to_string());
    map.insert("SubstanceSourceMaterial.organism.hybrid".to_string(), "1".to_string());
    map.insert(
        "SubstanceSourceMaterial.organism.hybrid.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.hybrid.hybridType".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.hybrid.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.hybrid.maternalOrganismId".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.hybrid.maternalOrganismName".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.hybrid.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.hybrid.paternalOrganismId".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.hybrid.paternalOrganismName".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSourceMaterial.organism.id".to_string(), "1".to_string());
    map.insert(
        "SubstanceSourceMaterial.organism.intraspecificDescription".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.intraspecificType".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.organismGeneral".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.organismGeneral.class".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.organismGeneral.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.organismGeneral.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.organismGeneral.kingdom".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.organismGeneral.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.organismGeneral.order".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.organism.organismGeneral.phylum".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSourceMaterial.organism.species".to_string(), "1".to_string());
    map.insert("SubstanceSourceMaterial.organismId".to_string(), "1".to_string());
    map.insert("SubstanceSourceMaterial.organismName".to_string(), "1".to_string());
    map.insert("SubstanceSourceMaterial.parentSubstanceId".to_string(), "*".to_string());
    map.insert(
        "SubstanceSourceMaterial.parentSubstanceName".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceSourceMaterial.partDescription".to_string(), "*".to_string());
    map.insert(
        "SubstanceSourceMaterial.partDescription.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.partDescription.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.partDescription.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.partDescription.part".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.partDescription.partLocation".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.sourceMaterialClass".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.sourceMaterialState".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSourceMaterial.sourceMaterialType".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSourceMaterial.text".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.code".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.code.code".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.code.comment".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.code.extension".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.code.id".to_string(), "1".to_string());
    map.insert(
        "SubstanceSpecification.code.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceSpecification.code.source".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.code.status".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.code.statusDate".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.comment".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.contained".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.description".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.domain".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.extension".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.id".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.identifier".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.implicitRules".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.language".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.meta".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.modifierExtension".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.moiety".to_string(), "*".to_string());
    map.insert(
        "SubstanceSpecification.moiety.amountQuantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.moiety.amountString".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSpecification.moiety.extension".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.moiety.id".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.moiety.identifier".to_string(), "1".to_string());
    map.insert(
        "SubstanceSpecification.moiety.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSpecification.moiety.molecularFormula".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSpecification.moiety.name".to_string(), "1".to_string());
    map.insert(
        "SubstanceSpecification.moiety.opticalActivity".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSpecification.moiety.role".to_string(), "1".to_string());
    map.insert(
        "SubstanceSpecification.moiety.stereochemistry".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSpecification.molecularWeight".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.name".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.name.domain".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.name.extension".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.name.id".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.name.jurisdiction".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.name.language".to_string(), "*".to_string());
    map.insert(
        "SubstanceSpecification.name.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceSpecification.name.name".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.name.official".to_string(), "*".to_string());
    map.insert(
        "SubstanceSpecification.name.official.authority".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSpecification.name.official.date".to_string(), "1".to_string());
    map.insert(
        "SubstanceSpecification.name.official.extension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceSpecification.name.official.id".to_string(), "1".to_string());
    map.insert(
        "SubstanceSpecification.name.official.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSpecification.name.official.status".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSpecification.name.preferred".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.name.source".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.name.status".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.name.synonym".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.name.translation".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.name.type".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.nucleicAcid".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.polymer".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.property".to_string(), "*".to_string());
    map.insert(
        "SubstanceSpecification.property.amountQuantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.property.amountString".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSpecification.property.category".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.property.code".to_string(), "1".to_string());
    map.insert(
        "SubstanceSpecification.property.definingSubstanceCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.property.definingSubstanceReference".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSpecification.property.extension".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.property.id".to_string(), "1".to_string());
    map.insert(
        "SubstanceSpecification.property.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSpecification.property.parameters".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSpecification.protein".to_string(), "1".to_string());
    map.insert(
        "SubstanceSpecification.referenceInformation".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSpecification.relationship".to_string(), "*".to_string());
    map.insert(
        "SubstanceSpecification.relationship.amountQuantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.relationship.amountRange".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.relationship.amountRatio".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.relationship.amountRatioLowLimit".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.relationship.amountString".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.relationship.amountType".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.relationship.extension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceSpecification.relationship.id".to_string(), "1".to_string());
    map.insert(
        "SubstanceSpecification.relationship.isDefining".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.relationship.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSpecification.relationship.relationship".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.relationship.source".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSpecification.relationship.substanceCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.relationship.substanceReference".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSpecification.source".to_string(), "*".to_string());
    map.insert("SubstanceSpecification.sourceMaterial".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.status".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.structure".to_string(), "1".to_string());
    map.insert(
        "SubstanceSpecification.structure.extension".to_string(),
        "*".to_string(),
    );
    map.insert("SubstanceSpecification.structure.id".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.structure.isotope".to_string(), "*".to_string());
    map.insert(
        "SubstanceSpecification.structure.isotope.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.isotope.halfLife".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.isotope.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.isotope.identifier".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.isotope.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.isotope.molecularWeight".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.isotope.molecularWeight.amount".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.isotope.molecularWeight.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.isotope.molecularWeight.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.isotope.molecularWeight.method".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.isotope.molecularWeight.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.isotope.molecularWeight.type".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.isotope.name".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.isotope.substitution".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.molecularFormula".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.molecularFormulaByMoiety".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.molecularWeight".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.opticalActivity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.representation".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.representation.attachment".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.representation.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.representation.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.representation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.representation.representation".to_string(),
        "1".to_string(),
    );
    map.insert(
        "SubstanceSpecification.structure.representation.type".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSpecification.structure.source".to_string(), "*".to_string());
    map.insert(
        "SubstanceSpecification.structure.stereochemistry".to_string(),
        "1".to_string(),
    );
    map.insert("SubstanceSpecification.text".to_string(), "1".to_string());
    map.insert("SubstanceSpecification.type".to_string(), "1".to_string());
    map.insert("SupplyDelivery.basedOn".to_string(), "*".to_string());
    map.insert("SupplyDelivery.contained".to_string(), "*".to_string());
    map.insert("SupplyDelivery.destination".to_string(), "1".to_string());
    map.insert("SupplyDelivery.extension".to_string(), "*".to_string());
    map.insert("SupplyDelivery.id".to_string(), "1".to_string());
    map.insert("SupplyDelivery.identifier".to_string(), "*".to_string());
    map.insert("SupplyDelivery.implicitRules".to_string(), "1".to_string());
    map.insert("SupplyDelivery.language".to_string(), "1".to_string());
    map.insert("SupplyDelivery.meta".to_string(), "1".to_string());
    map.insert("SupplyDelivery.modifierExtension".to_string(), "*".to_string());
    map.insert("SupplyDelivery.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("SupplyDelivery.occurrencePeriod".to_string(), "1".to_string());
    map.insert("SupplyDelivery.occurrenceTiming".to_string(), "1".to_string());
    map.insert("SupplyDelivery.partOf".to_string(), "*".to_string());
    map.insert("SupplyDelivery.patient".to_string(), "1".to_string());
    map.insert("SupplyDelivery.receiver".to_string(), "*".to_string());
    map.insert("SupplyDelivery.status".to_string(), "1".to_string());
    map.insert("SupplyDelivery.suppliedItem".to_string(), "1".to_string());
    map.insert("SupplyDelivery.suppliedItem.extension".to_string(), "*".to_string());
    map.insert("SupplyDelivery.suppliedItem.id".to_string(), "1".to_string());
    map.insert(
        "SupplyDelivery.suppliedItem.itemCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("SupplyDelivery.suppliedItem.itemReference".to_string(), "1".to_string());
    map.insert(
        "SupplyDelivery.suppliedItem.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("SupplyDelivery.suppliedItem.quantity".to_string(), "1".to_string());
    map.insert("SupplyDelivery.supplier".to_string(), "1".to_string());
    map.insert("SupplyDelivery.text".to_string(), "1".to_string());
    map.insert("SupplyDelivery.type".to_string(), "1".to_string());
    map.insert("SupplyRequest.authoredOn".to_string(), "1".to_string());
    map.insert("SupplyRequest.category".to_string(), "1".to_string());
    map.insert("SupplyRequest.contained".to_string(), "*".to_string());
    map.insert("SupplyRequest.deliverFrom".to_string(), "1".to_string());
    map.insert("SupplyRequest.deliverTo".to_string(), "1".to_string());
    map.insert("SupplyRequest.extension".to_string(), "*".to_string());
    map.insert("SupplyRequest.id".to_string(), "1".to_string());
    map.insert("SupplyRequest.identifier".to_string(), "*".to_string());
    map.insert("SupplyRequest.implicitRules".to_string(), "1".to_string());
    map.insert("SupplyRequest.itemCodeableConcept".to_string(), "1".to_string());
    map.insert("SupplyRequest.itemReference".to_string(), "1".to_string());
    map.insert("SupplyRequest.language".to_string(), "1".to_string());
    map.insert("SupplyRequest.meta".to_string(), "1".to_string());
    map.insert("SupplyRequest.modifierExtension".to_string(), "*".to_string());
    map.insert("SupplyRequest.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("SupplyRequest.occurrencePeriod".to_string(), "1".to_string());
    map.insert("SupplyRequest.occurrenceTiming".to_string(), "1".to_string());
    map.insert("SupplyRequest.parameter".to_string(), "*".to_string());
    map.insert("SupplyRequest.parameter.code".to_string(), "1".to_string());
    map.insert("SupplyRequest.parameter.extension".to_string(), "*".to_string());
    map.insert("SupplyRequest.parameter.id".to_string(), "1".to_string());
    map.insert("SupplyRequest.parameter.modifierExtension".to_string(), "*".to_string());
    map.insert("SupplyRequest.parameter.valueBoolean".to_string(), "1".to_string());
    map.insert(
        "SupplyRequest.parameter.valueCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("SupplyRequest.parameter.valueQuantity".to_string(), "1".to_string());
    map.insert("SupplyRequest.parameter.valueRange".to_string(), "1".to_string());
    map.insert("SupplyRequest.priority".to_string(), "1".to_string());
    map.insert("SupplyRequest.quantity".to_string(), "1".to_string());
    map.insert("SupplyRequest.reasonCode".to_string(), "*".to_string());
    map.insert("SupplyRequest.reasonReference".to_string(), "*".to_string());
    map.insert("SupplyRequest.requester".to_string(), "1".to_string());
    map.insert("SupplyRequest.status".to_string(), "1".to_string());
    map.insert("SupplyRequest.supplier".to_string(), "*".to_string());
    map.insert("SupplyRequest.text".to_string(), "1".to_string());
    map.insert("Task.authoredOn".to_string(), "1".to_string());
    map.insert("Task.basedOn".to_string(), "*".to_string());
    map.insert("Task.businessStatus".to_string(), "1".to_string());
    map.insert("Task.code".to_string(), "1".to_string());
    map.insert("Task.contained".to_string(), "*".to_string());
    map.insert("Task.description".to_string(), "1".to_string());
    map.insert("Task.encounter".to_string(), "1".to_string());
    map.insert("Task.executionPeriod".to_string(), "1".to_string());
    map.insert("Task.extension".to_string(), "*".to_string());
    map.insert("Task.focus".to_string(), "1".to_string());
    map.insert("Task.for".to_string(), "1".to_string());
    map.insert("Task.groupIdentifier".to_string(), "1".to_string());
    map.insert("Task.id".to_string(), "1".to_string());
    map.insert("Task.identifier".to_string(), "*".to_string());
    map.insert("Task.implicitRules".to_string(), "1".to_string());
    map.insert("Task.input".to_string(), "*".to_string());
    map.insert("Task.input.extension".to_string(), "*".to_string());
    map.insert("Task.input.id".to_string(), "1".to_string());
    map.insert("Task.input.modifierExtension".to_string(), "*".to_string());
    map.insert("Task.input.type".to_string(), "1".to_string());
    map.insert("Task.input.valueAddress".to_string(), "1".to_string());
    map.insert("Task.input.valueAge".to_string(), "1".to_string());
    map.insert("Task.input.valueAnnotation".to_string(), "1".to_string());
    map.insert("Task.input.valueAttachment".to_string(), "1".to_string());
    map.insert("Task.input.valueBase64Binary".to_string(), "1".to_string());
    map.insert("Task.input.valueBoolean".to_string(), "1".to_string());
    map.insert("Task.input.valueCanonical".to_string(), "1".to_string());
    map.insert("Task.input.valueCode".to_string(), "1".to_string());
    map.insert("Task.input.valueCodeableConcept".to_string(), "1".to_string());
    map.insert("Task.input.valueCoding".to_string(), "1".to_string());
    map.insert("Task.input.valueContactDetail".to_string(), "1".to_string());
    map.insert("Task.input.valueContactPoint".to_string(), "1".to_string());
    map.insert("Task.input.valueContributor".to_string(), "1".to_string());
    map.insert("Task.input.valueCount".to_string(), "1".to_string());
    map.insert("Task.input.valueDataRequirement".to_string(), "1".to_string());
    map.insert("Task.input.valueDate".to_string(), "1".to_string());
    map.insert("Task.input.valueDateTime".to_string(), "1".to_string());
    map.insert("Task.input.valueDecimal".to_string(), "1".to_string());
    map.insert("Task.input.valueDistance".to_string(), "1".to_string());
    map.insert("Task.input.valueDosage".to_string(), "1".to_string());
    map.insert("Task.input.valueDuration".to_string(), "1".to_string());
    map.insert("Task.input.valueExpression".to_string(), "1".to_string());
    map.insert("Task.input.valueHumanName".to_string(), "1".to_string());
    map.insert("Task.input.valueId".to_string(), "1".to_string());
    map.insert("Task.input.valueIdentifier".to_string(), "1".to_string());
    map.insert("Task.input.valueInstant".to_string(), "1".to_string());
    map.insert("Task.input.valueInteger".to_string(), "1".to_string());
    map.insert("Task.input.valueMarkdown".to_string(), "1".to_string());
    map.insert("Task.input.valueMeta".to_string(), "1".to_string());
    map.insert("Task.input.valueMoney".to_string(), "1".to_string());
    map.insert("Task.input.valueOid".to_string(), "1".to_string());
    map.insert("Task.input.valueParameterDefinition".to_string(), "1".to_string());
    map.insert("Task.input.valuePeriod".to_string(), "1".to_string());
    map.insert("Task.input.valuePositiveInt".to_string(), "1".to_string());
    map.insert("Task.input.valueQuantity".to_string(), "1".to_string());
    map.insert("Task.input.valueRange".to_string(), "1".to_string());
    map.insert("Task.input.valueRatio".to_string(), "1".to_string());
    map.insert("Task.input.valueReference".to_string(), "1".to_string());
    map.insert("Task.input.valueRelatedArtifact".to_string(), "1".to_string());
    map.insert("Task.input.valueSampledData".to_string(), "1".to_string());
    map.insert("Task.input.valueSignature".to_string(), "1".to_string());
    map.insert("Task.input.valueString".to_string(), "1".to_string());
    map.insert("Task.input.valueTime".to_string(), "1".to_string());
    map.insert("Task.input.valueTiming".to_string(), "1".to_string());
    map.insert("Task.input.valueTriggerDefinition".to_string(), "1".to_string());
    map.insert("Task.input.valueUnsignedInt".to_string(), "1".to_string());
    map.insert("Task.input.valueUri".to_string(), "1".to_string());
    map.insert("Task.input.valueUrl".to_string(), "1".to_string());
    map.insert("Task.input.valueUsageContext".to_string(), "1".to_string());
    map.insert("Task.input.valueUuid".to_string(), "1".to_string());
    map.insert("Task.instantiatesCanonical".to_string(), "1".to_string());
    map.insert("Task.instantiatesUri".to_string(), "1".to_string());
    map.insert("Task.insurance".to_string(), "*".to_string());
    map.insert("Task.intent".to_string(), "1".to_string());
    map.insert("Task.language".to_string(), "1".to_string());
    map.insert("Task.lastModified".to_string(), "1".to_string());
    map.insert("Task.location".to_string(), "1".to_string());
    map.insert("Task.meta".to_string(), "1".to_string());
    map.insert("Task.modifierExtension".to_string(), "*".to_string());
    map.insert("Task.note".to_string(), "*".to_string());
    map.insert("Task.output".to_string(), "*".to_string());
    map.insert("Task.output.extension".to_string(), "*".to_string());
    map.insert("Task.output.id".to_string(), "1".to_string());
    map.insert("Task.output.modifierExtension".to_string(), "*".to_string());
    map.insert("Task.output.type".to_string(), "1".to_string());
    map.insert("Task.output.valueAddress".to_string(), "1".to_string());
    map.insert("Task.output.valueAge".to_string(), "1".to_string());
    map.insert("Task.output.valueAnnotation".to_string(), "1".to_string());
    map.insert("Task.output.valueAttachment".to_string(), "1".to_string());
    map.insert("Task.output.valueBase64Binary".to_string(), "1".to_string());
    map.insert("Task.output.valueBoolean".to_string(), "1".to_string());
    map.insert("Task.output.valueCanonical".to_string(), "1".to_string());
    map.insert("Task.output.valueCode".to_string(), "1".to_string());
    map.insert("Task.output.valueCodeableConcept".to_string(), "1".to_string());
    map.insert("Task.output.valueCoding".to_string(), "1".to_string());
    map.insert("Task.output.valueContactDetail".to_string(), "1".to_string());
    map.insert("Task.output.valueContactPoint".to_string(), "1".to_string());
    map.insert("Task.output.valueContributor".to_string(), "1".to_string());
    map.insert("Task.output.valueCount".to_string(), "1".to_string());
    map.insert("Task.output.valueDataRequirement".to_string(), "1".to_string());
    map.insert("Task.output.valueDate".to_string(), "1".to_string());
    map.insert("Task.output.valueDateTime".to_string(), "1".to_string());
    map.insert("Task.output.valueDecimal".to_string(), "1".to_string());
    map.insert("Task.output.valueDistance".to_string(), "1".to_string());
    map.insert("Task.output.valueDosage".to_string(), "1".to_string());
    map.insert("Task.output.valueDuration".to_string(), "1".to_string());
    map.insert("Task.output.valueExpression".to_string(), "1".to_string());
    map.insert("Task.output.valueHumanName".to_string(), "1".to_string());
    map.insert("Task.output.valueId".to_string(), "1".to_string());
    map.insert("Task.output.valueIdentifier".to_string(), "1".to_string());
    map.insert("Task.output.valueInstant".to_string(), "1".to_string());
    map.insert("Task.output.valueInteger".to_string(), "1".to_string());
    map.insert("Task.output.valueMarkdown".to_string(), "1".to_string());
    map.insert("Task.output.valueMeta".to_string(), "1".to_string());
    map.insert("Task.output.valueMoney".to_string(), "1".to_string());
    map.insert("Task.output.valueOid".to_string(), "1".to_string());
    map.insert("Task.output.valueParameterDefinition".to_string(), "1".to_string());
    map.insert("Task.output.valuePeriod".to_string(), "1".to_string());
    map.insert("Task.output.valuePositiveInt".to_string(), "1".to_string());
    map.insert("Task.output.valueQuantity".to_string(), "1".to_string());
    map.insert("Task.output.valueRange".to_string(), "1".to_string());
    map.insert("Task.output.valueRatio".to_string(), "1".to_string());
    map.insert("Task.output.valueReference".to_string(), "1".to_string());
    map.insert("Task.output.valueRelatedArtifact".to_string(), "1".to_string());
    map.insert("Task.output.valueSampledData".to_string(), "1".to_string());
    map.insert("Task.output.valueSignature".to_string(), "1".to_string());
    map.insert("Task.output.valueString".to_string(), "1".to_string());
    map.insert("Task.output.valueTime".to_string(), "1".to_string());
    map.insert("Task.output.valueTiming".to_string(), "1".to_string());
    map.insert("Task.output.valueTriggerDefinition".to_string(), "1".to_string());
    map.insert("Task.output.valueUnsignedInt".to_string(), "1".to_string());
    map.insert("Task.output.valueUri".to_string(), "1".to_string());
    map.insert("Task.output.valueUrl".to_string(), "1".to_string());
    map.insert("Task.output.valueUsageContext".to_string(), "1".to_string());
    map.insert("Task.output.valueUuid".to_string(), "1".to_string());
    map.insert("Task.owner".to_string(), "1".to_string());
    map.insert("Task.partOf".to_string(), "*".to_string());
    map.insert("Task.performerType".to_string(), "*".to_string());
    map.insert("Task.priority".to_string(), "1".to_string());
    map.insert("Task.reasonCode".to_string(), "1".to_string());
    map.insert("Task.reasonReference".to_string(), "1".to_string());
    map.insert("Task.relevantHistory".to_string(), "*".to_string());
    map.insert("Task.requester".to_string(), "1".to_string());
    map.insert("Task.restriction".to_string(), "1".to_string());
    map.insert("Task.restriction.extension".to_string(), "*".to_string());
    map.insert("Task.restriction.id".to_string(), "1".to_string());
    map.insert("Task.restriction.modifierExtension".to_string(), "*".to_string());
    map.insert("Task.restriction.period".to_string(), "1".to_string());
    map.insert("Task.restriction.recipient".to_string(), "*".to_string());
    map.insert("Task.restriction.repetitions".to_string(), "1".to_string());
    map.insert("Task.status".to_string(), "1".to_string());
    map.insert("Task.statusReason".to_string(), "1".to_string());
    map.insert("Task.text".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.closure".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.closure.extension".to_string(), "*".to_string());
    map.insert("TerminologyCapabilities.closure.id".to_string(), "1".to_string());
    map.insert(
        "TerminologyCapabilities.closure.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.closure.translation".to_string(),
        "1".to_string(),
    );
    map.insert("TerminologyCapabilities.codeSearch".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.codeSystem".to_string(), "*".to_string());
    map.insert(
        "TerminologyCapabilities.codeSystem.extension".to_string(),
        "*".to_string(),
    );
    map.insert("TerminologyCapabilities.codeSystem.id".to_string(), "1".to_string());
    map.insert(
        "TerminologyCapabilities.codeSystem.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.codeSystem.subsumption".to_string(),
        "1".to_string(),
    );
    map.insert("TerminologyCapabilities.codeSystem.uri".to_string(), "1".to_string());
    map.insert(
        "TerminologyCapabilities.codeSystem.version".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.codeSystem.version.code".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.codeSystem.version.compositional".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.codeSystem.version.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.codeSystem.version.filter".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.codeSystem.version.filter.code".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.codeSystem.version.filter.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.codeSystem.version.filter.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.codeSystem.version.filter.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.codeSystem.version.filter.op".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.codeSystem.version.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.codeSystem.version.isDefault".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.codeSystem.version.language".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.codeSystem.version.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.codeSystem.version.property".to_string(),
        "*".to_string(),
    );
    map.insert("TerminologyCapabilities.contact".to_string(), "*".to_string());
    map.insert("TerminologyCapabilities.contained".to_string(), "*".to_string());
    map.insert("TerminologyCapabilities.copyright".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.date".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.description".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.expansion".to_string(), "1".to_string());
    map.insert(
        "TerminologyCapabilities.expansion.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.expansion.hierarchical".to_string(),
        "1".to_string(),
    );
    map.insert("TerminologyCapabilities.expansion.id".to_string(), "1".to_string());
    map.insert(
        "TerminologyCapabilities.expansion.incomplete".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.expansion.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("TerminologyCapabilities.expansion.paging".to_string(), "1".to_string());
    map.insert(
        "TerminologyCapabilities.expansion.parameter".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.expansion.parameter.documentation".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.expansion.parameter.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.expansion.parameter.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.expansion.parameter.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.expansion.parameter.name".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.expansion.textFilter".to_string(),
        "1".to_string(),
    );
    map.insert("TerminologyCapabilities.experimental".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.extension".to_string(), "*".to_string());
    map.insert("TerminologyCapabilities.id".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.implementation".to_string(), "1".to_string());
    map.insert(
        "TerminologyCapabilities.implementation.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.implementation.extension".to_string(),
        "*".to_string(),
    );
    map.insert("TerminologyCapabilities.implementation.id".to_string(), "1".to_string());
    map.insert(
        "TerminologyCapabilities.implementation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.implementation.url".to_string(),
        "1".to_string(),
    );
    map.insert("TerminologyCapabilities.implicitRules".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.jurisdiction".to_string(), "*".to_string());
    map.insert("TerminologyCapabilities.kind".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.language".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.lockedDate".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.meta".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.modifierExtension".to_string(), "*".to_string());
    map.insert("TerminologyCapabilities.name".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.publisher".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.purpose".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.software".to_string(), "1".to_string());
    map.insert(
        "TerminologyCapabilities.software.extension".to_string(),
        "*".to_string(),
    );
    map.insert("TerminologyCapabilities.software.id".to_string(), "1".to_string());
    map.insert(
        "TerminologyCapabilities.software.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("TerminologyCapabilities.software.name".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.software.version".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.status".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.text".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.title".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.translation".to_string(), "1".to_string());
    map.insert(
        "TerminologyCapabilities.translation.extension".to_string(),
        "*".to_string(),
    );
    map.insert("TerminologyCapabilities.translation.id".to_string(), "1".to_string());
    map.insert(
        "TerminologyCapabilities.translation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.translation.needsMap".to_string(),
        "1".to_string(),
    );
    map.insert("TerminologyCapabilities.url".to_string(), "1".to_string());
    map.insert("TerminologyCapabilities.useContext".to_string(), "*".to_string());
    map.insert("TerminologyCapabilities.validateCode".to_string(), "1".to_string());
    map.insert(
        "TerminologyCapabilities.validateCode.extension".to_string(),
        "*".to_string(),
    );
    map.insert("TerminologyCapabilities.validateCode.id".to_string(), "1".to_string());
    map.insert(
        "TerminologyCapabilities.validateCode.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TerminologyCapabilities.validateCode.translations".to_string(),
        "1".to_string(),
    );
    map.insert("TerminologyCapabilities.version".to_string(), "1".to_string());
    map.insert("TestReport.contained".to_string(), "*".to_string());
    map.insert("TestReport.extension".to_string(), "*".to_string());
    map.insert("TestReport.id".to_string(), "1".to_string());
    map.insert("TestReport.identifier".to_string(), "1".to_string());
    map.insert("TestReport.implicitRules".to_string(), "1".to_string());
    map.insert("TestReport.issued".to_string(), "1".to_string());
    map.insert("TestReport.language".to_string(), "1".to_string());
    map.insert("TestReport.meta".to_string(), "1".to_string());
    map.insert("TestReport.modifierExtension".to_string(), "*".to_string());
    map.insert("TestReport.name".to_string(), "1".to_string());
    map.insert("TestReport.participant".to_string(), "*".to_string());
    map.insert("TestReport.participant.display".to_string(), "1".to_string());
    map.insert("TestReport.participant.extension".to_string(), "*".to_string());
    map.insert("TestReport.participant.id".to_string(), "1".to_string());
    map.insert("TestReport.participant.modifierExtension".to_string(), "*".to_string());
    map.insert("TestReport.participant.type".to_string(), "1".to_string());
    map.insert("TestReport.participant.uri".to_string(), "1".to_string());
    map.insert("TestReport.result".to_string(), "1".to_string());
    map.insert("TestReport.score".to_string(), "1".to_string());
    map.insert("TestReport.setup".to_string(), "1".to_string());
    map.insert("TestReport.setup.action".to_string(), "*".to_string());
    map.insert("TestReport.setup.action.assert".to_string(), "1".to_string());
    map.insert("TestReport.setup.action.assert.detail".to_string(), "1".to_string());
    map.insert("TestReport.setup.action.assert.extension".to_string(), "*".to_string());
    map.insert("TestReport.setup.action.assert.id".to_string(), "1".to_string());
    map.insert("TestReport.setup.action.assert.message".to_string(), "1".to_string());
    map.insert(
        "TestReport.setup.action.assert.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("TestReport.setup.action.assert.result".to_string(), "1".to_string());
    map.insert("TestReport.setup.action.extension".to_string(), "*".to_string());
    map.insert("TestReport.setup.action.id".to_string(), "1".to_string());
    map.insert("TestReport.setup.action.modifierExtension".to_string(), "*".to_string());
    map.insert("TestReport.setup.action.operation".to_string(), "1".to_string());
    map.insert("TestReport.setup.action.operation.detail".to_string(), "1".to_string());
    map.insert(
        "TestReport.setup.action.operation.extension".to_string(),
        "*".to_string(),
    );
    map.insert("TestReport.setup.action.operation.id".to_string(), "1".to_string());
    map.insert("TestReport.setup.action.operation.message".to_string(), "1".to_string());
    map.insert(
        "TestReport.setup.action.operation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("TestReport.setup.action.operation.result".to_string(), "1".to_string());
    map.insert("TestReport.setup.extension".to_string(), "*".to_string());
    map.insert("TestReport.setup.id".to_string(), "1".to_string());
    map.insert("TestReport.setup.modifierExtension".to_string(), "*".to_string());
    map.insert("TestReport.status".to_string(), "1".to_string());
    map.insert("TestReport.teardown".to_string(), "1".to_string());
    map.insert("TestReport.teardown.action".to_string(), "*".to_string());
    map.insert("TestReport.teardown.action.extension".to_string(), "*".to_string());
    map.insert("TestReport.teardown.action.id".to_string(), "1".to_string());
    map.insert(
        "TestReport.teardown.action.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("TestReport.teardown.action.operation".to_string(), "1".to_string());
    map.insert("TestReport.teardown.extension".to_string(), "*".to_string());
    map.insert("TestReport.teardown.id".to_string(), "1".to_string());
    map.insert("TestReport.teardown.modifierExtension".to_string(), "*".to_string());
    map.insert("TestReport.test".to_string(), "*".to_string());
    map.insert("TestReport.test.action".to_string(), "*".to_string());
    map.insert("TestReport.test.action.assert".to_string(), "1".to_string());
    map.insert("TestReport.test.action.extension".to_string(), "*".to_string());
    map.insert("TestReport.test.action.id".to_string(), "1".to_string());
    map.insert("TestReport.test.action.modifierExtension".to_string(), "*".to_string());
    map.insert("TestReport.test.action.operation".to_string(), "1".to_string());
    map.insert("TestReport.test.description".to_string(), "1".to_string());
    map.insert("TestReport.test.extension".to_string(), "*".to_string());
    map.insert("TestReport.test.id".to_string(), "1".to_string());
    map.insert("TestReport.test.modifierExtension".to_string(), "*".to_string());
    map.insert("TestReport.test.name".to_string(), "1".to_string());
    map.insert("TestReport.testScript".to_string(), "1".to_string());
    map.insert("TestReport.tester".to_string(), "1".to_string());
    map.insert("TestReport.text".to_string(), "1".to_string());
    map.insert("TestScript.contact".to_string(), "*".to_string());
    map.insert("TestScript.contained".to_string(), "*".to_string());
    map.insert("TestScript.copyright".to_string(), "1".to_string());
    map.insert("TestScript.date".to_string(), "1".to_string());
    map.insert("TestScript.description".to_string(), "1".to_string());
    map.insert("TestScript.destination".to_string(), "*".to_string());
    map.insert("TestScript.destination.extension".to_string(), "*".to_string());
    map.insert("TestScript.destination.id".to_string(), "1".to_string());
    map.insert("TestScript.destination.index".to_string(), "1".to_string());
    map.insert("TestScript.destination.modifierExtension".to_string(), "*".to_string());
    map.insert("TestScript.destination.profile".to_string(), "1".to_string());
    map.insert("TestScript.experimental".to_string(), "1".to_string());
    map.insert("TestScript.extension".to_string(), "*".to_string());
    map.insert("TestScript.fixture".to_string(), "*".to_string());
    map.insert("TestScript.fixture.autocreate".to_string(), "1".to_string());
    map.insert("TestScript.fixture.autodelete".to_string(), "1".to_string());
    map.insert("TestScript.fixture.extension".to_string(), "*".to_string());
    map.insert("TestScript.fixture.id".to_string(), "1".to_string());
    map.insert("TestScript.fixture.modifierExtension".to_string(), "*".to_string());
    map.insert("TestScript.fixture.resource".to_string(), "1".to_string());
    map.insert("TestScript.id".to_string(), "1".to_string());
    map.insert("TestScript.identifier".to_string(), "1".to_string());
    map.insert("TestScript.implicitRules".to_string(), "1".to_string());
    map.insert("TestScript.jurisdiction".to_string(), "*".to_string());
    map.insert("TestScript.language".to_string(), "1".to_string());
    map.insert("TestScript.meta".to_string(), "1".to_string());
    map.insert("TestScript.metadata".to_string(), "1".to_string());
    map.insert("TestScript.metadata.capability".to_string(), "*".to_string());
    map.insert(
        "TestScript.metadata.capability.capabilities".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.metadata.capability.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.metadata.capability.destination".to_string(),
        "1".to_string(),
    );
    map.insert("TestScript.metadata.capability.extension".to_string(), "*".to_string());
    map.insert("TestScript.metadata.capability.id".to_string(), "1".to_string());
    map.insert("TestScript.metadata.capability.link".to_string(), "*".to_string());
    map.insert(
        "TestScript.metadata.capability.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("TestScript.metadata.capability.origin".to_string(), "*".to_string());
    map.insert("TestScript.metadata.capability.required".to_string(), "1".to_string());
    map.insert("TestScript.metadata.capability.validated".to_string(), "1".to_string());
    map.insert("TestScript.metadata.extension".to_string(), "*".to_string());
    map.insert("TestScript.metadata.id".to_string(), "1".to_string());
    map.insert("TestScript.metadata.link".to_string(), "*".to_string());
    map.insert("TestScript.metadata.link.description".to_string(), "1".to_string());
    map.insert("TestScript.metadata.link.extension".to_string(), "*".to_string());
    map.insert("TestScript.metadata.link.id".to_string(), "1".to_string());
    map.insert(
        "TestScript.metadata.link.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("TestScript.metadata.link.url".to_string(), "1".to_string());
    map.insert("TestScript.metadata.modifierExtension".to_string(), "*".to_string());
    map.insert("TestScript.modifierExtension".to_string(), "*".to_string());
    map.insert("TestScript.name".to_string(), "1".to_string());
    map.insert("TestScript.origin".to_string(), "*".to_string());
    map.insert("TestScript.origin.extension".to_string(), "*".to_string());
    map.insert("TestScript.origin.id".to_string(), "1".to_string());
    map.insert("TestScript.origin.index".to_string(), "1".to_string());
    map.insert("TestScript.origin.modifierExtension".to_string(), "*".to_string());
    map.insert("TestScript.origin.profile".to_string(), "1".to_string());
    map.insert("TestScript.profile".to_string(), "*".to_string());
    map.insert("TestScript.publisher".to_string(), "1".to_string());
    map.insert("TestScript.purpose".to_string(), "1".to_string());
    map.insert("TestScript.setup".to_string(), "1".to_string());
    map.insert("TestScript.setup.action".to_string(), "*".to_string());
    map.insert("TestScript.setup.action.assert".to_string(), "1".to_string());
    map.insert(
        "TestScript.setup.action.assert.compareToSourceExpression".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.compareToSourceId".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.compareToSourcePath".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.contentType".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.description".to_string(),
        "1".to_string(),
    );
    map.insert("TestScript.setup.action.assert.direction".to_string(), "1".to_string());
    map.insert("TestScript.setup.action.assert.expression".to_string(), "1".to_string());
    map.insert("TestScript.setup.action.assert.extension".to_string(), "*".to_string());
    map.insert(
        "TestScript.setup.action.assert.headerField".to_string(),
        "1".to_string(),
    );
    map.insert("TestScript.setup.action.assert.id".to_string(), "1".to_string());
    map.insert("TestScript.setup.action.assert.label".to_string(), "1".to_string());
    map.insert("TestScript.setup.action.assert.minimumId".to_string(), "1".to_string());
    map.insert(
        "TestScript.setup.action.assert.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.navigationLinks".to_string(),
        "1".to_string(),
    );
    map.insert("TestScript.setup.action.assert.operator".to_string(), "1".to_string());
    map.insert("TestScript.setup.action.assert.path".to_string(), "1".to_string());
    map.insert(
        "TestScript.setup.action.assert.requestMethod".to_string(),
        "1".to_string(),
    );
    map.insert("TestScript.setup.action.assert.requestURL".to_string(), "1".to_string());
    map.insert("TestScript.setup.action.assert.resource".to_string(), "1".to_string());
    map.insert("TestScript.setup.action.assert.response".to_string(), "1".to_string());
    map.insert(
        "TestScript.setup.action.assert.responseCode".to_string(),
        "1".to_string(),
    );
    map.insert("TestScript.setup.action.assert.sourceId".to_string(), "1".to_string());
    map.insert(
        "TestScript.setup.action.assert.validateProfileId".to_string(),
        "1".to_string(),
    );
    map.insert("TestScript.setup.action.assert.value".to_string(), "1".to_string());
    map.insert(
        "TestScript.setup.action.assert.warningOnly".to_string(),
        "1".to_string(),
    );
    map.insert("TestScript.setup.action.extension".to_string(), "*".to_string());
    map.insert("TestScript.setup.action.id".to_string(), "1".to_string());
    map.insert("TestScript.setup.action.modifierExtension".to_string(), "*".to_string());
    map.insert("TestScript.setup.action.operation".to_string(), "1".to_string());
    map.insert("TestScript.setup.action.operation.accept".to_string(), "1".to_string());
    map.insert(
        "TestScript.setup.action.operation.contentType".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.operation.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.operation.destination".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.operation.encodeRequestUrl".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.operation.extension".to_string(),
        "*".to_string(),
    );
    map.insert("TestScript.setup.action.operation.id".to_string(), "1".to_string());
    map.insert("TestScript.setup.action.operation.label".to_string(), "1".to_string());
    map.insert("TestScript.setup.action.operation.method".to_string(), "1".to_string());
    map.insert(
        "TestScript.setup.action.operation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("TestScript.setup.action.operation.origin".to_string(), "1".to_string());
    map.insert("TestScript.setup.action.operation.params".to_string(), "1".to_string());
    map.insert(
        "TestScript.setup.action.operation.requestHeader".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TestScript.setup.action.operation.requestHeader.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TestScript.setup.action.operation.requestHeader.field".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.operation.requestHeader.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.operation.requestHeader.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TestScript.setup.action.operation.requestHeader.value".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.operation.requestId".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.operation.resource".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.operation.responseId".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.operation.sourceId".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.operation.targetId".to_string(),
        "1".to_string(),
    );
    map.insert("TestScript.setup.action.operation.type".to_string(), "1".to_string());
    map.insert("TestScript.setup.action.operation.url".to_string(), "1".to_string());
    map.insert("TestScript.setup.extension".to_string(), "*".to_string());
    map.insert("TestScript.setup.id".to_string(), "1".to_string());
    map.insert("TestScript.setup.modifierExtension".to_string(), "*".to_string());
    map.insert("TestScript.status".to_string(), "1".to_string());
    map.insert("TestScript.teardown".to_string(), "1".to_string());
    map.insert("TestScript.teardown.action".to_string(), "*".to_string());
    map.insert("TestScript.teardown.action.extension".to_string(), "*".to_string());
    map.insert("TestScript.teardown.action.id".to_string(), "1".to_string());
    map.insert(
        "TestScript.teardown.action.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("TestScript.teardown.action.operation".to_string(), "1".to_string());
    map.insert("TestScript.teardown.extension".to_string(), "*".to_string());
    map.insert("TestScript.teardown.id".to_string(), "1".to_string());
    map.insert("TestScript.teardown.modifierExtension".to_string(), "*".to_string());
    map.insert("TestScript.test".to_string(), "*".to_string());
    map.insert("TestScript.test.action".to_string(), "*".to_string());
    map.insert("TestScript.test.action.assert".to_string(), "1".to_string());
    map.insert("TestScript.test.action.extension".to_string(), "*".to_string());
    map.insert("TestScript.test.action.id".to_string(), "1".to_string());
    map.insert("TestScript.test.action.modifierExtension".to_string(), "*".to_string());
    map.insert("TestScript.test.action.operation".to_string(), "1".to_string());
    map.insert("TestScript.test.description".to_string(), "1".to_string());
    map.insert("TestScript.test.extension".to_string(), "*".to_string());
    map.insert("TestScript.test.id".to_string(), "1".to_string());
    map.insert("TestScript.test.modifierExtension".to_string(), "*".to_string());
    map.insert("TestScript.test.name".to_string(), "1".to_string());
    map.insert("TestScript.text".to_string(), "1".to_string());
    map.insert("TestScript.title".to_string(), "1".to_string());
    map.insert("TestScript.url".to_string(), "1".to_string());
    map.insert("TestScript.useContext".to_string(), "*".to_string());
    map.insert("TestScript.variable".to_string(), "*".to_string());
    map.insert("TestScript.variable.defaultValue".to_string(), "1".to_string());
    map.insert("TestScript.variable.description".to_string(), "1".to_string());
    map.insert("TestScript.variable.expression".to_string(), "1".to_string());
    map.insert("TestScript.variable.extension".to_string(), "*".to_string());
    map.insert("TestScript.variable.headerField".to_string(), "1".to_string());
    map.insert("TestScript.variable.hint".to_string(), "1".to_string());
    map.insert("TestScript.variable.id".to_string(), "1".to_string());
    map.insert("TestScript.variable.modifierExtension".to_string(), "*".to_string());
    map.insert("TestScript.variable.name".to_string(), "1".to_string());
    map.insert("TestScript.variable.path".to_string(), "1".to_string());
    map.insert("TestScript.variable.sourceId".to_string(), "1".to_string());
    map.insert("TestScript.version".to_string(), "1".to_string());
    map.insert("Timing.code".to_string(), "1".to_string());
    map.insert("Timing.event".to_string(), "*".to_string());
    map.insert("Timing.extension".to_string(), "*".to_string());
    map.insert("Timing.id".to_string(), "1".to_string());
    map.insert("Timing.modifierExtension".to_string(), "*".to_string());
    map.insert("Timing.repeat".to_string(), "1".to_string());
    map.insert("Timing.repeat.boundsDuration".to_string(), "1".to_string());
    map.insert("Timing.repeat.boundsPeriod".to_string(), "1".to_string());
    map.insert("Timing.repeat.boundsRange".to_string(), "1".to_string());
    map.insert("Timing.repeat.count".to_string(), "1".to_string());
    map.insert("Timing.repeat.countMax".to_string(), "1".to_string());
    map.insert("Timing.repeat.dayOfWeek".to_string(), "*".to_string());
    map.insert("Timing.repeat.duration".to_string(), "1".to_string());
    map.insert("Timing.repeat.durationMax".to_string(), "1".to_string());
    map.insert("Timing.repeat.durationUnit".to_string(), "1".to_string());
    map.insert("Timing.repeat.extension".to_string(), "*".to_string());
    map.insert("Timing.repeat.frequency".to_string(), "1".to_string());
    map.insert("Timing.repeat.frequencyMax".to_string(), "1".to_string());
    map.insert("Timing.repeat.id".to_string(), "1".to_string());
    map.insert("Timing.repeat.offset".to_string(), "1".to_string());
    map.insert("Timing.repeat.period".to_string(), "1".to_string());
    map.insert("Timing.repeat.periodMax".to_string(), "1".to_string());
    map.insert("Timing.repeat.periodUnit".to_string(), "1".to_string());
    map.insert("Timing.repeat.timeOfDay".to_string(), "*".to_string());
    map.insert("Timing.repeat.when".to_string(), "*".to_string());
    map.insert("TriggerDefinition.condition".to_string(), "1".to_string());
    map.insert("TriggerDefinition.data".to_string(), "*".to_string());
    map.insert("TriggerDefinition.extension".to_string(), "*".to_string());
    map.insert("TriggerDefinition.id".to_string(), "1".to_string());
    map.insert("TriggerDefinition.name".to_string(), "1".to_string());
    map.insert("TriggerDefinition.timingDate".to_string(), "1".to_string());
    map.insert("TriggerDefinition.timingDateTime".to_string(), "1".to_string());
    map.insert("TriggerDefinition.timingReference".to_string(), "1".to_string());
    map.insert("TriggerDefinition.timingTiming".to_string(), "1".to_string());
    map.insert("TriggerDefinition.type".to_string(), "1".to_string());
    map.insert("UsageContext.code".to_string(), "1".to_string());
    map.insert("UsageContext.extension".to_string(), "*".to_string());
    map.insert("UsageContext.id".to_string(), "1".to_string());
    map.insert("UsageContext.valueCodeableConcept".to_string(), "1".to_string());
    map.insert("UsageContext.valueQuantity".to_string(), "1".to_string());
    map.insert("UsageContext.valueRange".to_string(), "1".to_string());
    map.insert("UsageContext.valueReference".to_string(), "1".to_string());
    map.insert("ValueSet.compose".to_string(), "1".to_string());
    map.insert("ValueSet.compose.exclude".to_string(), "*".to_string());
    map.insert("ValueSet.compose.extension".to_string(), "*".to_string());
    map.insert("ValueSet.compose.id".to_string(), "1".to_string());
    map.insert("ValueSet.compose.inactive".to_string(), "1".to_string());
    map.insert("ValueSet.compose.include".to_string(), "*".to_string());
    map.insert("ValueSet.compose.include.concept".to_string(), "*".to_string());
    map.insert("ValueSet.compose.include.concept.code".to_string(), "1".to_string());
    map.insert(
        "ValueSet.compose.include.concept.designation".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ValueSet.compose.include.concept.designation.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ValueSet.compose.include.concept.designation.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ValueSet.compose.include.concept.designation.language".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ValueSet.compose.include.concept.designation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ValueSet.compose.include.concept.designation.use".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ValueSet.compose.include.concept.designation.value".to_string(),
        "1".to_string(),
    );
    map.insert("ValueSet.compose.include.concept.display".to_string(), "1".to_string());
    map.insert(
        "ValueSet.compose.include.concept.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ValueSet.compose.include.concept.id".to_string(), "1".to_string());
    map.insert(
        "ValueSet.compose.include.concept.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ValueSet.compose.include.extension".to_string(), "*".to_string());
    map.insert("ValueSet.compose.include.filter".to_string(), "*".to_string());
    map.insert("ValueSet.compose.include.filter.extension".to_string(), "*".to_string());
    map.insert("ValueSet.compose.include.filter.id".to_string(), "1".to_string());
    map.insert(
        "ValueSet.compose.include.filter.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ValueSet.compose.include.filter.op".to_string(), "1".to_string());
    map.insert("ValueSet.compose.include.filter.property".to_string(), "1".to_string());
    map.insert("ValueSet.compose.include.filter.value".to_string(), "1".to_string());
    map.insert("ValueSet.compose.include.id".to_string(), "1".to_string());
    map.insert(
        "ValueSet.compose.include.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ValueSet.compose.include.system".to_string(), "1".to_string());
    map.insert("ValueSet.compose.include.valueSet".to_string(), "*".to_string());
    map.insert("ValueSet.compose.include.version".to_string(), "1".to_string());
    map.insert("ValueSet.compose.lockedDate".to_string(), "1".to_string());
    map.insert("ValueSet.compose.modifierExtension".to_string(), "*".to_string());
    map.insert("ValueSet.contact".to_string(), "*".to_string());
    map.insert("ValueSet.contained".to_string(), "*".to_string());
    map.insert("ValueSet.copyright".to_string(), "1".to_string());
    map.insert("ValueSet.date".to_string(), "1".to_string());
    map.insert("ValueSet.description".to_string(), "1".to_string());
    map.insert("ValueSet.expansion".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.contains".to_string(), "*".to_string());
    map.insert("ValueSet.expansion.contains.abstract".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.contains.code".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.contains.contains".to_string(), "*".to_string());
    map.insert("ValueSet.expansion.contains.designation".to_string(), "*".to_string());
    map.insert("ValueSet.expansion.contains.display".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.contains.extension".to_string(), "*".to_string());
    map.insert("ValueSet.expansion.contains.id".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.contains.inactive".to_string(), "1".to_string());
    map.insert(
        "ValueSet.expansion.contains.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ValueSet.expansion.contains.system".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.contains.version".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.extension".to_string(), "*".to_string());
    map.insert("ValueSet.expansion.id".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.identifier".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.modifierExtension".to_string(), "*".to_string());
    map.insert("ValueSet.expansion.offset".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.parameter".to_string(), "*".to_string());
    map.insert("ValueSet.expansion.parameter.extension".to_string(), "*".to_string());
    map.insert("ValueSet.expansion.parameter.id".to_string(), "1".to_string());
    map.insert(
        "ValueSet.expansion.parameter.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ValueSet.expansion.parameter.name".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.parameter.valueBoolean".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.parameter.valueCode".to_string(), "1".to_string());
    map.insert(
        "ValueSet.expansion.parameter.valueDateTime".to_string(),
        "1".to_string(),
    );
    map.insert("ValueSet.expansion.parameter.valueDecimal".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.parameter.valueInteger".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.parameter.valueString".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.parameter.valueUri".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.timestamp".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.total".to_string(), "1".to_string());
    map.insert("ValueSet.experimental".to_string(), "1".to_string());
    map.insert("ValueSet.extension".to_string(), "*".to_string());
    map.insert("ValueSet.id".to_string(), "1".to_string());
    map.insert("ValueSet.identifier".to_string(), "*".to_string());
    map.insert("ValueSet.immutable".to_string(), "1".to_string());
    map.insert("ValueSet.implicitRules".to_string(), "1".to_string());
    map.insert("ValueSet.jurisdiction".to_string(), "*".to_string());
    map.insert("ValueSet.language".to_string(), "1".to_string());
    map.insert("ValueSet.meta".to_string(), "1".to_string());
    map.insert("ValueSet.modifierExtension".to_string(), "*".to_string());
    map.insert("ValueSet.name".to_string(), "1".to_string());
    map.insert("ValueSet.publisher".to_string(), "1".to_string());
    map.insert("ValueSet.purpose".to_string(), "1".to_string());
    map.insert("ValueSet.status".to_string(), "1".to_string());
    map.insert("ValueSet.text".to_string(), "1".to_string());
    map.insert("ValueSet.title".to_string(), "1".to_string());
    map.insert("ValueSet.url".to_string(), "1".to_string());
    map.insert("ValueSet.useContext".to_string(), "*".to_string());
    map.insert("ValueSet.version".to_string(), "1".to_string());
    map.insert("VerificationResult.attestation".to_string(), "1".to_string());
    map.insert(
        "VerificationResult.attestation.communicationMethod".to_string(),
        "1".to_string(),
    );
    map.insert("VerificationResult.attestation.date".to_string(), "1".to_string());
    map.insert("VerificationResult.attestation.extension".to_string(), "*".to_string());
    map.insert("VerificationResult.attestation.id".to_string(), "1".to_string());
    map.insert(
        "VerificationResult.attestation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("VerificationResult.attestation.onBehalfOf".to_string(), "1".to_string());
    map.insert(
        "VerificationResult.attestation.proxyIdentityCertificate".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VerificationResult.attestation.proxySignature".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VerificationResult.attestation.sourceIdentityCertificate".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VerificationResult.attestation.sourceSignature".to_string(),
        "1".to_string(),
    );
    map.insert("VerificationResult.attestation.who".to_string(), "1".to_string());
    map.insert("VerificationResult.contained".to_string(), "*".to_string());
    map.insert("VerificationResult.extension".to_string(), "*".to_string());
    map.insert("VerificationResult.failureAction".to_string(), "1".to_string());
    map.insert("VerificationResult.frequency".to_string(), "1".to_string());
    map.insert("VerificationResult.id".to_string(), "1".to_string());
    map.insert("VerificationResult.implicitRules".to_string(), "1".to_string());
    map.insert("VerificationResult.language".to_string(), "1".to_string());
    map.insert("VerificationResult.lastPerformed".to_string(), "1".to_string());
    map.insert("VerificationResult.meta".to_string(), "1".to_string());
    map.insert("VerificationResult.modifierExtension".to_string(), "*".to_string());
    map.insert("VerificationResult.need".to_string(), "1".to_string());
    map.insert("VerificationResult.nextScheduled".to_string(), "1".to_string());
    map.insert("VerificationResult.primarySource".to_string(), "*".to_string());
    map.insert(
        "VerificationResult.primarySource.canPushUpdates".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VerificationResult.primarySource.communicationMethod".to_string(),
        "*".to_string(),
    );
    map.insert(
        "VerificationResult.primarySource.extension".to_string(),
        "*".to_string(),
    );
    map.insert("VerificationResult.primarySource.id".to_string(), "1".to_string());
    map.insert(
        "VerificationResult.primarySource.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "VerificationResult.primarySource.pushTypeAvailable".to_string(),
        "*".to_string(),
    );
    map.insert("VerificationResult.primarySource.type".to_string(), "*".to_string());
    map.insert(
        "VerificationResult.primarySource.validationDate".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VerificationResult.primarySource.validationStatus".to_string(),
        "1".to_string(),
    );
    map.insert("VerificationResult.primarySource.who".to_string(), "1".to_string());
    map.insert("VerificationResult.status".to_string(), "1".to_string());
    map.insert("VerificationResult.statusDate".to_string(), "1".to_string());
    map.insert("VerificationResult.target".to_string(), "*".to_string());
    map.insert("VerificationResult.targetLocation".to_string(), "*".to_string());
    map.insert("VerificationResult.text".to_string(), "1".to_string());
    map.insert("VerificationResult.validationProcess".to_string(), "*".to_string());
    map.insert("VerificationResult.validationType".to_string(), "1".to_string());
    map.insert("VerificationResult.validator".to_string(), "*".to_string());
    map.insert(
        "VerificationResult.validator.attestationSignature".to_string(),
        "1".to_string(),
    );
    map.insert("VerificationResult.validator.extension".to_string(), "*".to_string());
    map.insert("VerificationResult.validator.id".to_string(), "1".to_string());
    map.insert(
        "VerificationResult.validator.identityCertificate".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VerificationResult.validator.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("VerificationResult.validator.organization".to_string(), "1".to_string());
    map.insert("VisionPrescription.contained".to_string(), "*".to_string());
    map.insert("VisionPrescription.created".to_string(), "1".to_string());
    map.insert("VisionPrescription.dateWritten".to_string(), "1".to_string());
    map.insert("VisionPrescription.encounter".to_string(), "1".to_string());
    map.insert("VisionPrescription.extension".to_string(), "*".to_string());
    map.insert("VisionPrescription.id".to_string(), "1".to_string());
    map.insert("VisionPrescription.identifier".to_string(), "*".to_string());
    map.insert("VisionPrescription.implicitRules".to_string(), "1".to_string());
    map.insert("VisionPrescription.language".to_string(), "1".to_string());
    map.insert("VisionPrescription.lensSpecification".to_string(), "*".to_string());
    map.insert("VisionPrescription.lensSpecification.add".to_string(), "1".to_string());
    map.insert("VisionPrescription.lensSpecification.axis".to_string(), "1".to_string());
    map.insert(
        "VisionPrescription.lensSpecification.backCurve".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VisionPrescription.lensSpecification.brand".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VisionPrescription.lensSpecification.color".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VisionPrescription.lensSpecification.cylinder".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VisionPrescription.lensSpecification.diameter".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VisionPrescription.lensSpecification.duration".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VisionPrescription.lensSpecification.extension".to_string(),
        "*".to_string(),
    );
    map.insert("VisionPrescription.lensSpecification.eye".to_string(), "1".to_string());
    map.insert("VisionPrescription.lensSpecification.id".to_string(), "1".to_string());
    map.insert(
        "VisionPrescription.lensSpecification.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("VisionPrescription.lensSpecification.note".to_string(), "*".to_string());
    map.insert(
        "VisionPrescription.lensSpecification.power".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VisionPrescription.lensSpecification.prism".to_string(),
        "*".to_string(),
    );
    map.insert(
        "VisionPrescription.lensSpecification.prism.amount".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VisionPrescription.lensSpecification.prism.base".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VisionPrescription.lensSpecification.prism.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "VisionPrescription.lensSpecification.prism.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VisionPrescription.lensSpecification.prism.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "VisionPrescription.lensSpecification.product".to_string(),
        "1".to_string(),
    );
    map.insert(
        "VisionPrescription.lensSpecification.sphere".to_string(),
        "1".to_string(),
    );
    map.insert("VisionPrescription.meta".to_string(), "1".to_string());
    map.insert("VisionPrescription.modifierExtension".to_string(), "*".to_string());
    map.insert("VisionPrescription.patient".to_string(), "1".to_string());
    map.insert("VisionPrescription.prescriber".to_string(), "1".to_string());
    map.insert("VisionPrescription.status".to_string(), "1".to_string());
    map.insert("VisionPrescription.text".to_string(), "1".to_string());
    map.insert("base64Binary.extension".to_string(), "*".to_string());
    map.insert("base64Binary.id".to_string(), "1".to_string());
    map.insert("base64Binary.value".to_string(), "1".to_string());
    map.insert("boolean.extension".to_string(), "*".to_string());
    map.insert("boolean.id".to_string(), "1".to_string());
    map.insert("boolean.value".to_string(), "1".to_string());
    map.insert("canonical.extension".to_string(), "*".to_string());
    map.insert("canonical.id".to_string(), "1".to_string());
    map.insert("canonical.value".to_string(), "1".to_string());
    map.insert("code.extension".to_string(), "*".to_string());
    map.insert("code.id".to_string(), "1".to_string());
    map.insert("code.value".to_string(), "1".to_string());
    map.insert("date.extension".to_string(), "*".to_string());
    map.insert("date.id".to_string(), "1".to_string());
    map.insert("date.value".to_string(), "1".to_string());
    map.insert("dateTime.extension".to_string(), "*".to_string());
    map.insert("dateTime.id".to_string(), "1".to_string());
    map.insert("dateTime.value".to_string(), "1".to_string());
    map.insert("decimal.extension".to_string(), "*".to_string());
    map.insert("decimal.id".to_string(), "1".to_string());
    map.insert("decimal.value".to_string(), "1".to_string());
    map.insert("id.extension".to_string(), "*".to_string());
    map.insert("id.id".to_string(), "1".to_string());
    map.insert("id.value".to_string(), "1".to_string());
    map.insert("instant.extension".to_string(), "*".to_string());
    map.insert("instant.id".to_string(), "1".to_string());
    map.insert("instant.value".to_string(), "1".to_string());
    map.insert("integer.extension".to_string(), "*".to_string());
    map.insert("integer.id".to_string(), "1".to_string());
    map.insert("integer.value".to_string(), "1".to_string());
    map.insert("markdown.extension".to_string(), "*".to_string());
    map.insert("markdown.id".to_string(), "1".to_string());
    map.insert("markdown.value".to_string(), "1".to_string());
    map.insert("oid.extension".to_string(), "*".to_string());
    map.insert("oid.id".to_string(), "1".to_string());
    map.insert("oid.value".to_string(), "1".to_string());
    map.insert("positiveInt.extension".to_string(), "*".to_string());
    map.insert("positiveInt.id".to_string(), "1".to_string());
    map.insert("positiveInt.value".to_string(), "1".to_string());
    map.insert("string.extension".to_string(), "*".to_string());
    map.insert("string.id".to_string(), "1".to_string());
    map.insert("string.value".to_string(), "1".to_string());
    map.insert("time.extension".to_string(), "*".to_string());
    map.insert("time.id".to_string(), "1".to_string());
    map.insert("time.value".to_string(), "1".to_string());
    map.insert("unsignedInt.extension".to_string(), "*".to_string());
    map.insert("unsignedInt.id".to_string(), "1".to_string());
    map.insert("unsignedInt.value".to_string(), "1".to_string());
    map.insert("uri.extension".to_string(), "*".to_string());
    map.insert("uri.id".to_string(), "1".to_string());
    map.insert("uri.value".to_string(), "1".to_string());
    map.insert("url.extension".to_string(), "*".to_string());
    map.insert("url.id".to_string(), "1".to_string());
    map.insert("url.value".to_string(), "1".to_string());
    map.insert("uuid.extension".to_string(), "*".to_string());
    map.insert("uuid.id".to_string(), "1".to_string());
    map.insert("uuid.value".to_string(), "1".to_string());
    map.insert("xhtml.extension".to_string(), "*".to_string());
    map.insert("xhtml.id".to_string(), "1".to_string());
    map.insert("xhtml.value".to_string(), "1".to_string());
    map
}
