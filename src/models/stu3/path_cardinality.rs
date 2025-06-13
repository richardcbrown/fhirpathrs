use std::collections::HashMap;
pub fn path_cardinality() -> HashMap<String, String> {
    let mut map = HashMap::<String, String>::new();
    map.insert("Account.active".to_string(), "1".to_string());
    map.insert("Account.balance".to_string(), "1".to_string());
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
    map.insert("Account.period".to_string(), "1".to_string());
    map.insert("Account.status".to_string(), "1".to_string());
    map.insert("Account.subject".to_string(), "1".to_string());
    map.insert("Account.text".to_string(), "1".to_string());
    map.insert("Account.type".to_string(), "1".to_string());
    map.insert("ActivityDefinition.approvalDate".to_string(), "1".to_string());
    map.insert("ActivityDefinition.bodySite".to_string(), "*".to_string());
    map.insert("ActivityDefinition.code".to_string(), "1".to_string());
    map.insert("ActivityDefinition.contact".to_string(), "*".to_string());
    map.insert("ActivityDefinition.contained".to_string(), "*".to_string());
    map.insert("ActivityDefinition.contributor".to_string(), "*".to_string());
    map.insert("ActivityDefinition.copyright".to_string(), "1".to_string());
    map.insert("ActivityDefinition.date".to_string(), "1".to_string());
    map.insert("ActivityDefinition.description".to_string(), "1".to_string());
    map.insert("ActivityDefinition.dosage".to_string(), "*".to_string());
    map.insert("ActivityDefinition.dynamicValue".to_string(), "*".to_string());
    map.insert(
        "ActivityDefinition.dynamicValue.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ActivityDefinition.dynamicValue.expression".to_string(),
        "1".to_string(),
    );
    map.insert("ActivityDefinition.dynamicValue.extension".to_string(), "*".to_string());
    map.insert("ActivityDefinition.dynamicValue.id".to_string(), "1".to_string());
    map.insert("ActivityDefinition.dynamicValue.language".to_string(), "1".to_string());
    map.insert(
        "ActivityDefinition.dynamicValue.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ActivityDefinition.dynamicValue.path".to_string(), "1".to_string());
    map.insert("ActivityDefinition.effectivePeriod".to_string(), "1".to_string());
    map.insert("ActivityDefinition.experimental".to_string(), "1".to_string());
    map.insert("ActivityDefinition.extension".to_string(), "*".to_string());
    map.insert("ActivityDefinition.id".to_string(), "1".to_string());
    map.insert("ActivityDefinition.identifier".to_string(), "*".to_string());
    map.insert("ActivityDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("ActivityDefinition.jurisdiction".to_string(), "*".to_string());
    map.insert("ActivityDefinition.kind".to_string(), "1".to_string());
    map.insert("ActivityDefinition.language".to_string(), "1".to_string());
    map.insert("ActivityDefinition.lastReviewDate".to_string(), "1".to_string());
    map.insert("ActivityDefinition.library".to_string(), "*".to_string());
    map.insert("ActivityDefinition.location".to_string(), "1".to_string());
    map.insert("ActivityDefinition.meta".to_string(), "1".to_string());
    map.insert("ActivityDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("ActivityDefinition.name".to_string(), "1".to_string());
    map.insert("ActivityDefinition.participant".to_string(), "*".to_string());
    map.insert("ActivityDefinition.participant.extension".to_string(), "*".to_string());
    map.insert("ActivityDefinition.participant.id".to_string(), "1".to_string());
    map.insert(
        "ActivityDefinition.participant.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ActivityDefinition.participant.role".to_string(), "1".to_string());
    map.insert("ActivityDefinition.participant.type".to_string(), "1".to_string());
    map.insert("ActivityDefinition.productCodeableConcept".to_string(), "1".to_string());
    map.insert("ActivityDefinition.productReference".to_string(), "1".to_string());
    map.insert("ActivityDefinition.publisher".to_string(), "1".to_string());
    map.insert("ActivityDefinition.purpose".to_string(), "1".to_string());
    map.insert("ActivityDefinition.quantity".to_string(), "1".to_string());
    map.insert("ActivityDefinition.relatedArtifact".to_string(), "*".to_string());
    map.insert("ActivityDefinition.status".to_string(), "1".to_string());
    map.insert("ActivityDefinition.text".to_string(), "1".to_string());
    map.insert("ActivityDefinition.timingDateTime".to_string(), "1".to_string());
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
    map.insert("AdverseEvent.category".to_string(), "1".to_string());
    map.insert("AdverseEvent.contained".to_string(), "*".to_string());
    map.insert("AdverseEvent.date".to_string(), "1".to_string());
    map.insert("AdverseEvent.description".to_string(), "1".to_string());
    map.insert("AdverseEvent.eventParticipant".to_string(), "1".to_string());
    map.insert("AdverseEvent.extension".to_string(), "*".to_string());
    map.insert("AdverseEvent.id".to_string(), "1".to_string());
    map.insert("AdverseEvent.identifier".to_string(), "1".to_string());
    map.insert("AdverseEvent.implicitRules".to_string(), "1".to_string());
    map.insert("AdverseEvent.language".to_string(), "1".to_string());
    map.insert("AdverseEvent.location".to_string(), "1".to_string());
    map.insert("AdverseEvent.meta".to_string(), "1".to_string());
    map.insert("AdverseEvent.modifierExtension".to_string(), "*".to_string());
    map.insert("AdverseEvent.outcome".to_string(), "1".to_string());
    map.insert("AdverseEvent.reaction".to_string(), "*".to_string());
    map.insert("AdverseEvent.recorder".to_string(), "1".to_string());
    map.insert("AdverseEvent.referenceDocument".to_string(), "*".to_string());
    map.insert("AdverseEvent.seriousness".to_string(), "1".to_string());
    map.insert("AdverseEvent.study".to_string(), "*".to_string());
    map.insert("AdverseEvent.subject".to_string(), "1".to_string());
    map.insert("AdverseEvent.subjectMedicalHistory".to_string(), "*".to_string());
    map.insert("AdverseEvent.suspectEntity".to_string(), "*".to_string());
    map.insert("AdverseEvent.suspectEntity.causality".to_string(), "1".to_string());
    map.insert(
        "AdverseEvent.suspectEntity.causalityAssessment".to_string(),
        "1".to_string(),
    );
    map.insert(
        "AdverseEvent.suspectEntity.causalityAuthor".to_string(),
        "1".to_string(),
    );
    map.insert(
        "AdverseEvent.suspectEntity.causalityMethod".to_string(),
        "1".to_string(),
    );
    map.insert(
        "AdverseEvent.suspectEntity.causalityProductRelatedness".to_string(),
        "1".to_string(),
    );
    map.insert(
        "AdverseEvent.suspectEntity.causalityResult".to_string(),
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
    map.insert("AdverseEvent.type".to_string(), "1".to_string());
    map.insert("Age.code".to_string(), "1".to_string());
    map.insert("Age.comparator".to_string(), "1".to_string());
    map.insert("Age.extension".to_string(), "*".to_string());
    map.insert("Age.id".to_string(), "1".to_string());
    map.insert("Age.system".to_string(), "1".to_string());
    map.insert("Age.unit".to_string(), "1".to_string());
    map.insert("Age.value".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.assertedDate".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.asserter".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.category".to_string(), "*".to_string());
    map.insert("AllergyIntolerance.clinicalStatus".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.code".to_string(), "1".to_string());
    map.insert("AllergyIntolerance.contained".to_string(), "*".to_string());
    map.insert("AllergyIntolerance.criticality".to_string(), "1".to_string());
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
    map.insert("Appointment.comment".to_string(), "1".to_string());
    map.insert("Appointment.contained".to_string(), "*".to_string());
    map.insert("Appointment.created".to_string(), "1".to_string());
    map.insert("Appointment.description".to_string(), "1".to_string());
    map.insert("Appointment.end".to_string(), "1".to_string());
    map.insert("Appointment.extension".to_string(), "*".to_string());
    map.insert("Appointment.id".to_string(), "1".to_string());
    map.insert("Appointment.identifier".to_string(), "*".to_string());
    map.insert("Appointment.implicitRules".to_string(), "1".to_string());
    map.insert("Appointment.incomingReferral".to_string(), "*".to_string());
    map.insert("Appointment.indication".to_string(), "*".to_string());
    map.insert("Appointment.language".to_string(), "1".to_string());
    map.insert("Appointment.meta".to_string(), "1".to_string());
    map.insert("Appointment.minutesDuration".to_string(), "1".to_string());
    map.insert("Appointment.modifierExtension".to_string(), "*".to_string());
    map.insert("Appointment.participant".to_string(), "*".to_string());
    map.insert("Appointment.participant.actor".to_string(), "1".to_string());
    map.insert("Appointment.participant.extension".to_string(), "*".to_string());
    map.insert("Appointment.participant.id".to_string(), "1".to_string());
    map.insert("Appointment.participant.modifierExtension".to_string(), "*".to_string());
    map.insert("Appointment.participant.required".to_string(), "1".to_string());
    map.insert("Appointment.participant.status".to_string(), "1".to_string());
    map.insert("Appointment.participant.type".to_string(), "*".to_string());
    map.insert("Appointment.priority".to_string(), "1".to_string());
    map.insert("Appointment.reason".to_string(), "*".to_string());
    map.insert("Appointment.requestedPeriod".to_string(), "*".to_string());
    map.insert("Appointment.serviceCategory".to_string(), "1".to_string());
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
    map.insert("AuditEvent.agent.reference".to_string(), "1".to_string());
    map.insert("AuditEvent.agent.requestor".to_string(), "1".to_string());
    map.insert("AuditEvent.agent.role".to_string(), "*".to_string());
    map.insert("AuditEvent.agent.userId".to_string(), "1".to_string());
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
    map.insert("AuditEvent.entity.detail.value".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.extension".to_string(), "*".to_string());
    map.insert("AuditEvent.entity.id".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.identifier".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.lifecycle".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.modifierExtension".to_string(), "*".to_string());
    map.insert("AuditEvent.entity.name".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.query".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.reference".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.role".to_string(), "1".to_string());
    map.insert("AuditEvent.entity.securityLabel".to_string(), "*".to_string());
    map.insert("AuditEvent.entity.type".to_string(), "1".to_string());
    map.insert("AuditEvent.extension".to_string(), "*".to_string());
    map.insert("AuditEvent.id".to_string(), "1".to_string());
    map.insert("AuditEvent.implicitRules".to_string(), "1".to_string());
    map.insert("AuditEvent.language".to_string(), "1".to_string());
    map.insert("AuditEvent.meta".to_string(), "1".to_string());
    map.insert("AuditEvent.modifierExtension".to_string(), "*".to_string());
    map.insert("AuditEvent.outcome".to_string(), "1".to_string());
    map.insert("AuditEvent.outcomeDesc".to_string(), "1".to_string());
    map.insert("AuditEvent.purposeOfEvent".to_string(), "*".to_string());
    map.insert("AuditEvent.recorded".to_string(), "1".to_string());
    map.insert("AuditEvent.source".to_string(), "1".to_string());
    map.insert("AuditEvent.source.extension".to_string(), "*".to_string());
    map.insert("AuditEvent.source.id".to_string(), "1".to_string());
    map.insert("AuditEvent.source.identifier".to_string(), "1".to_string());
    map.insert("AuditEvent.source.modifierExtension".to_string(), "*".to_string());
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
    map.insert("Binary.content".to_string(), "1".to_string());
    map.insert("Binary.contentType".to_string(), "1".to_string());
    map.insert("Binary.id".to_string(), "1".to_string());
    map.insert("Binary.implicitRules".to_string(), "1".to_string());
    map.insert("Binary.language".to_string(), "1".to_string());
    map.insert("Binary.meta".to_string(), "1".to_string());
    map.insert("Binary.securityContext".to_string(), "1".to_string());
    map.insert("BodySite.active".to_string(), "1".to_string());
    map.insert("BodySite.code".to_string(), "1".to_string());
    map.insert("BodySite.contained".to_string(), "*".to_string());
    map.insert("BodySite.description".to_string(), "1".to_string());
    map.insert("BodySite.extension".to_string(), "*".to_string());
    map.insert("BodySite.id".to_string(), "1".to_string());
    map.insert("BodySite.identifier".to_string(), "*".to_string());
    map.insert("BodySite.image".to_string(), "*".to_string());
    map.insert("BodySite.implicitRules".to_string(), "1".to_string());
    map.insert("BodySite.language".to_string(), "1".to_string());
    map.insert("BodySite.meta".to_string(), "1".to_string());
    map.insert("BodySite.modifierExtension".to_string(), "*".to_string());
    map.insert("BodySite.patient".to_string(), "1".to_string());
    map.insert("BodySite.qualifier".to_string(), "*".to_string());
    map.insert("BodySite.text".to_string(), "1".to_string());
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
    map.insert("Bundle.total".to_string(), "1".to_string());
    map.insert("Bundle.type".to_string(), "1".to_string());
    map.insert("CapabilityStatement.acceptUnknown".to_string(), "1".to_string());
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
    map.insert("CapabilityStatement.messaging.event".to_string(), "*".to_string());
    map.insert(
        "CapabilityStatement.messaging.event.category".to_string(),
        "1".to_string(),
    );
    map.insert("CapabilityStatement.messaging.event.code".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.messaging.event.documentation".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.messaging.event.extension".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.messaging.event.focus".to_string(), "1".to_string());
    map.insert("CapabilityStatement.messaging.event.id".to_string(), "1".to_string());
    map.insert("CapabilityStatement.messaging.event.mode".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.messaging.event.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.messaging.event.request".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.messaging.event.response".to_string(),
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
    map.insert("CapabilityStatement.profile".to_string(), "*".to_string());
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
    map.insert(
        "CapabilityStatement.rest.operation.definition".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.operation.extension".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.rest.operation.id".to_string(), "1".to_string());
    map.insert(
        "CapabilityStatement.rest.operation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CapabilityStatement.rest.operation.name".to_string(), "1".to_string());
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
    map.insert(
        "CapabilityStatement.rest.security.certificate".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.security.certificate.blob".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.security.certificate.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.security.certificate.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.security.certificate.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "CapabilityStatement.rest.security.certificate.type".to_string(),
        "1".to_string(),
    );
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
    map.insert("CarePlan.activity.detail.category".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.code".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.dailyAmount".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.definition".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.description".to_string(), "1".to_string());
    map.insert("CarePlan.activity.detail.extension".to_string(), "*".to_string());
    map.insert("CarePlan.activity.detail.goal".to_string(), "*".to_string());
    map.insert("CarePlan.activity.detail.id".to_string(), "1".to_string());
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
    map.insert("CarePlan.activity.detail.prohibited".to_string(), "1".to_string());
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
    map.insert("CarePlan.author".to_string(), "*".to_string());
    map.insert("CarePlan.basedOn".to_string(), "*".to_string());
    map.insert("CarePlan.careTeam".to_string(), "*".to_string());
    map.insert("CarePlan.category".to_string(), "*".to_string());
    map.insert("CarePlan.contained".to_string(), "*".to_string());
    map.insert("CarePlan.context".to_string(), "1".to_string());
    map.insert("CarePlan.definition".to_string(), "*".to_string());
    map.insert("CarePlan.description".to_string(), "1".to_string());
    map.insert("CarePlan.extension".to_string(), "*".to_string());
    map.insert("CarePlan.goal".to_string(), "*".to_string());
    map.insert("CarePlan.id".to_string(), "1".to_string());
    map.insert("CarePlan.identifier".to_string(), "*".to_string());
    map.insert("CarePlan.implicitRules".to_string(), "1".to_string());
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
    map.insert("CareTeam.context".to_string(), "1".to_string());
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
    map.insert("CareTeam.participant.role".to_string(), "1".to_string());
    map.insert("CareTeam.period".to_string(), "1".to_string());
    map.insert("CareTeam.reasonCode".to_string(), "*".to_string());
    map.insert("CareTeam.reasonReference".to_string(), "*".to_string());
    map.insert("CareTeam.status".to_string(), "1".to_string());
    map.insert("CareTeam.subject".to_string(), "1".to_string());
    map.insert("CareTeam.text".to_string(), "1".to_string());
    map.insert("ChargeItem.account".to_string(), "*".to_string());
    map.insert("ChargeItem.bodysite".to_string(), "*".to_string());
    map.insert("ChargeItem.code".to_string(), "1".to_string());
    map.insert("ChargeItem.contained".to_string(), "*".to_string());
    map.insert("ChargeItem.context".to_string(), "1".to_string());
    map.insert("ChargeItem.definition".to_string(), "*".to_string());
    map.insert("ChargeItem.enteredDate".to_string(), "1".to_string());
    map.insert("ChargeItem.enterer".to_string(), "1".to_string());
    map.insert("ChargeItem.extension".to_string(), "*".to_string());
    map.insert("ChargeItem.factorOverride".to_string(), "1".to_string());
    map.insert("ChargeItem.id".to_string(), "1".to_string());
    map.insert("ChargeItem.identifier".to_string(), "1".to_string());
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
    map.insert("ChargeItem.participant".to_string(), "*".to_string());
    map.insert("ChargeItem.participant.actor".to_string(), "1".to_string());
    map.insert("ChargeItem.participant.extension".to_string(), "*".to_string());
    map.insert("ChargeItem.participant.id".to_string(), "1".to_string());
    map.insert("ChargeItem.participant.modifierExtension".to_string(), "*".to_string());
    map.insert("ChargeItem.participant.role".to_string(), "1".to_string());
    map.insert("ChargeItem.performingOrganization".to_string(), "1".to_string());
    map.insert("ChargeItem.priceOverride".to_string(), "1".to_string());
    map.insert("ChargeItem.quantity".to_string(), "1".to_string());
    map.insert("ChargeItem.reason".to_string(), "*".to_string());
    map.insert("ChargeItem.requestingOrganization".to_string(), "1".to_string());
    map.insert("ChargeItem.service".to_string(), "*".to_string());
    map.insert("ChargeItem.status".to_string(), "1".to_string());
    map.insert("ChargeItem.subject".to_string(), "1".to_string());
    map.insert("ChargeItem.supportingInformation".to_string(), "*".to_string());
    map.insert("ChargeItem.text".to_string(), "1".to_string());
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
    map.insert("Claim.diagnosis.packageCode".to_string(), "1".to_string());
    map.insert("Claim.diagnosis.sequence".to_string(), "1".to_string());
    map.insert("Claim.diagnosis.type".to_string(), "*".to_string());
    map.insert("Claim.employmentImpacted".to_string(), "1".to_string());
    map.insert("Claim.enterer".to_string(), "1".to_string());
    map.insert("Claim.extension".to_string(), "*".to_string());
    map.insert("Claim.facility".to_string(), "1".to_string());
    map.insert("Claim.fundsReserve".to_string(), "1".to_string());
    map.insert("Claim.hospitalization".to_string(), "1".to_string());
    map.insert("Claim.id".to_string(), "1".to_string());
    map.insert("Claim.identifier".to_string(), "*".to_string());
    map.insert("Claim.implicitRules".to_string(), "1".to_string());
    map.insert("Claim.information".to_string(), "*".to_string());
    map.insert("Claim.information.category".to_string(), "1".to_string());
    map.insert("Claim.information.code".to_string(), "1".to_string());
    map.insert("Claim.information.extension".to_string(), "*".to_string());
    map.insert("Claim.information.id".to_string(), "1".to_string());
    map.insert("Claim.information.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.information.reason".to_string(), "1".to_string());
    map.insert("Claim.information.sequence".to_string(), "1".to_string());
    map.insert("Claim.information.timingDate".to_string(), "1".to_string());
    map.insert("Claim.information.timingPeriod".to_string(), "1".to_string());
    map.insert("Claim.information.valueAttachment".to_string(), "1".to_string());
    map.insert("Claim.information.valueQuantity".to_string(), "1".to_string());
    map.insert("Claim.information.valueReference".to_string(), "1".to_string());
    map.insert("Claim.information.valueString".to_string(), "1".to_string());
    map.insert("Claim.insurance".to_string(), "*".to_string());
    map.insert("Claim.insurance.businessArrangement".to_string(), "1".to_string());
    map.insert("Claim.insurance.claimResponse".to_string(), "1".to_string());
    map.insert("Claim.insurance.coverage".to_string(), "1".to_string());
    map.insert("Claim.insurance.extension".to_string(), "*".to_string());
    map.insert("Claim.insurance.focal".to_string(), "1".to_string());
    map.insert("Claim.insurance.id".to_string(), "1".to_string());
    map.insert("Claim.insurance.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.insurance.preAuthRef".to_string(), "*".to_string());
    map.insert("Claim.insurance.sequence".to_string(), "1".to_string());
    map.insert("Claim.insurer".to_string(), "1".to_string());
    map.insert("Claim.item".to_string(), "*".to_string());
    map.insert("Claim.item.bodySite".to_string(), "1".to_string());
    map.insert("Claim.item.careTeamLinkId".to_string(), "*".to_string());
    map.insert("Claim.item.category".to_string(), "1".to_string());
    map.insert("Claim.item.detail".to_string(), "*".to_string());
    map.insert("Claim.item.detail.category".to_string(), "1".to_string());
    map.insert("Claim.item.detail.extension".to_string(), "*".to_string());
    map.insert("Claim.item.detail.factor".to_string(), "1".to_string());
    map.insert("Claim.item.detail.id".to_string(), "1".to_string());
    map.insert("Claim.item.detail.modifier".to_string(), "*".to_string());
    map.insert("Claim.item.detail.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.item.detail.net".to_string(), "1".to_string());
    map.insert("Claim.item.detail.programCode".to_string(), "*".to_string());
    map.insert("Claim.item.detail.quantity".to_string(), "1".to_string());
    map.insert("Claim.item.detail.revenue".to_string(), "1".to_string());
    map.insert("Claim.item.detail.sequence".to_string(), "1".to_string());
    map.insert("Claim.item.detail.service".to_string(), "1".to_string());
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
    map.insert("Claim.item.detail.subDetail.programCode".to_string(), "*".to_string());
    map.insert("Claim.item.detail.subDetail.quantity".to_string(), "1".to_string());
    map.insert("Claim.item.detail.subDetail.revenue".to_string(), "1".to_string());
    map.insert("Claim.item.detail.subDetail.sequence".to_string(), "1".to_string());
    map.insert("Claim.item.detail.subDetail.service".to_string(), "1".to_string());
    map.insert("Claim.item.detail.subDetail.udi".to_string(), "*".to_string());
    map.insert("Claim.item.detail.subDetail.unitPrice".to_string(), "1".to_string());
    map.insert("Claim.item.detail.udi".to_string(), "*".to_string());
    map.insert("Claim.item.detail.unitPrice".to_string(), "1".to_string());
    map.insert("Claim.item.diagnosisLinkId".to_string(), "*".to_string());
    map.insert("Claim.item.encounter".to_string(), "*".to_string());
    map.insert("Claim.item.extension".to_string(), "*".to_string());
    map.insert("Claim.item.factor".to_string(), "1".to_string());
    map.insert("Claim.item.id".to_string(), "1".to_string());
    map.insert("Claim.item.informationLinkId".to_string(), "*".to_string());
    map.insert("Claim.item.locationAddress".to_string(), "1".to_string());
    map.insert("Claim.item.locationCodeableConcept".to_string(), "1".to_string());
    map.insert("Claim.item.locationReference".to_string(), "1".to_string());
    map.insert("Claim.item.modifier".to_string(), "*".to_string());
    map.insert("Claim.item.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.item.net".to_string(), "1".to_string());
    map.insert("Claim.item.procedureLinkId".to_string(), "*".to_string());
    map.insert("Claim.item.programCode".to_string(), "*".to_string());
    map.insert("Claim.item.quantity".to_string(), "1".to_string());
    map.insert("Claim.item.revenue".to_string(), "1".to_string());
    map.insert("Claim.item.sequence".to_string(), "1".to_string());
    map.insert("Claim.item.service".to_string(), "1".to_string());
    map.insert("Claim.item.servicedDate".to_string(), "1".to_string());
    map.insert("Claim.item.servicedPeriod".to_string(), "1".to_string());
    map.insert("Claim.item.subSite".to_string(), "*".to_string());
    map.insert("Claim.item.udi".to_string(), "*".to_string());
    map.insert("Claim.item.unitPrice".to_string(), "1".to_string());
    map.insert("Claim.language".to_string(), "1".to_string());
    map.insert("Claim.meta".to_string(), "1".to_string());
    map.insert("Claim.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.organization".to_string(), "1".to_string());
    map.insert("Claim.originalPrescription".to_string(), "1".to_string());
    map.insert("Claim.patient".to_string(), "1".to_string());
    map.insert("Claim.payee".to_string(), "1".to_string());
    map.insert("Claim.payee.extension".to_string(), "*".to_string());
    map.insert("Claim.payee.id".to_string(), "1".to_string());
    map.insert("Claim.payee.modifierExtension".to_string(), "*".to_string());
    map.insert("Claim.payee.party".to_string(), "1".to_string());
    map.insert("Claim.payee.resourceType".to_string(), "1".to_string());
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
    map.insert("Claim.subType".to_string(), "*".to_string());
    map.insert("Claim.text".to_string(), "1".to_string());
    map.insert("Claim.total".to_string(), "1".to_string());
    map.insert("Claim.type".to_string(), "1".to_string());
    map.insert("Claim.use".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.adjudication".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.category".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.detail".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.detail.adjudication".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.detail.category".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.detail.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.detail.fee".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.detail.id".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.detail.modifier".to_string(), "*".to_string());
    map.insert(
        "ClaimResponse.addItem.detail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ClaimResponse.addItem.detail.noteNumber".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.detail.revenue".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.detail.service".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.fee".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.id".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.modifier".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.modifierExtension".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.noteNumber".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.revenue".to_string(), "1".to_string());
    map.insert("ClaimResponse.addItem.sequenceLinkId".to_string(), "*".to_string());
    map.insert("ClaimResponse.addItem.service".to_string(), "1".to_string());
    map.insert("ClaimResponse.communicationRequest".to_string(), "*".to_string());
    map.insert("ClaimResponse.contained".to_string(), "*".to_string());
    map.insert("ClaimResponse.created".to_string(), "1".to_string());
    map.insert("ClaimResponse.disposition".to_string(), "1".to_string());
    map.insert("ClaimResponse.error".to_string(), "*".to_string());
    map.insert("ClaimResponse.error.code".to_string(), "1".to_string());
    map.insert("ClaimResponse.error.detailSequenceLinkId".to_string(), "1".to_string());
    map.insert("ClaimResponse.error.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.error.id".to_string(), "1".to_string());
    map.insert("ClaimResponse.error.modifierExtension".to_string(), "*".to_string());
    map.insert("ClaimResponse.error.sequenceLinkId".to_string(), "1".to_string());
    map.insert(
        "ClaimResponse.error.subdetailSequenceLinkId".to_string(),
        "1".to_string(),
    );
    map.insert("ClaimResponse.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.form".to_string(), "1".to_string());
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
    map.insert("ClaimResponse.insurance.preAuthRef".to_string(), "*".to_string());
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
    map.insert("ClaimResponse.item.detail.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.item.detail.id".to_string(), "1".to_string());
    map.insert(
        "ClaimResponse.item.detail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ClaimResponse.item.detail.noteNumber".to_string(), "*".to_string());
    map.insert("ClaimResponse.item.detail.sequenceLinkId".to_string(), "1".to_string());
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
        "ClaimResponse.item.detail.subDetail.sequenceLinkId".to_string(),
        "1".to_string(),
    );
    map.insert("ClaimResponse.item.extension".to_string(), "*".to_string());
    map.insert("ClaimResponse.item.id".to_string(), "1".to_string());
    map.insert("ClaimResponse.item.modifierExtension".to_string(), "*".to_string());
    map.insert("ClaimResponse.item.noteNumber".to_string(), "*".to_string());
    map.insert("ClaimResponse.item.sequenceLinkId".to_string(), "1".to_string());
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
    map.insert("ClaimResponse.requestOrganization".to_string(), "1".to_string());
    map.insert("ClaimResponse.requestProvider".to_string(), "1".to_string());
    map.insert("ClaimResponse.reserved".to_string(), "1".to_string());
    map.insert("ClaimResponse.status".to_string(), "1".to_string());
    map.insert("ClaimResponse.text".to_string(), "1".to_string());
    map.insert("ClaimResponse.totalBenefit".to_string(), "1".to_string());
    map.insert("ClaimResponse.totalCost".to_string(), "1".to_string());
    map.insert("ClaimResponse.unallocDeductable".to_string(), "1".to_string());
    map.insert("ClinicalImpression.action".to_string(), "*".to_string());
    map.insert("ClinicalImpression.assessor".to_string(), "1".to_string());
    map.insert("ClinicalImpression.code".to_string(), "1".to_string());
    map.insert("ClinicalImpression.contained".to_string(), "*".to_string());
    map.insert("ClinicalImpression.context".to_string(), "1".to_string());
    map.insert("ClinicalImpression.date".to_string(), "1".to_string());
    map.insert("ClinicalImpression.description".to_string(), "1".to_string());
    map.insert("ClinicalImpression.effectiveDateTime".to_string(), "1".to_string());
    map.insert("ClinicalImpression.effectivePeriod".to_string(), "1".to_string());
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
    map.insert("ClinicalImpression.subject".to_string(), "1".to_string());
    map.insert("ClinicalImpression.summary".to_string(), "1".to_string());
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
    map.insert("CodeSystem.identifier".to_string(), "1".to_string());
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
    map.insert("Communication.basedOn".to_string(), "*".to_string());
    map.insert("Communication.category".to_string(), "*".to_string());
    map.insert("Communication.contained".to_string(), "*".to_string());
    map.insert("Communication.context".to_string(), "1".to_string());
    map.insert("Communication.definition".to_string(), "*".to_string());
    map.insert("Communication.extension".to_string(), "*".to_string());
    map.insert("Communication.id".to_string(), "1".to_string());
    map.insert("Communication.identifier".to_string(), "*".to_string());
    map.insert("Communication.implicitRules".to_string(), "1".to_string());
    map.insert("Communication.language".to_string(), "1".to_string());
    map.insert("Communication.medium".to_string(), "*".to_string());
    map.insert("Communication.meta".to_string(), "1".to_string());
    map.insert("Communication.modifierExtension".to_string(), "*".to_string());
    map.insert("Communication.notDone".to_string(), "1".to_string());
    map.insert("Communication.notDoneReason".to_string(), "1".to_string());
    map.insert("Communication.note".to_string(), "*".to_string());
    map.insert("Communication.partOf".to_string(), "*".to_string());
    map.insert("Communication.payload".to_string(), "*".to_string());
    map.insert("Communication.payload.contentAttachment".to_string(), "1".to_string());
    map.insert("Communication.payload.contentReference".to_string(), "1".to_string());
    map.insert("Communication.payload.contentString".to_string(), "1".to_string());
    map.insert("Communication.payload.extension".to_string(), "*".to_string());
    map.insert("Communication.payload.id".to_string(), "1".to_string());
    map.insert("Communication.payload.modifierExtension".to_string(), "*".to_string());
    map.insert("Communication.reasonCode".to_string(), "*".to_string());
    map.insert("Communication.reasonReference".to_string(), "*".to_string());
    map.insert("Communication.received".to_string(), "1".to_string());
    map.insert("Communication.recipient".to_string(), "*".to_string());
    map.insert("Communication.sender".to_string(), "1".to_string());
    map.insert("Communication.sent".to_string(), "1".to_string());
    map.insert("Communication.status".to_string(), "1".to_string());
    map.insert("Communication.subject".to_string(), "1".to_string());
    map.insert("Communication.text".to_string(), "1".to_string());
    map.insert("Communication.topic".to_string(), "*".to_string());
    map.insert("CommunicationRequest.authoredOn".to_string(), "1".to_string());
    map.insert("CommunicationRequest.basedOn".to_string(), "*".to_string());
    map.insert("CommunicationRequest.category".to_string(), "*".to_string());
    map.insert("CommunicationRequest.contained".to_string(), "*".to_string());
    map.insert("CommunicationRequest.context".to_string(), "1".to_string());
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
    map.insert("CommunicationRequest.requester.agent".to_string(), "1".to_string());
    map.insert("CommunicationRequest.requester.extension".to_string(), "*".to_string());
    map.insert("CommunicationRequest.requester.id".to_string(), "1".to_string());
    map.insert(
        "CommunicationRequest.requester.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("CommunicationRequest.requester.onBehalfOf".to_string(), "1".to_string());
    map.insert("CommunicationRequest.sender".to_string(), "1".to_string());
    map.insert("CommunicationRequest.status".to_string(), "1".to_string());
    map.insert("CommunicationRequest.subject".to_string(), "1".to_string());
    map.insert("CommunicationRequest.text".to_string(), "1".to_string());
    map.insert("CommunicationRequest.topic".to_string(), "*".to_string());
    map.insert("CompartmentDefinition.code".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.contact".to_string(), "*".to_string());
    map.insert("CompartmentDefinition.contained".to_string(), "*".to_string());
    map.insert("CompartmentDefinition.date".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.description".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.experimental".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.extension".to_string(), "*".to_string());
    map.insert("CompartmentDefinition.id".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.jurisdiction".to_string(), "*".to_string());
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
    map.insert("CompartmentDefinition.title".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.url".to_string(), "1".to_string());
    map.insert("CompartmentDefinition.useContext".to_string(), "*".to_string());
    map.insert("Composition.attester".to_string(), "*".to_string());
    map.insert("Composition.attester.extension".to_string(), "*".to_string());
    map.insert("Composition.attester.id".to_string(), "1".to_string());
    map.insert("Composition.attester.mode".to_string(), "*".to_string());
    map.insert("Composition.attester.modifierExtension".to_string(), "*".to_string());
    map.insert("Composition.attester.party".to_string(), "1".to_string());
    map.insert("Composition.attester.time".to_string(), "1".to_string());
    map.insert("Composition.author".to_string(), "*".to_string());
    map.insert("Composition.class".to_string(), "1".to_string());
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
    map.insert("Composition.section.code".to_string(), "1".to_string());
    map.insert("Composition.section.emptyReason".to_string(), "1".to_string());
    map.insert("Composition.section.entry".to_string(), "*".to_string());
    map.insert("Composition.section.extension".to_string(), "*".to_string());
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
        "ConceptMap.group.element.target.dependsOn.code".to_string(),
        "1".to_string(),
    );
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
    map.insert("ConceptMap.sourceReference".to_string(), "1".to_string());
    map.insert("ConceptMap.sourceUri".to_string(), "1".to_string());
    map.insert("ConceptMap.status".to_string(), "1".to_string());
    map.insert("ConceptMap.targetReference".to_string(), "1".to_string());
    map.insert("ConceptMap.targetUri".to_string(), "1".to_string());
    map.insert("ConceptMap.text".to_string(), "1".to_string());
    map.insert("ConceptMap.title".to_string(), "1".to_string());
    map.insert("ConceptMap.url".to_string(), "1".to_string());
    map.insert("ConceptMap.useContext".to_string(), "*".to_string());
    map.insert("ConceptMap.version".to_string(), "1".to_string());
    map.insert("Condition.abatementAge".to_string(), "1".to_string());
    map.insert("Condition.abatementBoolean".to_string(), "1".to_string());
    map.insert("Condition.abatementDateTime".to_string(), "1".to_string());
    map.insert("Condition.abatementPeriod".to_string(), "1".to_string());
    map.insert("Condition.abatementRange".to_string(), "1".to_string());
    map.insert("Condition.abatementString".to_string(), "1".to_string());
    map.insert("Condition.assertedDate".to_string(), "1".to_string());
    map.insert("Condition.asserter".to_string(), "1".to_string());
    map.insert("Condition.bodySite".to_string(), "*".to_string());
    map.insert("Condition.category".to_string(), "*".to_string());
    map.insert("Condition.clinicalStatus".to_string(), "1".to_string());
    map.insert("Condition.code".to_string(), "1".to_string());
    map.insert("Condition.contained".to_string(), "*".to_string());
    map.insert("Condition.context".to_string(), "1".to_string());
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
    map.insert("Condition.severity".to_string(), "1".to_string());
    map.insert("Condition.stage".to_string(), "1".to_string());
    map.insert("Condition.stage.assessment".to_string(), "*".to_string());
    map.insert("Condition.stage.extension".to_string(), "*".to_string());
    map.insert("Condition.stage.id".to_string(), "1".to_string());
    map.insert("Condition.stage.modifierExtension".to_string(), "*".to_string());
    map.insert("Condition.stage.summary".to_string(), "1".to_string());
    map.insert("Condition.subject".to_string(), "1".to_string());
    map.insert("Condition.text".to_string(), "1".to_string());
    map.insert("Condition.verificationStatus".to_string(), "1".to_string());
    map.insert("Consent.action".to_string(), "*".to_string());
    map.insert("Consent.actor".to_string(), "*".to_string());
    map.insert("Consent.actor.extension".to_string(), "*".to_string());
    map.insert("Consent.actor.id".to_string(), "1".to_string());
    map.insert("Consent.actor.modifierExtension".to_string(), "*".to_string());
    map.insert("Consent.actor.reference".to_string(), "1".to_string());
    map.insert("Consent.actor.role".to_string(), "1".to_string());
    map.insert("Consent.category".to_string(), "*".to_string());
    map.insert("Consent.consentingParty".to_string(), "*".to_string());
    map.insert("Consent.contained".to_string(), "*".to_string());
    map.insert("Consent.data".to_string(), "*".to_string());
    map.insert("Consent.data.extension".to_string(), "*".to_string());
    map.insert("Consent.data.id".to_string(), "1".to_string());
    map.insert("Consent.data.meaning".to_string(), "1".to_string());
    map.insert("Consent.data.modifierExtension".to_string(), "*".to_string());
    map.insert("Consent.data.reference".to_string(), "1".to_string());
    map.insert("Consent.dataPeriod".to_string(), "1".to_string());
    map.insert("Consent.dateTime".to_string(), "1".to_string());
    map.insert("Consent.except".to_string(), "*".to_string());
    map.insert("Consent.except.action".to_string(), "*".to_string());
    map.insert("Consent.except.actor".to_string(), "*".to_string());
    map.insert("Consent.except.actor.extension".to_string(), "*".to_string());
    map.insert("Consent.except.actor.id".to_string(), "1".to_string());
    map.insert("Consent.except.actor.modifierExtension".to_string(), "*".to_string());
    map.insert("Consent.except.actor.reference".to_string(), "1".to_string());
    map.insert("Consent.except.actor.role".to_string(), "1".to_string());
    map.insert("Consent.except.class".to_string(), "*".to_string());
    map.insert("Consent.except.code".to_string(), "*".to_string());
    map.insert("Consent.except.data".to_string(), "*".to_string());
    map.insert("Consent.except.data.extension".to_string(), "*".to_string());
    map.insert("Consent.except.data.id".to_string(), "1".to_string());
    map.insert("Consent.except.data.meaning".to_string(), "1".to_string());
    map.insert("Consent.except.data.modifierExtension".to_string(), "*".to_string());
    map.insert("Consent.except.data.reference".to_string(), "1".to_string());
    map.insert("Consent.except.dataPeriod".to_string(), "1".to_string());
    map.insert("Consent.except.extension".to_string(), "*".to_string());
    map.insert("Consent.except.id".to_string(), "1".to_string());
    map.insert("Consent.except.modifierExtension".to_string(), "*".to_string());
    map.insert("Consent.except.period".to_string(), "1".to_string());
    map.insert("Consent.except.purpose".to_string(), "*".to_string());
    map.insert("Consent.except.securityLabel".to_string(), "*".to_string());
    map.insert("Consent.except.type".to_string(), "1".to_string());
    map.insert("Consent.extension".to_string(), "*".to_string());
    map.insert("Consent.id".to_string(), "1".to_string());
    map.insert("Consent.identifier".to_string(), "1".to_string());
    map.insert("Consent.implicitRules".to_string(), "1".to_string());
    map.insert("Consent.language".to_string(), "1".to_string());
    map.insert("Consent.meta".to_string(), "1".to_string());
    map.insert("Consent.modifierExtension".to_string(), "*".to_string());
    map.insert("Consent.organization".to_string(), "*".to_string());
    map.insert("Consent.patient".to_string(), "1".to_string());
    map.insert("Consent.period".to_string(), "1".to_string());
    map.insert("Consent.policy".to_string(), "*".to_string());
    map.insert("Consent.policy.authority".to_string(), "1".to_string());
    map.insert("Consent.policy.extension".to_string(), "*".to_string());
    map.insert("Consent.policy.id".to_string(), "1".to_string());
    map.insert("Consent.policy.modifierExtension".to_string(), "*".to_string());
    map.insert("Consent.policy.uri".to_string(), "1".to_string());
    map.insert("Consent.policyRule".to_string(), "1".to_string());
    map.insert("Consent.purpose".to_string(), "*".to_string());
    map.insert("Consent.securityLabel".to_string(), "*".to_string());
    map.insert("Consent.sourceAttachment".to_string(), "1".to_string());
    map.insert("Consent.sourceIdentifier".to_string(), "1".to_string());
    map.insert("Consent.sourceReference".to_string(), "1".to_string());
    map.insert("Consent.status".to_string(), "1".to_string());
    map.insert("Consent.text".to_string(), "1".to_string());
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
    map.insert("Contract.action".to_string(), "*".to_string());
    map.insert("Contract.actionReason".to_string(), "*".to_string());
    map.insert("Contract.agent".to_string(), "*".to_string());
    map.insert("Contract.agent.actor".to_string(), "1".to_string());
    map.insert("Contract.agent.extension".to_string(), "*".to_string());
    map.insert("Contract.agent.id".to_string(), "1".to_string());
    map.insert("Contract.agent.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.agent.role".to_string(), "*".to_string());
    map.insert("Contract.applies".to_string(), "1".to_string());
    map.insert("Contract.authority".to_string(), "*".to_string());
    map.insert("Contract.bindingAttachment".to_string(), "1".to_string());
    map.insert("Contract.bindingReference".to_string(), "1".to_string());
    map.insert("Contract.contained".to_string(), "*".to_string());
    map.insert("Contract.contentDerivative".to_string(), "1".to_string());
    map.insert("Contract.decisionType".to_string(), "1".to_string());
    map.insert("Contract.domain".to_string(), "*".to_string());
    map.insert("Contract.extension".to_string(), "*".to_string());
    map.insert("Contract.friendly".to_string(), "*".to_string());
    map.insert("Contract.friendly.contentAttachment".to_string(), "1".to_string());
    map.insert("Contract.friendly.contentReference".to_string(), "1".to_string());
    map.insert("Contract.friendly.extension".to_string(), "*".to_string());
    map.insert("Contract.friendly.id".to_string(), "1".to_string());
    map.insert("Contract.friendly.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.id".to_string(), "1".to_string());
    map.insert("Contract.identifier".to_string(), "1".to_string());
    map.insert("Contract.implicitRules".to_string(), "1".to_string());
    map.insert("Contract.issued".to_string(), "1".to_string());
    map.insert("Contract.language".to_string(), "1".to_string());
    map.insert("Contract.legal".to_string(), "*".to_string());
    map.insert("Contract.legal.contentAttachment".to_string(), "1".to_string());
    map.insert("Contract.legal.contentReference".to_string(), "1".to_string());
    map.insert("Contract.legal.extension".to_string(), "*".to_string());
    map.insert("Contract.legal.id".to_string(), "1".to_string());
    map.insert("Contract.legal.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.meta".to_string(), "1".to_string());
    map.insert("Contract.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.rule".to_string(), "*".to_string());
    map.insert("Contract.rule.contentAttachment".to_string(), "1".to_string());
    map.insert("Contract.rule.contentReference".to_string(), "1".to_string());
    map.insert("Contract.rule.extension".to_string(), "*".to_string());
    map.insert("Contract.rule.id".to_string(), "1".to_string());
    map.insert("Contract.rule.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.securityLabel".to_string(), "*".to_string());
    map.insert("Contract.signer".to_string(), "*".to_string());
    map.insert("Contract.signer.extension".to_string(), "*".to_string());
    map.insert("Contract.signer.id".to_string(), "1".to_string());
    map.insert("Contract.signer.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.signer.party".to_string(), "1".to_string());
    map.insert("Contract.signer.signature".to_string(), "*".to_string());
    map.insert("Contract.signer.type".to_string(), "1".to_string());
    map.insert("Contract.status".to_string(), "1".to_string());
    map.insert("Contract.subType".to_string(), "*".to_string());
    map.insert("Contract.subject".to_string(), "*".to_string());
    map.insert("Contract.term".to_string(), "*".to_string());
    map.insert("Contract.term.action".to_string(), "*".to_string());
    map.insert("Contract.term.actionReason".to_string(), "*".to_string());
    map.insert("Contract.term.agent".to_string(), "*".to_string());
    map.insert("Contract.term.agent.actor".to_string(), "1".to_string());
    map.insert("Contract.term.agent.extension".to_string(), "*".to_string());
    map.insert("Contract.term.agent.id".to_string(), "1".to_string());
    map.insert("Contract.term.agent.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.term.agent.role".to_string(), "*".to_string());
    map.insert("Contract.term.applies".to_string(), "1".to_string());
    map.insert("Contract.term.extension".to_string(), "*".to_string());
    map.insert("Contract.term.group".to_string(), "*".to_string());
    map.insert("Contract.term.id".to_string(), "1".to_string());
    map.insert("Contract.term.identifier".to_string(), "1".to_string());
    map.insert("Contract.term.issued".to_string(), "1".to_string());
    map.insert("Contract.term.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.term.securityLabel".to_string(), "*".to_string());
    map.insert("Contract.term.subType".to_string(), "1".to_string());
    map.insert("Contract.term.text".to_string(), "1".to_string());
    map.insert("Contract.term.topic".to_string(), "*".to_string());
    map.insert("Contract.term.type".to_string(), "1".to_string());
    map.insert("Contract.term.valuedItem".to_string(), "*".to_string());
    map.insert("Contract.term.valuedItem.effectiveTime".to_string(), "1".to_string());
    map.insert(
        "Contract.term.valuedItem.entityCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("Contract.term.valuedItem.entityReference".to_string(), "1".to_string());
    map.insert("Contract.term.valuedItem.extension".to_string(), "*".to_string());
    map.insert("Contract.term.valuedItem.factor".to_string(), "1".to_string());
    map.insert("Contract.term.valuedItem.id".to_string(), "1".to_string());
    map.insert("Contract.term.valuedItem.identifier".to_string(), "1".to_string());
    map.insert(
        "Contract.term.valuedItem.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Contract.term.valuedItem.net".to_string(), "1".to_string());
    map.insert("Contract.term.valuedItem.points".to_string(), "1".to_string());
    map.insert("Contract.term.valuedItem.quantity".to_string(), "1".to_string());
    map.insert("Contract.term.valuedItem.unitPrice".to_string(), "1".to_string());
    map.insert("Contract.text".to_string(), "1".to_string());
    map.insert("Contract.topic".to_string(), "*".to_string());
    map.insert("Contract.type".to_string(), "1".to_string());
    map.insert("Contract.valuedItem".to_string(), "*".to_string());
    map.insert("Contract.valuedItem.effectiveTime".to_string(), "1".to_string());
    map.insert("Contract.valuedItem.entityCodeableConcept".to_string(), "1".to_string());
    map.insert("Contract.valuedItem.entityReference".to_string(), "1".to_string());
    map.insert("Contract.valuedItem.extension".to_string(), "*".to_string());
    map.insert("Contract.valuedItem.factor".to_string(), "1".to_string());
    map.insert("Contract.valuedItem.id".to_string(), "1".to_string());
    map.insert("Contract.valuedItem.identifier".to_string(), "1".to_string());
    map.insert("Contract.valuedItem.modifierExtension".to_string(), "*".to_string());
    map.insert("Contract.valuedItem.net".to_string(), "1".to_string());
    map.insert("Contract.valuedItem.points".to_string(), "1".to_string());
    map.insert("Contract.valuedItem.quantity".to_string(), "1".to_string());
    map.insert("Contract.valuedItem.unitPrice".to_string(), "1".to_string());
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
    map.insert("Coverage.contained".to_string(), "*".to_string());
    map.insert("Coverage.contract".to_string(), "*".to_string());
    map.insert("Coverage.dependent".to_string(), "1".to_string());
    map.insert("Coverage.extension".to_string(), "*".to_string());
    map.insert("Coverage.grouping".to_string(), "1".to_string());
    map.insert("Coverage.grouping.class".to_string(), "1".to_string());
    map.insert("Coverage.grouping.classDisplay".to_string(), "1".to_string());
    map.insert("Coverage.grouping.extension".to_string(), "*".to_string());
    map.insert("Coverage.grouping.group".to_string(), "1".to_string());
    map.insert("Coverage.grouping.groupDisplay".to_string(), "1".to_string());
    map.insert("Coverage.grouping.id".to_string(), "1".to_string());
    map.insert("Coverage.grouping.modifierExtension".to_string(), "*".to_string());
    map.insert("Coverage.grouping.plan".to_string(), "1".to_string());
    map.insert("Coverage.grouping.planDisplay".to_string(), "1".to_string());
    map.insert("Coverage.grouping.subClass".to_string(), "1".to_string());
    map.insert("Coverage.grouping.subClassDisplay".to_string(), "1".to_string());
    map.insert("Coverage.grouping.subGroup".to_string(), "1".to_string());
    map.insert("Coverage.grouping.subGroupDisplay".to_string(), "1".to_string());
    map.insert("Coverage.grouping.subPlan".to_string(), "1".to_string());
    map.insert("Coverage.grouping.subPlanDisplay".to_string(), "1".to_string());
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
    map.insert("Coverage.sequence".to_string(), "1".to_string());
    map.insert("Coverage.status".to_string(), "1".to_string());
    map.insert("Coverage.subscriber".to_string(), "1".to_string());
    map.insert("Coverage.subscriberId".to_string(), "1".to_string());
    map.insert("Coverage.text".to_string(), "1".to_string());
    map.insert("Coverage.type".to_string(), "1".to_string());
    map.insert("DataElement.contact".to_string(), "*".to_string());
    map.insert("DataElement.contained".to_string(), "*".to_string());
    map.insert("DataElement.copyright".to_string(), "1".to_string());
    map.insert("DataElement.date".to_string(), "1".to_string());
    map.insert("DataElement.element".to_string(), "*".to_string());
    map.insert("DataElement.experimental".to_string(), "1".to_string());
    map.insert("DataElement.extension".to_string(), "*".to_string());
    map.insert("DataElement.id".to_string(), "1".to_string());
    map.insert("DataElement.identifier".to_string(), "*".to_string());
    map.insert("DataElement.implicitRules".to_string(), "1".to_string());
    map.insert("DataElement.jurisdiction".to_string(), "*".to_string());
    map.insert("DataElement.language".to_string(), "1".to_string());
    map.insert("DataElement.mapping".to_string(), "*".to_string());
    map.insert("DataElement.mapping.comment".to_string(), "1".to_string());
    map.insert("DataElement.mapping.extension".to_string(), "*".to_string());
    map.insert("DataElement.mapping.id".to_string(), "1".to_string());
    map.insert("DataElement.mapping.identity".to_string(), "1".to_string());
    map.insert("DataElement.mapping.modifierExtension".to_string(), "*".to_string());
    map.insert("DataElement.mapping.name".to_string(), "1".to_string());
    map.insert("DataElement.mapping.uri".to_string(), "1".to_string());
    map.insert("DataElement.meta".to_string(), "1".to_string());
    map.insert("DataElement.modifierExtension".to_string(), "*".to_string());
    map.insert("DataElement.name".to_string(), "1".to_string());
    map.insert("DataElement.publisher".to_string(), "1".to_string());
    map.insert("DataElement.status".to_string(), "1".to_string());
    map.insert("DataElement.stringency".to_string(), "1".to_string());
    map.insert("DataElement.text".to_string(), "1".to_string());
    map.insert("DataElement.title".to_string(), "1".to_string());
    map.insert("DataElement.url".to_string(), "1".to_string());
    map.insert("DataElement.useContext".to_string(), "*".to_string());
    map.insert("DataElement.version".to_string(), "1".to_string());
    map.insert("DataRequirement.codeFilter".to_string(), "*".to_string());
    map.insert("DataRequirement.codeFilter.extension".to_string(), "*".to_string());
    map.insert("DataRequirement.codeFilter.id".to_string(), "1".to_string());
    map.insert("DataRequirement.codeFilter.path".to_string(), "1".to_string());
    map.insert("DataRequirement.codeFilter.valueCode".to_string(), "*".to_string());
    map.insert(
        "DataRequirement.codeFilter.valueCodeableConcept".to_string(),
        "*".to_string(),
    );
    map.insert("DataRequirement.codeFilter.valueCoding".to_string(), "*".to_string());
    map.insert(
        "DataRequirement.codeFilter.valueSetReference".to_string(),
        "1".to_string(),
    );
    map.insert("DataRequirement.codeFilter.valueSetString".to_string(), "1".to_string());
    map.insert("DataRequirement.dateFilter".to_string(), "*".to_string());
    map.insert("DataRequirement.dateFilter.extension".to_string(), "*".to_string());
    map.insert("DataRequirement.dateFilter.id".to_string(), "1".to_string());
    map.insert("DataRequirement.dateFilter.path".to_string(), "1".to_string());
    map.insert("DataRequirement.dateFilter.valueDateTime".to_string(), "1".to_string());
    map.insert("DataRequirement.dateFilter.valueDuration".to_string(), "1".to_string());
    map.insert("DataRequirement.dateFilter.valuePeriod".to_string(), "1".to_string());
    map.insert("DataRequirement.extension".to_string(), "*".to_string());
    map.insert("DataRequirement.id".to_string(), "1".to_string());
    map.insert("DataRequirement.mustSupport".to_string(), "*".to_string());
    map.insert("DataRequirement.profile".to_string(), "*".to_string());
    map.insert("DataRequirement.type".to_string(), "1".to_string());
    map.insert("DetectedIssue.author".to_string(), "1".to_string());
    map.insert("DetectedIssue.category".to_string(), "1".to_string());
    map.insert("DetectedIssue.contained".to_string(), "*".to_string());
    map.insert("DetectedIssue.date".to_string(), "1".to_string());
    map.insert("DetectedIssue.detail".to_string(), "1".to_string());
    map.insert("DetectedIssue.extension".to_string(), "*".to_string());
    map.insert("DetectedIssue.id".to_string(), "1".to_string());
    map.insert("DetectedIssue.identifier".to_string(), "1".to_string());
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
    map.insert("Device.model".to_string(), "1".to_string());
    map.insert("Device.modifierExtension".to_string(), "*".to_string());
    map.insert("Device.note".to_string(), "*".to_string());
    map.insert("Device.owner".to_string(), "1".to_string());
    map.insert("Device.patient".to_string(), "1".to_string());
    map.insert("Device.safety".to_string(), "*".to_string());
    map.insert("Device.status".to_string(), "1".to_string());
    map.insert("Device.text".to_string(), "1".to_string());
    map.insert("Device.type".to_string(), "1".to_string());
    map.insert("Device.udi".to_string(), "1".to_string());
    map.insert("Device.udi.carrierAIDC".to_string(), "1".to_string());
    map.insert("Device.udi.carrierHRF".to_string(), "1".to_string());
    map.insert("Device.udi.deviceIdentifier".to_string(), "1".to_string());
    map.insert("Device.udi.entryType".to_string(), "1".to_string());
    map.insert("Device.udi.extension".to_string(), "*".to_string());
    map.insert("Device.udi.id".to_string(), "1".to_string());
    map.insert("Device.udi.issuer".to_string(), "1".to_string());
    map.insert("Device.udi.jurisdiction".to_string(), "1".to_string());
    map.insert("Device.udi.modifierExtension".to_string(), "*".to_string());
    map.insert("Device.udi.name".to_string(), "1".to_string());
    map.insert("Device.url".to_string(), "1".to_string());
    map.insert("Device.version".to_string(), "1".to_string());
    map.insert("DeviceComponent.contained".to_string(), "*".to_string());
    map.insert("DeviceComponent.extension".to_string(), "*".to_string());
    map.insert("DeviceComponent.id".to_string(), "1".to_string());
    map.insert("DeviceComponent.identifier".to_string(), "1".to_string());
    map.insert("DeviceComponent.implicitRules".to_string(), "1".to_string());
    map.insert("DeviceComponent.language".to_string(), "1".to_string());
    map.insert("DeviceComponent.languageCode".to_string(), "1".to_string());
    map.insert("DeviceComponent.lastSystemChange".to_string(), "1".to_string());
    map.insert("DeviceComponent.measurementPrinciple".to_string(), "1".to_string());
    map.insert("DeviceComponent.meta".to_string(), "1".to_string());
    map.insert("DeviceComponent.modifierExtension".to_string(), "*".to_string());
    map.insert("DeviceComponent.operationalStatus".to_string(), "*".to_string());
    map.insert("DeviceComponent.parameterGroup".to_string(), "1".to_string());
    map.insert("DeviceComponent.parent".to_string(), "1".to_string());
    map.insert("DeviceComponent.productionSpecification".to_string(), "*".to_string());
    map.insert(
        "DeviceComponent.productionSpecification.componentId".to_string(),
        "1".to_string(),
    );
    map.insert(
        "DeviceComponent.productionSpecification.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "DeviceComponent.productionSpecification.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "DeviceComponent.productionSpecification.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "DeviceComponent.productionSpecification.productionSpec".to_string(),
        "1".to_string(),
    );
    map.insert(
        "DeviceComponent.productionSpecification.specType".to_string(),
        "1".to_string(),
    );
    map.insert("DeviceComponent.source".to_string(), "1".to_string());
    map.insert("DeviceComponent.text".to_string(), "1".to_string());
    map.insert("DeviceComponent.type".to_string(), "1".to_string());
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
    map.insert("DeviceMetric.identifier".to_string(), "1".to_string());
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
    map.insert("DeviceRequest.context".to_string(), "1".to_string());
    map.insert("DeviceRequest.definition".to_string(), "*".to_string());
    map.insert("DeviceRequest.extension".to_string(), "*".to_string());
    map.insert("DeviceRequest.groupIdentifier".to_string(), "1".to_string());
    map.insert("DeviceRequest.id".to_string(), "1".to_string());
    map.insert("DeviceRequest.identifier".to_string(), "*".to_string());
    map.insert("DeviceRequest.implicitRules".to_string(), "1".to_string());
    map.insert("DeviceRequest.intent".to_string(), "1".to_string());
    map.insert("DeviceRequest.language".to_string(), "1".to_string());
    map.insert("DeviceRequest.meta".to_string(), "1".to_string());
    map.insert("DeviceRequest.modifierExtension".to_string(), "*".to_string());
    map.insert("DeviceRequest.note".to_string(), "*".to_string());
    map.insert("DeviceRequest.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("DeviceRequest.occurrencePeriod".to_string(), "1".to_string());
    map.insert("DeviceRequest.occurrenceTiming".to_string(), "1".to_string());
    map.insert("DeviceRequest.performer".to_string(), "1".to_string());
    map.insert("DeviceRequest.performerType".to_string(), "1".to_string());
    map.insert("DeviceRequest.priorRequest".to_string(), "*".to_string());
    map.insert("DeviceRequest.priority".to_string(), "1".to_string());
    map.insert("DeviceRequest.reasonCode".to_string(), "*".to_string());
    map.insert("DeviceRequest.reasonReference".to_string(), "*".to_string());
    map.insert("DeviceRequest.relevantHistory".to_string(), "*".to_string());
    map.insert("DeviceRequest.requester".to_string(), "1".to_string());
    map.insert("DeviceRequest.requester.agent".to_string(), "1".to_string());
    map.insert("DeviceRequest.requester.extension".to_string(), "*".to_string());
    map.insert("DeviceRequest.requester.id".to_string(), "1".to_string());
    map.insert("DeviceRequest.requester.modifierExtension".to_string(), "*".to_string());
    map.insert("DeviceRequest.requester.onBehalfOf".to_string(), "1".to_string());
    map.insert("DeviceRequest.status".to_string(), "1".to_string());
    map.insert("DeviceRequest.subject".to_string(), "1".to_string());
    map.insert("DeviceRequest.supportingInfo".to_string(), "*".to_string());
    map.insert("DeviceRequest.text".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.bodySite".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.contained".to_string(), "*".to_string());
    map.insert("DeviceUseStatement.device".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.extension".to_string(), "*".to_string());
    map.insert("DeviceUseStatement.id".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.identifier".to_string(), "*".to_string());
    map.insert("DeviceUseStatement.implicitRules".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.indication".to_string(), "*".to_string());
    map.insert("DeviceUseStatement.language".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.meta".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.modifierExtension".to_string(), "*".to_string());
    map.insert("DeviceUseStatement.note".to_string(), "*".to_string());
    map.insert("DeviceUseStatement.recordedOn".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.source".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.status".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.subject".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.text".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.timingDateTime".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.timingPeriod".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.timingTiming".to_string(), "1".to_string());
    map.insert("DeviceUseStatement.whenUsed".to_string(), "1".to_string());
    map.insert("DiagnosticReport.basedOn".to_string(), "*".to_string());
    map.insert("DiagnosticReport.category".to_string(), "1".to_string());
    map.insert("DiagnosticReport.code".to_string(), "1".to_string());
    map.insert("DiagnosticReport.codedDiagnosis".to_string(), "*".to_string());
    map.insert("DiagnosticReport.conclusion".to_string(), "1".to_string());
    map.insert("DiagnosticReport.contained".to_string(), "*".to_string());
    map.insert("DiagnosticReport.context".to_string(), "1".to_string());
    map.insert("DiagnosticReport.effectiveDateTime".to_string(), "1".to_string());
    map.insert("DiagnosticReport.effectivePeriod".to_string(), "1".to_string());
    map.insert("DiagnosticReport.extension".to_string(), "*".to_string());
    map.insert("DiagnosticReport.id".to_string(), "1".to_string());
    map.insert("DiagnosticReport.identifier".to_string(), "*".to_string());
    map.insert("DiagnosticReport.image".to_string(), "*".to_string());
    map.insert("DiagnosticReport.image.comment".to_string(), "1".to_string());
    map.insert("DiagnosticReport.image.extension".to_string(), "*".to_string());
    map.insert("DiagnosticReport.image.id".to_string(), "1".to_string());
    map.insert("DiagnosticReport.image.link".to_string(), "1".to_string());
    map.insert("DiagnosticReport.image.modifierExtension".to_string(), "*".to_string());
    map.insert("DiagnosticReport.imagingStudy".to_string(), "*".to_string());
    map.insert("DiagnosticReport.implicitRules".to_string(), "1".to_string());
    map.insert("DiagnosticReport.issued".to_string(), "1".to_string());
    map.insert("DiagnosticReport.language".to_string(), "1".to_string());
    map.insert("DiagnosticReport.meta".to_string(), "1".to_string());
    map.insert("DiagnosticReport.modifierExtension".to_string(), "*".to_string());
    map.insert("DiagnosticReport.performer".to_string(), "*".to_string());
    map.insert("DiagnosticReport.performer.actor".to_string(), "1".to_string());
    map.insert("DiagnosticReport.performer.extension".to_string(), "*".to_string());
    map.insert("DiagnosticReport.performer.id".to_string(), "1".to_string());
    map.insert(
        "DiagnosticReport.performer.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("DiagnosticReport.performer.role".to_string(), "1".to_string());
    map.insert("DiagnosticReport.presentedForm".to_string(), "*".to_string());
    map.insert("DiagnosticReport.result".to_string(), "*".to_string());
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
    map.insert("DocumentManifest.content.extension".to_string(), "*".to_string());
    map.insert("DocumentManifest.content.id".to_string(), "1".to_string());
    map.insert(
        "DocumentManifest.content.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("DocumentManifest.content.pAttachment".to_string(), "1".to_string());
    map.insert("DocumentManifest.content.pReference".to_string(), "1".to_string());
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
    map.insert("DocumentReference.class".to_string(), "1".to_string());
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
    map.insert("DocumentReference.context.encounter".to_string(), "1".to_string());
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
        "DocumentReference.context.related.extension".to_string(),
        "*".to_string(),
    );
    map.insert("DocumentReference.context.related.id".to_string(), "1".to_string());
    map.insert(
        "DocumentReference.context.related.identifier".to_string(),
        "1".to_string(),
    );
    map.insert(
        "DocumentReference.context.related.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("DocumentReference.context.related.ref".to_string(), "1".to_string());
    map.insert(
        "DocumentReference.context.sourcePatientInfo".to_string(),
        "1".to_string(),
    );
    map.insert("DocumentReference.created".to_string(), "1".to_string());
    map.insert("DocumentReference.custodian".to_string(), "1".to_string());
    map.insert("DocumentReference.description".to_string(), "1".to_string());
    map.insert("DocumentReference.docStatus".to_string(), "1".to_string());
    map.insert("DocumentReference.extension".to_string(), "*".to_string());
    map.insert("DocumentReference.id".to_string(), "1".to_string());
    map.insert("DocumentReference.identifier".to_string(), "*".to_string());
    map.insert("DocumentReference.implicitRules".to_string(), "1".to_string());
    map.insert("DocumentReference.indexed".to_string(), "1".to_string());
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
    map.insert("Dosage.doseQuantity".to_string(), "1".to_string());
    map.insert("Dosage.doseRange".to_string(), "1".to_string());
    map.insert("Dosage.extension".to_string(), "*".to_string());
    map.insert("Dosage.id".to_string(), "1".to_string());
    map.insert("Dosage.maxDosePerAdministration".to_string(), "1".to_string());
    map.insert("Dosage.maxDosePerLifetime".to_string(), "1".to_string());
    map.insert("Dosage.maxDosePerPeriod".to_string(), "1".to_string());
    map.insert("Dosage.method".to_string(), "1".to_string());
    map.insert("Dosage.patientInstruction".to_string(), "1".to_string());
    map.insert("Dosage.rateQuantity".to_string(), "1".to_string());
    map.insert("Dosage.rateRange".to_string(), "1".to_string());
    map.insert("Dosage.rateRatio".to_string(), "1".to_string());
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
    map.insert(
        "ElementDefinition.binding.valueSetReference".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.binding.valueSetUri".to_string(), "1".to_string());
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
    map.insert("ElementDefinition.defaultValueCode".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.defaultValueCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.defaultValueCoding".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.defaultValueContactPoint".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.defaultValueCount".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueDate".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueDateTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueDecimal".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueDistance".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueDuration".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueHumanName".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueId".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueIdentifier".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueInstant".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueInteger".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueMarkdown".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueMeta".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueMoney".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueOid".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValuePeriod".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValuePositiveInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueQuantity".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueRange".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueRatio".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueReference".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueSampledData".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueSignature".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueString".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueTiming".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueUnsignedInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.defaultValueUri".to_string(), "1".to_string());
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
    map.insert("ElementDefinition.example.valueCode".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.example.valueCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.example.valueCoding".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.example.valueContactPoint".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.example.valueCount".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueDate".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueDateTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueDecimal".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueDistance".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueDuration".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueHumanName".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueId".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueIdentifier".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueInstant".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueInteger".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueMarkdown".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueMeta".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueMoney".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueOid".to_string(), "1".to_string());
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
        "ElementDefinition.example.valueSampledData".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.example.valueSignature".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueString".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.example.valueTiming".to_string(), "1".to_string());
    map.insert(
        "ElementDefinition.example.valueUnsignedInt".to_string(),
        "1".to_string(),
    );
    map.insert("ElementDefinition.example.valueUri".to_string(), "1".to_string());
    map.insert("ElementDefinition.extension".to_string(), "*".to_string());
    map.insert("ElementDefinition.fixedAddress".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedAge".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedAnnotation".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedAttachment".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedBase64Binary".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedBoolean".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedCode".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedCodeableConcept".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedCoding".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedContactPoint".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedCount".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedDate".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedDateTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedDecimal".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedDistance".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedDuration".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedHumanName".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedId".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedIdentifier".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedInstant".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedInteger".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedMarkdown".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedMeta".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedMoney".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedOid".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedPeriod".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedPositiveInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedQuantity".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedRange".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedRatio".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedReference".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedSampledData".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedSignature".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedString".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedTiming".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedUnsignedInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.fixedUri".to_string(), "1".to_string());
    map.insert("ElementDefinition.id".to_string(), "1".to_string());
    map.insert("ElementDefinition.isModifier".to_string(), "1".to_string());
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
    map.insert("ElementDefinition.mustSupport".to_string(), "1".to_string());
    map.insert("ElementDefinition.orderMeaning".to_string(), "1".to_string());
    map.insert("ElementDefinition.path".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternAddress".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternAge".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternAnnotation".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternAttachment".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternBase64Binary".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternBoolean".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternCode".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternCodeableConcept".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternCoding".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternContactPoint".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternCount".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternDate".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternDateTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternDecimal".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternDistance".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternDuration".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternHumanName".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternId".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternIdentifier".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternInstant".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternInteger".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternMarkdown".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternMeta".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternMoney".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternOid".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternPeriod".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternPositiveInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternQuantity".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternRange".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternRatio".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternReference".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternSampledData".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternSignature".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternString".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternTime".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternTiming".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternUnsignedInt".to_string(), "1".to_string());
    map.insert("ElementDefinition.patternUri".to_string(), "1".to_string());
    map.insert("ElementDefinition.representation".to_string(), "*".to_string());
    map.insert("ElementDefinition.requirements".to_string(), "1".to_string());
    map.insert("ElementDefinition.short".to_string(), "1".to_string());
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
    map.insert("ElementDefinition.type.profile".to_string(), "1".to_string());
    map.insert("ElementDefinition.type.targetProfile".to_string(), "1".to_string());
    map.insert("ElementDefinition.type.versioning".to_string(), "1".to_string());
    map.insert("EligibilityRequest.benefitCategory".to_string(), "1".to_string());
    map.insert("EligibilityRequest.benefitSubCategory".to_string(), "1".to_string());
    map.insert("EligibilityRequest.businessArrangement".to_string(), "1".to_string());
    map.insert("EligibilityRequest.contained".to_string(), "*".to_string());
    map.insert("EligibilityRequest.coverage".to_string(), "1".to_string());
    map.insert("EligibilityRequest.created".to_string(), "1".to_string());
    map.insert("EligibilityRequest.enterer".to_string(), "1".to_string());
    map.insert("EligibilityRequest.extension".to_string(), "*".to_string());
    map.insert("EligibilityRequest.facility".to_string(), "1".to_string());
    map.insert("EligibilityRequest.id".to_string(), "1".to_string());
    map.insert("EligibilityRequest.identifier".to_string(), "*".to_string());
    map.insert("EligibilityRequest.implicitRules".to_string(), "1".to_string());
    map.insert("EligibilityRequest.insurer".to_string(), "1".to_string());
    map.insert("EligibilityRequest.language".to_string(), "1".to_string());
    map.insert("EligibilityRequest.meta".to_string(), "1".to_string());
    map.insert("EligibilityRequest.modifierExtension".to_string(), "*".to_string());
    map.insert("EligibilityRequest.organization".to_string(), "1".to_string());
    map.insert("EligibilityRequest.patient".to_string(), "1".to_string());
    map.insert("EligibilityRequest.priority".to_string(), "1".to_string());
    map.insert("EligibilityRequest.provider".to_string(), "1".to_string());
    map.insert("EligibilityRequest.servicedDate".to_string(), "1".to_string());
    map.insert("EligibilityRequest.servicedPeriod".to_string(), "1".to_string());
    map.insert("EligibilityRequest.status".to_string(), "1".to_string());
    map.insert("EligibilityRequest.text".to_string(), "1".to_string());
    map.insert("EligibilityResponse.contained".to_string(), "*".to_string());
    map.insert("EligibilityResponse.created".to_string(), "1".to_string());
    map.insert("EligibilityResponse.disposition".to_string(), "1".to_string());
    map.insert("EligibilityResponse.error".to_string(), "*".to_string());
    map.insert("EligibilityResponse.error.code".to_string(), "1".to_string());
    map.insert("EligibilityResponse.error.extension".to_string(), "*".to_string());
    map.insert("EligibilityResponse.error.id".to_string(), "1".to_string());
    map.insert(
        "EligibilityResponse.error.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("EligibilityResponse.extension".to_string(), "*".to_string());
    map.insert("EligibilityResponse.form".to_string(), "1".to_string());
    map.insert("EligibilityResponse.id".to_string(), "1".to_string());
    map.insert("EligibilityResponse.identifier".to_string(), "*".to_string());
    map.insert("EligibilityResponse.implicitRules".to_string(), "1".to_string());
    map.insert("EligibilityResponse.inforce".to_string(), "1".to_string());
    map.insert("EligibilityResponse.insurance".to_string(), "*".to_string());
    map.insert(
        "EligibilityResponse.insurance.benefitBalance".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.category".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.excluded".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.financial".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.financial.allowedMoney"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.financial.allowedString"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.financial.allowedUnsignedInt"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.financial.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.financial.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.financial.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.financial.type".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.financial.usedMoney".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.financial.usedUnsignedInt"
            .to_string(),
        "1".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.name".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.network".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.subCategory".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.term".to_string(),
        "1".to_string(),
    );
    map.insert(
        "EligibilityResponse.insurance.benefitBalance.unit".to_string(),
        "1".to_string(),
    );
    map.insert("EligibilityResponse.insurance.contract".to_string(), "1".to_string());
    map.insert("EligibilityResponse.insurance.coverage".to_string(), "1".to_string());
    map.insert("EligibilityResponse.insurance.extension".to_string(), "*".to_string());
    map.insert("EligibilityResponse.insurance.id".to_string(), "1".to_string());
    map.insert(
        "EligibilityResponse.insurance.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("EligibilityResponse.insurer".to_string(), "1".to_string());
    map.insert("EligibilityResponse.language".to_string(), "1".to_string());
    map.insert("EligibilityResponse.meta".to_string(), "1".to_string());
    map.insert("EligibilityResponse.modifierExtension".to_string(), "*".to_string());
    map.insert("EligibilityResponse.outcome".to_string(), "1".to_string());
    map.insert("EligibilityResponse.request".to_string(), "1".to_string());
    map.insert("EligibilityResponse.requestOrganization".to_string(), "1".to_string());
    map.insert("EligibilityResponse.requestProvider".to_string(), "1".to_string());
    map.insert("EligibilityResponse.status".to_string(), "1".to_string());
    map.insert("EligibilityResponse.text".to_string(), "1".to_string());
    map.insert("Encounter.account".to_string(), "*".to_string());
    map.insert("Encounter.appointment".to_string(), "1".to_string());
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
    map.insert("Encounter.diagnosis.role".to_string(), "1".to_string());
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
    map.insert("Encounter.incomingReferral".to_string(), "*".to_string());
    map.insert("Encounter.language".to_string(), "1".to_string());
    map.insert("Encounter.length".to_string(), "1".to_string());
    map.insert("Encounter.location".to_string(), "*".to_string());
    map.insert("Encounter.location.extension".to_string(), "*".to_string());
    map.insert("Encounter.location.id".to_string(), "1".to_string());
    map.insert("Encounter.location.location".to_string(), "1".to_string());
    map.insert("Encounter.location.modifierExtension".to_string(), "*".to_string());
    map.insert("Encounter.location.period".to_string(), "1".to_string());
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
    map.insert("Encounter.reason".to_string(), "*".to_string());
    map.insert("Encounter.serviceProvider".to_string(), "1".to_string());
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
    map.insert("EnrollmentRequest.organization".to_string(), "1".to_string());
    map.insert("EnrollmentRequest.provider".to_string(), "1".to_string());
    map.insert("EnrollmentRequest.status".to_string(), "1".to_string());
    map.insert("EnrollmentRequest.subject".to_string(), "1".to_string());
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
    map.insert("EnrollmentResponse.requestOrganization".to_string(), "1".to_string());
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
    map.insert("ExpansionProfile.activeOnly".to_string(), "1".to_string());
    map.insert("ExpansionProfile.contact".to_string(), "*".to_string());
    map.insert("ExpansionProfile.contained".to_string(), "*".to_string());
    map.insert("ExpansionProfile.date".to_string(), "1".to_string());
    map.insert("ExpansionProfile.description".to_string(), "1".to_string());
    map.insert("ExpansionProfile.designation".to_string(), "1".to_string());
    map.insert("ExpansionProfile.designation.exclude".to_string(), "1".to_string());
    map.insert(
        "ExpansionProfile.designation.exclude.designation".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExpansionProfile.designation.exclude.designation.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExpansionProfile.designation.exclude.designation.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExpansionProfile.designation.exclude.designation.language".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExpansionProfile.designation.exclude.designation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExpansionProfile.designation.exclude.designation.use".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExpansionProfile.designation.exclude.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ExpansionProfile.designation.exclude.id".to_string(), "1".to_string());
    map.insert(
        "ExpansionProfile.designation.exclude.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExpansionProfile.designation.extension".to_string(), "*".to_string());
    map.insert("ExpansionProfile.designation.id".to_string(), "1".to_string());
    map.insert("ExpansionProfile.designation.include".to_string(), "1".to_string());
    map.insert(
        "ExpansionProfile.designation.include.designation".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExpansionProfile.designation.include.designation.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExpansionProfile.designation.include.designation.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExpansionProfile.designation.include.designation.language".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExpansionProfile.designation.include.designation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExpansionProfile.designation.include.designation.use".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExpansionProfile.designation.include.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ExpansionProfile.designation.include.id".to_string(), "1".to_string());
    map.insert(
        "ExpansionProfile.designation.include.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExpansionProfile.designation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExpansionProfile.displayLanguage".to_string(), "1".to_string());
    map.insert("ExpansionProfile.excludeNested".to_string(), "1".to_string());
    map.insert("ExpansionProfile.excludeNotForUI".to_string(), "1".to_string());
    map.insert("ExpansionProfile.excludePostCoordinated".to_string(), "1".to_string());
    map.insert("ExpansionProfile.excludedSystem".to_string(), "1".to_string());
    map.insert("ExpansionProfile.excludedSystem.extension".to_string(), "*".to_string());
    map.insert("ExpansionProfile.excludedSystem.id".to_string(), "1".to_string());
    map.insert(
        "ExpansionProfile.excludedSystem.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExpansionProfile.excludedSystem.system".to_string(), "1".to_string());
    map.insert("ExpansionProfile.excludedSystem.version".to_string(), "1".to_string());
    map.insert("ExpansionProfile.experimental".to_string(), "1".to_string());
    map.insert("ExpansionProfile.extension".to_string(), "*".to_string());
    map.insert("ExpansionProfile.fixedVersion".to_string(), "*".to_string());
    map.insert("ExpansionProfile.fixedVersion.extension".to_string(), "*".to_string());
    map.insert("ExpansionProfile.fixedVersion.id".to_string(), "1".to_string());
    map.insert("ExpansionProfile.fixedVersion.mode".to_string(), "1".to_string());
    map.insert(
        "ExpansionProfile.fixedVersion.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExpansionProfile.fixedVersion.system".to_string(), "1".to_string());
    map.insert("ExpansionProfile.fixedVersion.version".to_string(), "1".to_string());
    map.insert("ExpansionProfile.id".to_string(), "1".to_string());
    map.insert("ExpansionProfile.identifier".to_string(), "1".to_string());
    map.insert("ExpansionProfile.implicitRules".to_string(), "1".to_string());
    map.insert("ExpansionProfile.includeDefinition".to_string(), "1".to_string());
    map.insert("ExpansionProfile.includeDesignations".to_string(), "1".to_string());
    map.insert("ExpansionProfile.jurisdiction".to_string(), "*".to_string());
    map.insert("ExpansionProfile.language".to_string(), "1".to_string());
    map.insert("ExpansionProfile.limitedExpansion".to_string(), "1".to_string());
    map.insert("ExpansionProfile.meta".to_string(), "1".to_string());
    map.insert("ExpansionProfile.modifierExtension".to_string(), "*".to_string());
    map.insert("ExpansionProfile.name".to_string(), "1".to_string());
    map.insert("ExpansionProfile.publisher".to_string(), "1".to_string());
    map.insert("ExpansionProfile.status".to_string(), "1".to_string());
    map.insert("ExpansionProfile.text".to_string(), "1".to_string());
    map.insert("ExpansionProfile.url".to_string(), "1".to_string());
    map.insert("ExpansionProfile.useContext".to_string(), "*".to_string());
    map.insert("ExpansionProfile.version".to_string(), "1".to_string());
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
    map.insert("ExplanationOfBenefit.addItem.category".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.addItem.detail".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.addItem.detail.adjudication".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.category".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.addItem.detail.fee".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.addItem.detail.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.addItem.detail.modifier".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.noteNumber".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.revenue".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.addItem.detail.service".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.addItem.extension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.addItem.fee".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.addItem.id".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.addItem.modifier".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.addItem.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.addItem.noteNumber".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.addItem.revenue".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.addItem.sequenceLinkId".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.addItem.service".to_string(), "1".to_string());
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
    map.insert(
        "ExplanationOfBenefit.benefitBalance.subCategory".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.benefitBalance.term".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.benefitBalance.unit".to_string(), "1".to_string());
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
        "ExplanationOfBenefit.diagnosis.packageCode".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.diagnosis.sequence".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.diagnosis.type".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.disposition".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.employmentImpacted".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.enterer".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.extension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.facility".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.form".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.hospitalization".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.id".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.identifier".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.implicitRules".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.information".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.information.category".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.information.code".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.information.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.information.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.information.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.information.reason".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.information.sequence".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.information.timingDate".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.information.timingPeriod".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.information.valueAttachment".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.information.valueQuantity".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.information.valueReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.information.valueString".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.insurance".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.insurance.coverage".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.insurance.extension".to_string(), "*".to_string());
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
    map.insert("ExplanationOfBenefit.item.careTeamLinkId".to_string(), "*".to_string());
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
        "ExplanationOfBenefit.item.detail.programCode".to_string(),
        "*".to_string(),
    );
    map.insert("ExplanationOfBenefit.item.detail.quantity".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.detail.revenue".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.detail.sequence".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.detail.service".to_string(), "1".to_string());
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
        "ExplanationOfBenefit.item.detail.subDetail.service".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ExplanationOfBenefit.item.detail.subDetail.type".to_string(),
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
    map.insert("ExplanationOfBenefit.item.detail.type".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.detail.udi".to_string(), "*".to_string());
    map.insert(
        "ExplanationOfBenefit.item.detail.unitPrice".to_string(),
        "1".to_string(),
    );
    map.insert("ExplanationOfBenefit.item.diagnosisLinkId".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.item.encounter".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.item.extension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.item.factor".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.id".to_string(), "1".to_string());
    map.insert(
        "ExplanationOfBenefit.item.informationLinkId".to_string(),
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
    map.insert("ExplanationOfBenefit.item.procedureLinkId".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.item.programCode".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.item.quantity".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.revenue".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.sequence".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.service".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.servicedDate".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.servicedPeriod".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.item.subSite".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.item.udi".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.item.unitPrice".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.language".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.meta".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.modifierExtension".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.organization".to_string(), "1".to_string());
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
    map.insert("ExplanationOfBenefit.payee.resourceType".to_string(), "1".to_string());
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
    map.insert("ExplanationOfBenefit.precedence".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.prescription".to_string(), "1".to_string());
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
    map.insert("ExplanationOfBenefit.subType".to_string(), "*".to_string());
    map.insert("ExplanationOfBenefit.text".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.totalBenefit".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.totalCost".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.type".to_string(), "1".to_string());
    map.insert("ExplanationOfBenefit.unallocDeductable".to_string(), "1".to_string());
    map.insert("Extension.extension".to_string(), "*".to_string());
    map.insert("Extension.id".to_string(), "1".to_string());
    map.insert("Extension.url".to_string(), "1".to_string());
    map.insert("Extension.valueAddress".to_string(), "1".to_string());
    map.insert("Extension.valueAge".to_string(), "1".to_string());
    map.insert("Extension.valueAnnotation".to_string(), "1".to_string());
    map.insert("Extension.valueAttachment".to_string(), "1".to_string());
    map.insert("Extension.valueBase64Binary".to_string(), "1".to_string());
    map.insert("Extension.valueBoolean".to_string(), "1".to_string());
    map.insert("Extension.valueCode".to_string(), "1".to_string());
    map.insert("Extension.valueCodeableConcept".to_string(), "1".to_string());
    map.insert("Extension.valueCoding".to_string(), "1".to_string());
    map.insert("Extension.valueContactPoint".to_string(), "1".to_string());
    map.insert("Extension.valueCount".to_string(), "1".to_string());
    map.insert("Extension.valueDate".to_string(), "1".to_string());
    map.insert("Extension.valueDateTime".to_string(), "1".to_string());
    map.insert("Extension.valueDecimal".to_string(), "1".to_string());
    map.insert("Extension.valueDistance".to_string(), "1".to_string());
    map.insert("Extension.valueDuration".to_string(), "1".to_string());
    map.insert("Extension.valueHumanName".to_string(), "1".to_string());
    map.insert("Extension.valueId".to_string(), "1".to_string());
    map.insert("Extension.valueIdentifier".to_string(), "1".to_string());
    map.insert("Extension.valueInstant".to_string(), "1".to_string());
    map.insert("Extension.valueInteger".to_string(), "1".to_string());
    map.insert("Extension.valueMarkdown".to_string(), "1".to_string());
    map.insert("Extension.valueMeta".to_string(), "1".to_string());
    map.insert("Extension.valueMoney".to_string(), "1".to_string());
    map.insert("Extension.valueOid".to_string(), "1".to_string());
    map.insert("Extension.valuePeriod".to_string(), "1".to_string());
    map.insert("Extension.valuePositiveInt".to_string(), "1".to_string());
    map.insert("Extension.valueQuantity".to_string(), "1".to_string());
    map.insert("Extension.valueRange".to_string(), "1".to_string());
    map.insert("Extension.valueRatio".to_string(), "1".to_string());
    map.insert("Extension.valueReference".to_string(), "1".to_string());
    map.insert("Extension.valueSampledData".to_string(), "1".to_string());
    map.insert("Extension.valueSignature".to_string(), "1".to_string());
    map.insert("Extension.valueString".to_string(), "1".to_string());
    map.insert("Extension.valueTime".to_string(), "1".to_string());
    map.insert("Extension.valueTiming".to_string(), "1".to_string());
    map.insert("Extension.valueUnsignedInt".to_string(), "1".to_string());
    map.insert("Extension.valueUri".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.ageAge".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.ageRange".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.ageString".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.bornDate".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.bornPeriod".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.bornString".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.condition".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.condition.code".to_string(), "1".to_string());
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
    map.insert("FamilyMemberHistory.date".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.deceasedAge".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.deceasedBoolean".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.deceasedDate".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.deceasedRange".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.deceasedString".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.definition".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.estimatedAge".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.extension".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.gender".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.id".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.identifier".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.implicitRules".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.language".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.meta".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.modifierExtension".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.name".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.notDone".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.notDoneReason".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.note".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.patient".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.reasonCode".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.reasonReference".to_string(), "*".to_string());
    map.insert("FamilyMemberHistory.relationship".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.status".to_string(), "1".to_string());
    map.insert("FamilyMemberHistory.text".to_string(), "1".to_string());
    map.insert("Flag.author".to_string(), "1".to_string());
    map.insert("Flag.category".to_string(), "1".to_string());
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
    map.insert("Goal.meta".to_string(), "1".to_string());
    map.insert("Goal.modifierExtension".to_string(), "*".to_string());
    map.insert("Goal.note".to_string(), "*".to_string());
    map.insert("Goal.outcomeCode".to_string(), "*".to_string());
    map.insert("Goal.outcomeReference".to_string(), "*".to_string());
    map.insert("Goal.priority".to_string(), "1".to_string());
    map.insert("Goal.startCodeableConcept".to_string(), "1".to_string());
    map.insert("Goal.startDate".to_string(), "1".to_string());
    map.insert("Goal.status".to_string(), "1".to_string());
    map.insert("Goal.statusDate".to_string(), "1".to_string());
    map.insert("Goal.statusReason".to_string(), "1".to_string());
    map.insert("Goal.subject".to_string(), "1".to_string());
    map.insert("Goal.target".to_string(), "1".to_string());
    map.insert("Goal.target.detailCodeableConcept".to_string(), "1".to_string());
    map.insert("Goal.target.detailQuantity".to_string(), "1".to_string());
    map.insert("Goal.target.detailRange".to_string(), "1".to_string());
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
    map.insert("GraphDefinition.link.target.extension".to_string(), "*".to_string());
    map.insert("GraphDefinition.link.target.id".to_string(), "1".to_string());
    map.insert("GraphDefinition.link.target.link".to_string(), "*".to_string());
    map.insert(
        "GraphDefinition.link.target.modifierExtension".to_string(),
        "*".to_string(),
    );
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
    map.insert("Group.code".to_string(), "1".to_string());
    map.insert("Group.contained".to_string(), "*".to_string());
    map.insert("Group.extension".to_string(), "*".to_string());
    map.insert("Group.id".to_string(), "1".to_string());
    map.insert("Group.identifier".to_string(), "*".to_string());
    map.insert("Group.implicitRules".to_string(), "1".to_string());
    map.insert("Group.language".to_string(), "1".to_string());
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
    map.insert("GuidanceResponse.context".to_string(), "1".to_string());
    map.insert("GuidanceResponse.dataRequirement".to_string(), "*".to_string());
    map.insert("GuidanceResponse.evaluationMessage".to_string(), "*".to_string());
    map.insert("GuidanceResponse.extension".to_string(), "*".to_string());
    map.insert("GuidanceResponse.id".to_string(), "1".to_string());
    map.insert("GuidanceResponse.identifier".to_string(), "1".to_string());
    map.insert("GuidanceResponse.implicitRules".to_string(), "1".to_string());
    map.insert("GuidanceResponse.language".to_string(), "1".to_string());
    map.insert("GuidanceResponse.meta".to_string(), "1".to_string());
    map.insert("GuidanceResponse.modifierExtension".to_string(), "*".to_string());
    map.insert("GuidanceResponse.module".to_string(), "1".to_string());
    map.insert("GuidanceResponse.note".to_string(), "*".to_string());
    map.insert("GuidanceResponse.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("GuidanceResponse.outputParameters".to_string(), "1".to_string());
    map.insert("GuidanceResponse.performer".to_string(), "1".to_string());
    map.insert("GuidanceResponse.reasonCodeableConcept".to_string(), "1".to_string());
    map.insert("GuidanceResponse.reasonReference".to_string(), "1".to_string());
    map.insert("GuidanceResponse.requestId".to_string(), "1".to_string());
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
    map.insert("HealthcareService.category".to_string(), "1".to_string());
    map.insert("HealthcareService.characteristic".to_string(), "*".to_string());
    map.insert("HealthcareService.comment".to_string(), "1".to_string());
    map.insert("HealthcareService.contained".to_string(), "*".to_string());
    map.insert("HealthcareService.coverageArea".to_string(), "*".to_string());
    map.insert("HealthcareService.eligibility".to_string(), "1".to_string());
    map.insert("HealthcareService.eligibilityNote".to_string(), "1".to_string());
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
    map.insert("HealthcareService.programName".to_string(), "*".to_string());
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
    map.insert("ImagingManifest.author".to_string(), "1".to_string());
    map.insert("ImagingManifest.authoringTime".to_string(), "1".to_string());
    map.insert("ImagingManifest.contained".to_string(), "*".to_string());
    map.insert("ImagingManifest.description".to_string(), "1".to_string());
    map.insert("ImagingManifest.extension".to_string(), "*".to_string());
    map.insert("ImagingManifest.id".to_string(), "1".to_string());
    map.insert("ImagingManifest.identifier".to_string(), "1".to_string());
    map.insert("ImagingManifest.implicitRules".to_string(), "1".to_string());
    map.insert("ImagingManifest.language".to_string(), "1".to_string());
    map.insert("ImagingManifest.meta".to_string(), "1".to_string());
    map.insert("ImagingManifest.modifierExtension".to_string(), "*".to_string());
    map.insert("ImagingManifest.patient".to_string(), "1".to_string());
    map.insert("ImagingManifest.study".to_string(), "*".to_string());
    map.insert("ImagingManifest.study.endpoint".to_string(), "*".to_string());
    map.insert("ImagingManifest.study.extension".to_string(), "*".to_string());
    map.insert("ImagingManifest.study.id".to_string(), "1".to_string());
    map.insert("ImagingManifest.study.imagingStudy".to_string(), "1".to_string());
    map.insert("ImagingManifest.study.modifierExtension".to_string(), "*".to_string());
    map.insert("ImagingManifest.study.series".to_string(), "*".to_string());
    map.insert("ImagingManifest.study.series.endpoint".to_string(), "*".to_string());
    map.insert("ImagingManifest.study.series.extension".to_string(), "*".to_string());
    map.insert("ImagingManifest.study.series.id".to_string(), "1".to_string());
    map.insert("ImagingManifest.study.series.instance".to_string(), "*".to_string());
    map.insert(
        "ImagingManifest.study.series.instance.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ImagingManifest.study.series.instance.id".to_string(), "1".to_string());
    map.insert(
        "ImagingManifest.study.series.instance.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImagingManifest.study.series.instance.sopClass".to_string(),
        "1".to_string(),
    );
    map.insert("ImagingManifest.study.series.instance.uid".to_string(), "1".to_string());
    map.insert(
        "ImagingManifest.study.series.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ImagingManifest.study.series.uid".to_string(), "1".to_string());
    map.insert("ImagingManifest.study.uid".to_string(), "1".to_string());
    map.insert("ImagingManifest.text".to_string(), "1".to_string());
    map.insert("ImagingStudy.accession".to_string(), "1".to_string());
    map.insert("ImagingStudy.availability".to_string(), "1".to_string());
    map.insert("ImagingStudy.basedOn".to_string(), "*".to_string());
    map.insert("ImagingStudy.contained".to_string(), "*".to_string());
    map.insert("ImagingStudy.context".to_string(), "1".to_string());
    map.insert("ImagingStudy.description".to_string(), "1".to_string());
    map.insert("ImagingStudy.endpoint".to_string(), "*".to_string());
    map.insert("ImagingStudy.extension".to_string(), "*".to_string());
    map.insert("ImagingStudy.id".to_string(), "1".to_string());
    map.insert("ImagingStudy.identifier".to_string(), "*".to_string());
    map.insert("ImagingStudy.implicitRules".to_string(), "1".to_string());
    map.insert("ImagingStudy.interpreter".to_string(), "*".to_string());
    map.insert("ImagingStudy.language".to_string(), "1".to_string());
    map.insert("ImagingStudy.meta".to_string(), "1".to_string());
    map.insert("ImagingStudy.modalityList".to_string(), "*".to_string());
    map.insert("ImagingStudy.modifierExtension".to_string(), "*".to_string());
    map.insert("ImagingStudy.numberOfInstances".to_string(), "1".to_string());
    map.insert("ImagingStudy.numberOfSeries".to_string(), "1".to_string());
    map.insert("ImagingStudy.patient".to_string(), "1".to_string());
    map.insert("ImagingStudy.procedureCode".to_string(), "*".to_string());
    map.insert("ImagingStudy.procedureReference".to_string(), "*".to_string());
    map.insert("ImagingStudy.reason".to_string(), "1".to_string());
    map.insert("ImagingStudy.referrer".to_string(), "1".to_string());
    map.insert("ImagingStudy.series".to_string(), "*".to_string());
    map.insert("ImagingStudy.series.availability".to_string(), "1".to_string());
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
    map.insert("ImagingStudy.series.started".to_string(), "1".to_string());
    map.insert("ImagingStudy.series.uid".to_string(), "1".to_string());
    map.insert("ImagingStudy.started".to_string(), "1".to_string());
    map.insert("ImagingStudy.text".to_string(), "1".to_string());
    map.insert("ImagingStudy.uid".to_string(), "1".to_string());
    map.insert("Immunization.contained".to_string(), "*".to_string());
    map.insert("Immunization.date".to_string(), "1".to_string());
    map.insert("Immunization.doseQuantity".to_string(), "1".to_string());
    map.insert("Immunization.encounter".to_string(), "1".to_string());
    map.insert("Immunization.expirationDate".to_string(), "1".to_string());
    map.insert("Immunization.explanation".to_string(), "1".to_string());
    map.insert("Immunization.explanation.extension".to_string(), "*".to_string());
    map.insert("Immunization.explanation.id".to_string(), "1".to_string());
    map.insert(
        "Immunization.explanation.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Immunization.explanation.reason".to_string(), "*".to_string());
    map.insert("Immunization.explanation.reasonNotGiven".to_string(), "*".to_string());
    map.insert("Immunization.extension".to_string(), "*".to_string());
    map.insert("Immunization.id".to_string(), "1".to_string());
    map.insert("Immunization.identifier".to_string(), "*".to_string());
    map.insert("Immunization.implicitRules".to_string(), "1".to_string());
    map.insert("Immunization.language".to_string(), "1".to_string());
    map.insert("Immunization.location".to_string(), "1".to_string());
    map.insert("Immunization.lotNumber".to_string(), "1".to_string());
    map.insert("Immunization.manufacturer".to_string(), "1".to_string());
    map.insert("Immunization.meta".to_string(), "1".to_string());
    map.insert("Immunization.modifierExtension".to_string(), "*".to_string());
    map.insert("Immunization.notGiven".to_string(), "1".to_string());
    map.insert("Immunization.note".to_string(), "*".to_string());
    map.insert("Immunization.patient".to_string(), "1".to_string());
    map.insert("Immunization.practitioner".to_string(), "*".to_string());
    map.insert("Immunization.practitioner.actor".to_string(), "1".to_string());
    map.insert("Immunization.practitioner.extension".to_string(), "*".to_string());
    map.insert("Immunization.practitioner.id".to_string(), "1".to_string());
    map.insert(
        "Immunization.practitioner.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Immunization.practitioner.role".to_string(), "1".to_string());
    map.insert("Immunization.primarySource".to_string(), "1".to_string());
    map.insert("Immunization.reaction".to_string(), "*".to_string());
    map.insert("Immunization.reaction.date".to_string(), "1".to_string());
    map.insert("Immunization.reaction.detail".to_string(), "1".to_string());
    map.insert("Immunization.reaction.extension".to_string(), "*".to_string());
    map.insert("Immunization.reaction.id".to_string(), "1".to_string());
    map.insert("Immunization.reaction.modifierExtension".to_string(), "*".to_string());
    map.insert("Immunization.reaction.reported".to_string(), "1".to_string());
    map.insert("Immunization.reportOrigin".to_string(), "1".to_string());
    map.insert("Immunization.route".to_string(), "1".to_string());
    map.insert("Immunization.site".to_string(), "1".to_string());
    map.insert("Immunization.status".to_string(), "1".to_string());
    map.insert("Immunization.text".to_string(), "1".to_string());
    map.insert("Immunization.vaccinationProtocol".to_string(), "*".to_string());
    map.insert(
        "Immunization.vaccinationProtocol.authority".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Immunization.vaccinationProtocol.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Immunization.vaccinationProtocol.doseSequence".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Immunization.vaccinationProtocol.doseStatus".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Immunization.vaccinationProtocol.doseStatusReason".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Immunization.vaccinationProtocol.extension".to_string(),
        "*".to_string(),
    );
    map.insert("Immunization.vaccinationProtocol.id".to_string(), "1".to_string());
    map.insert(
        "Immunization.vaccinationProtocol.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Immunization.vaccinationProtocol.series".to_string(), "1".to_string());
    map.insert(
        "Immunization.vaccinationProtocol.seriesDoses".to_string(),
        "1".to_string(),
    );
    map.insert(
        "Immunization.vaccinationProtocol.targetDisease".to_string(),
        "*".to_string(),
    );
    map.insert("Immunization.vaccineCode".to_string(), "1".to_string());
    map.insert("ImmunizationRecommendation.contained".to_string(), "*".to_string());
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
        "ImmunizationRecommendation.recommendation.date".to_string(),
        "1".to_string(),
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
        "ImmunizationRecommendation.recommendation.doseNumber".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.extension".to_string(),
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
        "ImmunizationRecommendation.recommendation.protocol".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.protocol.authority".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.protocol.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.protocol.doseSequence".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.protocol.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.protocol.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.protocol.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "ImmunizationRecommendation.recommendation.protocol.series".to_string(),
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
        "1".to_string(),
    );
    map.insert("ImmunizationRecommendation.text".to_string(), "1".to_string());
    map.insert("ImplementationGuide.binary".to_string(), "*".to_string());
    map.insert("ImplementationGuide.contact".to_string(), "*".to_string());
    map.insert("ImplementationGuide.contained".to_string(), "*".to_string());
    map.insert("ImplementationGuide.copyright".to_string(), "1".to_string());
    map.insert("ImplementationGuide.date".to_string(), "1".to_string());
    map.insert("ImplementationGuide.dependency".to_string(), "*".to_string());
    map.insert("ImplementationGuide.dependency.extension".to_string(), "*".to_string());
    map.insert("ImplementationGuide.dependency.id".to_string(), "1".to_string());
    map.insert(
        "ImplementationGuide.dependency.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ImplementationGuide.dependency.type".to_string(), "1".to_string());
    map.insert("ImplementationGuide.dependency.uri".to_string(), "1".to_string());
    map.insert("ImplementationGuide.description".to_string(), "1".to_string());
    map.insert("ImplementationGuide.experimental".to_string(), "1".to_string());
    map.insert("ImplementationGuide.extension".to_string(), "*".to_string());
    map.insert("ImplementationGuide.fhirVersion".to_string(), "1".to_string());
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
    map.insert("ImplementationGuide.meta".to_string(), "1".to_string());
    map.insert("ImplementationGuide.modifierExtension".to_string(), "*".to_string());
    map.insert("ImplementationGuide.name".to_string(), "1".to_string());
    map.insert("ImplementationGuide.package".to_string(), "*".to_string());
    map.insert("ImplementationGuide.package.description".to_string(), "1".to_string());
    map.insert("ImplementationGuide.package.extension".to_string(), "*".to_string());
    map.insert("ImplementationGuide.package.id".to_string(), "1".to_string());
    map.insert(
        "ImplementationGuide.package.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ImplementationGuide.package.name".to_string(), "1".to_string());
    map.insert("ImplementationGuide.package.resource".to_string(), "*".to_string());
    map.insert(
        "ImplementationGuide.package.resource.acronym".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.package.resource.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.package.resource.example".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.package.resource.exampleFor".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.package.resource.extension".to_string(),
        "*".to_string(),
    );
    map.insert("ImplementationGuide.package.resource.id".to_string(), "1".to_string());
    map.insert(
        "ImplementationGuide.package.resource.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ImplementationGuide.package.resource.name".to_string(), "1".to_string());
    map.insert(
        "ImplementationGuide.package.resource.sourceReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "ImplementationGuide.package.resource.sourceUri".to_string(),
        "1".to_string(),
    );
    map.insert("ImplementationGuide.page".to_string(), "1".to_string());
    map.insert("ImplementationGuide.page.extension".to_string(), "*".to_string());
    map.insert("ImplementationGuide.page.format".to_string(), "1".to_string());
    map.insert("ImplementationGuide.page.id".to_string(), "1".to_string());
    map.insert("ImplementationGuide.page.kind".to_string(), "1".to_string());
    map.insert(
        "ImplementationGuide.page.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ImplementationGuide.page.package".to_string(), "*".to_string());
    map.insert("ImplementationGuide.page.page".to_string(), "*".to_string());
    map.insert("ImplementationGuide.page.source".to_string(), "1".to_string());
    map.insert("ImplementationGuide.page.title".to_string(), "1".to_string());
    map.insert("ImplementationGuide.page.type".to_string(), "*".to_string());
    map.insert("ImplementationGuide.publisher".to_string(), "1".to_string());
    map.insert("ImplementationGuide.status".to_string(), "1".to_string());
    map.insert("ImplementationGuide.text".to_string(), "1".to_string());
    map.insert("ImplementationGuide.url".to_string(), "1".to_string());
    map.insert("ImplementationGuide.useContext".to_string(), "*".to_string());
    map.insert("ImplementationGuide.version".to_string(), "1".to_string());
    map.insert("Library.approvalDate".to_string(), "1".to_string());
    map.insert("Library.contact".to_string(), "*".to_string());
    map.insert("Library.contained".to_string(), "*".to_string());
    map.insert("Library.content".to_string(), "*".to_string());
    map.insert("Library.contributor".to_string(), "*".to_string());
    map.insert("Library.copyright".to_string(), "1".to_string());
    map.insert("Library.dataRequirement".to_string(), "*".to_string());
    map.insert("Library.date".to_string(), "1".to_string());
    map.insert("Library.description".to_string(), "1".to_string());
    map.insert("Library.effectivePeriod".to_string(), "1".to_string());
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
    map.insert("Library.status".to_string(), "1".to_string());
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
    map.insert("Location.contained".to_string(), "*".to_string());
    map.insert("Location.description".to_string(), "1".to_string());
    map.insert("Location.endpoint".to_string(), "*".to_string());
    map.insert("Location.extension".to_string(), "*".to_string());
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
    map.insert("Location.type".to_string(), "1".to_string());
    map.insert("Measure.approvalDate".to_string(), "1".to_string());
    map.insert("Measure.clinicalRecommendationStatement".to_string(), "1".to_string());
    map.insert("Measure.compositeScoring".to_string(), "1".to_string());
    map.insert("Measure.contact".to_string(), "*".to_string());
    map.insert("Measure.contained".to_string(), "*".to_string());
    map.insert("Measure.contributor".to_string(), "*".to_string());
    map.insert("Measure.copyright".to_string(), "1".to_string());
    map.insert("Measure.date".to_string(), "1".to_string());
    map.insert("Measure.definition".to_string(), "*".to_string());
    map.insert("Measure.description".to_string(), "1".to_string());
    map.insert("Measure.disclaimer".to_string(), "1".to_string());
    map.insert("Measure.effectivePeriod".to_string(), "1".to_string());
    map.insert("Measure.experimental".to_string(), "1".to_string());
    map.insert("Measure.extension".to_string(), "*".to_string());
    map.insert("Measure.group".to_string(), "*".to_string());
    map.insert("Measure.group.description".to_string(), "1".to_string());
    map.insert("Measure.group.extension".to_string(), "*".to_string());
    map.insert("Measure.group.id".to_string(), "1".to_string());
    map.insert("Measure.group.identifier".to_string(), "1".to_string());
    map.insert("Measure.group.modifierExtension".to_string(), "*".to_string());
    map.insert("Measure.group.name".to_string(), "1".to_string());
    map.insert("Measure.group.population".to_string(), "*".to_string());
    map.insert("Measure.group.population.code".to_string(), "1".to_string());
    map.insert("Measure.group.population.criteria".to_string(), "1".to_string());
    map.insert("Measure.group.population.description".to_string(), "1".to_string());
    map.insert("Measure.group.population.extension".to_string(), "*".to_string());
    map.insert("Measure.group.population.id".to_string(), "1".to_string());
    map.insert("Measure.group.population.identifier".to_string(), "1".to_string());
    map.insert(
        "Measure.group.population.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Measure.group.population.name".to_string(), "1".to_string());
    map.insert("Measure.group.stratifier".to_string(), "*".to_string());
    map.insert("Measure.group.stratifier.criteria".to_string(), "1".to_string());
    map.insert("Measure.group.stratifier.extension".to_string(), "*".to_string());
    map.insert("Measure.group.stratifier.id".to_string(), "1".to_string());
    map.insert("Measure.group.stratifier.identifier".to_string(), "1".to_string());
    map.insert(
        "Measure.group.stratifier.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Measure.group.stratifier.path".to_string(), "1".to_string());
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
    map.insert("Measure.riskAdjustment".to_string(), "1".to_string());
    map.insert("Measure.scoring".to_string(), "1".to_string());
    map.insert("Measure.set".to_string(), "1".to_string());
    map.insert("Measure.status".to_string(), "1".to_string());
    map.insert("Measure.supplementalData".to_string(), "*".to_string());
    map.insert("Measure.supplementalData.criteria".to_string(), "1".to_string());
    map.insert("Measure.supplementalData.extension".to_string(), "*".to_string());
    map.insert("Measure.supplementalData.id".to_string(), "1".to_string());
    map.insert("Measure.supplementalData.identifier".to_string(), "1".to_string());
    map.insert(
        "Measure.supplementalData.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Measure.supplementalData.path".to_string(), "1".to_string());
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
    map.insert("MeasureReport.evaluatedResources".to_string(), "1".to_string());
    map.insert("MeasureReport.extension".to_string(), "*".to_string());
    map.insert("MeasureReport.group".to_string(), "*".to_string());
    map.insert("MeasureReport.group.extension".to_string(), "*".to_string());
    map.insert("MeasureReport.group.id".to_string(), "1".to_string());
    map.insert("MeasureReport.group.identifier".to_string(), "1".to_string());
    map.insert("MeasureReport.group.measureScore".to_string(), "1".to_string());
    map.insert("MeasureReport.group.modifierExtension".to_string(), "*".to_string());
    map.insert("MeasureReport.group.population".to_string(), "*".to_string());
    map.insert("MeasureReport.group.population.code".to_string(), "1".to_string());
    map.insert("MeasureReport.group.population.count".to_string(), "1".to_string());
    map.insert("MeasureReport.group.population.extension".to_string(), "*".to_string());
    map.insert("MeasureReport.group.population.id".to_string(), "1".to_string());
    map.insert("MeasureReport.group.population.identifier".to_string(), "1".to_string());
    map.insert(
        "MeasureReport.group.population.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MeasureReport.group.population.patients".to_string(), "1".to_string());
    map.insert("MeasureReport.group.stratifier".to_string(), "*".to_string());
    map.insert("MeasureReport.group.stratifier.extension".to_string(), "*".to_string());
    map.insert("MeasureReport.group.stratifier.id".to_string(), "1".to_string());
    map.insert("MeasureReport.group.stratifier.identifier".to_string(), "1".to_string());
    map.insert(
        "MeasureReport.group.stratifier.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MeasureReport.group.stratifier.stratum".to_string(), "*".to_string());
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
        "MeasureReport.group.stratifier.stratum.population.identifier".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.population.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.population.patients".to_string(),
        "1".to_string(),
    );
    map.insert(
        "MeasureReport.group.stratifier.stratum.value".to_string(),
        "1".to_string(),
    );
    map.insert("MeasureReport.id".to_string(), "1".to_string());
    map.insert("MeasureReport.identifier".to_string(), "1".to_string());
    map.insert("MeasureReport.implicitRules".to_string(), "1".to_string());
    map.insert("MeasureReport.language".to_string(), "1".to_string());
    map.insert("MeasureReport.measure".to_string(), "1".to_string());
    map.insert("MeasureReport.meta".to_string(), "1".to_string());
    map.insert("MeasureReport.modifierExtension".to_string(), "*".to_string());
    map.insert("MeasureReport.patient".to_string(), "1".to_string());
    map.insert("MeasureReport.period".to_string(), "1".to_string());
    map.insert("MeasureReport.reportingOrganization".to_string(), "1".to_string());
    map.insert("MeasureReport.status".to_string(), "1".to_string());
    map.insert("MeasureReport.text".to_string(), "1".to_string());
    map.insert("MeasureReport.type".to_string(), "1".to_string());
    map.insert("Media.basedOn".to_string(), "*".to_string());
    map.insert("Media.bodySite".to_string(), "1".to_string());
    map.insert("Media.contained".to_string(), "*".to_string());
    map.insert("Media.content".to_string(), "1".to_string());
    map.insert("Media.context".to_string(), "1".to_string());
    map.insert("Media.device".to_string(), "1".to_string());
    map.insert("Media.duration".to_string(), "1".to_string());
    map.insert("Media.extension".to_string(), "*".to_string());
    map.insert("Media.frames".to_string(), "1".to_string());
    map.insert("Media.height".to_string(), "1".to_string());
    map.insert("Media.id".to_string(), "1".to_string());
    map.insert("Media.identifier".to_string(), "*".to_string());
    map.insert("Media.implicitRules".to_string(), "1".to_string());
    map.insert("Media.language".to_string(), "1".to_string());
    map.insert("Media.meta".to_string(), "1".to_string());
    map.insert("Media.modifierExtension".to_string(), "*".to_string());
    map.insert("Media.note".to_string(), "*".to_string());
    map.insert("Media.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("Media.occurrencePeriod".to_string(), "1".to_string());
    map.insert("Media.operator".to_string(), "1".to_string());
    map.insert("Media.reasonCode".to_string(), "*".to_string());
    map.insert("Media.subject".to_string(), "1".to_string());
    map.insert("Media.subtype".to_string(), "1".to_string());
    map.insert("Media.text".to_string(), "1".to_string());
    map.insert("Media.type".to_string(), "1".to_string());
    map.insert("Media.view".to_string(), "1".to_string());
    map.insert("Media.width".to_string(), "1".to_string());
    map.insert("Medication.code".to_string(), "1".to_string());
    map.insert("Medication.contained".to_string(), "*".to_string());
    map.insert("Medication.extension".to_string(), "*".to_string());
    map.insert("Medication.form".to_string(), "1".to_string());
    map.insert("Medication.id".to_string(), "1".to_string());
    map.insert("Medication.image".to_string(), "*".to_string());
    map.insert("Medication.implicitRules".to_string(), "1".to_string());
    map.insert("Medication.ingredient".to_string(), "*".to_string());
    map.insert("Medication.ingredient.amount".to_string(), "1".to_string());
    map.insert("Medication.ingredient.extension".to_string(), "*".to_string());
    map.insert("Medication.ingredient.id".to_string(), "1".to_string());
    map.insert("Medication.ingredient.isActive".to_string(), "1".to_string());
    map.insert("Medication.ingredient.itemCodeableConcept".to_string(), "1".to_string());
    map.insert("Medication.ingredient.itemReference".to_string(), "1".to_string());
    map.insert("Medication.ingredient.modifierExtension".to_string(), "*".to_string());
    map.insert("Medication.isBrand".to_string(), "1".to_string());
    map.insert("Medication.isOverTheCounter".to_string(), "1".to_string());
    map.insert("Medication.language".to_string(), "1".to_string());
    map.insert("Medication.manufacturer".to_string(), "1".to_string());
    map.insert("Medication.meta".to_string(), "1".to_string());
    map.insert("Medication.modifierExtension".to_string(), "*".to_string());
    map.insert("Medication.package".to_string(), "1".to_string());
    map.insert("Medication.package.batch".to_string(), "*".to_string());
    map.insert("Medication.package.batch.expirationDate".to_string(), "1".to_string());
    map.insert("Medication.package.batch.extension".to_string(), "*".to_string());
    map.insert("Medication.package.batch.id".to_string(), "1".to_string());
    map.insert("Medication.package.batch.lotNumber".to_string(), "1".to_string());
    map.insert(
        "Medication.package.batch.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Medication.package.container".to_string(), "1".to_string());
    map.insert("Medication.package.content".to_string(), "*".to_string());
    map.insert("Medication.package.content.amount".to_string(), "1".to_string());
    map.insert("Medication.package.content.extension".to_string(), "*".to_string());
    map.insert("Medication.package.content.id".to_string(), "1".to_string());
    map.insert(
        "Medication.package.content.itemCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("Medication.package.content.itemReference".to_string(), "1".to_string());
    map.insert(
        "Medication.package.content.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Medication.package.extension".to_string(), "*".to_string());
    map.insert("Medication.package.id".to_string(), "1".to_string());
    map.insert("Medication.package.modifierExtension".to_string(), "*".to_string());
    map.insert("Medication.status".to_string(), "1".to_string());
    map.insert("Medication.text".to_string(), "1".to_string());
    map.insert("MedicationAdministration.category".to_string(), "1".to_string());
    map.insert("MedicationAdministration.contained".to_string(), "*".to_string());
    map.insert("MedicationAdministration.context".to_string(), "1".to_string());
    map.insert("MedicationAdministration.definition".to_string(), "*".to_string());
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
    map.insert("MedicationAdministration.notGiven".to_string(), "1".to_string());
    map.insert("MedicationAdministration.note".to_string(), "*".to_string());
    map.insert("MedicationAdministration.partOf".to_string(), "*".to_string());
    map.insert("MedicationAdministration.performer".to_string(), "*".to_string());
    map.insert("MedicationAdministration.performer.actor".to_string(), "1".to_string());
    map.insert(
        "MedicationAdministration.performer.extension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationAdministration.performer.id".to_string(), "1".to_string());
    map.insert(
        "MedicationAdministration.performer.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "MedicationAdministration.performer.onBehalfOf".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationAdministration.prescription".to_string(), "1".to_string());
    map.insert("MedicationAdministration.reasonCode".to_string(), "*".to_string());
    map.insert("MedicationAdministration.reasonNotGiven".to_string(), "*".to_string());
    map.insert("MedicationAdministration.reasonReference".to_string(), "*".to_string());
    map.insert("MedicationAdministration.status".to_string(), "1".to_string());
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
    map.insert(
        "MedicationDispense.medicationCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationDispense.medicationReference".to_string(), "1".to_string());
    map.insert("MedicationDispense.meta".to_string(), "1".to_string());
    map.insert("MedicationDispense.modifierExtension".to_string(), "*".to_string());
    map.insert("MedicationDispense.notDone".to_string(), "1".to_string());
    map.insert(
        "MedicationDispense.notDoneReasonCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("MedicationDispense.notDoneReasonReference".to_string(), "1".to_string());
    map.insert("MedicationDispense.note".to_string(), "*".to_string());
    map.insert("MedicationDispense.partOf".to_string(), "*".to_string());
    map.insert("MedicationDispense.performer".to_string(), "*".to_string());
    map.insert("MedicationDispense.performer.actor".to_string(), "1".to_string());
    map.insert("MedicationDispense.performer.extension".to_string(), "*".to_string());
    map.insert("MedicationDispense.performer.id".to_string(), "1".to_string());
    map.insert(
        "MedicationDispense.performer.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationDispense.performer.onBehalfOf".to_string(), "1".to_string());
    map.insert("MedicationDispense.quantity".to_string(), "1".to_string());
    map.insert("MedicationDispense.receiver".to_string(), "*".to_string());
    map.insert("MedicationDispense.status".to_string(), "1".to_string());
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
    map.insert("MedicationRequest.authoredOn".to_string(), "1".to_string());
    map.insert("MedicationRequest.basedOn".to_string(), "*".to_string());
    map.insert("MedicationRequest.category".to_string(), "1".to_string());
    map.insert("MedicationRequest.contained".to_string(), "*".to_string());
    map.insert("MedicationRequest.context".to_string(), "1".to_string());
    map.insert("MedicationRequest.definition".to_string(), "*".to_string());
    map.insert("MedicationRequest.detectedIssue".to_string(), "*".to_string());
    map.insert("MedicationRequest.dispenseRequest".to_string(), "1".to_string());
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
    map.insert("MedicationRequest.dosageInstruction".to_string(), "*".to_string());
    map.insert("MedicationRequest.eventHistory".to_string(), "*".to_string());
    map.insert("MedicationRequest.extension".to_string(), "*".to_string());
    map.insert("MedicationRequest.groupIdentifier".to_string(), "1".to_string());
    map.insert("MedicationRequest.id".to_string(), "1".to_string());
    map.insert("MedicationRequest.identifier".to_string(), "*".to_string());
    map.insert("MedicationRequest.implicitRules".to_string(), "1".to_string());
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
    map.insert("MedicationRequest.priorPrescription".to_string(), "1".to_string());
    map.insert("MedicationRequest.priority".to_string(), "1".to_string());
    map.insert("MedicationRequest.reasonCode".to_string(), "*".to_string());
    map.insert("MedicationRequest.reasonReference".to_string(), "*".to_string());
    map.insert("MedicationRequest.recorder".to_string(), "1".to_string());
    map.insert("MedicationRequest.requester".to_string(), "1".to_string());
    map.insert("MedicationRequest.requester.agent".to_string(), "1".to_string());
    map.insert("MedicationRequest.requester.extension".to_string(), "*".to_string());
    map.insert("MedicationRequest.requester.id".to_string(), "1".to_string());
    map.insert(
        "MedicationRequest.requester.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MedicationRequest.requester.onBehalfOf".to_string(), "1".to_string());
    map.insert("MedicationRequest.status".to_string(), "1".to_string());
    map.insert("MedicationRequest.subject".to_string(), "1".to_string());
    map.insert("MedicationRequest.substitution".to_string(), "1".to_string());
    map.insert("MedicationRequest.substitution.allowed".to_string(), "1".to_string());
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
    map.insert("MedicationStatement.reasonNotTaken".to_string(), "*".to_string());
    map.insert("MedicationStatement.reasonReference".to_string(), "*".to_string());
    map.insert("MedicationStatement.status".to_string(), "1".to_string());
    map.insert("MedicationStatement.subject".to_string(), "1".to_string());
    map.insert("MedicationStatement.taken".to_string(), "1".to_string());
    map.insert("MedicationStatement.text".to_string(), "1".to_string());
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
    map.insert("MessageDefinition.event".to_string(), "1".to_string());
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
    map.insert("MessageDefinition.id".to_string(), "1".to_string());
    map.insert("MessageDefinition.identifier".to_string(), "1".to_string());
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
    map.insert("MessageHeader.destination".to_string(), "*".to_string());
    map.insert("MessageHeader.destination.endpoint".to_string(), "1".to_string());
    map.insert("MessageHeader.destination.extension".to_string(), "*".to_string());
    map.insert("MessageHeader.destination.id".to_string(), "1".to_string());
    map.insert(
        "MessageHeader.destination.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("MessageHeader.destination.name".to_string(), "1".to_string());
    map.insert("MessageHeader.destination.target".to_string(), "1".to_string());
    map.insert("MessageHeader.enterer".to_string(), "1".to_string());
    map.insert("MessageHeader.event".to_string(), "1".to_string());
    map.insert("MessageHeader.extension".to_string(), "*".to_string());
    map.insert("MessageHeader.focus".to_string(), "*".to_string());
    map.insert("MessageHeader.id".to_string(), "1".to_string());
    map.insert("MessageHeader.implicitRules".to_string(), "1".to_string());
    map.insert("MessageHeader.language".to_string(), "1".to_string());
    map.insert("MessageHeader.meta".to_string(), "1".to_string());
    map.insert("MessageHeader.modifierExtension".to_string(), "*".to_string());
    map.insert("MessageHeader.reason".to_string(), "1".to_string());
    map.insert("MessageHeader.receiver".to_string(), "1".to_string());
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
    map.insert("MessageHeader.timestamp".to_string(), "1".to_string());
    map.insert("Meta.extension".to_string(), "*".to_string());
    map.insert("Meta.id".to_string(), "1".to_string());
    map.insert("Meta.lastUpdated".to_string(), "1".to_string());
    map.insert("Meta.profile".to_string(), "*".to_string());
    map.insert("Meta.security".to_string(), "*".to_string());
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
    map.insert("Money.code".to_string(), "1".to_string());
    map.insert("Money.comparator".to_string(), "1".to_string());
    map.insert("Money.extension".to_string(), "*".to_string());
    map.insert("Money.id".to_string(), "1".to_string());
    map.insert("Money.system".to_string(), "1".to_string());
    map.insert("Money.unit".to_string(), "1".to_string());
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
    map.insert("NamingSystem.replacedBy".to_string(), "1".to_string());
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
    map.insert("NutritionOrder.language".to_string(), "1".to_string());
    map.insert("NutritionOrder.meta".to_string(), "1".to_string());
    map.insert("NutritionOrder.modifierExtension".to_string(), "*".to_string());
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
    map.insert("Observation.comment".to_string(), "1".to_string());
    map.insert("Observation.component".to_string(), "*".to_string());
    map.insert("Observation.component.code".to_string(), "1".to_string());
    map.insert("Observation.component.dataAbsentReason".to_string(), "1".to_string());
    map.insert("Observation.component.extension".to_string(), "*".to_string());
    map.insert("Observation.component.id".to_string(), "1".to_string());
    map.insert("Observation.component.interpretation".to_string(), "1".to_string());
    map.insert("Observation.component.modifierExtension".to_string(), "*".to_string());
    map.insert("Observation.component.referenceRange".to_string(), "*".to_string());
    map.insert("Observation.component.valueAttachment".to_string(), "1".to_string());
    map.insert(
        "Observation.component.valueCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("Observation.component.valueDateTime".to_string(), "1".to_string());
    map.insert("Observation.component.valuePeriod".to_string(), "1".to_string());
    map.insert("Observation.component.valueQuantity".to_string(), "1".to_string());
    map.insert("Observation.component.valueRange".to_string(), "1".to_string());
    map.insert("Observation.component.valueRatio".to_string(), "1".to_string());
    map.insert("Observation.component.valueSampledData".to_string(), "1".to_string());
    map.insert("Observation.component.valueString".to_string(), "1".to_string());
    map.insert("Observation.component.valueTime".to_string(), "1".to_string());
    map.insert("Observation.contained".to_string(), "*".to_string());
    map.insert("Observation.context".to_string(), "1".to_string());
    map.insert("Observation.dataAbsentReason".to_string(), "1".to_string());
    map.insert("Observation.device".to_string(), "1".to_string());
    map.insert("Observation.effectiveDateTime".to_string(), "1".to_string());
    map.insert("Observation.effectivePeriod".to_string(), "1".to_string());
    map.insert("Observation.extension".to_string(), "*".to_string());
    map.insert("Observation.id".to_string(), "1".to_string());
    map.insert("Observation.identifier".to_string(), "*".to_string());
    map.insert("Observation.implicitRules".to_string(), "1".to_string());
    map.insert("Observation.interpretation".to_string(), "1".to_string());
    map.insert("Observation.issued".to_string(), "1".to_string());
    map.insert("Observation.language".to_string(), "1".to_string());
    map.insert("Observation.meta".to_string(), "1".to_string());
    map.insert("Observation.method".to_string(), "1".to_string());
    map.insert("Observation.modifierExtension".to_string(), "*".to_string());
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
    map.insert("Observation.related".to_string(), "*".to_string());
    map.insert("Observation.related.extension".to_string(), "*".to_string());
    map.insert("Observation.related.id".to_string(), "1".to_string());
    map.insert("Observation.related.modifierExtension".to_string(), "*".to_string());
    map.insert("Observation.related.target".to_string(), "1".to_string());
    map.insert("Observation.related.type".to_string(), "1".to_string());
    map.insert("Observation.specimen".to_string(), "1".to_string());
    map.insert("Observation.status".to_string(), "1".to_string());
    map.insert("Observation.subject".to_string(), "1".to_string());
    map.insert("Observation.text".to_string(), "1".to_string());
    map.insert("Observation.valueAttachment".to_string(), "1".to_string());
    map.insert("Observation.valueBoolean".to_string(), "1".to_string());
    map.insert("Observation.valueCodeableConcept".to_string(), "1".to_string());
    map.insert("Observation.valueDateTime".to_string(), "1".to_string());
    map.insert("Observation.valuePeriod".to_string(), "1".to_string());
    map.insert("Observation.valueQuantity".to_string(), "1".to_string());
    map.insert("Observation.valueRange".to_string(), "1".to_string());
    map.insert("Observation.valueRatio".to_string(), "1".to_string());
    map.insert("Observation.valueSampledData".to_string(), "1".to_string());
    map.insert("Observation.valueString".to_string(), "1".to_string());
    map.insert("Observation.valueTime".to_string(), "1".to_string());
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
    map.insert("OperationDefinition.idempotent".to_string(), "1".to_string());
    map.insert("OperationDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("OperationDefinition.instance".to_string(), "1".to_string());
    map.insert("OperationDefinition.jurisdiction".to_string(), "*".to_string());
    map.insert("OperationDefinition.kind".to_string(), "1".to_string());
    map.insert("OperationDefinition.language".to_string(), "1".to_string());
    map.insert("OperationDefinition.meta".to_string(), "1".to_string());
    map.insert("OperationDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("OperationDefinition.name".to_string(), "1".to_string());
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
        "OperationDefinition.parameter.binding.valueSetReference".to_string(),
        "1".to_string(),
    );
    map.insert(
        "OperationDefinition.parameter.binding.valueSetUri".to_string(),
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
    map.insert("OperationDefinition.parameter.profile".to_string(), "1".to_string());
    map.insert("OperationDefinition.parameter.searchType".to_string(), "1".to_string());
    map.insert("OperationDefinition.parameter.type".to_string(), "1".to_string());
    map.insert("OperationDefinition.parameter.use".to_string(), "1".to_string());
    map.insert("OperationDefinition.publisher".to_string(), "1".to_string());
    map.insert("OperationDefinition.purpose".to_string(), "1".to_string());
    map.insert("OperationDefinition.resource".to_string(), "*".to_string());
    map.insert("OperationDefinition.status".to_string(), "1".to_string());
    map.insert("OperationDefinition.system".to_string(), "1".to_string());
    map.insert("OperationDefinition.text".to_string(), "1".to_string());
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
    map.insert("Parameters.parameter.valueCode".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueCodeableConcept".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueCoding".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueContactPoint".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueCount".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueDate".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueDateTime".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueDecimal".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueDistance".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueDuration".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueHumanName".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueId".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueIdentifier".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueInstant".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueInteger".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueMarkdown".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueMeta".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueMoney".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueOid".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valuePeriod".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valuePositiveInt".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueQuantity".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueRange".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueRatio".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueReference".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueSampledData".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueSignature".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueString".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueTime".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueTiming".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueUnsignedInt".to_string(), "1".to_string());
    map.insert("Parameters.parameter.valueUri".to_string(), "1".to_string());
    map.insert("Patient.active".to_string(), "1".to_string());
    map.insert("Patient.address".to_string(), "*".to_string());
    map.insert("Patient.animal".to_string(), "1".to_string());
    map.insert("Patient.animal.breed".to_string(), "1".to_string());
    map.insert("Patient.animal.extension".to_string(), "*".to_string());
    map.insert("Patient.animal.genderStatus".to_string(), "1".to_string());
    map.insert("Patient.animal.id".to_string(), "1".to_string());
    map.insert("Patient.animal.modifierExtension".to_string(), "*".to_string());
    map.insert("Patient.animal.species".to_string(), "1".to_string());
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
    map.insert("PaymentNotice.contained".to_string(), "*".to_string());
    map.insert("PaymentNotice.created".to_string(), "1".to_string());
    map.insert("PaymentNotice.extension".to_string(), "*".to_string());
    map.insert("PaymentNotice.id".to_string(), "1".to_string());
    map.insert("PaymentNotice.identifier".to_string(), "*".to_string());
    map.insert("PaymentNotice.implicitRules".to_string(), "1".to_string());
    map.insert("PaymentNotice.language".to_string(), "1".to_string());
    map.insert("PaymentNotice.meta".to_string(), "1".to_string());
    map.insert("PaymentNotice.modifierExtension".to_string(), "*".to_string());
    map.insert("PaymentNotice.organization".to_string(), "1".to_string());
    map.insert("PaymentNotice.paymentStatus".to_string(), "1".to_string());
    map.insert("PaymentNotice.provider".to_string(), "1".to_string());
    map.insert("PaymentNotice.request".to_string(), "1".to_string());
    map.insert("PaymentNotice.response".to_string(), "1".to_string());
    map.insert("PaymentNotice.status".to_string(), "1".to_string());
    map.insert("PaymentNotice.statusDate".to_string(), "1".to_string());
    map.insert("PaymentNotice.target".to_string(), "1".to_string());
    map.insert("PaymentNotice.text".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.contained".to_string(), "*".to_string());
    map.insert("PaymentReconciliation.created".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail".to_string(), "*".to_string());
    map.insert("PaymentReconciliation.detail.amount".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail.date".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail.extension".to_string(), "*".to_string());
    map.insert("PaymentReconciliation.detail.id".to_string(), "1".to_string());
    map.insert(
        "PaymentReconciliation.detail.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("PaymentReconciliation.detail.payee".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail.request".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail.response".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail.submitter".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.detail.type".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.disposition".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.extension".to_string(), "*".to_string());
    map.insert("PaymentReconciliation.form".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.id".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.identifier".to_string(), "*".to_string());
    map.insert("PaymentReconciliation.implicitRules".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.language".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.meta".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.modifierExtension".to_string(), "*".to_string());
    map.insert("PaymentReconciliation.organization".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.outcome".to_string(), "1".to_string());
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
    map.insert("PaymentReconciliation.requestOrganization".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.requestProvider".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.status".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.text".to_string(), "1".to_string());
    map.insert("PaymentReconciliation.total".to_string(), "1".to_string());
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
        "PlanDefinition.action.condition.description".to_string(),
        "1".to_string(),
    );
    map.insert(
        "PlanDefinition.action.condition.expression".to_string(),
        "1".to_string(),
    );
    map.insert("PlanDefinition.action.condition.extension".to_string(), "*".to_string());
    map.insert("PlanDefinition.action.condition.id".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.condition.kind".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.condition.language".to_string(), "1".to_string());
    map.insert(
        "PlanDefinition.action.condition.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("PlanDefinition.action.definition".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.description".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.documentation".to_string(), "*".to_string());
    map.insert("PlanDefinition.action.dynamicValue".to_string(), "*".to_string());
    map.insert(
        "PlanDefinition.action.dynamicValue.description".to_string(),
        "1".to_string(),
    );
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
        "PlanDefinition.action.dynamicValue.language".to_string(),
        "1".to_string(),
    );
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
    map.insert("PlanDefinition.action.label".to_string(), "1".to_string());
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
    map.insert("PlanDefinition.action.textEquivalent".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.timingDateTime".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.timingDuration".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.timingPeriod".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.timingRange".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.timingTiming".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.title".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.transform".to_string(), "1".to_string());
    map.insert("PlanDefinition.action.triggerDefinition".to_string(), "*".to_string());
    map.insert("PlanDefinition.action.type".to_string(), "1".to_string());
    map.insert("PlanDefinition.approvalDate".to_string(), "1".to_string());
    map.insert("PlanDefinition.contact".to_string(), "*".to_string());
    map.insert("PlanDefinition.contained".to_string(), "*".to_string());
    map.insert("PlanDefinition.contributor".to_string(), "*".to_string());
    map.insert("PlanDefinition.copyright".to_string(), "1".to_string());
    map.insert("PlanDefinition.date".to_string(), "1".to_string());
    map.insert("PlanDefinition.description".to_string(), "1".to_string());
    map.insert("PlanDefinition.effectivePeriod".to_string(), "1".to_string());
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
    map.insert("PlanDefinition.status".to_string(), "1".to_string());
    map.insert("PlanDefinition.text".to_string(), "1".to_string());
    map.insert("PlanDefinition.title".to_string(), "1".to_string());
    map.insert("PlanDefinition.topic".to_string(), "*".to_string());
    map.insert("PlanDefinition.type".to_string(), "1".to_string());
    map.insert("PlanDefinition.url".to_string(), "1".to_string());
    map.insert("PlanDefinition.usage".to_string(), "1".to_string());
    map.insert("PlanDefinition.useContext".to_string(), "*".to_string());
    map.insert("PlanDefinition.version".to_string(), "1".to_string());
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
    map.insert("Procedure.basedOn".to_string(), "*".to_string());
    map.insert("Procedure.bodySite".to_string(), "*".to_string());
    map.insert("Procedure.category".to_string(), "1".to_string());
    map.insert("Procedure.code".to_string(), "1".to_string());
    map.insert("Procedure.complication".to_string(), "*".to_string());
    map.insert("Procedure.complicationDetail".to_string(), "*".to_string());
    map.insert("Procedure.contained".to_string(), "*".to_string());
    map.insert("Procedure.context".to_string(), "1".to_string());
    map.insert("Procedure.definition".to_string(), "*".to_string());
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
    map.insert("Procedure.language".to_string(), "1".to_string());
    map.insert("Procedure.location".to_string(), "1".to_string());
    map.insert("Procedure.meta".to_string(), "1".to_string());
    map.insert("Procedure.modifierExtension".to_string(), "*".to_string());
    map.insert("Procedure.notDone".to_string(), "1".to_string());
    map.insert("Procedure.notDoneReason".to_string(), "1".to_string());
    map.insert("Procedure.note".to_string(), "*".to_string());
    map.insert("Procedure.outcome".to_string(), "1".to_string());
    map.insert("Procedure.partOf".to_string(), "*".to_string());
    map.insert("Procedure.performedDateTime".to_string(), "1".to_string());
    map.insert("Procedure.performedPeriod".to_string(), "1".to_string());
    map.insert("Procedure.performer".to_string(), "*".to_string());
    map.insert("Procedure.performer.actor".to_string(), "1".to_string());
    map.insert("Procedure.performer.extension".to_string(), "*".to_string());
    map.insert("Procedure.performer.id".to_string(), "1".to_string());
    map.insert("Procedure.performer.modifierExtension".to_string(), "*".to_string());
    map.insert("Procedure.performer.onBehalfOf".to_string(), "1".to_string());
    map.insert("Procedure.performer.role".to_string(), "1".to_string());
    map.insert("Procedure.reasonCode".to_string(), "*".to_string());
    map.insert("Procedure.reasonReference".to_string(), "*".to_string());
    map.insert("Procedure.report".to_string(), "*".to_string());
    map.insert("Procedure.status".to_string(), "1".to_string());
    map.insert("Procedure.subject".to_string(), "1".to_string());
    map.insert("Procedure.text".to_string(), "1".to_string());
    map.insert("Procedure.usedCode".to_string(), "*".to_string());
    map.insert("Procedure.usedReference".to_string(), "*".to_string());
    map.insert("ProcedureRequest.asNeededBoolean".to_string(), "1".to_string());
    map.insert("ProcedureRequest.asNeededCodeableConcept".to_string(), "1".to_string());
    map.insert("ProcedureRequest.authoredOn".to_string(), "1".to_string());
    map.insert("ProcedureRequest.basedOn".to_string(), "*".to_string());
    map.insert("ProcedureRequest.bodySite".to_string(), "*".to_string());
    map.insert("ProcedureRequest.category".to_string(), "*".to_string());
    map.insert("ProcedureRequest.code".to_string(), "1".to_string());
    map.insert("ProcedureRequest.contained".to_string(), "*".to_string());
    map.insert("ProcedureRequest.context".to_string(), "1".to_string());
    map.insert("ProcedureRequest.definition".to_string(), "*".to_string());
    map.insert("ProcedureRequest.doNotPerform".to_string(), "1".to_string());
    map.insert("ProcedureRequest.extension".to_string(), "*".to_string());
    map.insert("ProcedureRequest.id".to_string(), "1".to_string());
    map.insert("ProcedureRequest.identifier".to_string(), "*".to_string());
    map.insert("ProcedureRequest.implicitRules".to_string(), "1".to_string());
    map.insert("ProcedureRequest.intent".to_string(), "1".to_string());
    map.insert("ProcedureRequest.language".to_string(), "1".to_string());
    map.insert("ProcedureRequest.meta".to_string(), "1".to_string());
    map.insert("ProcedureRequest.modifierExtension".to_string(), "*".to_string());
    map.insert("ProcedureRequest.note".to_string(), "*".to_string());
    map.insert("ProcedureRequest.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("ProcedureRequest.occurrencePeriod".to_string(), "1".to_string());
    map.insert("ProcedureRequest.occurrenceTiming".to_string(), "1".to_string());
    map.insert("ProcedureRequest.performer".to_string(), "1".to_string());
    map.insert("ProcedureRequest.performerType".to_string(), "1".to_string());
    map.insert("ProcedureRequest.priority".to_string(), "1".to_string());
    map.insert("ProcedureRequest.reasonCode".to_string(), "*".to_string());
    map.insert("ProcedureRequest.reasonReference".to_string(), "*".to_string());
    map.insert("ProcedureRequest.relevantHistory".to_string(), "*".to_string());
    map.insert("ProcedureRequest.replaces".to_string(), "*".to_string());
    map.insert("ProcedureRequest.requester".to_string(), "1".to_string());
    map.insert("ProcedureRequest.requester.agent".to_string(), "1".to_string());
    map.insert("ProcedureRequest.requester.extension".to_string(), "*".to_string());
    map.insert("ProcedureRequest.requester.id".to_string(), "1".to_string());
    map.insert(
        "ProcedureRequest.requester.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ProcedureRequest.requester.onBehalfOf".to_string(), "1".to_string());
    map.insert("ProcedureRequest.requisition".to_string(), "1".to_string());
    map.insert("ProcedureRequest.specimen".to_string(), "*".to_string());
    map.insert("ProcedureRequest.status".to_string(), "1".to_string());
    map.insert("ProcedureRequest.subject".to_string(), "1".to_string());
    map.insert("ProcedureRequest.supportingInfo".to_string(), "*".to_string());
    map.insert("ProcedureRequest.text".to_string(), "1".to_string());
    map.insert("ProcessRequest.action".to_string(), "1".to_string());
    map.insert("ProcessRequest.contained".to_string(), "*".to_string());
    map.insert("ProcessRequest.created".to_string(), "1".to_string());
    map.insert("ProcessRequest.exclude".to_string(), "*".to_string());
    map.insert("ProcessRequest.extension".to_string(), "*".to_string());
    map.insert("ProcessRequest.id".to_string(), "1".to_string());
    map.insert("ProcessRequest.identifier".to_string(), "*".to_string());
    map.insert("ProcessRequest.implicitRules".to_string(), "1".to_string());
    map.insert("ProcessRequest.include".to_string(), "*".to_string());
    map.insert("ProcessRequest.item".to_string(), "*".to_string());
    map.insert("ProcessRequest.item.extension".to_string(), "*".to_string());
    map.insert("ProcessRequest.item.id".to_string(), "1".to_string());
    map.insert("ProcessRequest.item.modifierExtension".to_string(), "*".to_string());
    map.insert("ProcessRequest.item.sequenceLinkId".to_string(), "1".to_string());
    map.insert("ProcessRequest.language".to_string(), "1".to_string());
    map.insert("ProcessRequest.meta".to_string(), "1".to_string());
    map.insert("ProcessRequest.modifierExtension".to_string(), "*".to_string());
    map.insert("ProcessRequest.nullify".to_string(), "1".to_string());
    map.insert("ProcessRequest.organization".to_string(), "1".to_string());
    map.insert("ProcessRequest.period".to_string(), "1".to_string());
    map.insert("ProcessRequest.provider".to_string(), "1".to_string());
    map.insert("ProcessRequest.reference".to_string(), "1".to_string());
    map.insert("ProcessRequest.request".to_string(), "1".to_string());
    map.insert("ProcessRequest.response".to_string(), "1".to_string());
    map.insert("ProcessRequest.status".to_string(), "1".to_string());
    map.insert("ProcessRequest.target".to_string(), "1".to_string());
    map.insert("ProcessRequest.text".to_string(), "1".to_string());
    map.insert("ProcessResponse.communicationRequest".to_string(), "*".to_string());
    map.insert("ProcessResponse.contained".to_string(), "*".to_string());
    map.insert("ProcessResponse.created".to_string(), "1".to_string());
    map.insert("ProcessResponse.disposition".to_string(), "1".to_string());
    map.insert("ProcessResponse.error".to_string(), "*".to_string());
    map.insert("ProcessResponse.extension".to_string(), "*".to_string());
    map.insert("ProcessResponse.form".to_string(), "1".to_string());
    map.insert("ProcessResponse.id".to_string(), "1".to_string());
    map.insert("ProcessResponse.identifier".to_string(), "*".to_string());
    map.insert("ProcessResponse.implicitRules".to_string(), "1".to_string());
    map.insert("ProcessResponse.language".to_string(), "1".to_string());
    map.insert("ProcessResponse.meta".to_string(), "1".to_string());
    map.insert("ProcessResponse.modifierExtension".to_string(), "*".to_string());
    map.insert("ProcessResponse.organization".to_string(), "1".to_string());
    map.insert("ProcessResponse.outcome".to_string(), "1".to_string());
    map.insert("ProcessResponse.processNote".to_string(), "*".to_string());
    map.insert("ProcessResponse.processNote.extension".to_string(), "*".to_string());
    map.insert("ProcessResponse.processNote.id".to_string(), "1".to_string());
    map.insert(
        "ProcessResponse.processNote.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ProcessResponse.processNote.text".to_string(), "1".to_string());
    map.insert("ProcessResponse.processNote.type".to_string(), "1".to_string());
    map.insert("ProcessResponse.request".to_string(), "1".to_string());
    map.insert("ProcessResponse.requestOrganization".to_string(), "1".to_string());
    map.insert("ProcessResponse.requestProvider".to_string(), "1".to_string());
    map.insert("ProcessResponse.status".to_string(), "1".to_string());
    map.insert("ProcessResponse.text".to_string(), "1".to_string());
    map.insert("Provenance.activity".to_string(), "1".to_string());
    map.insert("Provenance.agent".to_string(), "*".to_string());
    map.insert("Provenance.agent.extension".to_string(), "*".to_string());
    map.insert("Provenance.agent.id".to_string(), "1".to_string());
    map.insert("Provenance.agent.modifierExtension".to_string(), "*".to_string());
    map.insert("Provenance.agent.onBehalfOfReference".to_string(), "1".to_string());
    map.insert("Provenance.agent.onBehalfOfUri".to_string(), "1".to_string());
    map.insert("Provenance.agent.relatedAgentType".to_string(), "1".to_string());
    map.insert("Provenance.agent.role".to_string(), "*".to_string());
    map.insert("Provenance.agent.whoReference".to_string(), "1".to_string());
    map.insert("Provenance.agent.whoUri".to_string(), "1".to_string());
    map.insert("Provenance.contained".to_string(), "*".to_string());
    map.insert("Provenance.entity".to_string(), "*".to_string());
    map.insert("Provenance.entity.agent".to_string(), "*".to_string());
    map.insert("Provenance.entity.extension".to_string(), "*".to_string());
    map.insert("Provenance.entity.id".to_string(), "1".to_string());
    map.insert("Provenance.entity.modifierExtension".to_string(), "*".to_string());
    map.insert("Provenance.entity.role".to_string(), "1".to_string());
    map.insert("Provenance.entity.whatIdentifier".to_string(), "1".to_string());
    map.insert("Provenance.entity.whatReference".to_string(), "1".to_string());
    map.insert("Provenance.entity.whatUri".to_string(), "1".to_string());
    map.insert("Provenance.extension".to_string(), "*".to_string());
    map.insert("Provenance.id".to_string(), "1".to_string());
    map.insert("Provenance.implicitRules".to_string(), "1".to_string());
    map.insert("Provenance.language".to_string(), "1".to_string());
    map.insert("Provenance.location".to_string(), "1".to_string());
    map.insert("Provenance.meta".to_string(), "1".to_string());
    map.insert("Provenance.modifierExtension".to_string(), "*".to_string());
    map.insert("Provenance.period".to_string(), "1".to_string());
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
    map.insert("Questionnaire.description".to_string(), "1".to_string());
    map.insert("Questionnaire.effectivePeriod".to_string(), "1".to_string());
    map.insert("Questionnaire.experimental".to_string(), "1".to_string());
    map.insert("Questionnaire.extension".to_string(), "*".to_string());
    map.insert("Questionnaire.id".to_string(), "1".to_string());
    map.insert("Questionnaire.identifier".to_string(), "*".to_string());
    map.insert("Questionnaire.implicitRules".to_string(), "1".to_string());
    map.insert("Questionnaire.item".to_string(), "*".to_string());
    map.insert("Questionnaire.item.code".to_string(), "*".to_string());
    map.insert("Questionnaire.item.definition".to_string(), "1".to_string());
    map.insert("Questionnaire.item.enableWhen".to_string(), "*".to_string());
    map.insert(
        "Questionnaire.item.enableWhen.answerAttachment".to_string(),
        "1".to_string(),
    );
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
    map.insert("Questionnaire.item.enableWhen.answerUri".to_string(), "1".to_string());
    map.insert("Questionnaire.item.enableWhen.extension".to_string(), "*".to_string());
    map.insert("Questionnaire.item.enableWhen.hasAnswer".to_string(), "1".to_string());
    map.insert("Questionnaire.item.enableWhen.id".to_string(), "1".to_string());
    map.insert(
        "Questionnaire.item.enableWhen.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Questionnaire.item.enableWhen.question".to_string(), "1".to_string());
    map.insert("Questionnaire.item.extension".to_string(), "*".to_string());
    map.insert("Questionnaire.item.id".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initialAttachment".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initialBoolean".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initialCoding".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initialDate".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initialDateTime".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initialDecimal".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initialInteger".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initialQuantity".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initialReference".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initialString".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initialTime".to_string(), "1".to_string());
    map.insert("Questionnaire.item.initialUri".to_string(), "1".to_string());
    map.insert("Questionnaire.item.item".to_string(), "*".to_string());
    map.insert("Questionnaire.item.linkId".to_string(), "1".to_string());
    map.insert("Questionnaire.item.maxLength".to_string(), "1".to_string());
    map.insert("Questionnaire.item.modifierExtension".to_string(), "*".to_string());
    map.insert("Questionnaire.item.option".to_string(), "*".to_string());
    map.insert("Questionnaire.item.option.extension".to_string(), "*".to_string());
    map.insert("Questionnaire.item.option.id".to_string(), "1".to_string());
    map.insert(
        "Questionnaire.item.option.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("Questionnaire.item.option.valueCoding".to_string(), "1".to_string());
    map.insert("Questionnaire.item.option.valueDate".to_string(), "1".to_string());
    map.insert("Questionnaire.item.option.valueInteger".to_string(), "1".to_string());
    map.insert("Questionnaire.item.option.valueString".to_string(), "1".to_string());
    map.insert("Questionnaire.item.option.valueTime".to_string(), "1".to_string());
    map.insert("Questionnaire.item.options".to_string(), "1".to_string());
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
    map.insert("QuestionnaireResponse.context".to_string(), "1".to_string());
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
    map.insert("QuestionnaireResponse.item.subject".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.item.text".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.language".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.meta".to_string(), "1".to_string());
    map.insert("QuestionnaireResponse.modifierExtension".to_string(), "*".to_string());
    map.insert("QuestionnaireResponse.parent".to_string(), "*".to_string());
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
    map.insert("ReferralRequest.authoredOn".to_string(), "1".to_string());
    map.insert("ReferralRequest.basedOn".to_string(), "*".to_string());
    map.insert("ReferralRequest.contained".to_string(), "*".to_string());
    map.insert("ReferralRequest.context".to_string(), "1".to_string());
    map.insert("ReferralRequest.definition".to_string(), "*".to_string());
    map.insert("ReferralRequest.description".to_string(), "1".to_string());
    map.insert("ReferralRequest.extension".to_string(), "*".to_string());
    map.insert("ReferralRequest.groupIdentifier".to_string(), "1".to_string());
    map.insert("ReferralRequest.id".to_string(), "1".to_string());
    map.insert("ReferralRequest.identifier".to_string(), "*".to_string());
    map.insert("ReferralRequest.implicitRules".to_string(), "1".to_string());
    map.insert("ReferralRequest.intent".to_string(), "1".to_string());
    map.insert("ReferralRequest.language".to_string(), "1".to_string());
    map.insert("ReferralRequest.meta".to_string(), "1".to_string());
    map.insert("ReferralRequest.modifierExtension".to_string(), "*".to_string());
    map.insert("ReferralRequest.note".to_string(), "*".to_string());
    map.insert("ReferralRequest.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("ReferralRequest.occurrencePeriod".to_string(), "1".to_string());
    map.insert("ReferralRequest.priority".to_string(), "1".to_string());
    map.insert("ReferralRequest.reasonCode".to_string(), "*".to_string());
    map.insert("ReferralRequest.reasonReference".to_string(), "*".to_string());
    map.insert("ReferralRequest.recipient".to_string(), "*".to_string());
    map.insert("ReferralRequest.relevantHistory".to_string(), "*".to_string());
    map.insert("ReferralRequest.replaces".to_string(), "*".to_string());
    map.insert("ReferralRequest.requester".to_string(), "1".to_string());
    map.insert("ReferralRequest.requester.agent".to_string(), "1".to_string());
    map.insert("ReferralRequest.requester.extension".to_string(), "*".to_string());
    map.insert("ReferralRequest.requester.id".to_string(), "1".to_string());
    map.insert(
        "ReferralRequest.requester.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("ReferralRequest.requester.onBehalfOf".to_string(), "1".to_string());
    map.insert("ReferralRequest.serviceRequested".to_string(), "*".to_string());
    map.insert("ReferralRequest.specialty".to_string(), "1".to_string());
    map.insert("ReferralRequest.status".to_string(), "1".to_string());
    map.insert("ReferralRequest.subject".to_string(), "1".to_string());
    map.insert("ReferralRequest.supportingInfo".to_string(), "*".to_string());
    map.insert("ReferralRequest.text".to_string(), "1".to_string());
    map.insert("ReferralRequest.type".to_string(), "1".to_string());
    map.insert("RelatedArtifact.citation".to_string(), "1".to_string());
    map.insert("RelatedArtifact.display".to_string(), "1".to_string());
    map.insert("RelatedArtifact.document".to_string(), "1".to_string());
    map.insert("RelatedArtifact.extension".to_string(), "*".to_string());
    map.insert("RelatedArtifact.id".to_string(), "1".to_string());
    map.insert("RelatedArtifact.resource".to_string(), "1".to_string());
    map.insert("RelatedArtifact.type".to_string(), "1".to_string());
    map.insert("RelatedArtifact.url".to_string(), "1".to_string());
    map.insert("RelatedPerson.active".to_string(), "1".to_string());
    map.insert("RelatedPerson.address".to_string(), "*".to_string());
    map.insert("RelatedPerson.birthDate".to_string(), "1".to_string());
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
    map.insert("RelatedPerson.relationship".to_string(), "1".to_string());
    map.insert("RelatedPerson.telecom".to_string(), "*".to_string());
    map.insert("RelatedPerson.text".to_string(), "1".to_string());
    map.insert("RequestGroup.action".to_string(), "*".to_string());
    map.insert("RequestGroup.action.action".to_string(), "*".to_string());
    map.insert("RequestGroup.action.cardinalityBehavior".to_string(), "1".to_string());
    map.insert("RequestGroup.action.code".to_string(), "*".to_string());
    map.insert("RequestGroup.action.condition".to_string(), "*".to_string());
    map.insert("RequestGroup.action.condition.description".to_string(), "1".to_string());
    map.insert("RequestGroup.action.condition.expression".to_string(), "1".to_string());
    map.insert("RequestGroup.action.condition.extension".to_string(), "*".to_string());
    map.insert("RequestGroup.action.condition.id".to_string(), "1".to_string());
    map.insert("RequestGroup.action.condition.kind".to_string(), "1".to_string());
    map.insert("RequestGroup.action.condition.language".to_string(), "1".to_string());
    map.insert(
        "RequestGroup.action.condition.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("RequestGroup.action.description".to_string(), "1".to_string());
    map.insert("RequestGroup.action.documentation".to_string(), "*".to_string());
    map.insert("RequestGroup.action.extension".to_string(), "*".to_string());
    map.insert("RequestGroup.action.groupingBehavior".to_string(), "1".to_string());
    map.insert("RequestGroup.action.id".to_string(), "1".to_string());
    map.insert("RequestGroup.action.label".to_string(), "1".to_string());
    map.insert("RequestGroup.action.modifierExtension".to_string(), "*".to_string());
    map.insert("RequestGroup.action.participant".to_string(), "*".to_string());
    map.insert("RequestGroup.action.precheckBehavior".to_string(), "1".to_string());
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
    map.insert("RequestGroup.contained".to_string(), "*".to_string());
    map.insert("RequestGroup.context".to_string(), "1".to_string());
    map.insert("RequestGroup.definition".to_string(), "*".to_string());
    map.insert("RequestGroup.extension".to_string(), "*".to_string());
    map.insert("RequestGroup.groupIdentifier".to_string(), "1".to_string());
    map.insert("RequestGroup.id".to_string(), "1".to_string());
    map.insert("RequestGroup.identifier".to_string(), "*".to_string());
    map.insert("RequestGroup.implicitRules".to_string(), "1".to_string());
    map.insert("RequestGroup.intent".to_string(), "1".to_string());
    map.insert("RequestGroup.language".to_string(), "1".to_string());
    map.insert("RequestGroup.meta".to_string(), "1".to_string());
    map.insert("RequestGroup.modifierExtension".to_string(), "*".to_string());
    map.insert("RequestGroup.note".to_string(), "*".to_string());
    map.insert("RequestGroup.priority".to_string(), "1".to_string());
    map.insert("RequestGroup.reasonCodeableConcept".to_string(), "1".to_string());
    map.insert("RequestGroup.reasonReference".to_string(), "1".to_string());
    map.insert("RequestGroup.replaces".to_string(), "*".to_string());
    map.insert("RequestGroup.status".to_string(), "1".to_string());
    map.insert("RequestGroup.subject".to_string(), "1".to_string());
    map.insert("RequestGroup.text".to_string(), "1".to_string());
    map.insert("ResearchStudy.arm".to_string(), "*".to_string());
    map.insert("ResearchStudy.arm.code".to_string(), "1".to_string());
    map.insert("ResearchStudy.arm.description".to_string(), "1".to_string());
    map.insert("ResearchStudy.arm.extension".to_string(), "*".to_string());
    map.insert("ResearchStudy.arm.id".to_string(), "1".to_string());
    map.insert("ResearchStudy.arm.modifierExtension".to_string(), "*".to_string());
    map.insert("ResearchStudy.arm.name".to_string(), "1".to_string());
    map.insert("ResearchStudy.category".to_string(), "*".to_string());
    map.insert("ResearchStudy.contact".to_string(), "*".to_string());
    map.insert("ResearchStudy.contained".to_string(), "*".to_string());
    map.insert("ResearchStudy.description".to_string(), "1".to_string());
    map.insert("ResearchStudy.enrollment".to_string(), "*".to_string());
    map.insert("ResearchStudy.extension".to_string(), "*".to_string());
    map.insert("ResearchStudy.focus".to_string(), "*".to_string());
    map.insert("ResearchStudy.id".to_string(), "1".to_string());
    map.insert("ResearchStudy.identifier".to_string(), "*".to_string());
    map.insert("ResearchStudy.implicitRules".to_string(), "1".to_string());
    map.insert("ResearchStudy.jurisdiction".to_string(), "*".to_string());
    map.insert("ResearchStudy.keyword".to_string(), "*".to_string());
    map.insert("ResearchStudy.language".to_string(), "1".to_string());
    map.insert("ResearchStudy.meta".to_string(), "1".to_string());
    map.insert("ResearchStudy.modifierExtension".to_string(), "*".to_string());
    map.insert("ResearchStudy.note".to_string(), "*".to_string());
    map.insert("ResearchStudy.partOf".to_string(), "*".to_string());
    map.insert("ResearchStudy.period".to_string(), "1".to_string());
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
    map.insert("ResearchSubject.identifier".to_string(), "1".to_string());
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
    map.insert("RiskAssessment.comment".to_string(), "1".to_string());
    map.insert("RiskAssessment.condition".to_string(), "1".to_string());
    map.insert("RiskAssessment.contained".to_string(), "*".to_string());
    map.insert("RiskAssessment.context".to_string(), "1".to_string());
    map.insert("RiskAssessment.extension".to_string(), "*".to_string());
    map.insert("RiskAssessment.id".to_string(), "1".to_string());
    map.insert("RiskAssessment.identifier".to_string(), "1".to_string());
    map.insert("RiskAssessment.implicitRules".to_string(), "1".to_string());
    map.insert("RiskAssessment.language".to_string(), "1".to_string());
    map.insert("RiskAssessment.meta".to_string(), "1".to_string());
    map.insert("RiskAssessment.method".to_string(), "1".to_string());
    map.insert("RiskAssessment.mitigation".to_string(), "1".to_string());
    map.insert("RiskAssessment.modifierExtension".to_string(), "*".to_string());
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
    map.insert("RiskAssessment.reasonCodeableConcept".to_string(), "1".to_string());
    map.insert("RiskAssessment.reasonReference".to_string(), "1".to_string());
    map.insert("RiskAssessment.status".to_string(), "1".to_string());
    map.insert("RiskAssessment.subject".to_string(), "1".to_string());
    map.insert("RiskAssessment.text".to_string(), "1".to_string());
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
    map.insert("Schedule.serviceCategory".to_string(), "1".to_string());
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
    map.insert("Sequence.contained".to_string(), "*".to_string());
    map.insert("Sequence.coordinateSystem".to_string(), "1".to_string());
    map.insert("Sequence.device".to_string(), "1".to_string());
    map.insert("Sequence.extension".to_string(), "*".to_string());
    map.insert("Sequence.id".to_string(), "1".to_string());
    map.insert("Sequence.identifier".to_string(), "*".to_string());
    map.insert("Sequence.implicitRules".to_string(), "1".to_string());
    map.insert("Sequence.language".to_string(), "1".to_string());
    map.insert("Sequence.meta".to_string(), "1".to_string());
    map.insert("Sequence.modifierExtension".to_string(), "*".to_string());
    map.insert("Sequence.observedSeq".to_string(), "1".to_string());
    map.insert("Sequence.patient".to_string(), "1".to_string());
    map.insert("Sequence.performer".to_string(), "1".to_string());
    map.insert("Sequence.pointer".to_string(), "*".to_string());
    map.insert("Sequence.quality".to_string(), "*".to_string());
    map.insert("Sequence.quality.end".to_string(), "1".to_string());
    map.insert("Sequence.quality.extension".to_string(), "*".to_string());
    map.insert("Sequence.quality.fScore".to_string(), "1".to_string());
    map.insert("Sequence.quality.gtFP".to_string(), "1".to_string());
    map.insert("Sequence.quality.id".to_string(), "1".to_string());
    map.insert("Sequence.quality.method".to_string(), "1".to_string());
    map.insert("Sequence.quality.modifierExtension".to_string(), "*".to_string());
    map.insert("Sequence.quality.precision".to_string(), "1".to_string());
    map.insert("Sequence.quality.queryFP".to_string(), "1".to_string());
    map.insert("Sequence.quality.queryTP".to_string(), "1".to_string());
    map.insert("Sequence.quality.recall".to_string(), "1".to_string());
    map.insert("Sequence.quality.score".to_string(), "1".to_string());
    map.insert("Sequence.quality.standardSequence".to_string(), "1".to_string());
    map.insert("Sequence.quality.start".to_string(), "1".to_string());
    map.insert("Sequence.quality.truthFN".to_string(), "1".to_string());
    map.insert("Sequence.quality.truthTP".to_string(), "1".to_string());
    map.insert("Sequence.quality.type".to_string(), "1".to_string());
    map.insert("Sequence.quantity".to_string(), "1".to_string());
    map.insert("Sequence.readCoverage".to_string(), "1".to_string());
    map.insert("Sequence.referenceSeq".to_string(), "1".to_string());
    map.insert("Sequence.referenceSeq.chromosome".to_string(), "1".to_string());
    map.insert("Sequence.referenceSeq.extension".to_string(), "*".to_string());
    map.insert("Sequence.referenceSeq.genomeBuild".to_string(), "1".to_string());
    map.insert("Sequence.referenceSeq.id".to_string(), "1".to_string());
    map.insert("Sequence.referenceSeq.modifierExtension".to_string(), "*".to_string());
    map.insert("Sequence.referenceSeq.referenceSeqId".to_string(), "1".to_string());
    map.insert("Sequence.referenceSeq.referenceSeqPointer".to_string(), "1".to_string());
    map.insert("Sequence.referenceSeq.referenceSeqString".to_string(), "1".to_string());
    map.insert("Sequence.referenceSeq.strand".to_string(), "1".to_string());
    map.insert("Sequence.referenceSeq.windowEnd".to_string(), "1".to_string());
    map.insert("Sequence.referenceSeq.windowStart".to_string(), "1".to_string());
    map.insert("Sequence.repository".to_string(), "*".to_string());
    map.insert("Sequence.repository.datasetId".to_string(), "1".to_string());
    map.insert("Sequence.repository.extension".to_string(), "*".to_string());
    map.insert("Sequence.repository.id".to_string(), "1".to_string());
    map.insert("Sequence.repository.modifierExtension".to_string(), "*".to_string());
    map.insert("Sequence.repository.name".to_string(), "1".to_string());
    map.insert("Sequence.repository.readsetId".to_string(), "1".to_string());
    map.insert("Sequence.repository.type".to_string(), "1".to_string());
    map.insert("Sequence.repository.url".to_string(), "1".to_string());
    map.insert("Sequence.repository.variantsetId".to_string(), "1".to_string());
    map.insert("Sequence.specimen".to_string(), "1".to_string());
    map.insert("Sequence.text".to_string(), "1".to_string());
    map.insert("Sequence.type".to_string(), "1".to_string());
    map.insert("Sequence.variant".to_string(), "*".to_string());
    map.insert("Sequence.variant.cigar".to_string(), "1".to_string());
    map.insert("Sequence.variant.end".to_string(), "1".to_string());
    map.insert("Sequence.variant.extension".to_string(), "*".to_string());
    map.insert("Sequence.variant.id".to_string(), "1".to_string());
    map.insert("Sequence.variant.modifierExtension".to_string(), "*".to_string());
    map.insert("Sequence.variant.observedAllele".to_string(), "1".to_string());
    map.insert("Sequence.variant.referenceAllele".to_string(), "1".to_string());
    map.insert("Sequence.variant.start".to_string(), "1".to_string());
    map.insert("Sequence.variant.variantPointer".to_string(), "1".to_string());
    map.insert("ServiceDefinition.approvalDate".to_string(), "1".to_string());
    map.insert("ServiceDefinition.contact".to_string(), "*".to_string());
    map.insert("ServiceDefinition.contained".to_string(), "*".to_string());
    map.insert("ServiceDefinition.contributor".to_string(), "*".to_string());
    map.insert("ServiceDefinition.copyright".to_string(), "1".to_string());
    map.insert("ServiceDefinition.dataRequirement".to_string(), "*".to_string());
    map.insert("ServiceDefinition.date".to_string(), "1".to_string());
    map.insert("ServiceDefinition.description".to_string(), "1".to_string());
    map.insert("ServiceDefinition.effectivePeriod".to_string(), "1".to_string());
    map.insert("ServiceDefinition.experimental".to_string(), "1".to_string());
    map.insert("ServiceDefinition.extension".to_string(), "*".to_string());
    map.insert("ServiceDefinition.id".to_string(), "1".to_string());
    map.insert("ServiceDefinition.identifier".to_string(), "*".to_string());
    map.insert("ServiceDefinition.implicitRules".to_string(), "1".to_string());
    map.insert("ServiceDefinition.jurisdiction".to_string(), "*".to_string());
    map.insert("ServiceDefinition.language".to_string(), "1".to_string());
    map.insert("ServiceDefinition.lastReviewDate".to_string(), "1".to_string());
    map.insert("ServiceDefinition.meta".to_string(), "1".to_string());
    map.insert("ServiceDefinition.modifierExtension".to_string(), "*".to_string());
    map.insert("ServiceDefinition.name".to_string(), "1".to_string());
    map.insert("ServiceDefinition.operationDefinition".to_string(), "1".to_string());
    map.insert("ServiceDefinition.publisher".to_string(), "1".to_string());
    map.insert("ServiceDefinition.purpose".to_string(), "1".to_string());
    map.insert("ServiceDefinition.relatedArtifact".to_string(), "*".to_string());
    map.insert("ServiceDefinition.status".to_string(), "1".to_string());
    map.insert("ServiceDefinition.text".to_string(), "1".to_string());
    map.insert("ServiceDefinition.title".to_string(), "1".to_string());
    map.insert("ServiceDefinition.topic".to_string(), "*".to_string());
    map.insert("ServiceDefinition.trigger".to_string(), "*".to_string());
    map.insert("ServiceDefinition.url".to_string(), "1".to_string());
    map.insert("ServiceDefinition.usage".to_string(), "1".to_string());
    map.insert("ServiceDefinition.useContext".to_string(), "*".to_string());
    map.insert("ServiceDefinition.version".to_string(), "1".to_string());
    map.insert("Signature.blob".to_string(), "1".to_string());
    map.insert("Signature.contentType".to_string(), "1".to_string());
    map.insert("Signature.extension".to_string(), "*".to_string());
    map.insert("Signature.id".to_string(), "1".to_string());
    map.insert("Signature.onBehalfOfReference".to_string(), "1".to_string());
    map.insert("Signature.onBehalfOfUri".to_string(), "1".to_string());
    map.insert("Signature.type".to_string(), "*".to_string());
    map.insert("Signature.when".to_string(), "1".to_string());
    map.insert("Signature.whoReference".to_string(), "1".to_string());
    map.insert("Signature.whoUri".to_string(), "1".to_string());
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
    map.insert("Slot.serviceCategory".to_string(), "1".to_string());
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
    map.insert("Specimen.collection.extension".to_string(), "*".to_string());
    map.insert("Specimen.collection.id".to_string(), "1".to_string());
    map.insert("Specimen.collection.method".to_string(), "1".to_string());
    map.insert("Specimen.collection.modifierExtension".to_string(), "*".to_string());
    map.insert("Specimen.collection.quantity".to_string(), "1".to_string());
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
    map.insert("StructureDefinition.abstract".to_string(), "1".to_string());
    map.insert("StructureDefinition.baseDefinition".to_string(), "1".to_string());
    map.insert("StructureDefinition.contact".to_string(), "*".to_string());
    map.insert("StructureDefinition.contained".to_string(), "*".to_string());
    map.insert("StructureDefinition.context".to_string(), "*".to_string());
    map.insert("StructureDefinition.contextInvariant".to_string(), "*".to_string());
    map.insert("StructureDefinition.contextType".to_string(), "1".to_string());
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
        "StructureMap.group.rule.source.defaultValueContactPoint".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueCount".to_string(),
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
        "StructureMap.group.rule.source.defaultValueDuration".to_string(),
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
        "StructureMap.group.rule.source.defaultValueUnsignedInt".to_string(),
        "1".to_string(),
    );
    map.insert(
        "StructureMap.group.rule.source.defaultValueUri".to_string(),
        "1".to_string(),
    );
    map.insert("StructureMap.group.rule.source.element".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.source.extension".to_string(), "*".to_string());
    map.insert("StructureMap.group.rule.source.id".to_string(), "1".to_string());
    map.insert("StructureMap.group.rule.source.listMode".to_string(), "1".to_string());
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
    map.insert("Subscription.tag".to_string(), "*".to_string());
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
    map.insert("SupplyDelivery.basedOn".to_string(), "*".to_string());
    map.insert("SupplyDelivery.contained".to_string(), "*".to_string());
    map.insert("SupplyDelivery.destination".to_string(), "1".to_string());
    map.insert("SupplyDelivery.extension".to_string(), "*".to_string());
    map.insert("SupplyDelivery.id".to_string(), "1".to_string());
    map.insert("SupplyDelivery.identifier".to_string(), "1".to_string());
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
    map.insert("SupplyRequest.identifier".to_string(), "1".to_string());
    map.insert("SupplyRequest.implicitRules".to_string(), "1".to_string());
    map.insert("SupplyRequest.language".to_string(), "1".to_string());
    map.insert("SupplyRequest.meta".to_string(), "1".to_string());
    map.insert("SupplyRequest.modifierExtension".to_string(), "*".to_string());
    map.insert("SupplyRequest.occurrenceDateTime".to_string(), "1".to_string());
    map.insert("SupplyRequest.occurrencePeriod".to_string(), "1".to_string());
    map.insert("SupplyRequest.occurrenceTiming".to_string(), "1".to_string());
    map.insert("SupplyRequest.orderedItem".to_string(), "1".to_string());
    map.insert("SupplyRequest.orderedItem.extension".to_string(), "*".to_string());
    map.insert("SupplyRequest.orderedItem.id".to_string(), "1".to_string());
    map.insert(
        "SupplyRequest.orderedItem.itemCodeableConcept".to_string(),
        "1".to_string(),
    );
    map.insert("SupplyRequest.orderedItem.itemReference".to_string(), "1".to_string());
    map.insert(
        "SupplyRequest.orderedItem.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("SupplyRequest.orderedItem.quantity".to_string(), "1".to_string());
    map.insert("SupplyRequest.priority".to_string(), "1".to_string());
    map.insert("SupplyRequest.reasonCodeableConcept".to_string(), "1".to_string());
    map.insert("SupplyRequest.reasonReference".to_string(), "1".to_string());
    map.insert("SupplyRequest.requester".to_string(), "1".to_string());
    map.insert("SupplyRequest.requester.agent".to_string(), "1".to_string());
    map.insert("SupplyRequest.requester.extension".to_string(), "*".to_string());
    map.insert("SupplyRequest.requester.id".to_string(), "1".to_string());
    map.insert("SupplyRequest.requester.modifierExtension".to_string(), "*".to_string());
    map.insert("SupplyRequest.requester.onBehalfOf".to_string(), "1".to_string());
    map.insert("SupplyRequest.status".to_string(), "1".to_string());
    map.insert("SupplyRequest.supplier".to_string(), "*".to_string());
    map.insert("SupplyRequest.text".to_string(), "1".to_string());
    map.insert("Task.authoredOn".to_string(), "1".to_string());
    map.insert("Task.basedOn".to_string(), "*".to_string());
    map.insert("Task.businessStatus".to_string(), "1".to_string());
    map.insert("Task.code".to_string(), "1".to_string());
    map.insert("Task.contained".to_string(), "*".to_string());
    map.insert("Task.context".to_string(), "1".to_string());
    map.insert("Task.definitionReference".to_string(), "1".to_string());
    map.insert("Task.definitionUri".to_string(), "1".to_string());
    map.insert("Task.description".to_string(), "1".to_string());
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
    map.insert("Task.input.valueCode".to_string(), "1".to_string());
    map.insert("Task.input.valueCodeableConcept".to_string(), "1".to_string());
    map.insert("Task.input.valueCoding".to_string(), "1".to_string());
    map.insert("Task.input.valueContactPoint".to_string(), "1".to_string());
    map.insert("Task.input.valueCount".to_string(), "1".to_string());
    map.insert("Task.input.valueDate".to_string(), "1".to_string());
    map.insert("Task.input.valueDateTime".to_string(), "1".to_string());
    map.insert("Task.input.valueDecimal".to_string(), "1".to_string());
    map.insert("Task.input.valueDistance".to_string(), "1".to_string());
    map.insert("Task.input.valueDuration".to_string(), "1".to_string());
    map.insert("Task.input.valueHumanName".to_string(), "1".to_string());
    map.insert("Task.input.valueId".to_string(), "1".to_string());
    map.insert("Task.input.valueIdentifier".to_string(), "1".to_string());
    map.insert("Task.input.valueInstant".to_string(), "1".to_string());
    map.insert("Task.input.valueInteger".to_string(), "1".to_string());
    map.insert("Task.input.valueMarkdown".to_string(), "1".to_string());
    map.insert("Task.input.valueMeta".to_string(), "1".to_string());
    map.insert("Task.input.valueMoney".to_string(), "1".to_string());
    map.insert("Task.input.valueOid".to_string(), "1".to_string());
    map.insert("Task.input.valuePeriod".to_string(), "1".to_string());
    map.insert("Task.input.valuePositiveInt".to_string(), "1".to_string());
    map.insert("Task.input.valueQuantity".to_string(), "1".to_string());
    map.insert("Task.input.valueRange".to_string(), "1".to_string());
    map.insert("Task.input.valueRatio".to_string(), "1".to_string());
    map.insert("Task.input.valueReference".to_string(), "1".to_string());
    map.insert("Task.input.valueSampledData".to_string(), "1".to_string());
    map.insert("Task.input.valueSignature".to_string(), "1".to_string());
    map.insert("Task.input.valueString".to_string(), "1".to_string());
    map.insert("Task.input.valueTime".to_string(), "1".to_string());
    map.insert("Task.input.valueTiming".to_string(), "1".to_string());
    map.insert("Task.input.valueUnsignedInt".to_string(), "1".to_string());
    map.insert("Task.input.valueUri".to_string(), "1".to_string());
    map.insert("Task.intent".to_string(), "1".to_string());
    map.insert("Task.language".to_string(), "1".to_string());
    map.insert("Task.lastModified".to_string(), "1".to_string());
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
    map.insert("Task.output.valueCode".to_string(), "1".to_string());
    map.insert("Task.output.valueCodeableConcept".to_string(), "1".to_string());
    map.insert("Task.output.valueCoding".to_string(), "1".to_string());
    map.insert("Task.output.valueContactPoint".to_string(), "1".to_string());
    map.insert("Task.output.valueCount".to_string(), "1".to_string());
    map.insert("Task.output.valueDate".to_string(), "1".to_string());
    map.insert("Task.output.valueDateTime".to_string(), "1".to_string());
    map.insert("Task.output.valueDecimal".to_string(), "1".to_string());
    map.insert("Task.output.valueDistance".to_string(), "1".to_string());
    map.insert("Task.output.valueDuration".to_string(), "1".to_string());
    map.insert("Task.output.valueHumanName".to_string(), "1".to_string());
    map.insert("Task.output.valueId".to_string(), "1".to_string());
    map.insert("Task.output.valueIdentifier".to_string(), "1".to_string());
    map.insert("Task.output.valueInstant".to_string(), "1".to_string());
    map.insert("Task.output.valueInteger".to_string(), "1".to_string());
    map.insert("Task.output.valueMarkdown".to_string(), "1".to_string());
    map.insert("Task.output.valueMeta".to_string(), "1".to_string());
    map.insert("Task.output.valueMoney".to_string(), "1".to_string());
    map.insert("Task.output.valueOid".to_string(), "1".to_string());
    map.insert("Task.output.valuePeriod".to_string(), "1".to_string());
    map.insert("Task.output.valuePositiveInt".to_string(), "1".to_string());
    map.insert("Task.output.valueQuantity".to_string(), "1".to_string());
    map.insert("Task.output.valueRange".to_string(), "1".to_string());
    map.insert("Task.output.valueRatio".to_string(), "1".to_string());
    map.insert("Task.output.valueReference".to_string(), "1".to_string());
    map.insert("Task.output.valueSampledData".to_string(), "1".to_string());
    map.insert("Task.output.valueSignature".to_string(), "1".to_string());
    map.insert("Task.output.valueString".to_string(), "1".to_string());
    map.insert("Task.output.valueTime".to_string(), "1".to_string());
    map.insert("Task.output.valueTiming".to_string(), "1".to_string());
    map.insert("Task.output.valueUnsignedInt".to_string(), "1".to_string());
    map.insert("Task.output.valueUri".to_string(), "1".to_string());
    map.insert("Task.owner".to_string(), "1".to_string());
    map.insert("Task.partOf".to_string(), "*".to_string());
    map.insert("Task.performerType".to_string(), "*".to_string());
    map.insert("Task.priority".to_string(), "1".to_string());
    map.insert("Task.reason".to_string(), "1".to_string());
    map.insert("Task.relevantHistory".to_string(), "*".to_string());
    map.insert("Task.requester".to_string(), "1".to_string());
    map.insert("Task.requester.agent".to_string(), "1".to_string());
    map.insert("Task.requester.extension".to_string(), "*".to_string());
    map.insert("Task.requester.id".to_string(), "1".to_string());
    map.insert("Task.requester.modifierExtension".to_string(), "*".to_string());
    map.insert("Task.requester.onBehalfOf".to_string(), "1".to_string());
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
    map.insert("TestScript.rule".to_string(), "*".to_string());
    map.insert("TestScript.rule.extension".to_string(), "*".to_string());
    map.insert("TestScript.rule.id".to_string(), "1".to_string());
    map.insert("TestScript.rule.modifierExtension".to_string(), "*".to_string());
    map.insert("TestScript.rule.param".to_string(), "*".to_string());
    map.insert("TestScript.rule.param.extension".to_string(), "*".to_string());
    map.insert("TestScript.rule.param.id".to_string(), "1".to_string());
    map.insert("TestScript.rule.param.modifierExtension".to_string(), "*".to_string());
    map.insert("TestScript.rule.param.name".to_string(), "1".to_string());
    map.insert("TestScript.rule.param.value".to_string(), "1".to_string());
    map.insert("TestScript.rule.resource".to_string(), "1".to_string());
    map.insert("TestScript.ruleset".to_string(), "*".to_string());
    map.insert("TestScript.ruleset.extension".to_string(), "*".to_string());
    map.insert("TestScript.ruleset.id".to_string(), "1".to_string());
    map.insert("TestScript.ruleset.modifierExtension".to_string(), "*".to_string());
    map.insert("TestScript.ruleset.resource".to_string(), "1".to_string());
    map.insert("TestScript.ruleset.rule".to_string(), "*".to_string());
    map.insert("TestScript.ruleset.rule.extension".to_string(), "*".to_string());
    map.insert("TestScript.ruleset.rule.id".to_string(), "1".to_string());
    map.insert("TestScript.ruleset.rule.modifierExtension".to_string(), "*".to_string());
    map.insert("TestScript.ruleset.rule.param".to_string(), "*".to_string());
    map.insert("TestScript.ruleset.rule.param.extension".to_string(), "*".to_string());
    map.insert("TestScript.ruleset.rule.param.id".to_string(), "1".to_string());
    map.insert(
        "TestScript.ruleset.rule.param.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("TestScript.ruleset.rule.param.name".to_string(), "1".to_string());
    map.insert("TestScript.ruleset.rule.param.value".to_string(), "1".to_string());
    map.insert("TestScript.ruleset.rule.ruleId".to_string(), "1".to_string());
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
    map.insert("TestScript.setup.action.assert.rule".to_string(), "1".to_string());
    map.insert(
        "TestScript.setup.action.assert.rule.extension".to_string(),
        "*".to_string(),
    );
    map.insert("TestScript.setup.action.assert.rule.id".to_string(), "1".to_string());
    map.insert(
        "TestScript.setup.action.assert.rule.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("TestScript.setup.action.assert.rule.param".to_string(), "*".to_string());
    map.insert(
        "TestScript.setup.action.assert.rule.param.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.rule.param.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.rule.param.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.rule.param.name".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.rule.param.value".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.rule.ruleId".to_string(),
        "1".to_string(),
    );
    map.insert("TestScript.setup.action.assert.ruleset".to_string(), "1".to_string());
    map.insert(
        "TestScript.setup.action.assert.ruleset.extension".to_string(),
        "*".to_string(),
    );
    map.insert("TestScript.setup.action.assert.ruleset.id".to_string(), "1".to_string());
    map.insert(
        "TestScript.setup.action.assert.ruleset.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.ruleset.rule".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.ruleset.rule.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.ruleset.rule.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.ruleset.rule.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.ruleset.rule.param".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.ruleset.rule.param.extension".to_string(),
        "*".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.ruleset.rule.param.id".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.ruleset.rule.param.modifierExtension"
            .to_string(),
        "*".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.ruleset.rule.param.name".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.ruleset.rule.param.value".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.ruleset.rule.ruleId".to_string(),
        "1".to_string(),
    );
    map.insert(
        "TestScript.setup.action.assert.ruleset.rulesetId".to_string(),
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
    map.insert("TriggerDefinition.eventData".to_string(), "1".to_string());
    map.insert("TriggerDefinition.eventName".to_string(), "1".to_string());
    map.insert("TriggerDefinition.eventTimingDate".to_string(), "1".to_string());
    map.insert("TriggerDefinition.eventTimingDateTime".to_string(), "1".to_string());
    map.insert("TriggerDefinition.eventTimingReference".to_string(), "1".to_string());
    map.insert("TriggerDefinition.eventTimingTiming".to_string(), "1".to_string());
    map.insert("TriggerDefinition.extension".to_string(), "*".to_string());
    map.insert("TriggerDefinition.id".to_string(), "1".to_string());
    map.insert("TriggerDefinition.type".to_string(), "1".to_string());
    map.insert("UsageContext.code".to_string(), "1".to_string());
    map.insert("UsageContext.extension".to_string(), "*".to_string());
    map.insert("UsageContext.id".to_string(), "1".to_string());
    map.insert("UsageContext.valueCodeableConcept".to_string(), "1".to_string());
    map.insert("UsageContext.valueQuantity".to_string(), "1".to_string());
    map.insert("UsageContext.valueRange".to_string(), "1".to_string());
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
    map.insert("ValueSet.expansion.parameter.valueDecimal".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.parameter.valueInteger".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.parameter.valueString".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.parameter.valueUri".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.timestamp".to_string(), "1".to_string());
    map.insert("ValueSet.expansion.total".to_string(), "1".to_string());
    map.insert("ValueSet.experimental".to_string(), "1".to_string());
    map.insert("ValueSet.extensible".to_string(), "1".to_string());
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
    map.insert("VisionPrescription.contained".to_string(), "*".to_string());
    map.insert("VisionPrescription.dateWritten".to_string(), "1".to_string());
    map.insert("VisionPrescription.dispense".to_string(), "*".to_string());
    map.insert("VisionPrescription.dispense.add".to_string(), "1".to_string());
    map.insert("VisionPrescription.dispense.axis".to_string(), "1".to_string());
    map.insert("VisionPrescription.dispense.backCurve".to_string(), "1".to_string());
    map.insert("VisionPrescription.dispense.base".to_string(), "1".to_string());
    map.insert("VisionPrescription.dispense.brand".to_string(), "1".to_string());
    map.insert("VisionPrescription.dispense.color".to_string(), "1".to_string());
    map.insert("VisionPrescription.dispense.cylinder".to_string(), "1".to_string());
    map.insert("VisionPrescription.dispense.diameter".to_string(), "1".to_string());
    map.insert("VisionPrescription.dispense.duration".to_string(), "1".to_string());
    map.insert("VisionPrescription.dispense.extension".to_string(), "*".to_string());
    map.insert("VisionPrescription.dispense.eye".to_string(), "1".to_string());
    map.insert("VisionPrescription.dispense.id".to_string(), "1".to_string());
    map.insert(
        "VisionPrescription.dispense.modifierExtension".to_string(),
        "*".to_string(),
    );
    map.insert("VisionPrescription.dispense.note".to_string(), "*".to_string());
    map.insert("VisionPrescription.dispense.power".to_string(), "1".to_string());
    map.insert("VisionPrescription.dispense.prism".to_string(), "1".to_string());
    map.insert("VisionPrescription.dispense.product".to_string(), "1".to_string());
    map.insert("VisionPrescription.dispense.sphere".to_string(), "1".to_string());
    map.insert("VisionPrescription.encounter".to_string(), "1".to_string());
    map.insert("VisionPrescription.extension".to_string(), "*".to_string());
    map.insert("VisionPrescription.id".to_string(), "1".to_string());
    map.insert("VisionPrescription.identifier".to_string(), "*".to_string());
    map.insert("VisionPrescription.implicitRules".to_string(), "1".to_string());
    map.insert("VisionPrescription.language".to_string(), "1".to_string());
    map.insert("VisionPrescription.meta".to_string(), "1".to_string());
    map.insert("VisionPrescription.modifierExtension".to_string(), "*".to_string());
    map.insert("VisionPrescription.patient".to_string(), "1".to_string());
    map.insert("VisionPrescription.prescriber".to_string(), "1".to_string());
    map.insert("VisionPrescription.reasonCodeableConcept".to_string(), "1".to_string());
    map.insert("VisionPrescription.reasonReference".to_string(), "1".to_string());
    map.insert("VisionPrescription.status".to_string(), "1".to_string());
    map.insert("VisionPrescription.text".to_string(), "1".to_string());
    map.insert("base64Binary.extension".to_string(), "*".to_string());
    map.insert("base64Binary.id".to_string(), "1".to_string());
    map.insert("boolean.extension".to_string(), "*".to_string());
    map.insert("boolean.id".to_string(), "1".to_string());
    map.insert("code.extension".to_string(), "*".to_string());
    map.insert("code.id".to_string(), "1".to_string());
    map.insert("date.extension".to_string(), "*".to_string());
    map.insert("date.id".to_string(), "1".to_string());
    map.insert("dateTime.extension".to_string(), "*".to_string());
    map.insert("dateTime.id".to_string(), "1".to_string());
    map.insert("decimal.extension".to_string(), "*".to_string());
    map.insert("decimal.id".to_string(), "1".to_string());
    map.insert("id.extension".to_string(), "*".to_string());
    map.insert("id.id".to_string(), "1".to_string());
    map.insert("instant.extension".to_string(), "*".to_string());
    map.insert("instant.id".to_string(), "1".to_string());
    map.insert("integer.extension".to_string(), "*".to_string());
    map.insert("integer.id".to_string(), "1".to_string());
    map.insert("markdown.extension".to_string(), "*".to_string());
    map.insert("markdown.id".to_string(), "1".to_string());
    map.insert("oid.extension".to_string(), "*".to_string());
    map.insert("oid.id".to_string(), "1".to_string());
    map.insert("positiveInt.extension".to_string(), "*".to_string());
    map.insert("positiveInt.id".to_string(), "1".to_string());
    map.insert("string.extension".to_string(), "*".to_string());
    map.insert("string.id".to_string(), "1".to_string());
    map.insert("time.extension".to_string(), "*".to_string());
    map.insert("time.id".to_string(), "1".to_string());
    map.insert("unsignedInt.extension".to_string(), "*".to_string());
    map.insert("unsignedInt.id".to_string(), "1".to_string());
    map.insert("uri.extension".to_string(), "*".to_string());
    map.insert("uri.id".to_string(), "1".to_string());
    map.insert("uuid.extension".to_string(), "*".to_string());
    map.insert("uuid.id".to_string(), "1".to_string());
    map.insert("xhtml.extension".to_string(), "*".to_string());
    map.insert("xhtml.id".to_string(), "1".to_string());
    map
}
