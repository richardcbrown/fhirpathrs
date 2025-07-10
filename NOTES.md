## Notes

json! macro can panic, should probably not be used in library code

Questionnaire.item.item -> repeater map for types
Observation.component.value.value -> type mapping

is / as should also check if the type is a sub type

Quantity equality

Quantity conversion

Equality/Equivalent for complex objects -> respect Quantity conversion?

Resource.contained -> reset path to contained resource
Bundle.entry.resource -> correct type?

children() and descendants() - we currently return any nodes that are children/descendants, which seems to differ
with other implementations, there is discussion here https://chat.fhir.org/#narrow/channel/179298-fhirpath.2Ejs/topic/descendants.28.29
on the correct way to do this. This implementation also doesn't flatten array nodes out, which all other implementations appear to do.