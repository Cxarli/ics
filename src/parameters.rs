//! In the RFC 5545 specified parameters except for IANA and non-standard
//! parameters ("X"-prefix parameters).
//!
//! Parameters are key-value pairs which can specify a property in detail.
//!
//! For more information on the parameters, please refer to the specification [RFC 5545 3.2. Property Parameters](https://tools.ietf.org/html/rfc5545#section-3.2).
use components::Parameter;
use std::borrow::Cow;

parameter_builder!(AltRep, "ALTREP");
parameter_builder!(CN, "CN");
parameter_builder!(CUType, "CUTYPE");
parameter_builder!(DelegatedFrom, "DELEGATED-FROM");
parameter_builder!(DelegatedTo, "DELEGATED-TO");
parameter_builder!(Dir, "DIR");
parameter_builder!(Encoding, "ENCODING");
parameter_builder!(FmtType, "FMTTYPE");
parameter_builder!(FBType, "FBTYPE");
parameter_builder!(Language, "LANGUAGE");
parameter_builder!(Member, "MEMBER");
parameter_builder!(PartStat, "PARTSTAT");
parameter_builder!(Range, "RANGE");
parameter_builder!(Related, "RELATED");
parameter_builder!(RelType, "RELTYPE");
parameter_builder!(Role, "ROLE");
parameter_builder!(RSVP, "RSVP");
parameter_builder!(SentBy, "SENT-BY");
parameter_builder!(TZID, "TZID");
parameter_builder!(Value, "VALUE");

impl_default_parameter!(AltRep);
impl_default_parameter!(CN);
impl_default_parameter!(CUType, "INDIVIDUAL");
impl_default_parameter!(DelegatedFrom);
impl_default_parameter!(DelegatedTo);
impl_default_parameter!(Dir);
impl_default_parameter!(Encoding, "8BIT");
impl_default_parameter!(FmtType);
impl_default_parameter!(FBType, "BUSY");
impl_default_parameter!(Language);
impl_default_parameter!(Member);
impl_default_parameter!(PartStat, "NEEDS-ACTION");
impl_default_parameter!(Range, "THISANDFUTURE");
impl_default_parameter!(Related, "START");
impl_default_parameter!(RelType, "PARENT");
impl_default_parameter!(Role, "REQ-PARTICIPANT");
impl_default_parameter!(RSVP, "FALSE");
impl_default_parameter!(SentBy);
impl_default_parameter!(TZID);
impl_default_parameter!(Value);
