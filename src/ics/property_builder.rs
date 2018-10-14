use components::{Parameter, Parameters, Property};
use std::collections::BTreeMap;

property_builder!(CalScale, String::from("CALSCALE"));
property_builder!(Method, String::from("METHOD"));
property_builder!(ProdID, String::from("PRODID"));
property_builder!(Version, String::from("VERSION"));
property_builder!(Attach, String::from("ATTACH"));
property_builder!(Categories, String::from("CATEGORIES"));
property_builder!(Class, String::from("CLASS"));
property_builder!(Comment, String::from("COMMENT"));
property_builder!(Description, String::from("DESCRIPTION"));
property_builder!(Geo, String::from("GEO"));
property_builder!(Location, String::from("LOCATION"));
property_builder!(PercentComplete, String::from("PERCENT-COMPLETE"));
property_builder!(Priority, String::from("PRIORITY"));
property_builder!(Resources, String::from("RESOURCES"));
property_builder!(Status, String::from("STATUS"));
property_builder!(Summary, String::from("SUMMARY"));
property_builder!(Completed, String::from("COMPLETED"));
property_builder!(DtEnd, String::from("DTEND"));
property_builder!(Due, String::from("DUE"));
property_builder!(DtStart, String::from("DTSTART"));
property_builder!(Duration, String::from("DURATION"));
property_builder!(FreeBusy, String::from("FREEBUSY"));
property_builder!(Transp, String::from("TRANSP"));
property_builder!(TzID, String::from("TZID"));
property_builder!(TzName, String::from("TZNAME"));
property_builder!(TzOffsetFrom, String::from("TZOFFSETFROM"));
property_builder!(TzOffsetTo, String::from("TZOFFSETTO"));
property_builder!(TzURL, String::from("TZURL"));
property_builder!(Attendee, String::from("ATTENDEE"));
property_builder!(Contact, String::from("CONTACT"));
property_builder!(Organizer, String::from("ORGANIZER"));
property_builder!(RecurrenceID, String::from("RECURRENCE-ID"));
property_builder!(RelatedTo, String::from("RELATED-TO"));
property_builder!(URL, String::from("URL"));
property_builder!(UID, String::from("UID"));
property_builder!(ExDate, String::from("EXDATE"));
property_builder!(RDate, String::from("RDATE"));
property_builder!(RRule, String::from("RRULE"));
property_builder!(Action, String::from("ACTION"));
property_builder!(Repeat, String::from("REPEAT"));
property_builder!(Trigger, String::from("TRIGGER"));
property_builder!(Created, String::from("CREATED"));
property_builder!(DtStamp, String::from("DTSTAMP"));
property_builder!(LastModified, String::from("LAST-MODIFIED"));
property_builder!(Sequence, String::from("SEQUENCE"));
property_builder!(RequestStatus, String::from("REQUEST-STATUS"));

impl_default_property!(CalScale, String::from("GREGORIAN"));
impl_default_property!(Method);
impl_default_property!(ProdID);
impl_default_property!(Version);
impl_default_property!(Attach);
impl_default_property!(Categories);
impl_default_property!(Class, String::from("PUBLIC"));
impl_default_property!(Comment);
impl_default_property!(Description);
impl_default_property!(Geo);
impl_default_property!(Location);
impl_default_property!(PercentComplete);
impl_default_property!(Priority, String::from("0"));
impl_default_property!(Resources);
impl_default_property!(Status);
impl_default_property!(Summary);
impl_default_property!(Completed);
impl_default_property!(DtEnd);
impl_default_property!(Due);
impl_default_property!(DtStart);
impl_default_property!(Duration);
impl_default_property!(FreeBusy);
impl_default_property!(Transp, String::from("OPAQUE"));
impl_default_property!(TzID);
impl_default_property!(TzName);
impl_default_property!(TzOffsetFrom);
impl_default_property!(TzOffsetTo);
impl_default_property!(TzURL);
impl_default_property!(Attendee);
impl_default_property!(Contact);
impl_default_property!(Organizer);
impl_default_property!(RecurrenceID);
impl_default_property!(RelatedTo);
impl_default_property!(URL);
impl_default_property!(UID);
impl_default_property!(ExDate);
impl_default_property!(RDate);
impl_default_property!(RRule);
impl_default_property!(Action);
impl_default_property!(Repeat, String::from("0"));
impl_default_property!(Trigger);
impl_default_property!(Created);
impl_default_property!(DtStamp);
impl_default_property!(LastModified);
impl_default_property!(Sequence, String::from("0"));
impl_default_property!(RequestStatus);
